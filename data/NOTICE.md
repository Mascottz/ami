# My data notes

I keep my software license separate from my language data. I license my original software under Apache-2.0. I license my selected and arranged demo-data collection under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/), while I preserve the source-specific license and attribution recorded in [`corpus/manifest.json`](../corpus/manifest.json).

I cite the following pinned sources for the rows tagged `wiki:ha:873901`, `wiki:ig:97975`, and `wiki:pcm:36150`:

- I attribute my Hausa word rows to [Harshen Hausa, revision 873901](https://ha.wikipedia.org/w/index.php?oldid=873901), under CC BY-SA 4.0.
- I attribute my Igbo word rows to [Asụsụ Igbo, revision 97975](https://ig.wikipedia.org/w/index.php?oldid=97975), under CC BY-SA 3.0 Unported.
- I attribute my Pidgin word rows to [Naijá langwej, revision 36150](https://pcm.wikipedia.org/w/index.php?oldid=36150), under CC BY-SA 4.0.

I keep the license on each Wikipedia-derived row attached through its source key. I use Julia's pinned-page counts to rank my Wikipedia-derived rows. I keep hand-set weights on my manual smoke-test rows. I use only a few reviewed entries from the small extracts I fetched; I do not treat my count table as a balanced corpus.

I label my orthography smoke-test entries `manual:orthography`. I label my hand-picked Pidgin phrase tests `first-party:prompt` and `manual:phrase`. I keep my smoke tests in the demo lexicon so I can test optional suggestions; I do not present the lexicon as complete coverage or a prescriptive Pidgin standard.

I keep one Hausa form family, `gida` and `gidaje`, in [`morphology.tsv`](morphology.tsv). I cite [UCLA's Hausa noun-modifier examples](https://aflang.humanities.ucla.edu/language-materials/chadic-languages/hausa/hausa-online-grammar/noun-modifiers/) for the pair. I copy only the word forms as a factual example; I do not include the page's prose or definitions.

I keep the raw extracts out of Git. I keep each revision ID, timestamp, SHA-256 checksum, retrieval detail, transformation, license basis, and attribution link in the corpus manifest.
