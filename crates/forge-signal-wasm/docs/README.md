# forge-signal-wasm Documentation

These docs cover the shipped `forge-signal-wasm` product surface.

They are organized around feature use, not milestone history.

## Start Here

- [README.md](../README.md)
  Product overview and the shortest path through the package.
- [consuming_the_package.md](./consuming_the_package.md)
  Install, build, verify, import, and consume the package.
- [app_surface_reference.md](./app_surface_reference.md)
  Main app-facing API: local state, linked state, controllers, graphs,
  mutation helpers, and inspection surfaces.

## Subject Guide

- **Local state, derived values, linked state, controllers, and graphs**
  [app_surface_reference.md](./app_surface_reference.md)
- **Host capabilities**
  [host_capabilities.md](./host_capabilities.md)
- **Diagnostics, history, replay, export, and restore**
  [diagnostics_and_history_reference.md](./diagnostics_and_history_reference.md)
- **Aspects**
  [aspects_reference.md](./aspects_reference.md)
- **Compatibility / lower-level runtime surfaces**
  [compatibility_surface_reference.md](./compatibility_surface_reference.md)
- **React integration**
  [react_adapter_reference.md](./react_adapter_reference.md)

## Recommended Reading Order

For someone learning the package from scratch:

1. [README.md](../README.md)
2. [consuming_the_package.md](./consuming_the_package.md)
3. [app_surface_reference.md](./app_surface_reference.md)
4. [host_capabilities.md](./host_capabilities.md)
5. [diagnostics_and_history_reference.md](./diagnostics_and_history_reference.md)
6. [react_adapter_reference.md](./react_adapter_reference.md)

## What These Docs Intentionally Do Not Cover

This folder is for product-facing docs.

It does not carry:

- milestone sequencing
- implementation closeouts
- architecture history
- engineering planning prose

Those live here:

- [_docs/forge_signal_wasm](../../../_docs/forge_signal_wasm)

## Current Product Surface Summary

The package surface is organized around these feature families:

- local inputs, computed values, and outputs
- linked writable derived state
- controller artifacts
- graph publication and graph input/output contracts
- required and optional public inputs
- graph-boundary write, patch, reset, and transaction helpers
- host capability families
- diagnostics, history, replay, and export/restore
- compatibility lanes
- React integration
