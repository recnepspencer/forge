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

## Verify

```sh
npm run build
npm run test:demo-docs
npm run test:demo5
npm run test:demo6
```

The demo registry and displayed examples share their code through
`src/state/demoCodeSamples.ts`. Public documentation navigation is governed by
`crates/worth-signal-wasm/docs/metadata/public-documentation.json`; do not add
an independent documentation tree inside this app.
