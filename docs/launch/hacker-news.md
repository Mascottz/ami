# My Show HN draft

I keep this post unpublished until I have a public repository and a live demo URL.

## My title

Show HN: I’m building Àmì for Hausa, Igbo, and Nigerian Pidgin

## My post

I’m building Àmì as a small, local-first writing helper for Hausa, Igbo, and Nigerian Pidgin. I wanted a quiet way to keep language-specific marks and nearby forms within reach without sending typed text to a service.

I can type `how far` in the Pidgin demo and see `how far?`. I can type the three-word test `I dey fi` and see `I dey fine.`. I keep both as optional descriptive suggestions, never as corrections. I can also choose Hausa hooked letters or Igbo subdots from a small key row.

I wrote the runtime core in Rust and built a WebAssembly target. I use Julia for offline word counts, CUE for language-table validation, and Node's built-in modules for the local server. I use no npm packages or runtime service. My text stays in the browser tab after the page and engine load.

I started with one revision-pinned Wikipedia article per language. My derived table contains 469 word rows and 965 tokens. I do not present the table as representative frequency data or call my lexicon a trained model. I record each revision, checksum, attribution, license, and transformation, and keep raw extracts out of Git. I have one cited Hausa morphology pair, `gida` and `gidaje`, not a full morphology system.

I have local tests across Rust, Julia, CUE, WebAssembly, and Node. I measured a warmed Node-to-WebAssembly call on the demo slice, but I have not measured browser rendering, mobile hardware, or end-to-end keyboard latency. I have not watched the GitHub workflow pass yet.

I am looking for reviewable open corpus sources, cited morphology examples, and language-specific evaluation notes. I keep the product scope small until I can support a larger claim with evidence.

Repository: I will add the public URL before posting.

Demo: I will add a live URL before posting.
