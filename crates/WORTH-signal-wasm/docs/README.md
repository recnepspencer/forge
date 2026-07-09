# worth-signal-wasm Documentation

These docs are organized by product layer, not by filename history.

Start with the short learning path when you are new. Use `resources/` for
ordinary route-first API work, `forms/` for the shipped forms surface,
`resource-contracts/` when you need the proof and delivery rules underneath
resource behavior, `api-reference/` for exact surfaces, and `app-surface/` for
the broader signal app model.

## Start Here

- [start_here.md](./start_here.md)
  The shortest path to route-first resources, line usage, and the raw escape
  hatch.
- [Feature Index](./learn/feature-index.md)
  One-line index of every first-class resource feature and its canonical doc.
- [Recipes](./learn/recipes.md)
  Task-first examples for the most common resource and forms happy paths.

## Forms

- [Forms Docs Home](./forms/README.md)
  The navigation root for the rewritten forms docs.
- [Forms Overview](./forms/index.md)
  The shortest path into the shipped `signals.form(...)` surface.
- [Getting Started](./forms/getting-started/README.md)
  Starting a form, choosing a source, and common first patterns.
- [Form State](./forms/state/README.md)
  Source truth, draft, effective values, fields, repeated items, attachments,
  and adapters.
- [Changes And Patching](./forms/changes/README.md)
  Dirty state, patch plans, complex edit forms, unchanged submit denial, and
  broad replacement posture.
- [Validation](./forms/validation/README.md)
  Validation artifacts, parse failures, visible messages, async validation,
  canonicalization, and source compatibility.
- [Availability And Permissions](./forms/availability/README.md)
  Availability states, admission rules, readiness blockers, and regulated
  requirements like approval or signature.
- [Steps And Multi-Step Forms](./forms/steps/README.md)
  Controller-local step declarations, step readiness, step actions, and step
  navigation.
- [Actions And Submit](./forms/actions/README.md)
  Action declarations, action plans, submit execution, recovery, and repeated
  attempt posture.
- [Layout And Accessibility](./forms/layout/README.md)
  Layout hints, label sizing, layout measurement, and accessibility reports.
- [Inputs And Controls](./forms/inputs/README.md)
  Input adapter declaration, custom control integration, commit/focus
  reporting, and control-level availability.
- [Interaction And Host Facts](./forms/interaction/README.md)
  Interaction history, input-adapter capabilities, host facts, and host-side
  blockers.
- [Lifecycle](./forms/lifecycle/README.md)
  Entry bootstrap, visible lifecycle state, handoffs, exit posture, and
  external presentation lanes.
- [Attachments And Media](./forms/media/README.md)
  Attachment presentation, transfer readback, media visibility, and evidence
  field behavior.
- [Resource-Backed Forms](./forms/resource-backed/README.md)
  Resource-line source truth, drift, merge, settlement, replay/restore, and
  resource-backed action execution.
- [Collaboration](./forms/collaboration/README.md)
  Shared editing posture, locks, leases, comments, presence, and
  branch-backed collaboration.
- [Route-Coupled Forms](./forms/route-coupling/README.md)
  Route-authority handoff, draft continuity, and route-coupled audit truth.
- [Diagnostics And History](./forms/diagnostics/README.md)
  Compact current-state reads and retained form history surfaces.
- [Verification](./forms/verification/README.md)
  Proof-bearing digest packages for the full forms surface.

## Resources

- [Resources Docs Home](./resources/README.md)
  The navigation root for the rewritten resource docs.
- [Resource Overview](./resources/index.md)
  The shortest path into the new task-first resource tree.
- [Start Here](./resources/start-here/README.md)
  First resource, shape choice, and common recipes.
- [Fetching Data](./resources/fetching/README.md)
  Single records, collections, paged lists, auth/context, and request policy.
- [Updating Data](./resources/updating/README.md)
  Standard writes, local patches, effect profiles, and post-write inspection.
- [Handling Server Responses](./resources/responses/README.md)
  Mutation responses, partial canonical truth, fallback reconciliation, and
  response-owned read updates.
- [Working With Lists](./resources/lists/README.md)
  Collection identity, visible selection, narrow row updates, and summaries.
- [Caching And Refresh](./resources/caching/README.md)
  Line reuse, stale/fresh state, invalidation, refresh, and derived truth.
- [Partial Updates And Derived Views](./resources/partial-updates/README.md)
  Narrow patches, automatic derived views, and explicit aspect declarations.
- [Uploads And Transfers](./resources/transfers/README.md)
  Upload preparation, processing jobs, and transfer result reads.
- [Downloads And Binary Data](./resources/downloads/README.md)
  Builder-owned binary descriptors, multipart handoff, and download
  availability.
- [Inspecting And Debugging Resources](./resources/debugging/README.md)
  Grouped line reads, retained history, delivery/compatibility inspection, and
  exact recovery.
- [Effects And Recovery](./resources/effects/README.md)
  Effect profiles, branch-native envelopes, merge/rebase, and rollback.
- [Using Resources In Forms](./resources/forms/README.md)
  Resource-line form sources, settlement, drift, merge, and mutation-response
  readback from the resource side.
- [Using Resources In Routes](./resources/router/README.md)
  Route-resource declarations, prefetch/warmup, and projected/admitted route
  capability reads.
- [Advanced Resource Modeling](./resources/advanced/README.md)
  Raw family declarations, canonical identity, detail patch declarations, and
  raw lines.
- [Verification And Proof](./resources/verification/README.md)
  Verification packages, topology proof, mutation-response support, and
  delivery/compatibility digests.
- [Transfers](./resources/transfers.md)
  The older flat transfer page for the same feature area.
- [Downloads](./resources/downloads.md)
  The older flat download page for the same feature area.
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

## Router

- [Router Docs Home](./router/README.md)
- [Router Overview](./router/index.md)
- [Authority](./router/authority/README.md)
- [Projection](./router/projection/README.md)
- [Admission](./router/admission/README.md)
- [History](./router/history/README.md)
- [Recovery](./router/recovery/README.md)
- [Speculation](./router/speculation/README.md)
- [Breadcrumbs](./router/breadcrumbs/README.md)
- [Restore And Replay](./router/restore/README.md)
- [Route Resources](./router/resources/README.md)
- [Transitions](./router/transitions/README.md)
- [Boundaries](./router/boundaries/README.md)
- [Runtime Placement](./router/runtime_placement/README.md)
- [Router And Forms](./router/forms/README.md)
- [Diagnostics And Proof](./router/diagnostics/README.md)

## Reading Order

1. [start_here.md](./start_here.md)
2. [Feature Index](./learn/feature-index.md)
3. [Forms Overview](./forms/index.md) or [Recipes](./learn/recipes.md), depending on which product lane you need
4. the one feature page that matches your task
5. the matching reference page only if you need lower-level detail
