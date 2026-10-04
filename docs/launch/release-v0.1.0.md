# My v0.1.0 release draft

I have not published this release, created the public repository, or pushed a release tag. I keep the copy ready for review after I verify GitHub CI and enable private vulnerability reporting.

## My release summary

I ship the first local-first writing slice for Hausa (`ha`), Igbo (`ig`), and Nigerian Pidgin (`pcm`). I include optional diacritic assistance, whole-word suggestions, bounded fuzzy matching, one source-checked Hausa morphology pair, and descriptive Pidgin phrase suggestions.

I compile the runtime engine from Rust to WebAssembly. I use Julia for offline counts, CUE to validate language tables, and Node's built-in modules for a local server. I use no npm packages or runtime service.

## My verified local checks

I pass nine Rust tests, seven Julia tests, CUE language-table validation, Clippy with warnings denied, a release WebAssembly build, and eight Node tests. I pass `node scripts/verify-corpus.mjs` for three pinned Wikipedia revisions and a derived count table of 469 word rows and 965 tokens.

I have not watched the GitHub workflow pass on GitHub. I have not measured browser rendering, mobile hardware, or end-to-end keyboard latency.

## My current limits

I use one pinned Wikipedia article per language and a small reviewed demo slice. I do not call the counts representative or the lexicon complete. I have not completed parallel-text license review, native-speaker evaluation, or full morphology for any language. I keep Nigerian Pidgin descriptive and never present a suggestion as a correction.

## My release checks

- I verify the public repository and demo links.
- I verify the first GitHub Actions run.
- I enable private vulnerability reporting.
- I confirm source attribution, license notes, and clean generated-artifact status.
- I create and publish the `v0.1.0` tag only after all checks pass.
