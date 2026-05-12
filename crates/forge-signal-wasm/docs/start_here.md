# Start Here

This is the shortest path through the shipped resource surface.

## Default Mental Model

For API-backed state, the normal lane is:

1. declare shared request posture with `signals.api(...)`
2. declare one route with `api.url(...)`
3. finish it with `.detail(...)`, `.list(...)`, `.paged(...)`, `.create(...)`,
   `.update(...)`, or `.remove(...)`
4. materialize a line with `family.line(...)`
5. start reading with `line.summary()`

Small example:

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

const api = signals.api({
  baseUrl: "/api",
});

const userDetail = api.url("/users/:userId").detail({
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});

const line = userDetail.line({ userId: "u1" });

console.log(line.summary());
console.log(line.value());
```

## If You Already Know Your Task

- fetch or write ordinary resources:
  [Fetch And Write Resources](./resources/fetch-and-write.md)
- auth, policy, continuation, and request posture:
  [Request Posture And Policy](./resources/request-posture-and-policy.md)
- collection patching, summaries, or delivery:
  [Collections And Delivery](./resources/collections-and-delivery.md)
- signed or multipart upload, or deferred processing:
  [Transfers](./resources/transfers.md)
- downloads or multipart downloads:
  [Downloads](./resources/downloads.md)
- line reads, diagnostics, and history:
  [Line Inspection](./resources/line-inspection.md)
- exact restore, replay availability, and verification packages:
  [History And Restore](./resource-contracts/history-and-restore.md)
- branch-native optimistic effects, response topology proof, JSON effects, and
  UI lifecycle events:
  [Branch-Native Resource Effects](./resources/branch-native-effects.md)
- effect envelopes, merge/rebase, rollback proof, topology proof, JSON path
  proof, or closeout matrices:
  [Effect Envelope Contract](./resource-contracts/effect-envelope.md),
  [Effect Merge And Rebase](./resources/merge-and-rebase.md),
  [History And Restore](./resource-contracts/history-and-restore.md),
  [Response Topology Proof](./resource-contracts/response-topology-proof.md),
  [JSON Path Effects](./resources/json-effects.md), and
  [Effect Closeout Matrix](./resource-contracts/closeout-matrix.md)
- external push packets and basis refresh:
  [External Delivery And Compatibility](./resources/external-delivery-and-compatibility.md)
- raw family declarations:
  [Raw Escape Hatch](./resources/raw-escape-hatch.md)

## What Not To Do First

Do not start with `signals.resource.*(...)` unless you already know you need
manual identity, raw request shape control, or compatibility-specific behavior.

The raw lane is real and supported. It just should not be the first stop for
ordinary app code.

## Next Reads

- [Feature Index](./learn/feature-index.md)
- [Resource Recipes](./learn/recipes.md)
