const { readFile } = require('node:fs/promises');
const { WASI } = require('wasi');
const { argv, env } = require('node:process');
const { join } = require('node:path');

const wasi = new WASI({
  version: 'preview1',
  args: argv,
  env,
});

(async () => {
  const wasm = await WebAssembly.compile(
    await readFile(join(__dirname, './target/wasm32-wasi-preview1-threads/debug/rspack.wasm')),
  );
  const memory = new WebAssembly.Memory({initial: 16384, maximum: 16384, shared:true});

  const cfg = {...wasi.getImportObject(),
    env: { Math_asos: Math.acos, Math_asin: Math.asin,memory },
    wasi: {},
  };
  console.log(cfg);
  const instance = await WebAssembly.instantiate(wasm, cfg);

  wasi.start(instance);
})();