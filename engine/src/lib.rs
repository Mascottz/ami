use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

const LANGUAGE_TABLES: &str = include_str!("../../language/tables.json");
const DEMO_LEXICON: &str = include_str!("../../data/demo-lexicon.tsv");
const MORPHOLOGY: &str = include_str!("../../data/morphology.tsv");
const WIKI_FREQUENCIES: &str = include_str!("../../data/wiki-frequencies.tsv");
const MAX_INPUT_BYTES: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineError(String);

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for EngineError {}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LanguageFile {
    pub languages: Vec<LanguageTable>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LanguageTable {
    pub code: String,
    pub label: String,
    pub hooked: Vec<LetterPair>,
    pub subdots: Vec<LetterPair>,
    pub overdots: Vec<LetterPair>,
    pub tone_samples: Vec<String>,
    pub tone_policy: String,
    pub pidgin_policy: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LetterPair {
    pub key: String,
    pub lower: String,
    pub upper: String,
    pub feature: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EntryKind {
    Word,
    Phrase,
}

#[derive(Debug, Clone)]
struct Entry {
    code: String,
    surface: String,
    normalized: String,
    weight: u32,
    kind: EntryKind,
    source: String,
    note: String,
}

#[derive(Debug, Clone)]
struct Paradigm {
    code: String,
    lemma: String,
    form: String,
    source: String,
    note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionKind {
    Word,
    Phrase,
    Diacritic,
    Morphology,
    Fuzzy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Suggestion {
    pub text: String,
    pub kind: SuggestionKind,
    pub source: String,
    pub note: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResponse {
    pub engine: String,
    pub language: String,
    pub text: String,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct RunRequest {
    language: String,
    text: String,
}

#[derive(Debug, Default)]
struct TrieNode {
    children: BTreeMap<String, usize>,
    entries: Vec<usize>,
}

#[derive(Debug)]
struct GraphemeTrie {
    nodes: Vec<TrieNode>,
}

impl Default for GraphemeTrie {
    fn default() -> Self {
        Self {
            nodes: vec![TrieNode::default()],
        }
    }
}

impl GraphemeTrie {
    fn insert(&mut self, value: &str, entry_index: usize) {
        let mut node_index = 0;
        for grapheme in value.graphemes(true) {
            let next = if let Some(index) = self.nodes[node_index].children.get(grapheme) {
                *index
            } else {
                let index = self.nodes.len();
                self.nodes.push(TrieNode::default());
                self.nodes[node_index]
                    .children
                    .insert(grapheme.to_owned(), index);
                index
            };
            node_index = next;
        }
        self.nodes[node_index].entries.push(entry_index);
    }

    fn prefix(&self, value: &str) -> Vec<usize> {
        let mut node_index = 0;
        for grapheme in value.graphemes(true) {
            let Some(next) = self.nodes[node_index].children.get(grapheme) else {
                return Vec::new();
            };
            node_index = *next;
        }

        let mut result = Vec::new();
        let mut stack = vec![node_index];
        while let Some(index) = stack.pop() {
            result.extend(self.nodes[index].entries.iter().copied());
            stack.extend(self.nodes[index].children.values().copied());
        }
        result
    }
}

#[derive(Debug, Clone)]
struct RankedSuggestion {
    suggestion: Suggestion,
    tier: u8,
    distance: usize,
    weight: u32,
}

#[derive(Debug)]
pub struct Engine {
    languages: HashMap<String, LanguageTable>,
    entries: Vec<Entry>,
    paradigms: Vec<Paradigm>,
    entry_indexes: HashMap<(String, EntryKind), GraphemeTrie>,
    word_entries: HashMap<String, Vec<usize>>,
}

impl Engine {
    pub fn demo() -> Result<Self, EngineError> {
        Self::from_data(LANGUAGE_TABLES, DEMO_LEXICON, MORPHOLOGY, WIKI_FREQUENCIES)
    }

    pub fn from_data(
        language_json: &str,
        lexicon_tsv: &str,
        morphology_tsv: &str,
        frequency_tsv: &str,
    ) -> Result<Self, EngineError> {
        let language_file: LanguageFile = serde_json::from_str(language_json).map_err(|error| {
            EngineError(format!("I could not read my language tables: {error}"))
        })?;
        validate_language_tables(&language_file.languages)?;

        let languages = language_file
            .languages
            .into_iter()
            .map(|table| (table.code.clone(), table))
            .collect::<HashMap<_, _>>();
        let mut entries = parse_lexicon(lexicon_tsv, &languages)?;
        let frequencies = parse_frequencies(frequency_tsv, &languages)?;
        for entry in &mut entries {
            if let Some((revision, word)) =
                wiki_source_key(&entry.source, &entry.code, &entry.surface)
            {
                if let Some(count) = frequencies.get(&(entry.code.clone(), revision, word)) {
                    entry.weight = *count;
                }
            }
        }
        let paradigms = parse_morphology(morphology_tsv, &languages)?;
        let mut entry_indexes: HashMap<(String, EntryKind), GraphemeTrie> = HashMap::new();
        let mut word_entries: HashMap<String, Vec<usize>> = HashMap::new();

        for (index, entry) in entries.iter().enumerate() {
            entry_indexes
                .entry((entry.code.clone(), entry.kind))
                .or_default()
                .insert(&entry.normalized, index);
            if entry.kind == EntryKind::Word {
                word_entries
                    .entry(entry.code.clone())
                    .or_default()
                    .push(index);
            }
        }

        Ok(Self {
            languages,
            entries,
            paradigms,
            entry_indexes,
            word_entries,
        })
    }

    pub fn languages(&self) -> Vec<LanguageTable> {
        let mut result = self.languages.values().cloned().collect::<Vec<_>>();
        result.sort_by(|left, right| left.code.cmp(&right.code));
        result
    }

    pub fn suggest(
        &self,
        code: &str,
        input: &str,
        limit: usize,
    ) -> Result<Vec<Suggestion>, EngineError> {
        let Some(table) = self.languages.get(code) else {
            return Err(EngineError(format!(
                "I do not have a language table for {code}."
            )));
        };
        if input.len() > MAX_INPUT_BYTES {
            return Err(EngineError(format!(
                "I keep each input below {MAX_INPUT_BYTES} UTF-8 bytes."
            )));
        }
        if limit == 0 || input.trim().is_empty() {
            return Ok(Vec::new());
        }

        let normalized_input = canonical(input.trim());
        let mut ranked: HashMap<String, RankedSuggestion> = HashMap::new();

        self.add_phrase_prefixes(code, &normalized_input, &mut ranked);
        self.add_phrase_fuzzy(code, input, &normalized_input, &mut ranked);

        if let Some((token, token_range)) = last_token(input) {
            let (core, core_range) = token_core(token);
            if !core.is_empty() {
                let core_start = token_range.start + core_range.start;
                let core_end = token_range.start + core_range.end;
                self.add_diacritic_suggestions(
                    table,
                    input,
                    core,
                    core_start..core_end,
                    &mut ranked,
                );
                self.add_word_prefixes(code, input, core, core_start..core_end, &mut ranked);
                self.add_morphology(code, input, core, core_start..core_end, &mut ranked);
                let has_phrase_match = ranked
                    .values()
                    .any(|item| item.suggestion.kind == SuggestionKind::Phrase);
                if !has_phrase_match {
                    self.add_word_fuzzy(code, input, core, core_start..core_end, &mut ranked);
                }
            }
        }

        let mut ranked = ranked.into_values().collect::<Vec<_>>();
        ranked.sort_by(|left, right| {
            left.tier
                .cmp(&right.tier)
                .then(left.distance.cmp(&right.distance))
                .then(right.weight.cmp(&left.weight))
                .then(left.suggestion.text.cmp(&right.suggestion.text))
        });
        Ok(ranked
            .into_iter()
            .take(limit.min(12))
            .map(|item| item.suggestion)
            .collect())
    }

    fn add_phrase_prefixes(
        &self,
        code: &str,
        normalized_input: &str,
        ranked: &mut HashMap<String, RankedSuggestion>,
    ) {
        let key = (code.to_owned(), EntryKind::Phrase);
        let Some(index) = self.entry_indexes.get(&key) else {
            return;
        };
        for entry_index in index.prefix(normalized_input) {
            let entry = &self.entries[entry_index];
            if entry.normalized == normalized_input {
                continue;
            }
            push_ranked(
                ranked,
                RankedSuggestion {
                    suggestion: Suggestion {
                        text: entry.surface.clone(),
                        kind: SuggestionKind::Phrase,
                        source: entry.source.clone(),
                        note: entry.note.clone(),
                    },
                    tier: 0,
                    distance: 0,
                    weight: entry.weight,
                },
            );
        }
    }

    fn add_phrase_fuzzy(
        &self,
        code: &str,
        input: &str,
        normalized_input: &str,
        ranked: &mut HashMap<String, RankedSuggestion>,
    ) {
        let Some(max_distance) = fuzzy_limit(normalized_input.graphemes(true).count()) else {
            return;
        };
        let query_words = normalized_input.split_whitespace().collect::<Vec<_>>();
        if query_words.is_empty() {
            return;
        }

        for entry in self
            .entries
            .iter()
            .filter(|entry| entry.code == code && entry.kind == EntryKind::Phrase)
        {
            let phrase_words = entry.normalized.split_whitespace().collect::<Vec<_>>();
            if query_words.len() > phrase_words.len() || phrase_words.is_empty() {
                continue;
            }
            if query_words[..query_words.len() - 1]
                .iter()
                .zip(phrase_words.iter())
                .any(|(query, phrase)| query != phrase)
            {
                continue;
            }
            let query_tail = token_core(query_words[query_words.len() - 1]).0;
            let phrase_tail = token_core(phrase_words[query_words.len() - 1]).0;
            let distance = if phrase_tail.starts_with(query_tail) {
                0
            } else {
                let Some(distance) =
                    bounded_damerau_levenshtein(query_tail, phrase_tail, max_distance)
                else {
                    continue;
                };
                distance
            };
            if distance == 0 && entry.normalized.starts_with(normalized_input) {
                continue;
            }
            let replacement = entry.surface.clone();
            if canonical(&replacement) == canonical(input.trim()) {
                continue;
            }
            push_ranked(
                ranked,
                RankedSuggestion {
                    suggestion: Suggestion {
                        text: replacement,
                        kind: SuggestionKind::Phrase,
                        source: entry.source.clone(),
                        note: entry.note.clone(),
                    },
                    tier: 1,
                    distance,
                    weight: entry.weight,
                },
            );
        }
    }

    fn add_diacritic_suggestions(
        &self,
        table: &LanguageTable,
        input: &str,
        core: &str,
        range: std::ops::Range<usize>,
        ranked: &mut HashMap<String, RankedSuggestion>,
    ) {
        let normalized_core = canonical(core);
        for pair in table
            .hooked
            .iter()
            .chain(table.subdots.iter())
            .chain(table.overdots.iter())
        {
            if canonical(&pair.key) != normalized_core {
                continue;
            }
            let mapped = if core.chars().any(char::is_uppercase) {
                pair.upper.as_str()
            } else {
                pair.lower.as_str()
            };
            let text = replace_range(input, range.clone(), mapped);
            push_ranked(
                ranked,
                RankedSuggestion {
                    suggestion: Suggestion {
                        text,
                        kind: SuggestionKind::Diacritic,
                        source: format!("language-table:{}", table.code),
                        note: "I offer this orthographic form; I keep the typed text unchanged."
                            .to_owned(),
                    },
                    tier: 2,
                    distance: 0,
                    weight: 0,
                },
            );
        }
    }

    fn add_word_prefixes(
        &self,
        code: &str,
        input: &str,
        core: &str,
        range: std::ops::Range<usize>,
        ranked: &mut HashMap<String, RankedSuggestion>,
    ) {
        let key = (code.to_owned(), EntryKind::Word);
        let Some(index) = self.entry_indexes.get(&key) else {
            return;
        };
        let normalized_core = canonical(core);
        for entry_index in index.prefix(&normalized_core) {
            let entry = &self.entries[entry_index];
            if entry.normalized == normalized_core {
                continue;
            }
            let text = replace_range(input, range.clone(), &entry.surface);
            push_ranked(
                ranked,
                RankedSuggestion {
                    suggestion: Suggestion {
                        text,
                        kind: SuggestionKind::Word,
                        source: entry.source.clone(),
                        note: entry.note.clone(),
                    },
                    tier: 3,
                    distance: 0,
                    weight: entry.weight,
                },
            );
        }
    }

    fn add_morphology(
        &self,
        code: &str,
        input: &str,
        core: &str,
        range: std::ops::Range<usize>,
        ranked: &mut HashMap<String, RankedSuggestion>,
    ) {
        let normalized_core = canonical(core);
        for paradigm in self
            .paradigms
            .iter()
            .filter(|paradigm| paradigm.code == code)
        {
            let alternative = if normalized_core == canonical(&paradigm.lemma) {
                Some(&paradigm.form)
            } else if normalized_core == canonical(&paradigm.form) {
                Some(&paradigm.lemma)
            } else {
                None
            };
            let Some(alternative) = alternative else {
                continue;
            };
            let text = replace_range(input, range.clone(), alternative);
            push_ranked(
                ranked,
                RankedSuggestion {
                    suggestion: Suggestion {
                        text,
                        kind: SuggestionKind::Morphology,
                        source: paradigm.source.clone(),
                        note: paradigm.note.clone(),
                    },
                    tier: 4,
                    distance: 0,
                    weight: 1,
                },
            );
        }
    }

    fn add_word_fuzzy(
        &self,
        code: &str,
        input: &str,
        core: &str,
        range: std::ops::Range<usize>,
        ranked: &mut HashMap<String, RankedSuggestion>,
    ) {
        let query = canonical(core);
        let Some(max_distance) = fuzzy_limit(query.graphemes(true).count()) else {
            return;
        };
        let Some(entries) = self.word_entries.get(code) else {
            return;
        };
        for entry_index in entries {
            let entry = &self.entries[*entry_index];
            let Some(distance) =
                bounded_damerau_levenshtein(&query, &entry.normalized, max_distance)
            else {
                continue;
            };
            if distance == 0 {
                continue;
            }
            let text = replace_range(input, range.clone(), &entry.surface);
            push_ranked(
                ranked,
                RankedSuggestion {
                    suggestion: Suggestion {
                        text,
                        kind: SuggestionKind::Fuzzy,
                        source: entry.source.clone(),
                        note: "I offer a nearby spelling; I never change the typed text."
                            .to_owned(),
                    },
                    tier: 5,
                    distance,
                    weight: entry.weight,
                },
            );
        }
    }
}

fn validate_language_tables(tables: &[LanguageTable]) -> Result<(), EngineError> {
    let expected = HashSet::from(["ha", "ig", "pcm"]);
    let mut seen = HashSet::new();
    for table in tables {
        if !expected.contains(table.code.as_str()) || !seen.insert(table.code.as_str()) {
            return Err(EngineError(
                "I found an unknown or duplicate language code in my tables.".to_owned(),
            ));
        }
        if table.label.trim().is_empty() {
            return Err(EngineError(
                "I keep a visible label for every language table.".to_owned(),
            ));
        }
        for pair in table
            .hooked
            .iter()
            .chain(table.subdots.iter())
            .chain(table.overdots.iter())
        {
            for value in [&pair.key, &pair.lower, &pair.upper] {
                if !is_single_nfc_grapheme(value) {
                    return Err(EngineError(format!(
                        "I require one NFC grapheme for the {} letter pair.",
                        pair.feature
                    )));
                }
            }
        }
        for sample in &table.tone_samples {
            if !is_single_nfc_grapheme(sample) || !sample.nfd().any(is_combining_mark) {
                return Err(EngineError(
                    "I require every tone sample to be one NFC grapheme with a mark.".to_owned(),
                ));
            }
        }
        if table.code == "pcm"
            && (table.pidgin_policy != "descriptive" || table.tone_policy != "none")
        {
            return Err(EngineError(
                "I keep Pidgin descriptive and avoid a tone-mark policy.".to_owned(),
            ));
        }
        if table.code != "pcm" && table.pidgin_policy != "not-applicable" {
            return Err(EngineError(
                "I keep my Pidgin policy attached only to Pidgin.".to_owned(),
            ));
        }
    }
    if seen != expected {
        return Err(EngineError(
            "I require Hausa, Igbo, and Nigerian Pidgin language tables.".to_owned(),
        ));
    }
    Ok(())
}

fn parse_lexicon(
    input: &str,
    languages: &HashMap<String, LanguageTable>,
) -> Result<Vec<Entry>, EngineError> {
    let mut entries = Vec::new();
    for (line_number, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields = line.splitn(6, '\t').collect::<Vec<_>>();
        if fields.len() != 6 {
            return Err(EngineError(format!(
                "I need six tab-separated fields on lexicon line {}.",
                line_number + 1
            )));
        }
        let code = fields[0];
        if !languages.contains_key(code) {
            return Err(EngineError(format!(
                "I found an unknown language code on lexicon line {}.",
                line_number + 1
            )));
        }
        let weight = fields[1].parse::<u32>().map_err(|_| {
            EngineError(format!(
                "I need a non-negative integer weight on lexicon line {}.",
                line_number + 1
            ))
        })?;
        let kind = match fields[2] {
            "word" => EntryKind::Word,
            "phrase" => EntryKind::Phrase,
            _ => {
                return Err(EngineError(format!(
                    "I found an unknown entry kind on lexicon line {}.",
                    line_number + 1
                )))
            }
        };
        let surface = fields[3];
        if surface.trim().is_empty() || surface.nfc().collect::<String>() != surface {
            return Err(EngineError(format!(
                "I require a non-empty NFC surface on lexicon line {}.",
                line_number + 1
            )));
        }
        if fields[4].trim().is_empty() || fields[5].trim().is_empty() {
            return Err(EngineError(format!(
                "I require source and note fields on lexicon line {}.",
                line_number + 1
            )));
        }
        entries.push(Entry {
            code: code.to_owned(),
            surface: surface.to_owned(),
            normalized: canonical(surface),
            weight,
            kind,
            source: fields[4].to_owned(),
            note: fields[5].to_owned(),
        });
    }
    Ok(entries)
}

fn parse_frequencies(
    input: &str,
    languages: &HashMap<String, LanguageTable>,
) -> Result<HashMap<(String, u64, String), u32>, EngineError> {
    let mut frequencies = HashMap::new();
    for (line_number, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') || line == "code\trevision_id\tword\tcount" {
            continue;
        }
        let fields = line.splitn(4, '\t').collect::<Vec<_>>();
        if fields.len() != 4 {
            return Err(EngineError(format!(
                "I need four tab-separated fields on frequency line {}.",
                line_number + 1
            )));
        }
        let code = fields[0];
        if !languages.contains_key(code) {
            return Err(EngineError(format!(
                "I found an unknown language code on frequency line {}.",
                line_number + 1
            )));
        }
        let revision = fields[1].parse::<u64>().map_err(|_| {
            EngineError(format!(
                "I need a numeric revision on frequency line {}.",
                line_number + 1
            ))
        })?;
        let word = fields[2];
        if word.is_empty() || word.nfc().collect::<String>() != word || canonical(word) != word {
            return Err(EngineError(format!(
                "I require a non-empty, lowercase NFC word on frequency line {}.",
                line_number + 1
            )));
        }
        let count = fields[3].parse::<u32>().map_err(|_| {
            EngineError(format!(
                "I need a positive integer count on frequency line {}.",
                line_number + 1
            ))
        })?;
        if count == 0 {
            return Err(EngineError(format!(
                "I do not keep zero counts on frequency line {}.",
                line_number + 1
            )));
        }
        let key = (code.to_owned(), revision, word.to_owned());
        if frequencies.insert(key, count).is_some() {
            return Err(EngineError(format!(
                "I found a duplicate word count on frequency line {}.",
                line_number + 1
            )));
        }
    }
    Ok(frequencies)
}

fn wiki_source_key(source: &str, code: &str, surface: &str) -> Option<(u64, String)> {
    let mut parts = source.split(':');
    if parts.next()? != "wiki" || parts.next()? != code {
        return None;
    }
    let revision = parts.next()?.parse::<u64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((revision, canonical(surface)))
}

fn parse_morphology(
    input: &str,
    languages: &HashMap<String, LanguageTable>,
) -> Result<Vec<Paradigm>, EngineError> {
    let mut paradigms = Vec::new();
    for (line_number, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields = line.splitn(5, '\t').collect::<Vec<_>>();
        if fields.len() != 5 {
            return Err(EngineError(format!(
                "I need five tab-separated fields on morphology line {}.",
                line_number + 1
            )));
        }
        let code = fields[0];
        if !languages.contains_key(code) {
            return Err(EngineError(format!(
                "I found an unknown language code on morphology line {}.",
                line_number + 1
            )));
        }
        for form in [fields[1], fields[2]] {
            if form.trim().is_empty() || form.nfc().collect::<String>() != form {
                return Err(EngineError(format!(
                    "I require NFC forms on morphology line {}.",
                    line_number + 1
                )));
            }
        }
        if fields[3].trim().is_empty() || fields[4].trim().is_empty() {
            return Err(EngineError(format!(
                "I require a source and note on morphology line {}.",
                line_number + 1
            )));
        }
        paradigms.push(Paradigm {
            code: code.to_owned(),
            lemma: fields[1].to_owned(),
            form: fields[2].to_owned(),
            source: fields[3].to_owned(),
            note: fields[4].to_owned(),
        });
    }
    Ok(paradigms)
}

fn is_single_nfc_grapheme(value: &str) -> bool {
    value.nfc().collect::<String>() == value && value.graphemes(true).count() == 1
}

fn canonical(value: &str) -> String {
    value
        .nfc()
        .collect::<String>()
        .to_lowercase()
        .nfc()
        .collect()
}

fn fuzzy_limit(grapheme_count: usize) -> Option<usize> {
    match grapheme_count {
        0..=2 => None,
        3..=6 => Some(1),
        _ => Some(2),
    }
}

pub fn bounded_damerau_levenshtein(left: &str, right: &str, limit: usize) -> Option<usize> {
    let left = canonical(left)
        .graphemes(true)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let right = canonical(right)
        .graphemes(true)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if left.len().abs_diff(right.len()) > limit {
        return None;
    }

    let mut matrix = vec![vec![0usize; right.len() + 1]; left.len() + 1];
    for (index, row) in matrix.iter_mut().enumerate().take(left.len() + 1) {
        row[0] = index;
    }
    for (index, value) in matrix[0].iter_mut().enumerate().take(right.len() + 1) {
        *value = index;
    }

    for left_index in 1..=left.len() {
        for right_index in 1..=right.len() {
            let substitution = usize::from(left[left_index - 1] != right[right_index - 1]);
            let mut value = (matrix[left_index - 1][right_index] + 1)
                .min(matrix[left_index][right_index - 1] + 1)
                .min(matrix[left_index - 1][right_index - 1] + substitution);
            if left_index > 1
                && right_index > 1
                && left[left_index - 1] == right[right_index - 2]
                && left[left_index - 2] == right[right_index - 1]
            {
                value = value.min(matrix[left_index - 2][right_index - 2] + 1);
            }
            matrix[left_index][right_index] = value;
        }
    }

    let distance = matrix[left.len()][right.len()];
    (distance <= limit).then_some(distance)
}

fn last_token(value: &str) -> Option<(&str, std::ops::Range<usize>)> {
    if value.chars().last().is_some_and(char::is_whitespace) {
        return None;
    }
    let end = value
        .char_indices()
        .rev()
        .find(|(_, character)| !character.is_whitespace())
        .map(|(index, character)| index + character.len_utf8())?;
    let start = value[..end]
        .char_indices()
        .rev()
        .find(|(_, character)| character.is_whitespace())
        .map(|(index, character)| index + character.len_utf8())
        .unwrap_or(0);
    Some((&value[start..end], start..end))
}

fn token_core(value: &str) -> (&str, std::ops::Range<usize>) {
    let graphemes = value.grapheme_indices(true).collect::<Vec<_>>();
    let Some(first) = graphemes.iter().position(|(_, grapheme)| {
        grapheme
            .chars()
            .next()
            .is_some_and(|character| character.is_alphanumeric())
    }) else {
        return ("", 0..0);
    };
    let last = graphemes
        .iter()
        .rposition(|(_, grapheme)| {
            grapheme
                .chars()
                .next()
                .is_some_and(|character| character.is_alphanumeric())
        })
        .unwrap_or(first);
    let start = graphemes[first].0;
    let end = if last + 1 < graphemes.len() {
        graphemes[last + 1].0
    } else {
        value.len()
    };
    (&value[start..end], start..end)
}

fn replace_range(value: &str, range: std::ops::Range<usize>, replacement: &str) -> String {
    let mut output = String::with_capacity(value.len() + replacement.len());
    output.push_str(&value[..range.start]);
    output.push_str(replacement);
    output.push_str(&value[range.end..]);
    output
}

fn push_ranked(ranked: &mut HashMap<String, RankedSuggestion>, candidate: RankedSuggestion) {
    let key = canonical(&candidate.suggestion.text);
    match ranked.get(&key) {
        Some(existing)
            if (existing.tier, existing.distance, u32::MAX - existing.weight)
                <= (
                    candidate.tier,
                    candidate.distance,
                    u32::MAX - candidate.weight,
                ) => {}
        _ => {
            ranked.insert(key, candidate);
        }
    }
}

pub fn run_json(input: &str) -> Result<String, EngineError> {
    if input.len() > MAX_INPUT_BYTES {
        return Err(EngineError(format!(
            "I keep each request below {MAX_INPUT_BYTES} UTF-8 bytes."
        )));
    }
    let request: RunRequest = serde_json::from_str(input)
        .map_err(|error| EngineError(format!("I could not read the request: {error}")))?;
    let engine = Engine::demo()?;
    let suggestions = engine.suggest(&request.language, &request.text, 8)?;
    serde_json::to_string(&RunResponse {
        engine: "ami-demo-0.1.0".to_owned(),
        language: request.language,
        text: request.text,
        suggestions,
    })
    .map_err(|error| EngineError(format!("I could not serialize my response: {error}")))
}

#[cfg(target_arch = "wasm32")]
mod wasm_abi {
    use super::{Engine, EngineError, RunRequest, RunResponse, MAX_INPUT_BYTES};
    use serde::Serialize;
    use std::mem;
    use std::slice;
    use std::sync::OnceLock;

    static ENGINE: OnceLock<Result<Engine, EngineError>> = OnceLock::new();

    fn engine() -> &'static Engine {
        ENGINE
            .get_or_init(Engine::demo)
            .as_ref()
            .expect("I checked my embedded language data before release.")
    }

    #[no_mangle]
    pub extern "C" fn ami_alloc(capacity: u32) -> u32 {
        let mut buffer = Vec::<u8>::with_capacity(capacity as usize);
        let pointer = buffer.as_mut_ptr();
        mem::forget(buffer);
        pointer as u32
    }

    #[no_mangle]
    pub unsafe extern "C" fn ami_free_input(pointer: u32, capacity: u32) {
        if pointer != 0 {
            drop(Vec::from_raw_parts(
                pointer as *mut u8,
                0,
                capacity as usize,
            ));
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn ami_run(pointer: u32, length: u32, output_length_pointer: u32) -> u32 {
        if output_length_pointer == 0 {
            return 0;
        }
        if pointer == 0 || length as usize > MAX_INPUT_BYTES {
            return write_output(
                error_json("I received an invalid input buffer."),
                output_length_pointer,
            );
        }
        let input = slice::from_raw_parts(pointer as *const u8, length as usize);
        let result = match std::str::from_utf8(input) {
            Ok(input) => run_with_engine(input, engine()),
            Err(_) => Err("I require valid UTF-8 input.".to_owned()),
        };
        let output = match result {
            Ok(output) => output,
            Err(error) => error_json(&error),
        };
        write_output(output, output_length_pointer)
    }

    fn run_with_engine(input: &str, engine: &Engine) -> Result<String, String> {
        let request: RunRequest = serde_json::from_str(input)
            .map_err(|error| format!("I could not read the request: {error}"))?;
        let suggestions = engine
            .suggest(&request.language, &request.text, 8)
            .map_err(|error| error.to_string())?;
        serde_json::to_string(&RunResponse {
            engine: "ami-demo-0.1.0".to_owned(),
            language: request.language,
            text: request.text,
            suggestions,
        })
        .map_err(|error| format!("I could not serialize my response: {error}"))
    }

    fn error_json(message: &str) -> String {
        #[derive(Serialize)]
        struct ErrorResponse<'a> {
            error: &'a str,
        }
        serde_json::to_string(&ErrorResponse { error: message })
            .unwrap_or_else(|_| "{\"error\":\"I could not create a response.\"}".to_owned())
    }

    fn write_output(output: String, output_length_pointer: u32) -> u32 {
        let boxed = output.into_bytes().into_boxed_slice();
        let length = boxed.len() as u32;
        unsafe {
            *(output_length_pointer as *mut u32) = length;
        }
        Box::into_raw(boxed) as *mut u8 as u32
    }

    #[no_mangle]
    pub unsafe extern "C" fn ami_free_output(pointer: u32, length: u32) {
        if pointer != 0 {
            let slice = std::ptr::slice_from_raw_parts_mut(pointer as *mut u8, length as usize);
            drop(Box::from_raw(slice));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_normalization::UnicodeNormalization;

    fn demo() -> Engine {
        Engine::demo().expect("I can load my checked-in demo data.")
    }

    fn has_suggestion(suggestions: &[Suggestion], text: &str, kind: SuggestionKind) -> bool {
        suggestions
            .iter()
            .any(|suggestion| suggestion.text == text && suggestion.kind == kind)
    }

    #[test]
    fn tables_keep_hausa_hooked_letters_and_igbo_marks_in_single_graphemes() {
        let engine = demo();
        let hausa = engine.languages.get("ha").expect("I keep my Hausa table.");
        let hooked = hausa
            .hooked
            .iter()
            .map(|pair| pair.lower.as_str())
            .collect::<Vec<_>>();
        assert_eq!(hooked, ["ɓ", "ɗ", "ƙ", "ƴ"]);

        let igbo = engine.languages.get("ig").expect("I keep my Igbo table.");
        let subdots = igbo
            .subdots
            .iter()
            .map(|pair| pair.lower.as_str())
            .collect::<Vec<_>>();
        assert_eq!(subdots, ["ị", "ọ", "ụ"]);
        assert_eq!(igbo.overdots[0].lower, "ṅ");

        for table in engine.languages.values() {
            for sample in &table.tone_samples {
                assert!(is_single_nfc_grapheme(sample));
                assert_eq!(sample.nfc().collect::<String>(), *sample);
            }
        }
    }

    #[test]
    fn diacritic_assistance_preserves_uppercase_and_input_boundaries() {
        let engine = demo();
        let hausa_lower = engine.suggest("ha", "b", 8).unwrap();
        assert!(has_suggestion(&hausa_lower, "ɓ", SuggestionKind::Diacritic));
        let hausa_upper = engine.suggest("ha", "B", 8).unwrap();
        assert!(has_suggestion(&hausa_upper, "Ɓ", SuggestionKind::Diacritic));
        let igbo = engine.suggest("ig", "o!", 8).unwrap();
        assert!(has_suggestion(&igbo, "ọ!", SuggestionKind::Diacritic));
        let igbo_upper = engine.suggest("ig", "I", 8).unwrap();
        assert!(has_suggestion(&igbo_upper, "Ị", SuggestionKind::Diacritic));
    }

    #[test]
    fn fuzzy_matching_uses_unicode_graphemes_and_keeps_diacritics() {
        let engine = demo();
        let hausa = engine.suggest("ha", "kasa", 8).unwrap();
        assert!(has_suggestion(&hausa, "ƙasa", SuggestionKind::Fuzzy));
        let igbo = engine.suggest("ig", "oma", 8).unwrap();
        assert!(has_suggestion(&igbo, "ọma", SuggestionKind::Fuzzy));
        assert_eq!(bounded_damerau_levenshtein("ị", "i\u{0323}", 0), Some(0));
        assert_eq!(bounded_damerau_levenshtein("ƴa", "ya", 1), Some(1));
        assert_eq!(bounded_damerau_levenshtein("ab", "ba", 1), Some(1));
    }

    #[test]
    fn pidgin_suggestions_stay_descriptive_and_cover_two_word_and_three_word_inputs() {
        let engine = demo();
        let greeting = engine.suggest("pcm", "how far", 8).unwrap();
        assert!(has_suggestion(
            &greeting,
            "how far?",
            SuggestionKind::Phrase
        ));
        assert!(greeting
            .iter()
            .all(|suggestion| suggestion.text != "how for"));
        let three_words = engine.suggest("pcm", "I dey fi", 8).unwrap();
        assert!(has_suggestion(
            &three_words,
            "I dey fine.",
            SuggestionKind::Phrase
        ));
        assert!(three_words.iter().all(|suggestion| {
            suggestion.kind != SuggestionKind::Fuzzy
                || !suggestion.note.to_lowercase().contains("wrong")
        }));
    }

    #[test]
    fn morphology_offers_a_source_checked_neighbor_without_replacing_input() {
        let engine = demo();
        let suggestions = engine.suggest("ha", "gida", 8).unwrap();
        assert!(has_suggestion(
            &suggestions,
            "gidaje",
            SuggestionKind::Morphology
        ));
        assert_eq!(engine.suggest("ha", "gida", 0).unwrap().len(), 0);
    }

    #[test]
    fn pinned_julia_counts_rank_source_tagged_words() {
        let engine = demo();
        let harshen = engine
            .entries
            .iter()
            .find(|entry| entry.source == "wiki:ha:873901" && entry.surface == "Harshen")
            .expect("I keep my pinned Hausa word.");
        assert_eq!(harshen.weight, 8);
        let igbo = engine
            .entries
            .iter()
            .find(|entry| entry.source == "wiki:ig:97975" && entry.surface == "ọma")
            .expect("I keep my pinned Igbo word.");
        assert_eq!(igbo.weight, 1);
    }

    #[test]
    fn whole_word_prefixes_and_bounded_phrase_fuzziness_work() {
        let engine = demo();
        let hausa = engine.suggest("ha", "Harsh", 8).unwrap();
        assert!(has_suggestion(&hausa, "Harshen", SuggestionKind::Word));
        let phrase = engine.suggest("pcm", "how fr", 8).unwrap();
        assert!(has_suggestion(&phrase, "how far?", SuggestionKind::Phrase));
        assert!(engine.suggest("zz", "word", 8).is_err());
    }

    #[test]
    fn composed_and_decomposed_input_match_without_splitting_a_grapheme() {
        let engine = demo();
        let composed = "ọ";
        let decomposed = "o\u{0323}";
        assert_eq!(canonical(composed), canonical(decomposed));
        assert_eq!(composed.graphemes(true).count(), 1);
        assert_eq!(decomposed.graphemes(true).count(), 1);
        let suggestions = engine.suggest("ig", decomposed, 8).unwrap();
        assert!(has_suggestion(&suggestions, "ọwụwa", SuggestionKind::Word));
    }

    #[test]
    fn json_boundary_is_stable_and_unicode_safe() {
        let output = run_json(r#"{"language":"ha","text":"B"}"#).unwrap();
        let value: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(value["language"], "ha");
        assert_eq!(value["suggestions"][0]["text"], "Ɓ");
        assert!(run_json("not-json").is_err());
    }
}
