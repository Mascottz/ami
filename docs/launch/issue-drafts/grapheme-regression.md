---
title: I add a grapheme regression for one new orthography form
labels:
  - good first issue
  - testing
---

# I add a grapheme regression for one new orthography form

## Why I want this

I already test Hausa hooks, Igbo subdots, and composed or decomposed input. I want every new orthography example to keep its grapheme boundaries in Rust, JavaScript, and WebAssembly.

## My scope

- I choose one real, source-backed Hausa or Igbo form that is missing from the current fixtures.
- I write a test that compares the NFC and decomposed spelling without splitting a grapheme.
- I check the cursor helper when the fixture includes a cursor edit.
- I run `scripts/test-all.sh` and keep the change limited to the fixture, test, and any needed source note.

## I call this done when

- I cite the source for the chosen form.
- I show the same expected grapheme behavior in the relevant tests.
- I keep all typed text unchanged until an explicit suggestion action.

## My boundaries

I do not add an unsupported spelling to make a test pass. I do not assume ASCII, strip marks, or add a runtime package.
