# forge-signal-wasm Documentation

`forge-signal-wasm` is the web-facing product layer for Forge Signal. These
docs cover the shipped package surface: app authoring, controller composition,
graph contracts, aspects, diagnostics, history, host capabilities,
compatibility lanes, React integration, and the roadmap.

## Start Here

- [README.md](../README.md)
  The product overview and the shortest path through the package.
- [consuming_the_package.md](consuming_the_package.md)
  Build, prepare, install, import, package shape, and consumer expectations.
- [app_surface_reference.md](app_surface_reference.md)
  The main product API: `createSignals()`, `input`, `computed`, `output`,
  `scope`, `controller`, `signals.graph(...)`, graph contracts, graph-native
  operations, `watch`, `effect`, `transaction`, `batch`, and `nuke`.

## Subject Guide

If you know the topic you are looking for, use this map:

- **State and derivation**
  [app_surface_reference.md](app_surface_reference.md)
- **Controller-first authoring and graph-owned lifecycle**
  [app_surface_reference.md](app_surface_reference.md)
- **Public graph contracts, graph input/output operations, and restore/import posture**
  [app_surface_reference.md](app_surface_reference.md),
  [diagnostics_and_history_reference.md](diagnostics_and_history_reference.md)
- **Aspects**
  [aspects_reference.md](aspects_reference.md)
- **Diagnostics, latest observation, latest flow, history, replay, snapshots, and export/import**
  [diagnostics_and_history_reference.md](diagnostics_and_history_reference.md)
- **Host capabilities**
  [host_capabilities.md](host_capabilities.md)
- **Compatibility / lower-level runtime surface**
  [compatibility_surface_reference.md](compatibility_surface_reference.md)
- **React integration**
  [react_adapter_reference.md](react_adapter_reference.md)
- **Roadmap and architecture direction**
  [_docs/forge_signal_wasm](../../../_docs/forge_signal_wasm)

## Reference Docs

- [app_surface_reference.md](app_surface_reference.md)
  Reference for the public app-facing product surface.
- [aspects_reference.md](aspects_reference.md)
  Aspect-aware reads, produced aspects, targeted invalidation, and version
  reporting.
- [diagnostics_and_history_reference.md](diagnostics_and_history_reference.md)
  Diagnostics, history, branching, replay, snapshots, graph contract
  inspection, and export/import truth.
- [host_capabilities.md](host_capabilities.md)
  `hostCapabilityPlan(...)`, admitted host families, `signals.host.*`, and
  host-capability diagnostics and transport posture.
- [compatibility_surface_reference.md](compatibility_surface_reference.md)
  The lower-level compatibility surface for `SignalApp`, `SignalRuntime`, and
  related runtime-facing contracts, including aspect-aware read/write shapes.
- [react_adapter_reference.md](react_adapter_reference.md)
  The `forge-signal-wasm/react` consumer surface.

## Engineering Docs

Engineering specs, closeouts, and roadmaps live in:

- [_docs/forge_signal_wasm](../../../_docs/forge_signal_wasm)

That folder is for implementation plans and architectural history. This
package docs folder stays focused on the shipped product surface.

## Product Surface Summary

The current package surface is organized around these major subjects:

- `input`, `computed`, and `output`
- controller artifacts and scoped authoring
- `signals.graph(...)` as the explicit publication boundary
- public graph `inputs` and `outputs`
- graph-native `writeInputs`, `patchInputs`, `resetInputs`, `apply`, and
  graph-scoped `transaction(...)`
- public input authority classes such as `writable`, `readOnly`, and `imported`
- aspect-aware derivation and invalidation
- diagnostics, latest observation, latest flow, history, replay, snapshots,
  and contract inspection
- graph-native export/import and exact restore posture
- typed host capability families for browser/runtime-local facts
- compatibility/runtime lanes for lower-level consumers
- a React adapter for app integration

## Reading Order

For someone learning the package from scratch:

1. [README.md](../README.md)
2. [consuming_the_package.md](consuming_the_package.md)
3. [app_surface_reference.md](app_surface_reference.md)
4. [aspects_reference.md](aspects_reference.md)
5. [diagnostics_and_history_reference.md](diagnostics_and_history_reference.md)
6. [host_capabilities.md](host_capabilities.md)
7. [react_adapter_reference.md](react_adapter_reference.md)

For architecture / milestone work:

1. [_docs/forge_signal_wasm](../../../_docs/forge_signal_wasm)

## Why This Index Is Explicit

The docs index should make the subject breadth obvious enough that nobody
mistakes the package for just:

- primitive signals
- a thin wasm export shell
- a React helper
- or a graph API without aspect, diagnostics, history, or host/runtime truth

If a real package subject exists, the docs README should point at it directly.
