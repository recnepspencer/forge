# forge-signal-wasm Documentation

`forge-signal-wasm` is the framework-agnostic web runtime for Forge Signal.
It exposes an app-first API for state, derivation, observation, diagnostics,
history, and compatibility access, plus a React adapter through the package
subpath.

## Start Here

- [README.md](../README.md)
  Product overview and the shortest happy path.
- [consuming_the_package.md](consuming_the_package.md)
  Build, prepare, install, import, and package-shape guidance for other apps.
- [app_surface_reference.md](app_surface_reference.md)
  Reference for `createSignals()`, `input`, `computed`, `output`, `watch`,
  `effect`, `transaction`, `batch`, and `nuke`.
- [diagnostics_and_history_reference.md](diagnostics_and_history_reference.md)
  Reference for diagnostics, latest observation, latest flow, history, branch,
  snapshot, replay, and adapter/export surfaces.
- [compatibility_surface_reference.md](compatibility_surface_reference.md)
  Reference for the lower-level `SignalApp` and `SignalRuntime` compatibility
  surfaces.
- [aspects_reference.md](aspects_reference.md)
  Reference for aspect-aware reads, produced aspects, aspect-targeted writes,
  and why subscriptions remain node-scoped.
- [react_adapter_reference.md](react_adapter_reference.md)
  Reference for `forge-signal-wasm/react`.

## Design And Architecture

- [web_runtime_spec.md](web_runtime_spec.md)
  Web runtime product and architecture spec.
- [react_adapter_spec.md](react_adapter_spec.md)
  React-domain adapter spec.
- [host_callback_computed_spec.md](host_callback_computed_spec.md)
  Callback-first computed-node spec for normal TypeScript authoring with
  dynamic dependencies and diagnostics parity.

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
- `computed` is callback-first on the package surface.
- `output` is a public projection intended for host and framework consumption.
- `output` is callback-first on the product surface, while `outputSpec(...)`
  remains available for explicit portable recipe authoring.
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
