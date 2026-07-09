# Consuming worth-signal-wasm

## What This Guide Is

This guide covers how to install, build, verify, and consume the public
`worth-signal-wasm` package.

Use this when you need the package entrypoints, local package workflow, or the
smallest useful examples for the shipped surface.

The root package is intentionally mixed:

- `import init from "worth-signal-wasm"` gives the raw wasm init entry
- named imports like `createSignals` and `hostCapabilityPlan` give the modern
  app-facing surface

## Why You Use It

- install the npm package cleanly
- consume a locally prepared package during workspace development
- understand the main callable surface before moving into the deeper reference
  docs
- verify that the tarball you are about to publish is internally consistent

If you just want to start coding, you only need:

1. `npm install worth-signal-wasm`
2. `import { createSignals } from "worth-signal-wasm"`
3. the small example below

The rest of this guide is for local package work and publishing from this repo.

Package contract note:

- the published package is ESM-first
- `import` and bundler resolution are the supported consumer paths
- CommonJS callers should use dynamic `import(...)` instead of `require(...)`

## Stable Entry Points

- `createSignals(...)`
- `createReactSignalsStore(...)`
- `signals.spec.*`
- `signals.graph(...)`
- `signals.importGraph(...)`
- `signals.resource.detail(...)`
- `signals.resource.collection(...)`
- `signals.resource.paged(...)`

Package-preparation and proof entrypoints:

- `scripts/wasm/publish-worth-signal-wasm.ps1 -SkipPublish`
- `scripts/wasm/verify-worth-signal-wasm-package.mjs`

## Install Shapes

### Public npm package

```bash
npm install worth-signal-wasm
```

### Public npm package with React adapter

```bash
npm install worth-signal-wasm react
```

### Local prepared package during development

Build from the Worth workspace root:

```powershell
wasm-pack build crates/worth-signal-wasm --target bundler --out-dir pkg
```

Prepare the package:

```powershell
node scripts/wasm/prepare-worth-signal-wasm-package.mjs crates/worth-signal-wasm/pkg
```

Verify the package:

```powershell
node scripts/wasm/verify-worth-signal-wasm-package.mjs crates/worth-signal-wasm/pkg
```

Then consume the local folder:

```json
{
  "dependencies": {
    "worth-signal-wasm": "file:../path/to/worth/crates/worth-signal-wasm/pkg"
  }
}
```

## Public Publish Flow

If you are publishing from this workspace, use this flow:

```powershell
wasm-pack build crates/worth-signal-wasm --target bundler --out-dir pkg
node scripts/wasm/prepare-worth-signal-wasm-package.mjs crates/worth-signal-wasm/pkg
node scripts/wasm/verify-worth-signal-wasm-package.mjs crates/worth-signal-wasm/pkg
cd crates/worth-signal-wasm/pkg
npm publish --access public
```

Or use the one-command release gate:

```powershell
scripts/wasm/publish-worth-signal-wasm.ps1 -SkipPublish
```

Good to know:

- the verifier is not just a packaging nicety
- it checks that the built tarball contains the files the public entrypoints
  actually need
- it also checks that a clean consumer can import and type-check the package

## Core Imports

### Main callable surface

```ts
import { createSignals } from "worth-signal-wasm";
```

### Host capability helpers

```ts
import {
  createSignals,
  hostCapabilityPlan,
  visibilityCapability,
} from "worth-signal-wasm";
```

### Resource helpers

```ts
import {
  createSignals,
  resourceParamIdentity,
  resourceParams,
} from "worth-signal-wasm";
```

### React adapter

```ts
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "worth-signal-wasm/react";
```

## Small Example

This is the simplest useful example for the current app lane:

```ts
import { createSignals } from "worth-signal-wasm";

const signals = await createSignals();

const count = signals.input(1);
const doubled = signals.computed(() => count() * 2);

signals.transaction((tx) => {
  tx.set(count, 2);
});

console.log(doubled());
```

Why this is the best starting example:

- it uses handle-based local authoring
- it does not rely on explicit ids
- it still shows the real runtime mutation path

## Real Example

This is a more realistic consumer shape that uses local state, linked state,
controller composition, and graph publication:

