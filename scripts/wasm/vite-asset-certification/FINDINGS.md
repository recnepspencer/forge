# Gate 0 findings (measured)

Last full matrix run is recorded in `last-gate0-report.json`.

## Track closeout status

| Track | Status | Evidence |
| --- | --- | --- |
| **1** Fail-closed HTML diagnostics + init retry | **closed** | Prepared entry `assertWasmMagic`; `scripts/wasm/html_as_wasm_diagnostic.test.mjs`; package verify static asserts; SPA cell reason `harnessDetectedHtmlWasmAndPackageDiagnostic` |
| **2** Explicit `./wasm` / `./worker` exports | **closed** | `package.json` exports + package verify tarball/export asserts |
| **3** `createSignals({ assets })` + worker WASM bootstrap | **closed** | Admission tests; worker bootstrap in prepared package; `vite7-assets-*` cells **passed** under forced prebundle |
| **4** Docs/demo cutover + packed demo proof | **closed** | Host/bundler matrix in support-status; install/construction docs; demo `createDemoSignals` assets cutover; `verify-worth-signals-demo-packed.mjs` / `npm run test:packed-wasm` |
| **5** Publish-time ESM chunking | **closed** | Multi-entry esbuild prepare; JS files **590 → 7** (see `scripts/wasm/js-chunk-certification/`); verify hard cap ≤ 40; Gate 0 vite8 + vite7-assets + spa green; packed demo green. QA: facade splitting disabled + `chunks/[name]` policy (hashed chunks and chunk-relative bridge/wasm externals are unsafe). |
| **6** WASM size pipeline | **closed** | `release-wasm` + explicit `wasm-opt -Oz --strip-producers` (`--no-opt` on wasm-pack); WASM **13,092,002 → 9,625,124**; tarball **~4.51MB → 3.36MB**; publish JS minify (`index.js` **2.16MB → 1.10MB**); verify magic/size/path-leak + raw `createRawSignals().read` abort smoke; Gate 0 + packed demo green. QA2: abort smoke was JS-admission-only (fixed); extra wasm-opt flags beyond strip-producers were ≤180B. |

## Measured outcomes

| Cell | Result | Independent observation |
| --- | --- | --- |
| `vite8-dev-mainThread` | passed | WASM via `@fs/.../node_modules/worth-signals-wasm/worth_signal_wasm_bg.wasm`, prefix `00 61 73 6d`, prebundle cache present |
| `vite8-dev-workerFirst` | passed | same WASM magic; worker script served; construction succeeded |
| `vite8-preview-mainThread` | passed (prior run) | emitted `/assets/worth_signal_wasm_bg-*.wasm`, magic `00 61 73 6d` |
| `vite8-preview-workerFirst` | passed (prior run) | emitted worker + wasm assets; construction succeeded |
| `spa-fallback-mainThread` | passed | forced HTML for `.wasm` observed as `3c 21 64 6f`; construction failed with package HTML diagnostic |
| `vite7-dev-mainThread` | failed | requested `node_modules/.vite/deps/…wasm` body `3c 21 64 6f` |
| `vite7-dev-workerFirst` | failed | worker requested beside `.vite/deps` → 404 |
| `vite7-assets-mainThread` | **passed** | `createSignals({ assets: { wasmUrl } })` via `worth-signals-wasm/wasm?url`; WASM magic + construction |
| `vite7-assets-workerFirst` | **passed** | `assets: { wasmUrl, workerUrl }` via `wasm?url` + `worker?worker&url`; WASM magic + worker + construction |

## Decision implications

1. **Vite 8 + packed tarball + forced optimizeDeps**: default relative asset URLs are viable when the consumer allows Vite to serve the rebased package files and sets `worker.format: "es"`.
2. **Vite 7 default path**: still broken exactly as originally reported (HTML-as-WASM / missing worker beside `.vite/deps`).
3. **Vite 7 + `createSignals({ assets })`**: proven repair path under the same forced-prebundle packed-tarball world.
4. **Diagnostics**: HTML masquerading as WASM is rejected with a package remediation error (not only a bare `CompileError`).
5. **Package size (Track 6)**: baseline WASM **13,092,002** → **9,625,124** bytes (cap `10,112,755`); tarball **~4.51MB → 3.36MB** after wasm-opt + publish JS minify. Publish requires Binaryen on PATH.

Latest decision string:

```text
vite8DefaultRelativeAssetsAppearViable_withWorkerFormatEsAndFsAllow; vite7DefaultStillBroken; createSignalsAssetsInjectionProvenOnVite7
```

## Harness requirements discovered during Gate 0

- Allocate free loopback ports (fixed ports collide with other local Vite apps).
- Launch Vite through `node node_modules/vite/bin/vite.js` on Windows (avoid `npm.cmd` spawn `EINVAL`).
- Set `worker.format: "es"` or production build fails on worker top-level await.
- Allow package file serving (`server.fs.strict: false` / allow) so Vite 8’s `@fs` rebase can fetch the real `.wasm`.
