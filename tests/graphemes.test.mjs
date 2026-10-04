import assert from 'node:assert/strict';
import test from 'node:test';
import { graphemeBoundaries, snapSelectionToGraphemes } from '../web/graphemes.mjs';

test('I keep a hooked letter and a tone-marked Igbo cluster whole', () => {
  assert.deepEqual(graphemeBoundaries('ɓị́ọ'), [0, 1, 3, 4]);
  assert.deepEqual(snapSelectionToGraphemes('ɓị́ọ', 2, 2), [1, 3]);
});
