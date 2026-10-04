# My architecture proposal

I am proposing the stack and boundaries here before I build a screen. I want approval on this architecture before I turn the design note into a browser demo. I am describing a plan, not claiming that any component already exists.

## My product boundary

I build Àmì as a local-first orthography engine for Hausa (`ha`), Igbo (`ig`), and Nigerian Pidgin (`pcm`). I keep keystroke processing on the device. I send no typed text to a service, store no typing history, and add no analytics.

I keep the first demo small enough to run with Node and no account. I plan to include a reproducible WebAssembly build with the demo assets so a fresh checkout can start without installing Rust, Julia, or CUE. I keep toolchains and dependency caches out of the repository.

## My stack, with reasons

### Rust

I use Rust for the runtime core: a grapheme-aware prefix trie, bounded fuzzy matching, small language-specific morphology rules, and candidate generation. I compile the same core for native tests and WebAssembly; I keep browser typing local and avoid a network round trip.

I set a warm-tab target of under 10 milliseconds at the 95th percentile for a suggestion pass on the planned demo lexicon. I treat that number as a target until I publish a repeatable benchmark with hardware and corpus size.

I normalize to NFC and walk extended grapheme clusters rather than bytes or Unicode scalar values. I use the [Unicode grapheme-boundary standard](https://unicode.org/reports/tr29/), the [Unicode normalization standard](https://unicode.org/reports/tr15/), the [Rust grapheme-segmentation crate](https://crates.io/crates/unicode-segmentation), and the [Rust normalization crate](https://crates.io/crates/unicode-normalization) as implementation references.

I keep hooked letters, subdots, combining tone marks, and composed or decomposed forms in native and WebAssembly tests from the first engine commit. I never assume ASCII for text processing. I keep language-specific aliases for plain-keyboard input, then return the correct Unicode spelling without stripping distinctions from stored words.

### Julia

I use Julia for offline corpus statistics, not for the keystroke path. I count grapheme, letter, and word frequencies from licensed text snapshots, build deterministic rank tables, and export compact UTF-8 data for Rust. I keep the ranking recipe visible and reproducible, with no Julia package dependencies in the first release.

I use Julia's [Unicode and string documentation](https://docs.julialang.org/en/v1/manual/strings/) as the reference for text handling. I pin the Julia version in the project and CI rather than relying on whichever local version happens to be installed.

### CUE

I use CUE for language-table data and validation. I plan to keep the tables for Hausa hooked letters, Igbo subdots, and supported tone marks in checked-in data, then validate required fields and allowed inventories with `cue vet` before a build or release.

I pair CUE schema checks with Rust Unicode tests because I need both structural validation and grapheme-aware validation. I use the [CUE validation guide](https://cuelang.org/docs/concept/how-cue-enables-data-validation/) as the schema reference. I do not put runtime language logic in CUE.

### Node with no runtime packages

I use Node's built-in HTTP and file-system modules to serve the static demo. I write the page in plain HTML, CSS, and JavaScript, and I add no package dependency, framework, remote font, CDN script, or analytics call.

I run the compiled WebAssembly core directly in the browser. I use Node only as the tiny local file server, so typed text stays inside the tab. I use Node's [built-in HTTP API](https://nodejs.org/api/http.html) and [built-in test runner](https://nodejs.org/api/test.html) as the runtime references.

## My data path

I keep CUE-validated language tables separate from corpus text. I record a source URL, exact upstream revision or dump date, checksum, license, attribution note, and transformation steps for every corpus snapshot. I keep raw dumps out of Git and check in only compact derived data when the source terms allow redistribution.

I use Julia to produce deterministic frequency tables from approved snapshots. I build a trie and morphology rules in Rust, then rank candidates from the generated counts, prefix fit, and bounded grapheme-level edit distance. I expose the score or a short evidence note in the demo rather than pretending a suggestion is a universal correction.

I keep the Pidgin model descriptive. I preserve observed variants, surface nearby forms as optional suggestions, and never mark a Pidgin spelling as wrong. I do not force an automatic replacement in any language.

## My Unicode test floor

I pin expected behavior by language before I add more data. I test Hausa `ɓ`, `ɗ`, `ƙ`, and `ƴ`; Igbo `ị`, `ọ`, and `ụ`; combining tone marks; composed and decomposed equivalents; case handling; and cursor-safe grapheme boundaries. I test `how far` as a Pidgin phrase and keep distinct Pidgin spellings eligible for descriptive suggestions.

I run Rust unit tests, Julia tests, CUE validation fixtures, Node's built-in test runner, and a WebAssembly build in continuous integration. I pin language toolchains and lock Rust dependencies. I keep CI logs free of corpus text and keystroke samples.

## My demo and repository budget

I plan a single command, `node server.mjs`, for the demo. I include only the small, reproducible WebAssembly module and compact demo data needed for that command. I keep Rust, Julia, CUE, and Node toolchains in local or CI caches, never in the repository.

I plan to keep the first-time demo path below five minutes on a machine with Node installed. I will measure that path before I call the path done. I have not built the server, engine, language tables, or screen yet.

## My research links

I keep corpus provenance and licensing notes in [my corpus page](corpus.md). I use the official [Wikimedia reuse terms](https://foundation.wikimedia.org/wiki/Policy:Terms_of_Use), the [Tatoeba download and license page](https://tatoeba.org/en/downloads), and each upstream dataset's own license page before I ingest text.