```ts
import { createSignals } from "worth-signal-wasm";

const signals = await createSignals();

const itemWorkspace = signals.graph("itemWorkspace", (graph) => {
  const editor = graph.controller("editor", ({ input, computed, linked }) => {
    const serverItem = input({
      id: "task-7",
      title: "Ship docs",
      workflowTargetStateId: "ready",
    });

    const draft = input({});

    const effectiveItem = computed(() => ({
      ...serverItem(),
      ...draft(),
    }));

    const selectedWorkflowTarget = linked({
      source: () => [
        { id: "draft", label: "Draft" },
        { id: "ready", label: "Ready" },
      ],
      computation: (options, previous) => (
        options.find((option) => option.id === previous?.value?.id) ?? options[0]
      ),
    });

    const dirtyState = computed(() => Object.keys(draft()).length > 0);

    return {
      inputs: { serverItem, draft, selectedWorkflowTarget },
      outputs: { effectiveItem, dirtyState },
    };
  });

  return graph.expose({
    inputs: {
      serverItem: graph.input.required(editor.inputs.serverItem, {
        authority: "readOnly",
      }),
      draft: graph.input.optional(editor.inputs.draft),
      selectedWorkflowTarget: graph.input.optional(
        editor.inputs.selectedWorkflowTarget,
      ),
    },
    outputs: {
      effectiveItem: editor.outputs.effectiveItem,
      dirtyState: editor.outputs.dirtyState,
    },
  });
});

itemWorkspace.patchInput("draft", {
  title: "Ready to ship",
});

console.log(itemWorkspace.read());
```

## Main Lane vs Explicit Named Lane

The normal app lane is:

- `signals.input(value)`
- `signals.computed(() => ...)`
- `signals.output(() => ...)`

The explicit named lane is:

- `signals.spec.input("name", value)`
- `signals.spec.computedCallback("name", () => ...)`
- `signals.spec.outputCallback("name", () => ...)`

Use the app lane for ordinary application code. Use `signals.spec` when you
need structural names because names are part of the contract.

Add `debugName` only when you want friendlier diagnostics or clearer inspection
output. It is optional metadata, not part of local identity.

If you are unsure which lane to use, use the normal app lane.

## Graph Boundaries

When you publish a graph, that is where explicit public names become real:

```ts
const graph = signals.graph("counter", {
  inputs: {
    count,
  },
  outputs: {
    doubled,
  },
});
```

Graph inputs can also carry public input posture:

```ts
graph.expose({
  inputs: {
    serverItem: graph.input.required(editor.inputs.serverItem, {
      authority: "readOnly",
    }),
    draft: graph.input.optional(editor.inputs.draft),
  },
  outputs: {
    effectiveItem: editor.outputs.effectiveItem,
  },
});
```

That is where:

- required vs optional becomes explicit
- authority classes become explicit
- public contract names become explicit

## Mutation Helpers

Local input helpers:

```ts
draft.patch({ done: true });
draft.assign({ title: "Ready to ship" });
draft.reset();
```

Graph boundary helpers:

```ts
graph.writeInput("draft", { title: "Queued" });
graph.patchInput("draft", { reviewer: "Avery" });
graph.resetInput("draft");
```

These helpers still lower through the same runtime mutation model as
`transaction(...)`.

## Import And Restore

Published graphs can export exact same-runtime restore artifacts:

```ts
const definition = graph.exportDefinition();
const snapshot = graph.exportSnapshot();
const restoredGraph = signals.importGraph(definition, snapshot);
```

Good to know:

- this is for exact graph restore
- portable graph import is still denied on this surface
- `importPosture()` tells you what kind of restore is actually admitted

## Explicit Compatibility Recovery

The normal package lane is:

```ts
const signals = await createSignals();
```

If worker-first construction is unavailable, recover explicitly:

```ts
import { createSignals } from "worth-signal-wasm";

let signals;

try {
  signals = await createSignals();
} catch (error) {
  if (error?.artifactFamily !== "workerUnavailableConstruction") {
    throw error;
  }
  signals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
}
```

Do not assume `createSignals()` silently falls back. The package keeps the
worker-unavailable lane typed so callers can decide whether compatibility is
acceptable.

## What To Read Next

- [App Surface Overview](../app-surface/overview.md)
- [Resource Overview](../resources/overview.md)
- [Resource Family Authoring Reference](../api-reference/resource-family-authoring.md)
- [Resource Line Reference](../api-reference/resource-line.md)
- [Resource Recipes](../learn/recipes.md)
- [Host Capabilities](../app-surface/host-capabilities.md)
- [Diagnostics And History](../app-surface/diagnostics-and-history.md)
- [React Adapter](../app-surface/react-adapter.md)
