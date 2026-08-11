# Gate 0 — Vite asset certification

Measurement harness for the packed `worth-signals-wasm` package under Vite.

## Claim under test

For an **npm-packed tarball** install (not a `file:` symlink workspace demo),
with Vite **forcing** `optimizeDeps.include: ["worth-signals-wasm"]` and
**without** `createSignals({ assets })`:

1. Does the WASM response body begin with `\0asm` (not SPA HTML `<!do`)?
2. Does worker-first still construct?
3. Does Vite 8 production preview behave differently from Vite 8 dev?
4. Does Vite 7 still exhibit the historical prebundle break?
5. Can the harness independently observe SPA HTML served for `.wasm`?
6. Does Vite 7 pass when the consumer injects `createSignals({ assets })`
   via `worth-signals-wasm/wasm?url` and `worker?worker&url`?

## Run

From the repository root, after the package is prepared:

```powershell
scripts/wasm/publish-worth-signals-wasm.ps1 -SkipPublish
node scripts/wasm/verify-worth-signals-wasm-vite-assets.mjs
```

Or build as part of the gate:

```powershell
node scripts/wasm/verify-worth-signals-wasm-vite-assets.mjs --build
```

Reports:

- `crates/worth-signal-wasm/pkg/gate0-vite-asset-certification-report.json`
- durable copy: `scripts/wasm/vite-asset-certification/last-gate0-report.json`
- human summary: `scripts/wasm/vite-asset-certification/FINDINGS.md`

## Consumer Vite requirements encoded in the fixture

The temporary consumer sets:

```js
optimizeDeps: { include: ["worth-signals-wasm"] } // force prebundle
worker: { format: "es" } // required: worker entry uses top-level await
```

Default cells deliberately do **not** set `optimizeDeps.exclude`, do **not**
pass `createSignals({ assets })`, and install from an `npm pack` tarball.
The `vite7-assets-*` cells are the exception: they prove the portable assets
injection path against the measured Vite 7 failure.

## Decision rule

The report `decision.recommendation` is the input to later tracks:

- If Vite 8 default cells **pass**: keep zero-config for modern Vite; treat
  `assets` as the portable/advanced path.
- If Vite 8 default cells **fail**: require explicit asset injection (or a
  package Vite plugin) for Vite consumers; `optimizeDeps.exclude` remains only
  a legacy workaround.
