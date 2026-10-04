# My contributing notes

I welcome small, evidence-led changes that make Àmì more useful without making its claims bigger than its data.

## My local checks

I run `scripts/test-all.sh` before I submit a code change. I run `node scripts/verify-corpus.mjs` after I change a pinned source, a preprocessing step, or a generated count table. I keep the test output free of raw corpus text and personal typing samples.

## My language data

I record a source URL, immutable revision or release, retrieval date, SHA-256 checksum, language, license, attribution, and exact transformations for every source that influences a suggestion. I keep raw dumps outside Git. I review license compatibility before I add parallel text or derived data.

I keep Hausa hooks, Igbo subdots, tone-marked graphemes, and composed or decomposed equivalents intact. I add grapheme-aware tests with every change to Unicode handling. I do not split text by bytes or assume ASCII.

I keep Nigerian Pidgin descriptive. I add spelling variants as optional evidence, never as a verdict. I do not add red underlines, forced replacements, or language-policing copy.

## My code and copy

I keep Node free of runtime packages. I keep local toolchains in the cache and generated build output out of Git. I write project copy in my own first-person, casual, direct voice. I avoid em dashes and keep source text, license text, and technical identifiers exactly as required.

## My changes

I keep each change narrow. I update the design or architecture notes before I introduce a new interaction or runtime boundary. I include the reason, a test, and source provenance when a change alters suggestions or rankings. I do not claim coverage that I have not measured.
