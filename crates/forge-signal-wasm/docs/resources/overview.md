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
  [Fetch And Write Resources](./fetch-and-write.md)
- response-owned write reconciliation, identity migration, partial mapping, and
  fallback posture:
  [Mutation Response Reconciliation](./mutation-response-reconciliation.md)
- mutation-response support categories, denial-only lanes, typed unavailable
  fallback lanes, and intentional out-of-scope posture:
  [Mutation Response Closeout Matrix](../resource-contracts/mutation-response-closeout-matrix.md)
- auth, request context, retry policy, continuation, and request posture:
  [Request Posture And Policy](./request-posture-and-policy.md)
- collection identity, reconcile, patch helpers, and delivery helpers:
  [Collections And Delivery](./collections-and-delivery.md)
  Use `signals.resource.response.array(...)`,
  `signals.resource.response.objectItems<T>()(...)`, or
  `signals.resource.response.collection<T>()(...)` with `.response(...)` for
  ordinary collection-shaped responses. Use the branch-native effects page when
  the response shape needs the full topology family: connection, entity store,
  grouped, named, sparse page, map, discriminated tuple, tree, detail, summary,
  or JSON path aspects.
- signed upload, multipart upload, and deferred processing:
  [Transfers](./transfers.md)
- downloads and multipart download handoff:
  [Downloads](./downloads.md)
- grouped line reads, diagnostics, and history entrypoints:
  [Line Inspection](./line-inspection.md)
- retained history, exact restore, replay availability, and verification:
  [History And Restore](../resource-contracts/history-and-restore.md)
- branch-native optimistic effects, response-lens topology declarations, JSON
  effects, advanced topology effects, and UI lifecycle events:
  [Branch-Native Resource Effects](./branch-native-effects.md)
- sealed effect envelopes behind diagnostics, history, rollback, merge,
  authority, and counters:
  [Effect Envelope Contract](../resource-contracts/effect-envelope.md)
- resource effect merge planning, execution, conflict artifacts, policy binding,
  and mapping-unavailable results:
  [Effect Merge And Rebase](./merge-and-rebase.md)
- response lens topology proof for advanced collection, detail, and summary
  effects:
  [Response Topology Proof](../resource-contracts/response-topology-proof.md)
- nested JSON path item-aspect effects:
  [JSON Path Effects](./json-effects.md)
- profile capability proof lanes:
  [Effect Closeout Matrix](../resource-contracts/closeout-matrix.md)
- external definitions, pushed packets, compatibility delivery, and basis
  refresh:
  [External Delivery And Compatibility](./external-delivery-and-compatibility.md)
- raw family declarations and manual identity control:
  [Raw Escape Hatch](./raw-escape-hatch.md)

## Fast Decisions

- "How do I fetch one thing or write one thing?"
  [Fetch And Write Resources](./fetch-and-write.md)
- "How do I make a write response update detail, collection, summary, or draft
  identity truth?"
  [Mutation Response Reconciliation](./mutation-response-reconciliation.md)
- "How do I declare retry, headers, or continuation?"
  [Request Posture And Policy](./request-posture-and-policy.md)
- "How do I patch or deliver into a collection?"
  [Collections And Delivery](./collections-and-delivery.md)
- "How do I do multipart upload?"
  [Transfers](./transfers.md)
- "How do I do multipart download?"
  [Downloads](./downloads.md)
- "How do I understand whether exact restore is available?"
  [History And Restore](../resource-contracts/history-and-restore.md)
- "How do I explain an effect envelope, merge artifact, rollback kind, topology
  proof, JSON path proof, or closeout matrix?"
  [Effect Envelope Contract](../resource-contracts/effect-envelope.md),
  [Effect Merge And Rebase](./merge-and-rebase.md),
  [Response Topology Proof](../resource-contracts/response-topology-proof.md),
  [JSON Path Effects](./json-effects.md), and
  [Effect Closeout Matrix](../resource-contracts/closeout-matrix.md),
  [Mutation Response Closeout Matrix](../resource-contracts/mutation-response-closeout-matrix.md)
- "How do I apply external basis refresh?"
  [External Delivery And Compatibility](./external-delivery-and-compatibility.md)

## Task-First Companion

- [Resource Recipes](../learn/recipes.md)

## Lower-Level References

Once you already know the feature, use the lower-level reference pages for
exact surface detail:

- [Route Authoring Reference](../api-reference/route-authoring.md)
- [Resource Family Authoring Reference](../api-reference/resource-family-authoring.md)
- [Resource Line Reference](../api-reference/resource-line.md)
- [Resource Request And Policy Reference](../api-reference/resource-request-and-policy.md)
- [Reconciliation Contract](../resource-contracts/reconciliation.md)
- [Resource Transfers Reference](../api-reference/resource-transfers.md)
- [Resource Binary And Download Reference](../api-reference/resource-binary-and-download.md)
- [Inspection And History Contract](../resource-contracts/inspection-and-history.md)
- [Delivery And Compatibility Contract](../resource-contracts/delivery-and-compatibility.md)
- [Effect Envelope Contract](../resource-contracts/effect-envelope.md)
- [Response Topology Proof](../resource-contracts/response-topology-proof.md)
- [Effect Closeout Matrix](../resource-contracts/closeout-matrix.md)
