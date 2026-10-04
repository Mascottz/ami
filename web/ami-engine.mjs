const encoder = new TextEncoder();
const decoder = new TextDecoder();

export function attachAmiEngine(instance) {
  const { memory, ami_alloc, ami_free_input, ami_run, ami_free_output } = instance.exports;
  if (!memory || !ami_alloc || !ami_free_input || !ami_run || !ami_free_output) {
    throw new Error('I could not find my Àmì WebAssembly exports.');
  }

  return {
    suggest(language, text) {
      const request = encoder.encode(JSON.stringify({ language, text }));
      const inputPointer = ami_alloc(request.length);
      new Uint8Array(memory.buffer, inputPointer, request.length).set(request);
      const outputLengthPointer = ami_alloc(4);
      let outputPointer = 0;
      let outputLength = 0;
      try {
        outputPointer = ami_run(inputPointer, request.length, outputLengthPointer);
        if (outputPointer === 0) throw new Error('I did not receive a response buffer.');
        outputLength = new DataView(memory.buffer).getUint32(outputLengthPointer, true);
        const responseBytes = new Uint8Array(memory.buffer, outputPointer, outputLength);
        const response = JSON.parse(decoder.decode(responseBytes));
        if (response.error) throw new Error(response.error);
        return response;
      } finally {
        if (outputPointer !== 0) ami_free_output(outputPointer, outputLength);
        ami_free_input(inputPointer, request.length);
        ami_free_input(outputLengthPointer, 4);
      }
    },
  };
}

export async function loadAmiEngine(url = new URL('./ami_engine.wasm', import.meta.url)) {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`I could not load my local engine: HTTP ${response.status}.`);
  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes);
  return attachAmiEngine(instance);
}
