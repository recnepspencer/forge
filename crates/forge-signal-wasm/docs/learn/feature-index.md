# Feature Index

This page exists so every shipped resource feature has one obvious canonical
home.

## Start Here

- quickest entrypoint into the feature-first resource docs:
  [start_here.md](../start_here.md)

## Authoring Features

- shared API roots and nested scopes:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- detail, list, paged, create, update, remove:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- request params:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- advanced `verb(...)`, `body<T>()`, `headers(...)`:
  [Fetch And Write Resources](../resources/fetch-and-write.md)
- auth, request context, retry policy, and continuation:
  [Request Posture And Policy](../resources/request-posture-and-policy.md)
- direct-array collections:
  [Collections And Delivery](../resources/collections-and-delivery.md)
- reconcile collections:
  [Collections And Delivery](../resources/collections-and-delivery.md)

## Patch / Delivery Features

- family-owned `patch` helpers:
  [Collections And Delivery](../resources/collections-and-delivery.md)
- family-owned `delivery` helpers:
  [Collections And Delivery](../resources/collections-and-delivery.md)

## Transfer Features

- signed upload:
  [Transfers](../resources/transfers.md)
- multipart upload:
  [Transfers](../resources/transfers.md)
- deferred processing:
  [Transfers](../resources/transfers.md)

## Download Features

- builder-owned `.downloads(...)`:
  [Downloads](../resources/downloads.md)
- binary descriptors:
  [Downloads](../resources/downloads.md)
- multipart downloads:
  [Downloads](../resources/downloads.md)

## Read / Debug Features

- `line.summary()`:
  [Line Inspection](../resources/line-inspection.md)
- request inspection:
  [Line Inspection](../resources/line-inspection.md)
- diagnostics and history:
  [Line Inspection](../resources/line-inspection.md)
- upload, processing, and download reads:
  [Line Inspection](../resources/line-inspection.md)
- retained history, exact restore, replay availability, and verification:
  [History And Restore](../resource-contracts/history-and-restore.md)
- branch-native optimistic effects, response-lens topology declarations, JSON
  effects, advanced topology effects, and UI lifecycle events:
  [Branch-Native Resource Effects](../resources/branch-native-effects.md)
- sealed effect envelopes:
  [Effect Envelope Contract](../resource-contracts/effect-envelope.md)
- resource effect merge and rebase:
  [Effect Merge And Rebase](../resources/merge-and-rebase.md)
- resource effect rollback:
  [History And Restore](../resource-contracts/history-and-restore.md)
- response topology proof:
  [Response Topology Proof](../resource-contracts/response-topology-proof.md)
- JSON path item-aspect effects:
  [JSON Path Effects](../resources/json-effects.md)
- effect profile closeout matrices:
  [Effect Closeout Matrix](../resource-contracts/closeout-matrix.md)

## External / Compatibility Features

- external resource definitions:
  [External Delivery And Compatibility](../resources/external-delivery-and-compatibility.md)
- basis refresh and compatibility delivery:
  [External Delivery And Compatibility](../resources/external-delivery-and-compatibility.md)

## Escape Hatch

- raw `signals.resource.*(...)` family declarations:
  [Raw Escape Hatch](../resources/raw-escape-hatch.md)

## Task-First Companion

- [Resource Recipes](./recipes.md)
