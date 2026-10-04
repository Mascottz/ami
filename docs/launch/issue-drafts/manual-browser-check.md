---
title: I document my manual browser smoke check
labels:
  - good first issue
  - documentation
---

# I document my manual browser smoke check

## Why I want this

I have Node-to-WebAssembly tests and a grapheme-safe cursor helper. I have not written a repeatable manual checklist for the real writing screen in a browser.

## My scope

- I write a short checklist for a current desktop browser and one mobile browser.
- I check the language switch, keyboard-only focus, a diacritic-key insertion, and suggestion acceptance or dismissal.
- I type `how far` and `I dey fi` in Pidgin and confirm that I see suggestions without a red underline or forced replacement.
- I note the browser name and version, viewport, and any observed issue.
- I keep the checklist dependency-free and store no personal writing sample.

## I call this done when

- I can follow the steps from a fresh `node server.mjs` session.
- I can record pass, fail, or unverified for each check.
- I keep browser measurements separate from my Node microbenchmark.

## My boundaries

I do not claim that a manual check proves broad browser compatibility. I do not add a browser automation package or a runtime service for this documentation task.
