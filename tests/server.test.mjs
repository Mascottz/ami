import assert from 'node:assert/strict';
import test from 'node:test';
import { createStaticServer } from '../server.mjs';

test('I serve the local page and WebAssembly without extra packages', async (context) => {
  const server = createStaticServer();
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', resolve);
  });
  context.after(() => new Promise((resolve, reject) => {
    server.close((error) => error ? reject(error) : resolve());
  }));

  const address = server.address();
  const origin = `http://127.0.0.1:${address.port}`;
  const page = await fetch(origin);
  assert.equal(page.status, 200);
  assert.match(await page.text(), /Àmì/);
  assert.equal(page.headers.get('content-security-policy')?.includes('wasm-unsafe-eval'), true);

  const wasm = await fetch(`${origin}/ami_engine.wasm`);
  assert.equal(wasm.status, 200);
  assert.equal(wasm.headers.get('content-type'), 'application/wasm');
  assert.ok((await wasm.arrayBuffer()).byteLength > 1000);

  const missing = await fetch(`${origin}/not-in-my-demo`);
  assert.equal(missing.status, 404);
  assert.match(await missing.text(), /I could not find the requested page/);
});
