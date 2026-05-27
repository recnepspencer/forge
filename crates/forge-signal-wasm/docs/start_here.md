# Start Here

This is the shortest path through the shipped product surfaces.

## Default Mental Model

For API-backed state, the normal lane is:

1. declare shared request posture with `signals.api(...)`
2. declare one route with `api.url(...)`
3. finish it with `.detail(...)`, `.list(...)`, `.paged(...)`, `.create(...)`,
   `.update(...)`, `.remove(...)`, `.mutation(...)`, or `.command(...)`
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

- new task-first resource docs home:
  [Resources Docs Home](./resources/README.md)
- first resource or shape choice:
  [Start Here](./resources/start-here/README.md),
  [Your First Resource](./resources/start-here/your-first-resource.md), and
  [Choose A Resource Shape](./resources/start-here/choose-a-resource-shape.md)
- fetch one record, a collection, or a paged list:
  [Fetching Data](./resources/fetching/README.md),
  [Fetch A Single Record](./resources/fetching/fetch-a-single-record.md),
  [Fetch A Collection](./resources/fetching/fetch-a-collection.md), and
  [Fetch A Paged List](./resources/fetching/fetch-a-paged-list.md)
- auth, request context, or request policy:
  [Request Auth And Context](./resources/fetching/request-auth-and-context.md)
  and [Request Policies](./resources/fetching/request-policies.md)
- create, update, remove, local patches, or effect profiles:
  [Updating Data](./resources/updating/README.md),
  [Write A Resource](./resources/updating/write-a-resource.md),
  [Submit Patches And Replacements](./resources/updating/submit-patches-and-replacements.md),
  [Choose An Effect Profile](./resources/updating/choose-an-effect-profile.md), and
  [What Happens After A Write](./resources/updating/what-happens-after-a-write.md)
- mutation responses, partial canonical truth, or fallback reconciliation:
  [Handling Server Responses](./resources/responses/README.md),
  [Understand Mutation Responses](./resources/responses/understand-mutation-responses.md), and
  [Handle Fallback Reconciliation](./resources/responses/handle-fallback-reconciliation.md)
- list identity, speculative row truth, or narrow row updates:
  [Working With Lists](./resources/lists/README.md),
  [Visible Selection](./resources/lists/visible-selection.md), and
  [Update One Item Without Replacing Everything](./resources/lists/update-one-item-without-replacing-everything.md)
- stale lines, refresh, revalidate, or cache identity:
  [Caching And Refresh](./resources/caching/README.md),
  [Stale, Pending, And Settled State](./resources/caching/stale-pending-and-settled-state.md), and
  [Invalidation And Refresh](./resources/caching/invalidation-and-refresh.md)
- partial updates, derived views, or item/region/summary targeting:
  [Partial Updates And Derived Views](./resources/partial-updates/README.md),
  [Automatic Derived Views](./resources/partial-updates/automatic-derived-views.md), and
  [Update One Region, Field, Or Item](./resources/partial-updates/update-one-region-field-or-item.md)
- upload preparation, queued work, or transfer posture:
  [Uploads And Transfers](./resources/transfers/README.md),
  [Upload Files](./resources/transfers/upload-files.md), and
  [Track Processing Jobs](./resources/transfers/track-processing-jobs.md)
- builder-owned downloads, binary descriptors, or multipart download handoff:
  [Downloads And Binary Data](./resources/downloads/README.md),
  [Offer Downloads](./resources/downloads/offer-downloads.md), and
  [File, Media, And Export Downloads](./resources/downloads/file-media-and-export-downloads.md)
- grouped line inspection, retained history, or exact recovery:
  [Inspecting And Debugging Resources](./resources/debugging/README.md),
  [Inspect A Resource Line](./resources/debugging/inspect-a-resource-line.md), and
  [Restore, Replay, And Recover](./resources/debugging/restore-replay-and-recover.md)
- optimistic effects, merge/rebase, or rollback posture:
  [Effects And Recovery](./resources/effects/README.md),
  [Branch-Native Effects](./resources/effects/branch-native-effects.md), and
  [Rollback And Recovery](./resources/effects/rollback-and-recovery.md)
- resource-backed forms, settlement, drift, or merge preview projection:
  [Using Resources In Forms](./resources/forms/README.md),
  [Use A Resource As Form Source](./resources/forms/use-a-resource-as-form-source.md), and
  [Handle Resource Drift And Merge](./resources/forms/handle-resource-drift-and-merge.md)
- route-bound resources, warmup, or projected/admitted capability reads:
  [Using Resources In Routes](./resources/router/README.md),
  [Declare Route Resources](./resources/router/declare-route-resources.md), and
  [Prefetch And Warmup Route Resources](./resources/router/prefetch-and-warmup-route-resources.md)
- raw family authoring, canonical identity, or direct detail/list declarations:
  [Advanced Resource Modeling](./resources/advanced/README.md),
  [Resource Family Identity](./resources/advanced/resource-family-identity.md), and
  [Raw Resource Lines](./resources/advanced/raw-resource-lines.md)
- verification packages, topology proof, or mutation-response support matrices:
  [Verification And Proof](./resources/verification/README.md),
  [Verification Packages](./resources/verification/verification-packages.md), and
  [Mutation-Response Closeout Matrix](./resources/verification/mutation-response-closeout-matrix.md)
- older flat transfer and download pages:
  [Transfers](./resources/transfers.md) and
  [Downloads](./resources/downloads.md)
- fetch or write ordinary resources:
  [Fetch And Write Resources](./resources/fetch-and-write.md)
- response-owned write reconciliation, identity migration, and fallback proof:
  [Mutation Response Reconciliation](./resources/mutation-response-reconciliation.md)
- auth, policy, continuation, and request posture:
  [Request Posture And Policy](./resources/request-posture-and-policy.md)
- collection patching, summaries, or delivery:
  [Collections And Delivery](./resources/collections-and-delivery.md)
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
