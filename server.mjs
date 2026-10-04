#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { statSync } from 'node:fs';
import { access, readFile, stat } from 'node:fs/promises';
import { createServer } from 'node:http';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.dirname(fileURLToPath(import.meta.url));
const PUBLIC = path.join(ROOT, 'web');
const WASM_PATH = path.join(ROOT, 'engine', 'target', 'wasm32-unknown-unknown', 'release', 'ami_engine.wasm');
const BUILD_INPUTS = [
  'engine/Cargo.toml',
  'engine/Cargo.lock',
  'engine/src/lib.rs',
  'language/tables.json',
  'data/demo-lexicon.tsv',
  'data/morphology.tsv',
  'data/wiki-frequencies.tsv',
];

async function exists(file) {
  try {
    await access(file);
    return true;
  } catch {
    return false;
  }
}

function rustEnvironment() {
  const env = { ...process.env };
  const cacheRoot = path.join(os.homedir(), '.cache', 'ami');
  const cachedCargo = path.join(cacheRoot, 'cargo');
  const cachedRustup = path.join(cacheRoot, 'rustup');
  if (!env.CARGO_HOME && existsSync(cachedCargo)) env.CARGO_HOME = cachedCargo;
  if (!env.RUSTUP_HOME && existsSync(cachedRustup)) env.RUSTUP_HOME = cachedRustup;
  const cargoBin = path.join(env.CARGO_HOME || cachedCargo, 'bin');
  env.PATH = [cargoBin, env.PATH || ''].filter(Boolean).join(path.delimiter);
  return env;
}

function existsSync(file) {
  try {
    statSync(file);
    return true;
  } catch {
    return false;
  }
}

function isFileSync(file) {
  try {
    return statSync(file).isFile();
  } catch {
    return false;
  }
}

function findCargo(env) {
  const candidates = [
    process.env.CARGO,
    env.CARGO_HOME ? path.join(env.CARGO_HOME, 'bin', 'cargo') : null,
    ...String(env.PATH || '').split(path.delimiter).map((directory) => path.join(directory, 'cargo')),
  ].filter(Boolean);
  return candidates.find((candidate) => existsSync(candidate) && isFileSync(candidate));
}

async function needsBuild() {
  if (!(await exists(WASM_PATH))) return true;
  const wasmTime = (await stat(WASM_PATH)).mtimeMs;
  for (const input of BUILD_INPUTS) {
    if ((await stat(path.join(ROOT, input))).mtimeMs > wasmTime) return true;
  }
  return false;
}

export async function ensureWasm() {
  if (!(await needsBuild())) {
    console.log('I reuse my current local WebAssembly build.');
    return WASM_PATH;
  }
  const env = rustEnvironment();
  const cargo = findCargo(env);
  if (!cargo) throw new Error('I could not find Cargo in my path or local toolchain cache.');
  console.log('I build my local Rust engine for WebAssembly.');
  const result = spawnSync(cargo, [
    'build',
    '--release',
    '--locked',
    '--manifest-path', path.join(ROOT, 'engine', 'Cargo.toml'),
    '--target', 'wasm32-unknown-unknown',
  ], { cwd: ROOT, env, stdio: 'inherit' });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error('I could not build my WebAssembly engine.');
  return WASM_PATH;
}

function contentType(file) {
  const extension = path.extname(file);
  return ({
    '.html': 'text/html; charset=utf-8',
    '.css': 'text/css; charset=utf-8',
    '.mjs': 'text/javascript; charset=utf-8',
    '.wasm': 'application/wasm',
    '.svg': 'image/svg+xml',
    '.json': 'application/json; charset=utf-8',
  })[extension] || 'application/octet-stream';
}

function applyHeaders(response, type) {
  response.setHeader('Content-Type', type);
  response.setHeader('Cache-Control', 'no-store');
  response.setHeader('X-Content-Type-Options', 'nosniff');
  response.setHeader('Referrer-Policy', 'no-referrer');
  response.setHeader('Permissions-Policy', 'camera=(), microphone=(), geolocation=()');
  response.setHeader(
    'Content-Security-Policy',
    "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; connect-src 'self'; img-src 'self' data:; font-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'",
  );
}

export function createStaticServer() {
  return createServer(async (request, response) => {
    if (request.method !== 'GET' && request.method !== 'HEAD') {
      response.writeHead(405, { 'Content-Type': 'text/plain; charset=utf-8', Allow: 'GET, HEAD' });
      response.end('I allow only GET and HEAD requests.');
      return;
    }

    let pathname;
    try {
      pathname = decodeURIComponent(new URL(request.url, 'http://local.invalid').pathname);
    } catch {
      response.writeHead(400, { 'Content-Type': 'text/plain; charset=utf-8' });
      response.end('I could not read the requested path.');
      return;
    }

    let file;
    if (pathname === '/ami_engine.wasm') {
      file = WASM_PATH;
    } else {
      const relative = pathname === '/' ? 'index.html' : pathname.replace(/^\/+/, '');
      file = path.resolve(PUBLIC, relative);
      if (!file.startsWith(`${PUBLIC}${path.sep}`)) {
        response.writeHead(404, { 'Content-Type': 'text/plain; charset=utf-8' });
        response.end('I could not find the requested page.');
        return;
      }
    }

    try {
      const bytes = await readFile(file);
      applyHeaders(response, contentType(file));
      response.writeHead(200);
      response.end(request.method === 'HEAD' ? undefined : bytes);
    } catch {
      response.writeHead(404, { 'Content-Type': 'text/plain; charset=utf-8' });
      response.end('I could not find the requested page.');
    }
  });
}

export async function startServer() {
  await ensureWasm();
  const server = createStaticServer();
  const port = Number(process.env.PORT || 4173);
  server.listen(port, '0.0.0.0', () => {
    console.log(`I serve my Àmì demo on port ${port}.`);
  });
  return server;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  startServer().catch((error) => {
    console.error(error.message.startsWith('I ') ? error.message : `I could not start my demo: ${error.message}`);
    process.exitCode = 1;
  });
}
