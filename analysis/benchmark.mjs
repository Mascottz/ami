import { readFile } from 'node:fs/promises';
import { cpus } from 'node:os';
import { performance } from 'node:perf_hooks';
import { attachAmiEngine } from '../web/ami-engine.mjs';

const wasmUrl = new URL('../engine/target/wasm32-unknown-unknown/release/ami_engine.wasm', import.meta.url);
const lexiconUrl = new URL('../data/demo-lexicon.tsv', import.meta.url);
const bytes = await readFile(wasmUrl);
const lexiconText = await readFile(lexiconUrl, 'utf8');
const lexiconRows = lexiconText.split(/\r?\n/u).filter((line) => line && !line.startsWith('#')).length;
const cpuModel = cpus()[0]?.model || 'unknown CPU';
const { instance } = await WebAssembly.instantiate(bytes);
const engine = attachAmiEngine(instance);
const inputs = [
  ['pcm', 'how far'],
  ['pcm', 'I dey fi'],
  ['ha', 'kasa'],
  ['ig', 'oma'],
];

for (let index = 0; index < 100; index++) engine.suggest(...inputs[index % inputs.length]);
const samples = [];
for (let index = 0; index < 1000; index++) {
  const start = performance.now();
  engine.suggest(...inputs[index % inputs.length]);
  samples.push(performance.now() - start);
}
samples.sort((left, right) => left - right);
const percentile = (value) => samples[Math.ceil(samples.length * value) - 1];
console.log(`I measured 1,000 warmed Wasm calls in Node ${process.version} on ${cpuModel}, with ${lexiconRows} seed entries and ${bytes.byteLength} Wasm bytes; median ${percentile(0.5).toFixed(3)} ms, p95 ${percentile(0.95).toFixed(3)} ms, p99 ${percentile(0.99).toFixed(3)} ms, max ${samples.at(-1).toFixed(3)} ms.`);
console.log('I did not measure browser rendering, mobile hardware, or end-to-end keyboard latency.');
