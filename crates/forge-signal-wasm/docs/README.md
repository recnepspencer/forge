# forge-signal-wasm Documentation

These docs cover the shipped `forge-signal-wasm` product surface.

They are organized around feature use, not milestone history.

If you are not sure where to start:

- building local app state: start with
  [app_surface_reference.md](./app_surface_reference.md)
- building API-backed state: start with
  [api_resources_overview.md](./api_resources_overview.md)
- trying to install or consume the package: start with
  [consuming_the_package.md](./consuming_the_package.md)

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
- **API resources**
  [api_resources_overview.md](./api_resources_overview.md)
- **Resource family authoring**
  [resource_family_authoring_reference.md](./resource_family_authoring_reference.md)
- **Resource lines**
  [resource_line_reference.md](./resource_line_reference.md)
- **Resource request and policy posture**
  [resource_request_and_policy_reference.md](./resource_request_and_policy_reference.md)
- **Resource reconciliation**
  [resource_reconciliation_reference.md](./resource_reconciliation_reference.md)
- **Resource transfers**
  [resource_transfers_reference.md](./resource_transfers_reference.md)
- **Resource binary and download support**
  [resource_binary_and_download_reference.md](./resource_binary_and_download_reference.md)
- **Resource delivery and compatibility**
  [resource_delivery_and_compatibility_reference.md](./resource_delivery_and_compatibility_reference.md)
- **Resource inspection and history**
  [resource_inspection_and_history_reference.md](./resource_inspection_and_history_reference.md)
- **Resource recipes**
  [resource_recipes.md](./resource_recipes.md)
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
4. [api_resources_overview.md](./api_resources_overview.md)
5. [resource_family_authoring_reference.md](./resource_family_authoring_reference.md)
6. [resource_line_reference.md](./resource_line_reference.md)
7. [resource_request_and_policy_reference.md](./resource_request_and_policy_reference.md)
8. [resource_reconciliation_reference.md](./resource_reconciliation_reference.md)
9. [resource_transfers_reference.md](./resource_transfers_reference.md)
10. [resource_binary_and_download_reference.md](./resource_binary_and_download_reference.md)
11. [resource_delivery_and_compatibility_reference.md](./resource_delivery_and_compatibility_reference.md)
12. [resource_inspection_and_history_reference.md](./resource_inspection_and_history_reference.md)
13. [resource_recipes.md](./resource_recipes.md)
14. [host_capabilities.md](./host_capabilities.md)
15. [diagnostics_and_history_reference.md](./diagnostics_and_history_reference.md)
16. [react_adapter_reference.md](./react_adapter_reference.md)

## Fast Paths

If you already know what you are trying to build, use one of these shorter
paths.

### I just want local app state

1. [consuming_the_package.md](./consuming_the_package.md)
2. [app_surface_reference.md](./app_surface_reference.md)
3. [diagnostics_and_history_reference.md](./diagnostics_and_history_reference.md)

### I just want API-backed state

1. [api_resources_overview.md](./api_resources_overview.md)
2. [resource_family_authoring_reference.md](./resource_family_authoring_reference.md)
3. [resource_line_reference.md](./resource_line_reference.md)
4. [resource_recipes.md](./resource_recipes.md)

### I just want React usage

1. [consuming_the_package.md](./consuming_the_package.md)
2. [app_surface_reference.md](./app_surface_reference.md)
3. [react_adapter_reference.md](./react_adapter_reference.md)

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

The package has three main lanes:

1. Local signal and graph authoring
   Inputs, computed values, outputs, linked state, controllers, graphs, and
   graph-boundary mutation helpers.
2. API resources
   Detail, collection, and paged resources with request posture, lifecycle,
   refresh, patching, delivery, uploads, downloads, diagnostics, and history.
3. Integration and tooling
   Host capabilities, diagnostics/history, compatibility surfaces, and the
   optional React adapter.
