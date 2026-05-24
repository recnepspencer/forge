# Start Here

This is the shortest path through the shipped product surfaces.

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

const signals = await createSignals();

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

For runtime-owned forms, the normal lane is:

1. call `signals.form(...)`
2. declare `field(...)`, `repeated(...)`, or `evidence(...)`
3. add validation, availability, admission, steps, and actions only where the
   form actually needs them
4. read `effective()`, `dirty()`, `readiness()`, `actions()`, or
   `diagnosticsSummary()`

Small example:

```ts
const source = signals.input({ title: "Ship docs", done: false });

const form = signals.form({
  source,
  fields: ({ field }) => ({
    title: field("title"),
    done: field("done"),
  }),
});

form.fields.title.set("Ship docs today");

console.log(form.effective());
console.log(form.readiness());
```

## If You Already Know Your Task

- fetch or write ordinary resources:
  [Fetch And Write Resources](./resources/fetch-and-write.md)
- response-owned write reconciliation, identity migration, and fallback proof:
  [Mutation Response Reconciliation](./resources/mutation-response-reconciliation.md)
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
  [Effect Closeout Matrix](./resource-contracts/closeout-matrix.md),
  [Mutation Response Closeout Matrix](./resource-contracts/mutation-response-closeout-matrix.md)
- external push packets and basis refresh:
  [External Delivery And Compatibility](./resources/external-delivery-and-compatibility.md)
- raw family declarations:
  [Raw Escape Hatch](./resources/raw-escape-hatch.md)
- ordinary local forms and the rewritten forms docs:
  [Forms Overview](./forms/index.md)
- semantic dirty truth, patch planning, and complex edit forms:
  [Changes And Patching](./forms/changes/README.md)
- validation, messages, async checks, and source compatibility:
  [Validation](./forms/validation/README.md)
- field availability, submit blockers, and approval-style requirements:
  [Availability And Permissions](./forms/availability/README.md)
- multi-step forms and submit execution:
  [Steps And Multi-Step Forms](./forms/steps/README.md) and
  [Actions And Submit](./forms/actions/README.md)
- label sizing, layout hints, and accessibility reads:
  [Layout And Accessibility](./forms/layout/README.md)
- dropdowns, searches, imperative widgets, and control capability truth:
  [Inputs And Controls](./forms/inputs/README.md)
- focus state, input capability support, and host facts:
  [Interaction And Host Facts](./forms/interaction/README.md)
- entry bootstrap, exit state, and handoff visibility:
  [Lifecycle](./forms/lifecycle/README.md)
- attachment/media state and transfer readback:
  [Attachments And Media](./forms/media/README.md)
- resource-backed source truth and resource-backed execution:
  [Resource-Backed Forms](./forms/resource-backed/README.md)
- collaboration posture, reviewer lanes, and branch-backed shared editing:
  [Collaboration](./forms/collaboration/README.md)
- route-coupled draft handoff and continuity:
  [Route-Coupled Forms](./forms/route-coupling/README.md)
- form debugging, retained history, and verification:
  [Diagnostics And History](./forms/diagnostics/README.md) and
  [Verification](./forms/verification/README.md)

## What Not To Do First

Do not start with `signals.resource.*(...)` unless you already know you need
manual identity, raw request shape control, or compatibility-specific behavior.

Do not start by building your own form controller, draft store, validation
cache, or action state machine beside `signals.form(...)`. The shipped forms
surface already owns those boundaries.

The raw lane is real and supported. It just should not be the first stop for
ordinary app code.

## Next Reads

- [Feature Index](./learn/feature-index.md)
- [Recipes](./learn/recipes.md)
- [Forms Overview](./forms/index.md)
