# forge-signal-wasm Documentation

These docs are organized by product layer, not by filename history.

Start with the short learning path when you are new. Use `resources/` for
ordinary app work, `resource-contracts/` when you need the proof and delivery
rules underneath resource behavior, `api-reference/` for exact surfaces, and
`app-surface/` for the broader signal app model.

## Start Here

- [start_here.md](./start_here.md)
  The shortest path to route-first resources, line usage, and the raw escape
  hatch.
- [Feature Index](./learn/feature-index.md)
  One-line index of every first-class resource feature and its canonical doc.
- [Resource Recipes](./learn/recipes.md)
  Task-first examples for the most common happy paths.

## Resources

- [Resource Overview](./resources/overview.md)
  The feature router for server-backed resource state.
- [Fetch And Write Resources](./resources/fetch-and-write.md)
  Detail, list, paged, create, update, remove, params, and advanced
  `verb/body/headers`.
- [Request Posture And Policy](./resources/request-posture-and-policy.md)
  Auth, headers, request context, policy profiles, continuation, and request
  posture inspection.
- [Collections And Delivery](./resources/collections-and-delivery.md)
  `items(...)`, `reconcile(...)`, aspects, summaries, patch helpers, and
  delivery helpers.
- [Mutation Response Reconciliation](./resources/mutation-response-reconciliation.md)
  response-owned create/update/remove reconciliation, identity migration,
  partial mapping, fallback posture, and compact mutation-response evidence.
- [Transfers](./resources/transfers.md)
  Signed upload, multipart upload, finalize-required flows, and deferred
  processing.
- [Downloads](./resources/downloads.md)
  Builder-owned downloads, binary descriptors, and multipart download handoff.
- [Line Inspection](./resources/line-inspection.md)
  `line.summary()`, request inspection, diagnostics, history, upload,
  processing, and download reads.
- [Branch-Native Resource Effects](./resources/branch-native-effects.md)
  Branch-native optimistic effects, profiles, lifecycle, inspection, rollback,
  merge, response topology proof, JSON effects, and UI lifecycle reads.
- [Effect Merge And Rebase](./resources/merge-and-rebase.md)
  `planEffectMerge(...)`, `mergeEffect(...)`, conflict artifacts, policy
  binding, host-region evidence, and mapping-unavailable results.
- [JSON Path Effects](./resources/json-effects.md)
  Nested JSON item aspects, required and optional path policy, immutable writes,
  denial posture, rollback, and JSON path proof.
- [External Delivery And Compatibility](./resources/external-delivery-and-compatibility.md)
  External definitions, pushed packets, basis refresh, and compatibility
  delivery.
- [Raw Escape Hatch](./resources/raw-escape-hatch.md)
  When to use `signals.resource.*(...)` directly and how it relates to the
  pleasant lane.

## Resource Contracts

- [History And Restore](./resource-contracts/history-and-restore.md)
  History availability, exact restore, exact replay posture, verification
  packages, and effect rollback.
- [Effect Envelope Contract](./resource-contracts/effect-envelope.md)
  The sealed `ResourceEffectEnvelope` record behind diagnostics, history,
  rollback, merge, authority, and cost proof.
- [Response Topology Proof](./resource-contracts/response-topology-proof.md)
  Response lens topology proof for array, collection, object-items, connection,
  entity-store, map, grouped, named, sparse, tree, detail, and summary effects.
- [Effect Closeout Matrix](./resource-contracts/closeout-matrix.md)
  `resource.effects.closeoutMatrix(profile)` and the proof lanes behind profile
  capability claims.
- [Mutation Response Closeout Matrix](./resource-contracts/mutation-response-closeout-matrix.md)
  The product support matrix for exact mutation-response ergonomics, precise
  denials, typed unavailable fallback lanes, and intentional out-of-scope work.
- [Reconciliation Contract](./resource-contracts/reconciliation.md)
- [Delivery And Compatibility Contract](./resource-contracts/delivery-and-compatibility.md)
- [Inspection And History Contract](./resource-contracts/inspection-and-history.md)

## API Reference

Use these when you already know the feature and want the lower-level surface
details.

- [Route Authoring Reference](./api-reference/route-authoring.md)
- [Resource Family Authoring Reference](./api-reference/resource-family-authoring.md)
- [Resource Request And Policy Reference](./api-reference/resource-request-and-policy.md)
- [Resource Transfers Reference](./api-reference/resource-transfers.md)
- [Resource Binary And Download Reference](./api-reference/resource-binary-and-download.md)
- [Resource Line Reference](./api-reference/resource-line.md)
- [Compatibility Surface](./api-reference/compatibility-surface.md)

## App Surface

- [Install, Build, And Publish](./package/install-and-publish.md)
- [App Surface Overview](./app-surface/overview.md)
- [React Adapter](./app-surface/react-adapter.md)
- [Host Capabilities](./app-surface/host-capabilities.md)
- [Diagnostics And History](./app-surface/diagnostics-and-history.md)
- [Aspects](./app-surface/aspects.md)

## Reading Order

1. [start_here.md](./start_here.md)
2. [Feature Index](./learn/feature-index.md)
3. [Resource Recipes](./learn/recipes.md)
4. the one feature page that matches your task
5. the matching reference page only if you need lower-level detail
