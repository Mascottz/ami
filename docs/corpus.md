# My corpus notes

I have now fetched and verified one revision-pinned Wikipedia article for each starting language. I keep the raw plaintext files under `corpus/raw/`, outside Git. I use a tiny, source-tagged demo lexicon and a reproducible Julia word-count table; I make no trained or production-quality model claim.

## My pinned Wikipedia revisions

| Language | Article and pinned revision | Revision date | Plaintext bytes | Word types | Word tokens | License | SHA-256 of UTF-8 plaintext |
| --- | --- | --- | ---: | ---: | ---: | --- | --- |
| Hausa (`ha`) | [Harshen Hausa, revision 873901](https://ha.wikipedia.org/w/index.php?oldid=873901) | 2026-07-01 | 3,562 | 270 | 587 | CC BY-SA 4.0 | `59b19868bac846bd4747b51207b4d91ac639383f96eeee81526c32f57fdcd209` |
| Igbo (`ig`) | [Asụsụ Igbo, revision 97975](https://ig.wikipedia.org/w/index.php?oldid=97975) | 2022-09-29 | 2,113 | 169 | 328 | CC BY-SA 3.0 Unported | `532801bba82dc92024892d0a32f19d1824e8f1e3360ad9a952658ee86309117c` |
| Nigerian Pidgin (`pcm`) | [Naijá langwej, revision 36150](https://pcm.wikipedia.org/w/index.php?oldid=36150) | 2026-08-01 | 292 | 30 | 50 | CC BY-SA 4.0 | `ad8c360711ae1ac950eda3bcd155744b08e833d1d0c2434044411f26d40d35f0` |

I requested each revision through the MediaWiki Action API with `revids=<revision ID>` and `explaintext=1`. I saved the returned extract as UTF-8 without an added newline. I record the page IDs, revision IDs, timestamps, license basis, API URLs, and transformations in [`corpus/manifest.json`](../corpus/manifest.json). I verify every checksum before I let the Julia step read a file. I fetch the pinned text with `node scripts/fetch-wikipedia.mjs`.

I count word tokens with Julia 1.13.1, NFC normalization, Unicode lowercasing, and the Unicode letter-plus-mark pattern `\p{L}\p{M}`. I keep Hausa hooked letters, Igbo subdots, and tone marks. I record the resulting 469 source-word rows in [`data/wiki-frequencies.tsv`](../data/wiki-frequencies.tsv). I count 965 tokens across one article per language, including mixed text and article artifacts. I use the table only to rank a few source-tagged demo entries; I do not present my count table as balanced language-frequency evidence.

## My attribution and data licenses

I credit each Wikipedia text to the named article and link to the exact revision history above. I assign the post-2023-06-07 revisions to CC BY-SA 4.0 and the 2022 Igbo revision to CC BY-SA 3.0 Unported. I follow the [Creative Commons notice on Wikipedia's 2023 license change](https://creativecommons.org/2023/06/29/wikipedia-moves-to-cc-4-0-licenses/) and the [Wikimedia Foundation reuse terms](https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use). I keep source-specific attribution and checksums in the manifest, and I describe the data-file terms in [`data/NOTICE.md`](../data/NOTICE.md).

I keep software licensing separate from corpus-data licensing. I license my original software under Apache-2.0. I license my selected and arranged demo data under CC BY-SA 4.0 where the source terms require share-alike; I keep every source's original license and attribution visible. I do not bundle raw article extracts in Git.

## My current demo slice

I hand-select a few source-tagged words from each pinned article, then use Julia counts to rank my selected entries. My manual rows exist only to exercise Unicode and suggestion paths. My Pidgin smoke tests include `how far?` as an optional phrase suggestion and `I dey fine.` as a three-word reply for my useful-suggestion test. I do not mark Pidgin input wrong, underline it as an error, or force a replacement.

I keep one reference-checked Hausa family, `gida` and `gidaje`, as an optional morphology suggestion. I cite the [UCLA Hausa noun-modifier examples](https://aflang.humanities.ucla.edu/language-materials/chadic-languages/hausa/hausa-online-grammar/noun-modifiers/) for this word pair. I use only the forms as a factual morphology example; I do not copy the page's explanations. I have not built a complete Hausa, Igbo, or Pidgin morphology model.

## My open parallel-text research

### Tatoeba

I found Hausa and Igbo in the [official Tatoeba download listing](https://tatoeba.org/en/downloads). I read the download page as listing general sentence files under CC BY 2.0 France and a separate subset under CC0 1.0. I will favor the CC0 subset for a low-friction evaluation slice, keep sentence IDs and source metadata, and review attribution for any CC BY records before reuse. I have not confirmed a Nigerian Pidgin export, so I do not count Tatoeba as a Pidgin seed yet. I cite the [Tatoeba terms of use](https://tatoeba.org/en/terms_of_use) for per-sentence reuse terms.

### MAFAND-MT

I found MAFAND-MT as a parallel source covering Hausa, Igbo, and Nigerian Pidgin. I read the [dataset card](https://huggingface.co/datasets/masakhane/mafand/blob/main/README.md) as identifying CC BY-NC 4.0. I keep MAFAND out of shipped training data for now because I do not want a non-commercial restriction attached to Àmì's model assets.

### Okwu African Language Parallel Corpus

I found an [Okwu African Language Parallel Corpus repository](https://github.com/Okwu-AI/corpus) with a `pcm-eng` configuration. I read the [licensing notes](https://github.com/Okwu-AI/corpus/blob/main/LICENSING.md) as assigning CDLA-Permissive-2.0 to released data and CC BY 4.0 to documentation. I read the [getting-started note](https://github.com/Okwu-AI/corpus/blob/main/docs/getting-started.md) as describing source and record metadata, and the [limitations note](https://github.com/Okwu-AI/corpus/blob/main/docs/known-limitations.md) as describing everyday register and short vocabulary entries.

I keep the Okwu corpus as a promising Pidgin candidate, not an ingested source. I will pin an immutable release, inspect upstream provenance, and resolve the attribution notes before I use the source. I also found an attribution line in the licensing notes pairing CC BY with the CDLA data license; I will resolve the license tension before ingestion.

### IgboAPI

I found the [IgboAPI dataset paper](https://arxiv.org/html/2405.00997v1). I read the paper as documenting a multi-dialect Igbo-English resource under CC BY-NC-SA 4.0. I exclude the dataset from shipped data while I review the non-commercial and share-alike terms.

## My intake checklist

I record an immutable source revision, retrieval date, SHA-256 checksum, language, license, attribution text, source URL, and exact preprocessing steps for every snapshot. I keep raw dumps outside Git. I keep source-specific provenance beside every generated frequency table and model artifact.

I do not mix license families without review. I do not claim a source as production training material when I have only built a small demo slice. I follow the applicable Wikimedia license and attribution terms for each revision instead of flattening all wiki history into one label.

## My references

I cite [Wikimedia text reuse terms](https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use), [Wikimedia dump downloads](https://dumps.wikimedia.org/), [Wikipedia's 2023 license change](https://creativecommons.org/2023/06/29/wikipedia-moves-to-cc-4-0-licenses/), [Tatoeba downloads](https://tatoeba.org/en/downloads), [Tatoeba reuse terms](https://tatoeba.org/en/terms_of_use), [MAFAND-MT licensing](https://huggingface.co/datasets/masakhane/mafand/blob/main/README.md), [Okwu corpus licensing](https://github.com/Okwu-AI/corpus/blob/main/LICENSING.md), [Okwu corpus metadata](https://github.com/Okwu-AI/corpus/blob/main/docs/getting-started.md), [Okwu corpus limitations](https://github.com/Okwu-AI/corpus/blob/main/docs/known-limitations.md), [IgboAPI licensing and scope](https://arxiv.org/html/2405.00997v1), [UCLA's Hausa examples](https://aflang.humanities.ucla.edu/language-materials/chadic-languages/hausa/hausa-online-grammar/noun-modifiers/), [Nigerian Pidgin orthography research](https://doi.org/10.1515/9781501513541-007), and [the Manchester language-contact study](https://languagecontact.humanities.manchester.ac.uk/McrLC/casestudies/FG_NigerianPidgin.html).
