# Track 6 — WASM size certification

Gate 6.0 / Track 6 oracles for the published `worth_signal_wasm_bg.wasm`.

## Reports

- `baseline-pre-track6.json` — pre-Track-6 publish artifact (**13,092,002** bytes)
- `post-track6.json` — after `release-wasm` + explicit `wasm-opt -Oz`

## Measure

```powershell
node scripts/wasm/measure-worth-signals-wasm-size.mjs
```

Package verify enforces magic bytes, `MAX_WASM_BYTES` from
`scripts/wasm/wasm_size_policy.mjs`, and path-leak needles.
