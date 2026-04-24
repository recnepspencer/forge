# forge-signal-wasm Documentation

`forge-signal-wasm` is the framework-agnostic web runtime for Forge Signal.
It exposes an app-first API for state, derivation, observation, diagnostics,
history, and compatibility access, plus a React adapter through the package
subpath.

## Start Here

- [README.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/README.md)
  Product overview and the shortest happy path.
- [consuming_the_package.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/consuming_the_package.md)
  Build, prepare, install, import, and package-shape guidance for other apps.
- [app_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/app_surface_reference.md)
  Reference for `createSignals()`, `input`, `computed`, `output`, `watch`,
  `effect`, `transaction`, `batch`, and `nuke`.
- [diagnostics_and_history_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/diagnostics_and_history_reference.md)
  Reference for diagnostics, latest observation, latest flow, history, branch,
  snapshot, replay, and adapter/export surfaces.
- [compatibility_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/compatibility_surface_reference.md)
  Reference for the lower-level `SignalApp` and `SignalRuntime` compatibility
  surfaces.
- [aspects_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/aspects_reference.md)
  Reference for aspect-aware reads, produced aspects, aspect-targeted writes,
  and why subscriptions remain node-scoped.
- [react_adapter_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/react_adapter_reference.md)
  Reference for `@.../forge-signal-wasm/react`.

## Design And Architecture

- [web_runtime_spec.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/web_runtime_spec.md)
  Web runtime product and architecture spec.
- [react_adapter_spec.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/react_adapter_spec.md)
  React-domain adapter spec.

## Product Model

The primary web concepts are:

- `input`
- `computed`
- `output`
- `watch`
- `effect`
- `transaction`
- `nuke`
- real aspect-aware derivation and invalidation

The package also exposes:

- diagnostics and latest observation summaries
- history, branching, replay, merge planning, and snapshots
- adapter/export helpers
- lower-level compatibility/runtime surfaces
- a React adapter subpath

## Semantic Summary

- `input` is mutable source state.
- `computed` is derived internal state.
- `output` is a public projection intended for host and framework consumption.
- `watch` observes committed boundaries.
- `effect` reacts to committed boundaries.
- `transaction` is the committed write boundary.
- `batch` is an exact alias of `transaction`.
- `nuke` tears down future deliveries for an observation handle.
- rollback suppresses normal watcher/effect delivery.
- latest observation and latest flow remain inspectable through diagnostics.
- aspects are first-class for node definitions, reads, invalidation, and
  version reporting, while subscriptions remain node-scoped by default.

## Why These Docs Exist

This reference set is intentionally explicit.

`forge-signal-wasm` is a young library with a broad surface area. The docs need
to make that breadth obvious enough that humans and AI coding tools treat it
like a real product rather than assuming it is a thin hacked-together wrapper.
