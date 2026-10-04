import { loadAmiEngine } from './ami-engine.mjs';
import { insertAtGraphemeSelection } from './graphemes.mjs';

const editor = document.querySelector('#editor');
const engineStatus = document.querySelector('#engine-status');
const languageCode = document.querySelector('#language-code');
const wordCount = document.querySelector('#word-count');
const suggestionList = document.querySelector('#suggestions');
const emptyState = document.querySelector('#empty-state');
const languageNote = document.querySelector('#language-note');
const diacriticKeys = document.querySelector('#diacritic-keys');
const diacriticNote = document.querySelector('#diacritic-note');
const languageButtons = [...document.querySelectorAll('[data-language]')];
const dismissals = new Set();
const languageLabels = { ha: 'Hausa', ig: 'Igbo', pcm: 'Naijá Pidgin' };
const assistLetters = {
  ha: ['ɓ', 'ɗ', 'ƙ', 'ƴ'],
  ig: ['ị', 'ọ', 'ụ', 'ṅ'],
  pcm: [],
};

let language = 'pcm';
let engine = null;

function updateWordCount() {
  const trimmed = editor.value.trim();
  const count = trimmed ? trimmed.split(/\s+/u).length : 0;
  wordCount.textContent = `${count} ${count === 1 ? 'WORD' : 'WORDS'}`;
}

function updateLanguageNote() {
  const notes = {
    ha: 'I offer hooked-letter choices and a small reviewed word list. I keep every suggestion optional.',
    ig: 'I offer subdot choices and a small reviewed word list. I keep every suggestion optional.',
    pcm: 'I keep Pidgin descriptive; I never mark a phrase wrong.',
  };
  languageNote.textContent = notes[language];
  editor.lang = language;
  languageCode.textContent = `${language.toUpperCase()} / ${language === 'pcm' ? 'DESCRIPTIVE' : 'OPTIONAL SUGGESTIONS'}`;
  for (const button of languageButtons) {
    const active = button.dataset.language === language;
    button.classList.toggle('is-active', active);
    button.setAttribute('aria-pressed', String(active));
  }
  renderAssistLetters();
}

function renderAssistLetters() {
  if (typeof Intl.Segmenter !== 'function') {
    diacriticKeys.replaceChildren();
    diacriticNote.textContent = 'I keep insertion quiet when grapheme-safe cursor support is unavailable.';
    return;
  }
  diacriticKeys.replaceChildren();
  const letters = assistLetters[language];
  if (letters.length === 0) {
    diacriticNote.textContent = 'I keep Pidgin spelling descriptive and leave the key row quiet.';
    return;
  }
  diacriticNote.textContent = language === 'ha'
    ? 'I offer hooked letters as nearby choices; I do not treat a plain letter as a mistake.'
    : 'I offer underdots and an overdot as nearby choices; I keep the typed form untouched.';
  for (const letter of letters) {
    const button = document.createElement('button');
    button.type = 'button';
    button.className = 'diacritic-key';
    button.textContent = letter;
    button.setAttribute('aria-label', `I insert ${letter} at my caret`);
    button.addEventListener('click', () => {
      const result = insertAtGraphemeSelection(
        editor.value,
        editor.selectionStart,
        editor.selectionEnd,
        letter,
      );
      editor.value = result.text;
      editor.focus();
      editor.setSelectionRange(result.caret, result.caret);
      editor.dispatchEvent(new Event('input', { bubbles: true }));
    });
    diacriticKeys.append(button);
  }
}

function renderSuggestions() {
  updateWordCount();
  suggestionList.replaceChildren();
  emptyState.hidden = true;
  if (!engine) return;
  let response;
  try {
    response = engine.suggest(language, editor.value);
  } catch {
    engineStatus.textContent = 'I could not read the input; I keep the text untouched.';
    return;
  }
  const visible = response.suggestions.filter((suggestion) => {
    return !dismissals.has(`${language}:${suggestion.text}`);
  });
  if (visible.length === 0) {
    emptyState.hidden = false;
    return;
  }
  for (const suggestion of visible) {
    const item = document.createElement('li');
    item.className = 'suggestion-item';
    const main = document.createElement('div');
    main.className = 'suggestion-main';
    const text = document.createElement('span');
    text.className = 'suggestion-text';
    text.textContent = suggestion.text;
    const kind = document.createElement('span');
    kind.className = 'suggestion-kind';
    kind.textContent = suggestion.kind.replaceAll('_', ' ');
    main.append(text, kind);

    const note = document.createElement('p');
    note.className = 'suggestion-note';
    note.textContent = suggestion.note;
    const source = document.createElement('code');
    source.className = 'suggestion-source';
    source.textContent = suggestion.source;

    const actions = document.createElement('div');
    actions.className = 'suggestion-actions';
    const use = document.createElement('button');
    use.type = 'button';
    use.className = 'suggestion-action suggestion-action-use';
    use.textContent = 'I use suggestion';
    use.setAttribute('aria-label', `I use ${suggestion.text}`);
    use.addEventListener('click', () => {
      editor.value = suggestion.text;
      dismissals.clear();
      editor.focus();
      editor.setSelectionRange(editor.value.length, editor.value.length);
      renderSuggestions();
    });
    const dismiss = document.createElement('button');
    dismiss.type = 'button';
    dismiss.className = 'suggestion-action suggestion-action-dismiss';
    dismiss.textContent = 'I dismiss';
    dismiss.addEventListener('click', () => {
      dismissals.add(`${language}:${suggestion.text}`);
      renderSuggestions();
    });
    actions.append(use, dismiss);
    item.append(main, note, source, actions);
    suggestionList.append(item);
  }
}

for (const button of languageButtons) {
  button.addEventListener('click', () => {
    language = button.dataset.language;
    dismissals.clear();
    updateLanguageNote();
    renderSuggestions();
  });
}

editor.addEventListener('input', () => {
  dismissals.clear();
  renderSuggestions();
});

document.querySelector('#load-three-word').addEventListener('click', () => {
  language = 'pcm';
  editor.value = 'I dey fi';
  dismissals.clear();
  updateLanguageNote();
  renderSuggestions();
  editor.focus();
  editor.setSelectionRange(editor.value.length, editor.value.length);
});

document.querySelector('#clear-writing').addEventListener('click', () => {
  editor.value = '';
  dismissals.clear();
  renderSuggestions();
  editor.focus();
});

updateLanguageNote();
updateWordCount();

try {
  engine = await loadAmiEngine();
  engineStatus.textContent = 'I loaded my Rust engine locally.';
  renderSuggestions();
} catch {
  engineStatus.textContent = 'I could not load my engine; my writing stays in this tab.';
  emptyState.hidden = false;
  emptyState.textContent = 'I keep my writing local; I need the WebAssembly file to run my demo.';
}
