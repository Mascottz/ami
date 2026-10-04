#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { mkdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const manifest = JSON.parse(await readFile(path.join(root, 'corpus', 'manifest.json'), 'utf8'));
const julia = process.env.JULIA_BIN || 'julia';
const temporaryOutput = path.join(root, '.cache', 'ami', 'corpus', 'wiki-frequencies.tsv');
await mkdir(path.dirname(temporaryOutput), { recursive: true });

function run(command, arguments_) {
  const result = spawnSync(command, arguments_, { cwd: root, stdio: 'inherit' });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`I could not complete ${command}.`);
}

run(process.execPath, [path.join(root, 'scripts', 'fetch-wikipedia.mjs')]);
run(julia, [path.join(root, 'analysis', 'build_frequencies.jl'), temporaryOutput]);

const artifact = manifest.derivedArtifacts.find(({ path: itemPath }) => itemPath === 'data/wiki-frequencies.tsv');
if (!artifact) throw new Error('I could not find my pinned frequency artifact record.');
const bytes = await readFile(temporaryOutput);
const digest = createHash('sha256').update(bytes).digest('hex');
if (digest !== artifact.sha256Utf8) {
  throw new Error(`I found a frequency-table checksum change: ${digest}.`);
}
console.log(`I verified ${artifact.uniqueWordRows} frequency rows and ${artifact.wordTokenCount} tokens.`);
