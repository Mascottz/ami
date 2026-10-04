# My architecture notes

I approved the stack and boundaries before I built the screen. I now have a working first slice across CUE, Julia, Rust, WebAssembly, Node, and plain browser code. I keep this page honest about what runs and what remains a target.

## My product boundary

I build Àmì as a local-first orthography engine for Hausa (`ha`), Igbo (`ig`), and Nigerian Pidgin (`pcm`). I keep keystroke processing in the tab. I send no typed text to a service, store no typing history, and add no analytics.

I run the static demo with `node server.mjs`. I compile WebAssembly on first run when the pinned Rust toolchain is available; I use no npm package, remote font, CDN script, or runtime service. I keep toolchains and dependency caches outside the repository.

## My runtime core

I use Rust for the runtime core. I build a grapheme-level prefix trie, bounded Damerau-Levenshtein matching, language-table diacritic assistance, phrase matching, and a data-driven form-family lookup. I compile the same crate for native tests and `wasm32-unknown-unknown`. I call a small C ABI from plain JavaScript, so I do not need a JavaScript package to load the core.

I normalize to NFC and compare extended grapheme clusters rather than bytes or Unicode scalar values. I use the [Unicode grapheme-boundary standard](https://unicode.org/reports/tr29/), the [Unicode normalization standard](https://unicode.org/reports/tr15/), the [Rust grapheme-segmentation crate](https://crates.io/crates/unicode-segmentation), and the [Rust normalization crate](https://crates.io/crates/unicode-normalization) as implementation references.

I keep Hausa hooked letters, Igbo subdots, tone-marked graphemes, and composed or decomposed input in native, JavaScript, and WebAssembly tests. I never assume ASCII for text processing. I return Unicode candidates without stripping distinctions from stored words or replacing text automatically.

I include one reference-checked Hausa family, `gida` and `gidaje`, in my current data. I have not built full morphology for Hausa, Igbo, or Nigerian Pidgin; I add a form family only with a cited and reviewed source.

## My performance target

I set a warm-tab target below 10 milliseconds at p95 for a suggestion pass. I wrote [`analysis/benchmark.mjs`](../analysis/benchmark.mjs) and measured 1,000 warmed Rust-to-Wasm calls on an Intel Xeon Processor at 2.60 GHz with Node 24.18.0, 29 demo seed entries, 469 frequency rows, and a 463,894-byte Wasm file. My latest local run measured a 0.026 ms median, 0.076 ms p95, 0.202 ms p99, and 0.951 ms maximum. I did not measure browser rendering, mobile hardware, or end-to-end keyboard latency, so I keep the browser target unverified.

## My offline statistics

I use Julia 1.13.1 for offline word counts, not for the keystroke path. I count Unicode letter-plus-mark tokens from one pinned Wikipedia article per language, retain the source spellings, and export deterministic UTF-8 counts. I use the counts only as a small ranking signal for selected demo words; I never present the table as representative language frequencies.

I use Julia's [Unicode and string documentation](https://docs.julialang.org/en/v1/manual/strings/) as the reference for text handling. I keep my statistics step free of package dependencies.

## My language tables

I use CUE 0.17.0 to validate checked-in inventories for Hausa hooks, Igbo subdots and overdots, tone samples, and the descriptive Pidgin policy. I pair structural checks with Rust's NFC and grapheme checks. I use the [CUE validation guide](https://cuelang.org/docs/concept/how-cue-enables-data-validation/) as the schema reference. I do not put runtime language logic in CUE.

## My static demo

I use Node's built-in HTTP and file-system modules to serve plain HTML, CSS, JavaScript, and WebAssembly. I let `node server.mjs` compile the Wasm file when the artifact is missing or stale. I add no runtime package, framework, remote font, CDN script, or analytics call. I use Node's [built-in HTTP API](https://nodejs.org/api/http.html) and [built-in test runner](https://nodejs.org/api/test.html) as runtime references.

I measured the local server's first startup at about three seconds with a warmed Cargo cache; the Wasm compile itself took 1.87 seconds in that run. I have not measured a clean-machine toolchain download or the browser's first load.

## My data path and provenance

I keep CUE-validated language tables separate from corpus text. I record a source URL, exact revision, date, checksum, license, attribution, and transformations in [`corpus/manifest.json`](../corpus/manifest.json). I keep raw extracts outside Git and check each file's SHA-256 before I let Julia read the text.

I use Julia to make a deterministic count table from my pinned extracts. I select a small subset for my demo lexicon, and Rust uses the source-pinned counts to rank the selected entries. I show a short evidence note and source tag; I do not show a made-up confidence score or present a suggestion as a universal correction.

I keep data licensing separate from my Apache-2.0 software license. I record the source-specific terms and attribution in [`data/NOTICE.md`](../data/NOTICE.md). I have not ingested open parallel text; I keep Tatoeba, MAFAND-MT, Okwu, and IgboAPI decisions in [my corpus notes](corpus.md).

I keep Pidgin descriptive. I preserve the demo spelling and phrase choices as optional suggestions, never mark Pidgin input wrong, and never force a replacement.

## My test floor

I run Rust unit tests for Hausa `ɓ`, `ɗ`, `ƙ`, and `ƴ`; Igbo `ị`, `ọ`, and `ụ`; tone-marked graphemes; composed and decomposed equivalents; case handling; morphology; whole-word suggestions; and Pidgin phrases. I run Julia Unicode-frequency tests, CUE rejection fixtures for hooked letters and subdots, and Node tests against the compiled WebAssembly core, grapheme-safe cursor helpers, and the diacritic fallback without `Intl.Segmenter`.

I have added a pinned workflow at [`.github/workflows/ci.yml`](../.github/workflows/ci.yml). I have not watched the workflow pass on GitHub because I have not published the repository yet.

## My first-run budget

I keep `node server.mjs` as the one-command demo target. I let my demo command build the Rust WebAssembly module if needed, using a pinned toolchain and the lockfile. I measured a warmed local start below five minutes; I have not measured a clean machine or a first-time toolchain install.

## My pinned toolchains

I pin Rust 1.99.0, Julia 1.13.1, CUE 0.17.0, and Node 24.18.0. I keep exact versions in small text files and keep binary downloads, package caches, and build output outside Git.

## My research links

I keep corpus provenance and licensing notes in [my corpus page](corpus.md). I use the official [Wikimedia reuse terms](https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use), the [Creative Commons notice on Wikimedia's 2023 license change](https://creativecommons.org/2023/06/29/wikipedia-moves-to-cc-4-0-licenses/), the [Tatoeba download and license page](https://tatoeba.org/en/downloads), and each upstream dataset's own license page before I ingest more text.
