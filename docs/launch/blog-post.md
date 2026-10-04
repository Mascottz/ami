# I built a small local-first writing tool for Hausa, Igbo, and Nigerian Pidgin

I wanted to keep language-specific marks and nearby spellings close while typing. I did not want a web service to receive my text. I started Àmì as a local-first orthography engine for Hausa (`ha`), Igbo (`ig`), and Nigerian Pidgin (`pcm`).

I wrote the first version as a small, inspectable slice rather than a full dictionary or language model. I can type `how far` in the Pidgin demo and see `how far?`. I can type three words, `I dey fi`, and see `I dey fine.`. I keep both as optional suggestions, not verdicts. I do not underline Pidgin as wrong or replace a phrase without an explicit action.

I also keep Hausa hooked letters and Igbo subdots nearby. I can choose a mark at the cursor, and I keep cursor edits aligned to whole graphemes. I preserve each letter cluster, especially Hausa `ɓ` and Igbo `ị`; I never treat a diacritic as an ASCII byte or split a grapheme casually. I include one source-checked Hausa family, `gida` and `gidaje`, as a morphology neighbor. I have not built complete morphology for any of the three languages.

I kept the runtime local and boring on purpose. I wrote the engine in Rust, compiled the engine to WebAssembly, and used plain browser JavaScript to connect the core to the writing surface. I use Julia to count words offline and CUE to check language tables. I use Node's built-in modules to serve the demo, with no npm packages, external font request, analytics call, or runtime service. After the page and engine load, my typed text stays in the tab.

I started corpus work with one revision-pinned Wikipedia article per language. I record exact revision IDs, retrieval dates, checksums, license bases, attribution, and transformations in my [corpus notes](../corpus.md) and [manifest](../../corpus/manifest.json). I keep raw extracts out of Git. I use Julia to produce 469 word rows from 965 tokens. I use the counts only to rank a small selected demo slice; I do not present the table as balanced language-frequency evidence. I have not added parallel text because I still need to finish license review for candidate sources.

I built the design system before the screen. I use warm paper, night ink, iris, mineral blue, and a restrained citron accent. I use a serif display face with monospace language and data labels. I keep the layout open and editorial, not dashboard-like. I also keep a small [social preview](../assets/social-preview.png) built from the same tokens.

I have local tests for grapheme-safe matching, Hausa hooks, Igbo subdots, morphology lookup, and descriptive Pidgin suggestions. My current suite passes nine Rust tests, seven Julia tests, CUE validation, a release WebAssembly build, and eight Node tests. I also measured one thousand warmed Node-to-WebAssembly calls on the small demo slice. In my latest local run, the p95 was 0.076 milliseconds. I have not measured browser rendering, mobile hardware, or end-to-end keyboard latency, and I have not watched the GitHub workflow pass on GitHub.

I still need more licensed text, native-speaker review, broader morphology, and a real browser performance measurement. I am keeping Àmì small until I can support every larger claim with sources and tests. I have not published the repository or a live demo yet, so I will insert both links before I publish this post.

## My references

I keep my pinned sources, license notes, and open dataset research in [my corpus notes](../corpus.md). I keep the current implementation boundary and test status in [my architecture notes](../architecture.md).
