const segmenter = typeof Intl.Segmenter === 'function'
  ? new Intl.Segmenter(undefined, { granularity: 'grapheme' })
  : null;

export function graphemeBoundaries(value) {
  if (!segmenter) throw new Error('I need grapheme-aware segmentation for cursor edits.');
  const boundaries = [0];
  for (const part of segmenter.segment(value)) {
    const boundary = part.index + part.segment.length;
    if (boundary !== boundaries.at(-1)) boundaries.push(boundary);
  }
  return boundaries;
}

export function snapSelectionToGraphemes(value, selectionStart, selectionEnd) {
  const boundaries = graphemeBoundaries(value);
  const start = Math.min(selectionStart, selectionEnd);
  const end = Math.max(selectionStart, selectionEnd);
  let snappedStart = 0;
  let snappedEnd = value.length;
  for (const boundary of boundaries) {
    if (boundary <= start) snappedStart = boundary;
    if (boundary >= end) {
      snappedEnd = boundary;
      break;
    }
  }
  return [snappedStart, snappedEnd];
}

export function insertAtGraphemeSelection(value, selectionStart, selectionEnd, insertion) {
  const [start, end] = snapSelectionToGraphemes(value, selectionStart, selectionEnd);
  const text = value.slice(0, start) + insertion + value.slice(end);
  const caret = start + insertion.length;
  return { text, caret };
}
