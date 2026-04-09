import * as esbuild from "esbuild";

await Promise.all([
  esbuild.build({
    entryPoints: ["papi-entry.js"],
    bundle: true,
    format: "esm",
    outfile: "dist/papi-bundle.js",
  }),
  esbuild.build({
    entryPoints: ["papi-worker-entry.js"],
    bundle: true,
    format: "esm",
    ignoreAnnotations: true,
    outfile: "dist/smoldot-worker.js",
  }),
]);
