# API Resources Overview

This page is the feature router for server-backed resource state.

If you are trying to answer "which resource feature owns my problem?", start
here and then jump to the canonical feature page.

## Default Path

For ordinary app code, the default lane is:

1. `signals.api(...)`
2. `api.scope(...)` when one feature area needs shared defaults
3. `api.url(...)`
4. the semantic finalizer or feature step that matches your endpoint
5. `family.line(...)`
6. `line.summary()`

## Feature Map

- fetch, list, paged, create, update, remove, and advanced request shaping:
  [feature_fetch_and_write_api_resources.md](./feature_fetch_and_write_api_resources.md)
- auth, request context, retry policy, continuation, and request posture:
  [feature_request_posture_and_policy.md](./feature_request_posture_and_policy.md)
- collection identity, reconcile, patch helpers, and delivery helpers:
  [feature_collections_and_delivery.md](./feature_collections_and_delivery.md)
- signed upload, multipart upload, and deferred processing:
  [feature_transfers.md](./feature_transfers.md)
- downloads and multipart download handoff:
  [feature_downloads.md](./feature_downloads.md)
- grouped line reads, diagnostics, and history entrypoints:
  [feature_line_inspection.md](./feature_line_inspection.md)
- retained history, exact restore, replay availability, and verification:
  [feature_history_and_restore.md](./feature_history_and_restore.md)
- external definitions, pushed packets, compatibility delivery, and basis
  refresh:
  [feature_external_delivery_and_compatibility.md](./feature_external_delivery_and_compatibility.md)
- raw family declarations and manual identity control:
  [feature_raw_escape_hatch.md](./feature_raw_escape_hatch.md)

## Fast Decisions

- "How do I fetch one thing or write one thing?"
  [feature_fetch_and_write_api_resources.md](./feature_fetch_and_write_api_resources.md)
- "How do I declare retry, headers, or continuation?"
  [feature_request_posture_and_policy.md](./feature_request_posture_and_policy.md)
- "How do I patch or deliver into a collection?"
  [feature_collections_and_delivery.md](./feature_collections_and_delivery.md)
- "How do I do multipart upload?"
  [feature_transfers.md](./feature_transfers.md)
- "How do I do multipart download?"
  [feature_downloads.md](./feature_downloads.md)
- "How do I understand whether exact restore is available?"
  [feature_history_and_restore.md](./feature_history_and_restore.md)
- "How do I apply external basis refresh?"
  [feature_external_delivery_and_compatibility.md](./feature_external_delivery_and_compatibility.md)

## Task-First Companion

- [resource_recipes.md](./resource_recipes.md)

## Lower-Level References

Once you already know the feature, use the lower-level reference pages for
exact surface detail:

- [api_route_authoring_reference.md](./api_route_authoring_reference.md)
- [resource_family_authoring_reference.md](./resource_family_authoring_reference.md)
- [resource_line_reference.md](./resource_line_reference.md)
- [resource_request_and_policy_reference.md](./resource_request_and_policy_reference.md)
- [resource_reconciliation_reference.md](./resource_reconciliation_reference.md)
- [resource_transfers_reference.md](./resource_transfers_reference.md)
- [resource_binary_and_download_reference.md](./resource_binary_and_download_reference.md)
- [resource_inspection_and_history_reference.md](./resource_inspection_and_history_reference.md)
- [resource_delivery_and_compatibility_reference.md](./resource_delivery_and_compatibility_reference.md)
