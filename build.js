import * as esbuild from "esbuild";
import { cpSync } from "fs";

// Bundle main.js + polkadot-api deps, keep wasm-pack output external
await esbuild.build({
  entryPoints: ["main.js"],
  bundle: true,
  format: "esm",
  external: ["./pkg/*"],
  outfile: "dist/app-bundle.js",
});

// Copy wasm-pack output to dist/pkg/
cpSync("pkg", "dist/pkg", { recursive: true });
