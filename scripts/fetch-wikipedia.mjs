#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const manifestPath = path.join(root, 'corpus', 'manifest.json');
const manifest = JSON.parse(await readFile(manifestPath, 'utf8'));

for (const source of manifest.sources) {
  const response = await fetch(source.apiUrl, {
    headers: { 'user-agent': 'Àmì/0.1 (revision-pinned corpus intake)' },
    signal: AbortSignal.timeout(30_000),
  });
  if (!response.ok) {
    throw new Error(`I could not fetch my ${source.code} revision: HTTP ${response.status}.`);
  }
  const payload = await response.json();
  const page = payload?.query?.pages?.[0];
  const revision = page?.revisions?.[0];
  if (!page || page.pageid !== source.pageId || page.title !== source.title) {
    throw new Error(`I could not verify the pinned page for ${source.code}.`);
  }
  if (revision?.revid !== source.revisionId) {
    throw new Error(`I received a different revision for ${source.code}.`);
  }
  const text = Buffer.from(page.extract ?? '', 'utf8');
  const digest = createHash('sha256').update(text).digest('hex');
  if (digest !== source.sha256Utf8Plaintext) {
    throw new Error(`I found a checksum change for ${source.code}; I stop before using this text.`);
  }
  const outputPath = path.join(root, source.localRawPath);
  await mkdir(path.dirname(outputPath), { recursive: true });
  await writeFile(outputPath, text);
  console.log(`I verified ${source.code} revision ${source.revisionId} and kept its raw text outside Git.`);
}
