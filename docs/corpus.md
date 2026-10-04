# My corpus notes

I researched seed sources before I ingest anything. I have not downloaded a corpus snapshot, copied corpus text into this repository, or trained a model. I keep this page as the source map and will add exact revisions, checksums, counts, and transformations before any source influences a shipped suggestion.

## My three starting sources

### Hausa

I start from the [Hausa Wikipedia](https://ha.wikipedia.org/) with language code `ha`, as requested. I will record the dump date, source page history, checksum, and applicable reuse terms when I ingest text. I cite the [Wikimedia Foundation reuse terms](https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use) and the [official Wikimedia dump index](https://dumps.wikimedia.org/) for the text source and attribution path. I use the [UCLA Hausa writing reference](https://aflang.humanities.ucla.edu/language-materials/chadic-languages/hausa/hausa-online-grammar/pronunciation-writing/hausa-writing/) for Roman Hausa hooked letters and tone-marking context.

### Igbo

I start from the [Igbo Wikipedia](https://ig.wikipedia.org/) with language code `ig`, as requested. I will record the dump date, source page history, checksum, and applicable reuse terms when I ingest text. I cite the [Wikimedia Foundation reuse terms](https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use) and the [official Wikimedia dump index](https://dumps.wikimedia.org/) for the text source and attribution path. I use the [Onwu Committee orthography record](https://books.google.com/books?id=q_hftAEACAAJ) and [Frances Pritchett's Columbia research notes](http://www.columbia.edu/itc/mealac/pritchett/00fwp/igbo/igbohistory.html) for underdot and tone-mark references.

### Nigerian Pidgin

I start from the [Nigerian Pidgin Wikipedia](https://pcm.wikipedia.org/) with language code `pcm`, as requested. I will record the dump date, source page history, checksum, and applicable reuse terms when I ingest text. I cite the [Wikimedia Foundation reuse terms](https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use) and the [official Wikimedia dump index](https://dumps.wikimedia.org/) for the text source and attribution path.

I treat Pidgin variation as evidence, not an error category. I base that choice on linguistic work about a working Nigerian Pidgin orthography and documented variation, including [my reference to the orthography chapter](https://doi.org/10.1515/9781501513541-007) and [my reference to the Manchester language-contact study](https://languagecontact.humanities.manchester.ac.uk/McrLC/casestudies/FG_NigerianPidgin.html). I will keep spelling variants visible and suggestions optional.

## My open parallel-text research

### Tatoeba

I found Hausa and Igbo in the [official Tatoeba download listing](https://tatoeba.org/en/downloads). I read the download page as listing general sentence files under CC BY 2.0 France and a separate subset under CC0 1.0. I will favor the CC0 subset for a low-friction evaluation slice, keep sentence IDs and source metadata, and review attribution for any CC BY records before reuse. I have not confirmed a Nigerian Pidgin export, so I do not count Tatoeba as a Pidgin seed yet. I cite the [Tatoeba terms of use](https://tatoeba.org/en/terms_of_use) for per-sentence reuse terms.

### MAFAND-MT

I found MAFAND-MT as a parallel source covering Hausa, Igbo, and Nigerian Pidgin. I read the [dataset card](https://huggingface.co/datasets/masakhane/mafand/blob/main/README.md) as identifying CC BY-NC 4.0. I keep MAFAND out of shipped training data for now because I do not want a non-commercial restriction attached to Àmì's model assets.

### Okwu African Language Parallel Corpus

I found an [Okwu African Language Parallel Corpus repository](https://github.com/Okwu-AI/corpus) with a `pcm-eng` configuration. I read the [licensing notes](https://github.com/Okwu-AI/corpus/blob/main/LICENSING.md) as assigning CDLA-Permissive-2.0 to released data and CC BY 4.0 to documentation. I read the [getting-started note](https://github.com/Okwu-AI/corpus/blob/main/docs/getting-started.md) as describing source and record metadata, and the [limitations note](https://github.com/Okwu-AI/corpus/blob/main/docs/known-limitations.md) as describing everyday register and short vocabulary entries.

I keep the Okwu corpus as a promising Pidgin candidate, not an ingested source. I will pin an immutable release, inspect upstream provenance, and resolve the attribution notes before I use the source. I also found an attribution line in the licensing notes that invokes CC BY alongside the CDLA data license, so I will resolve that tension before ingestion.

### IgboAPI

I found the [IgboAPI dataset paper](https://arxiv.org/html/2405.00997v1). I read the paper as documenting a multi-dialect Igbo-English resource under CC BY-NC-SA 4.0. I exclude that dataset from shipped data while I review the non-commercial and share-alike terms.

## My intake checklist

I record an immutable source revision, retrieval date, SHA-256 checksum, language, license, attribution text, source URL, and exact preprocessing steps for every snapshot. I keep raw dumps outside Git. I keep source-specific provenance beside every generated frequency table and model artifact.

I do not mix license families without review. I do not claim training from any source that I have only listed here. I update the language rows on this page with real source revisions and word counts after each approved ingest. I follow the applicable Wikimedia license and attribution terms for each revision instead of flattening all wiki history into one label.

## My references

I cite [Wikimedia text reuse terms](https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use), [Wikimedia dump downloads](https://dumps.wikimedia.org/), [Tatoeba downloads](https://tatoeba.org/en/downloads), [Tatoeba reuse terms](https://tatoeba.org/en/terms_of_use), [MAFAND-MT licensing](https://huggingface.co/datasets/masakhane/mafand/blob/main/README.md), [Okwu corpus licensing](https://github.com/Okwu-AI/corpus/blob/main/LICENSING.md), [Okwu corpus metadata](https://github.com/Okwu-AI/corpus/blob/main/docs/getting-started.md), [Okwu corpus limitations](https://github.com/Okwu-AI/corpus/blob/main/docs/known-limitations.md), [IgboAPI licensing and scope](https://arxiv.org/html/2405.00997v1), [Nigerian Pidgin orthography research](https://doi.org/10.1515/9781501513541-007), and [the Manchester language-contact study](https://languagecontact.humanities.manchester.ac.uk/McrLC/casestudies/FG_NigerianPidgin.html).
