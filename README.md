# PAPI + WASM Light Client Demo

A demo web app that connects to Polkadot Asset Hub using [polkadot-api (PAPI)](https://papi.how/) with a JSON-RPC provider fully implemented in Rust/WASM.

The WASM module embeds [smoldot](https://github.com/smol-dot/smoldot) (a Rust light client) and exposes raw JSON-RPC send/receive functions that the JS side wires into PAPI's `createClient()`. This means the entire light client runs in the browser as a WASM module, with no external RPC dependency.

## What it shows

- **Live block streaming** from Asset Hub via PAPI's `blocks$` observable
- **Account balance query** for a well-known account (Alice), using PAPI's `watchValue` with live updates
- **Two modes** to compare:
  - **WASM Light Client** (`/`) - smoldot compiled to WASM, fully trustless, no RPC server needed
  - **WebSocket RPC** (`/?rpc=wss://polkadot-asset-hub-rpc.polkadot.io`) - traditional RPC connection

## Architecture

```
Browser
  index.html
    +-- app-bundle.js (esbuild)
    |     +-- polkadot-api: createClient()
    |     +-- main.js: provider glue + UI
    |
    +-- WASM binary (Rust, wasm-pack)
          +-- smoldot-light 1.0 (Rust light client)
          +-- WASM platform (vendored from subxt-lightclient)
          +-- Exports: WasmProvider { new, start, send, disconnect }
```

The `WasmProvider` WASM export implements PAPI's `JsonRpcProvider` interface:

```js
const provider = (onMessage) => {
    wasmProvider.start((resp) => onMessage(JSON.parse(resp)));
    return {
        send: (msg) => wasmProvider.send(JSON.stringify(msg)),
        disconnect: () => wasmProvider.disconnect(),
    };
};
const client = createClient(provider);
```

## Prerequisites

- [Rust](https://rustup.rs/) with the `wasm32-unknown-unknown` target
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/)

## Build and run

```bash
npm install
make build
make serve
```

Then open http://localhost:8081.

## Project structure

| File | Description |
|------|-------------|
| `src/lib.rs` | `WasmProvider` - wraps smoldot-light, exports JSON-RPC interface to JS via wasm-bindgen |
| `src/platform/` | WASM platform implementation for smoldot (vendored from subxt-lightclient) |
| `main.js` | JS entry point - initializes WASM provider or WebSocket, creates PAPI client, renders UI |
| `index.html` | Single page with dark theme, mode toggle, balance display, and block list |
| `build.js` | esbuild config - bundles main.js + PAPI deps, keeps wasm-pack output external |
| `polkadot.json` | Polkadot relay chain spec (for smoldot) |
| `asset_hub.json` | Asset Hub parachain spec (for smoldot) |
