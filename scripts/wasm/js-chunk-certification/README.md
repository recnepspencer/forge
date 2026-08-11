# Track 5 — JS footprint certification

Gate 5.0 / Track 5 oracles for the published `worth-signals-wasm` JS layout.

## Reports

- `baseline-pre-track5.json` — unbundled 1:1 strip-types emit (**590** `.js` files)
- `post-track5.json` — multi-entry esbuild publish layout (**7** `.js` files)

## Measure

```powershell
node scripts/wasm/prepare-worth-signals-wasm-package.mjs crates/worth-signal-wasm/pkg
node scripts/wasm/measure-worth-signals-wasm-js-footprint.mjs
```

Package verify enforces `BUNDLED_JS_FILE_CAP` (40) from
`scripts/wasm/bundle_worth_signals_wasm_entries.mjs`.
