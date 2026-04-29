# Consuming forge-signal-wasm

This guide shows how to install the public npm package, how to consume the
local prepared package, and how to use the main entrypoints in real app code.

## Install Shapes

### Public npm package

```bash
npm install forge-signal-wasm
```

### Public npm package with React adapter

```bash
npm install forge-signal-wasm react
```

### Local prepared package during development

Build from the Forge workspace root:

```powershell
wasm-pack build crates/forge-signal-wasm --target bundler --out-dir pkg
```

Then prepare the package. Public npm example:

```powershell
$env:FORGE_SIGNAL_WASM_PACKAGE_NAME='forge-signal-wasm'
$env:FORGE_SIGNAL_WASM_REGISTRY='https://registry.npmjs.org'
$env:FORGE_SIGNAL_WASM_PUBLISH_ACCESS='public'
$env:FORGE_SIGNAL_WASM_NOTICE_MODE='none'
node scripts/wasm/prepare-forge-signal-wasm-package.mjs crates/forge-signal-wasm/pkg
```

Then run the release-proof verifier:

```powershell
node scripts/wasm/verify-forge-signal-wasm-package.mjs crates/forge-signal-wasm/pkg
```

Private scoped example:

```powershell
$env:FORGE_SIGNAL_WASM_SCOPE='aust-group'
node scripts/wasm/prepare-forge-signal-wasm-package.mjs crates/forge-signal-wasm/pkg
```

Then consume the local folder:

```json
{
  "dependencies": {
    "forge-signal-wasm": "file:../path/to/forge/crates/forge-signal-wasm/pkg"
  }
}
```

## Public Publish Flow

Once the package is built and prepared, the honest publish lane is:

```powershell
wasm-pack build crates/forge-signal-wasm --target bundler --out-dir pkg
$env:FORGE_SIGNAL_WASM_PACKAGE_NAME='forge-signal-wasm'
$env:FORGE_SIGNAL_WASM_REGISTRY='https://registry.npmjs.org'
$env:FORGE_SIGNAL_WASM_PUBLISH_ACCESS='public'
$env:FORGE_SIGNAL_WASM_NOTICE_MODE='none'
node scripts/wasm/prepare-forge-signal-wasm-package.mjs crates/forge-signal-wasm/pkg
node scripts/wasm/verify-forge-signal-wasm-package.mjs crates/forge-signal-wasm/pkg
cd crates/forge-signal-wasm/pkg
npm publish --access public
```

The verifier is not optional ceremony. It is the mechanical proof that the
prepared tarball contains the files the public entrypoints actually reference,
and that a clean consumer can import and type-check the package.

### One-command release gate

Use the publish helper when you want one command that rebuilds, prepares, and
verifies the public package without actually publishing yet:

```powershell
scripts/wasm/publish-forge-signal-wasm.ps1 -SkipPublish
```

That command now defaults to the public package lane:

- package name: `forge-signal-wasm`
- registry: `https://registry.npmjs.org`
- access: `public`
- notice mode: `none`

If you need a private/scoped lane, pass explicit overrides such as `-Scope`,
`-Registry`, `-Access`, and `-NoticeMode`.

If you pass `-Scope` without `-PackageName`, the helper now intentionally falls
back to the scoped naming pattern:

```powershell
scripts/wasm/publish-forge-signal-wasm.ps1 `
  -Scope aust-group `
  -Registry https://npm.pkg.github.com `
  -NoticeMode proprietary `
  -SkipPublish
```

That produces `@aust-group/forge-signal-wasm` instead of silently staying on
the public unscoped package name.

## Import Surface

### Core runtime

```ts
import { createSignals } from "forge-signal-wasm";
```

### React adapter

```ts
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "forge-signal-wasm/react";
```

## Simple App Example

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

const count = signals.input("count", 1);
const doubled = signals.computed("doubled", () => count() * 2);

signals.transaction((tx) => {
  tx.set(count, 2);
});

console.log(doubled());
```

## More Complete App Example

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

const enabled = signals.input("enabled", true);
const name = signals.input("name", "Ada");
const count = signals.input("count", 1);

const label = signals.computed("label", () => {
  return enabled() ? `${name()} x${count()}` : "disabled";
});

const panel = signals.output("panel", {
  reads: ["enabled", "name", "count", "label"],
  expr: {
    kind: "object",
    fields: [
      ["enabled", { kind: "read", id: "enabled" }],
      ["name", { kind: "read", id: "name" }],
      ["count", { kind: "read", id: "count" }],
      ["label", { kind: "read", id: "label" }],
    ],
  },
});

const watchHandle = signals.watch(panel, (notice) => {
  console.log("panel changed", notice);
});

signals.transaction((tx) => {
  tx.set(count, 3);
});

console.log(panel());
signals.nuke(watchHandle);
```

## Diagnostics Example

Simple:

```ts
const why = signals.diagnostics().why("label");
console.log(why.callback?.currentReads);
```

Complex:

```ts
const diagnostics = signals.diagnostics();
const perf = diagnostics.performanceSummary();
const latestObservation = diagnostics.latestObservation();

console.log({
  deliveries: latestObservation?.deliveredEventCount,
  callbackCaptures: perf.computeCallbackCaptureCount,
  dependencyPatches: perf.computeCallbackDependencyPatchCount,
});
```

## React Example

```tsx
import { createSignals } from "forge-signal-wasm";
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "forge-signal-wasm/react";

const signals = createSignals();
const store = createReactSignalsStore(signals);

const count = signals.input("count", 1);
const doubled = signals.computed("doubled", () => count() * 2);
const panel = signals.output("panel", {
  reads: ["count", "doubled"],
  expr: {
    kind: "object",
    fields: [
      ["count", { kind: "read", id: "count" }],
      ["doubled", { kind: "read", id: "doubled" }],
    ],
  },
});

function Counter() {
  const countValue = useSignalValue<number>(count, store);
  const doubledValue = useSignalValue<number>(doubled, store);
  const panelValue = useOutputValue<{ count: number; doubled: number }>(panel, store);
  const diagnostics = useSignalsDiagnostics(store);

  return { countValue, doubledValue, panelValue, diagnostics };
}
```

## Practical Notes

- Prefer callback-first `computed(() => ...)` for ordinary app code.
- Keep `computedSpec(...)` for explicit portable recipe authoring.
- `output(...)` remains spec-authored today.
- `output(() => ...)` is intentionally deferred.
- The React adapter consumes runtime truth; it does not recalculate derived
  values locally.

## What To Read Next

- [app_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/app_surface_reference.md)
- [diagnostics_and_history_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/diagnostics_and_history_reference.md)
- [react_adapter_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/react_adapter_reference.md)
