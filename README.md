# Àmì

I am building Àmì as a local-first orthography engine for Hausa, Igbo, and Nigerian Pidgin. I offer diacritic assistance, whole-word suggestions, fuzzy matching, a small source-checked Hausa morphology family, and descriptive Pidgin phrase suggestions.

I keep `ami` as the ASCII repository slug. I write the product name as Àmì everywhere else.

## What I have built

I have a responsive writing screen, a Rust engine compiled to WebAssembly, Unicode grapheme-aware matching, Julia word counts, and CUE-validated language tables. I keep the code and model data local to the browser tab. I do not replace text until I explicitly accept a suggestion.

I can type `how far` in the Pidgin demo and see `how far?`. I can type the three-word test `I dey fi` and see `I dey fine.`. I keep both as optional phrase suggestions, not corrections.

## What remains a demo

I use one pinned Wikipedia article per language and a small, manually reviewed entry slice. My 965-token, 469-row count table is not representative of Hausa, Igbo, or Nigerian Pidgin. I make no trained-language-model or complete-dictionary claim.

I include one source-checked Hausa pair, `gida` and `gidaje`, as an optional morphology neighbor. I have not built full morphology for any language. I keep Pidgin descriptive, show only a few demo phrases, and never mark Pidgin text wrong or underline it as an error.

I have not added open parallel text, native-speaker evaluation, browser performance measurements, a public GitHub release, or a v0.1.0 tag. I leave each item open instead of implying completion.

## Run my demo

I pin Node 24.18.0, Rust 1.99.0, Julia 1.13.1, and CUE 0.17.0. I start the demo with one command:

```sh
node server.mjs
```

I let the server build the WebAssembly file when the artifact is missing or stale. I do not use npm packages or a runtime service. I keep my local toolchain cache outside the repository.

## Run my checks

I run my pinned Rust, Julia, CUE, WebAssembly, and Node tests with one command:

```sh
scripts/test-all.sh
```

I verify the pinned source checksums and Julia output with `node scripts/verify-corpus.mjs`. I need network access to verify my pinned sources; I do not need an external service to use the running demo.

## My project notes

- [My design note](docs/design.md)
- [My architecture notes](docs/architecture.md)
- [My corpus notes](docs/corpus.md)
- [My data notice](data/NOTICE.md)
- [My pinned corpus manifest](corpus/manifest.json)

## My license

I license my original code and project documentation under [Apache-2.0](LICENSE). I keep corpus-derived data under the source-specific terms and attribution in [my data notice](data/NOTICE.md).
