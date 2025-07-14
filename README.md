# base16.wasm ([deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/toHex))

WebAssembly port of Base16

```bash
npm i @hazae41/base16.wasm
```

[**Node Package 📦**](https://www.npmjs.com/package/@hazae41/base16.wasm)

## Features
- Reproducible building
- Pre-bundled and streamed
- Zero-copy memory slices

## Modules
- base16ct

## Algorithms
- Base16

## Usage

```typescript
import { Base16Wasm, base16_encode_lower, base16_decode_mixed } from "@hazae41/base16.wasm";

// Wait for WASM to load
await Base16Wasm.initBundled();

const bytes = crypto.getRandomValues(new Uint8Array(256))
using memory = new Memory(bytes)

const text = base16_encode_lower(memory)
using memory2 = base16_decode_mixed(text)

console.log(memory2.bytes)
```

## Building

### Unreproducible building

You need to install [Rust](https://www.rust-lang.org/tools/install)

Then, install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
cargo install wasm-pack
```

Finally, do a clean install and build

```bash
npm ci && npm run build
```

### Reproducible building

You can build the exact same bytecode using Docker, just be sure you're on a `linux/amd64` host

```bash
docker compose up --build
```

Then check that all the files are the same using `npm diff`

```bash
npm diff
```

If the output is empty then the bytecode is the same as the one I commited

### Automated checks

Each time I release a new version on GitHub, the GitHub's CI clones the GitHub repository, reproduces the build, and throws an error if the NPM release is different. If a version is present on NPM but not on GitHub, do not use it!
