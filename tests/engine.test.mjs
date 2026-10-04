import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import { attachAmiEngine } from '../web/ami-engine.mjs';
import { graphemeBoundaries, insertAtGraphemeSelection } from '../web/graphemes.mjs';

const wasmPath = new URL('../engine/target/wasm32-unknown-unknown/release/ami_engine.wasm', import.meta.url);
const wasmBytes = await readFile(wasmPath);
const { instance } = await WebAssembly.instantiate(wasmBytes);
const engine = attachAmiEngine(instance);

test('I receive the Pidgin greeting from my local Wasm core', () => {
  const result = engine.suggest('pcm', 'how far');
  assert.equal(result.suggestions[0].text, 'how far?');
  assert.equal(result.suggestions[0].kind, 'phrase');
});

test('I get a descriptive suggestion after three Pidgin words', () => {
  const result = engine.suggest('pcm', 'I dey fi');
  assert.ok(result.suggestions.some(({ text, kind }) => text === 'I dey fine.' && kind === 'phrase'));
});

test('I keep Hausa hooks and Igbo subdots in local suggestions', () => {
  const hausa = engine.suggest('ha', 'B');
  const igbo = engine.suggest('ig', 'oma');
  assert.ok(hausa.suggestions.some(({ text }) => text === 'Ɓ'));
  assert.ok(igbo.suggestions.some(({ text }) => text === 'ọma'));
});

test('I snap cursor edits to whole graphemes', () => {
  const value = 'aị́b';
  const boundaries = graphemeBoundaries(value);
  assert.deepEqual(boundaries, [0, 1, 3, 4]);
  const result = insertAtGraphemeSelection(value, 2, 2, 'ọ');
  assert.equal(result.text, 'aọb');
  assert.equal(result.caret, 2);
});

test('I leave a correction choice out of the input until I accept it', () => {
  const original = 'how far';
  const result = engine.suggest('pcm', original);
  assert.equal(original, 'how far');
  assert.equal(result.text, original);
  assert.equal(result.suggestions[0].text, 'how far?');
});
