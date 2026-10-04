import assert from 'node:assert/strict';
import test from 'node:test';

class FakeElement {
  constructor(dataset = {}) {
    this.dataset = dataset;
    this.listeners = new Map();
    this.children = [];
    this.classList = { toggle: () => {} };
    this.value = '';
    this.hidden = false;
    this.selectionStart = 0;
    this.selectionEnd = 0;
  }

  addEventListener(name, callback) {
    this.listeners.set(name, callback);
  }

  setAttribute() {}

  replaceChildren(...children) {
    this.children = children;
  }

  append(...children) {
    this.children.push(...children);
  }

  focus() {}

  setSelectionRange() {}

  dispatchEvent() {}
}

test('I keep diacritic insertion quiet without Intl.Segmenter', async () => {
  const originalSegmenter = Object.getOwnPropertyDescriptor(Intl, 'Segmenter');
  const originalFetch = globalThis.fetch;
  const originalDocument = globalThis.document;
  const languageButtons = ['ha', 'ig', 'pcm'].map((code) => new FakeElement({ language: code }));
  const selectors = [
    '#editor',
    '#engine-status',
    '#language-code',
    '#word-count',
    '#suggestions',
    '#empty-state',
    '#language-note',
    '#diacritic-keys',
    '#diacritic-note',
    '#load-three-word',
    '#clear-writing',
  ];
  const elements = new Map(selectors.map((selector) => [selector, new FakeElement()]));
  let createdElements = 0;
  globalThis.document = {
    querySelector: (selector) => elements.get(selector),
    querySelectorAll: () => languageButtons,
    createElement: () => {
      createdElements += 1;
      return new FakeElement();
    },
  };
  globalThis.fetch = async () => ({ ok: false, status: 404 });
  Object.defineProperty(Intl, 'Segmenter', { configurable: true, writable: true, value: undefined });

  try {
    await import(`../web/app.mjs?fallback=${Date.now()}`);
    languageButtons.find((button) => button.dataset.language === 'ha').listeners.get('click')();

    assert.equal(elements.get('#editor').lang, 'ha');
    assert.deepEqual(elements.get('#diacritic-keys').children, []);
    assert.equal(
      elements.get('#diacritic-note').textContent,
      'I keep insertion quiet when grapheme-safe cursor support is unavailable.',
    );
    assert.equal(createdElements, 0);
  } finally {
    Object.defineProperty(Intl, 'Segmenter', originalSegmenter);
    globalThis.fetch = originalFetch;
    if (originalDocument === undefined) delete globalThis.document;
    else globalThis.document = originalDocument;
  }
});
