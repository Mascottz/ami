# Àmì

I got tired of autocorrect treating Hausa, Igbo, and Naija Pidgin like typos for twenty years. I am building Àmì as a local-first orthography engine with diacritic assistance, whole-word suggestions by language, and a descriptive Pidgin model that suggests without lecturing.

I keep `ami` as the ASCII repository slug. I write the product name as Àmì everywhere else.

## What I want to make

I want `b` to put Hausa `ɓ` one tap away. I want `o` to offer Igbo `ọ`. I want `how far` to get a useful Naija Pidgin suggestion, with no red underline or shame.

## What I have built so far

I have approved the design and architecture notes before implementation. I have not built a browser screen, engine, or trained language model yet. I have not ingested external source text. I treat this README as a project promise, not a shipping claim.

## My project notes

I keep the design system, stack reasoning, corpus sources, and license notes in these files:

- [My design note](docs/design.md)
- [My architecture proposal](docs/architecture.md)
- [My corpus notes](docs/corpus.md)

## My license

I license my original code and project documentation under [Apache-2.0](LICENSE). I keep third-party corpus artifacts under separate source-specific terms and notices.

## My bar

I want a stranger to type three words of Pidgin, see a useful suggestion, understand the reason, and choose whether to keep the suggestion. I want every test and every source to make that promise provable.
