# forge-signal-wasm Documentation

These docs are organized around shipped features and common developer tasks.

If you are trying to answer "how do I do multipart upload?" or "where do I go
for downloads?", start with the feature pages below instead of the older
category references.

## Start Here

- [start_here.md](./start_here.md)
  The shortest path to route-first resources, line usage, and the raw escape
  hatch.
- [feature_index.md](./feature_index.md)
  One-line index of every first-class resource feature and its canonical doc.
- [resource_recipes.md](./resource_recipes.md)
  Task-first examples for the most common happy paths.

## Feature Docs

- [feature_fetch_and_write_api_resources.md](./feature_fetch_and_write_api_resources.md)
  Detail, list, paged, create, update, remove, params, and advanced
  `verb/body/headers`.
- [feature_request_posture_and_policy.md](./feature_request_posture_and_policy.md)
  Auth, headers, request context, policy profiles, continuation, and request
  posture inspection.
- [feature_collections_and_delivery.md](./feature_collections_and_delivery.md)
  `items(...)`, `reconcile(...)`, aspects, summaries, patch helpers, and
  delivery helpers.
- [feature_transfers.md](./feature_transfers.md)
  Signed upload, multipart upload, finalize-required flows, and deferred
  processing.
- [feature_downloads.md](./feature_downloads.md)
  Builder-owned downloads, binary descriptors, and multipart download handoff.
- [feature_line_inspection.md](./feature_line_inspection.md)
  `line.summary()`, request inspection, diagnostics, history, upload,
  processing, and download reads.
- [feature_history_and_restore.md](./feature_history_and_restore.md)
  History availability, exact restore, exact replay posture, and verification
  packages.
- [feature_external_delivery_and_compatibility.md](./feature_external_delivery_and_compatibility.md)
  External definitions, pushed packets, basis refresh, and compatibility
  delivery.
- [feature_raw_escape_hatch.md](./feature_raw_escape_hatch.md)
  When to use `signals.resource.*(...)` directly and how it relates to the
  pleasant lane.

## Reference Docs

Use these when you already know the feature and want the lower-level surface
details.

- [api_route_authoring_reference.md](./api_route_authoring_reference.md)
- [resource_family_authoring_reference.md](./resource_family_authoring_reference.md)
- [resource_request_and_policy_reference.md](./resource_request_and_policy_reference.md)
- [resource_reconciliation_reference.md](./resource_reconciliation_reference.md)
- [resource_transfers_reference.md](./resource_transfers_reference.md)
- [resource_binary_and_download_reference.md](./resource_binary_and_download_reference.md)
- [resource_line_reference.md](./resource_line_reference.md)
- [resource_inspection_and_history_reference.md](./resource_inspection_and_history_reference.md)
- [resource_delivery_and_compatibility_reference.md](./resource_delivery_and_compatibility_reference.md)

## Non-Resource Docs

- [app_surface_reference.md](./app_surface_reference.md)
- [consuming_the_package.md](./consuming_the_package.md)
- [react_adapter_reference.md](./react_adapter_reference.md)
- [host_capabilities.md](./host_capabilities.md)
- [diagnostics_and_history_reference.md](./diagnostics_and_history_reference.md)
- [compatibility_surface_reference.md](./compatibility_surface_reference.md)
- [api_resources_overview.md](./api_resources_overview.md)
- [aspects_reference.md](./aspects_reference.md)

## Reading Order

1. [start_here.md](./start_here.md)
2. [feature_index.md](./feature_index.md)
3. [resource_recipes.md](./resource_recipes.md)
4. the one feature page that matches your task
5. the matching reference page only if you need lower-level detail
