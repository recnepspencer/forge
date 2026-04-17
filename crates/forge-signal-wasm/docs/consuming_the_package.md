# Consuming forge-signal-wasm

This guide explains how to build, prepare, install, and import
`forge-signal-wasm` into another web application.

## Package Shapes

There are two package shapes to be aware of:

- raw `wasm-pack` output in `crates/forge-signal-wasm/pkg`
- prepared package output after the Forge packaging script rewrites metadata and
  exports

For real app consumption, use the prepared package.

The prepare step:

- rewrites the package name into the scoped private package form
- installs the `./react` subpath export
- copies the stronger hand-authored TypeScript declarations
- compiles and includes the React adapter subpath
- makes the package metadata honest for private registry use

## Build And Prepare

From the Forge workspace root:

```powershell
wasm-pack build crates/forge-signal-wasm --target bundler --out-dir pkg
```

Then prepare the package:

```powershell
$env:FORGE_SIGNAL_WASM_SCOPE='aust-group'
node scripts/wasm/prepare-forge-signal-wasm-package.mjs crates/forge-signal-wasm/pkg
```

After preparation, the package is ready to be consumed as:

```text
@aust-group/forge-signal-wasm
```

If you use a different scope, set `FORGE_SIGNAL_WASM_SCOPE` accordingly.

## Install Into Another App

### Local File Install

In another app's `package.json`:

```json
{
  "dependencies": {
    "@aust-group/forge-signal-wasm": "file:../path/to/forge/crates/forge-signal-wasm/pkg"
  }
}
```

Then install normally with your package manager.

### Private Registry Install

If you publish the prepared package to GitHub Packages or another private
registry, install it under the scoped package name:

```json
{
  "dependencies": {
    "@aust-group/forge-signal-wasm": "0.1.0"
  }
}
```

The prep script writes package metadata and `.npmrc` support for private
package distribution.

## Import Surface

### Core App-First Runtime

```ts
import { createSignals } from "@aust-group/forge-signal-wasm";
```

### React Adapter

```ts
import {
  createReactSignalsStore,
  useSignalValue,
  useOutputValue,
  useSignalsDiagnostics,
} from "@aust-group/forge-signal-wasm/react";
```

## Minimal Example

```ts
import { createSignals } from "@aust-group/forge-signal-wasm";

const signals = createSignals();

const count = signals.input("count", 1);

const doubled = signals.computed("doubled", {
  reads: ["count"],
  expr: {
    kind: "multiply",
    args: [
      { kind: "read", id: "count" },
      { kind: "value", value: 2 },
    ],
  },
});

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

signals.transaction((tx) => {
  tx.set(count, 2);
});

console.log(panel.get());
```

## Runtime Behavior Notes

- current web execution is intentionally serial by default
- the package does not require a separate user-called wasm bootstrap function
- `createSignals()` is the primary entrypoint
- `watch` and `effect` inherit committed observation semantics from
  `forge-signal`
- rollback suppresses normal delivery

## What To Read Next

- [app_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/app_surface_reference.md)
- [diagnostics_and_history_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/diagnostics_and_history_reference.md)
- [react_adapter_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/react_adapter_reference.md)
