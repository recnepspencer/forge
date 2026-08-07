# Worth Signals Demo

The product site and interactive demonstration suite for
`worth-signals-wasm`.

## Run locally

```sh
npm install
npm run dev
```

Vite serves the site at `http://127.0.0.1:5173` by default. The shared local
preview used by this workspace may instead run at `http://127.0.0.1:4173`.

Local development depends on `file:../../crates/worth-signal-wasm/pkg` for
fast iteration. That is not the published consumer shape.

## Bundler recipe

The demo constructs runtimes through `src/platform/createDemoSignals.ts`, which
injects the portable asset URLs:

```ts
import wasmUrl from "worth-signals-wasm/wasm?url";
import workerUrl from "worth-signals-wasm/worker?worker&url";
await createSignals({ assets: { wasmUrl, workerUrl } });
```

Vite is configured with `worker.format: "es"`. Missing `.wasm` / worker routes
must return 404, not SPA `index.html`.

## Verify

```sh
npm run build
npm run test:demo-docs
npm run test:demo5
npm run test:demo6
npm run test:packed-wasm
```

`test:packed-wasm` installs from an `npm pack` tarball (with
`preserveSymlinks: false`) and runs a Vite production build. Use that path when
claiming the demo works as a real npm consumer.

The demo registry and displayed examples share their code through
`src/state/demoCodeSamples.ts`. Public documentation navigation is governed by
`crates/worth-signal-wasm/docs/metadata/public-documentation.json`; do not add
an independent documentation tree inside this app.
