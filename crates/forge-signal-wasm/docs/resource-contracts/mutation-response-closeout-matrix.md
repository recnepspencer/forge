# Mutation Response Closeout Matrix

Use this page when you need to answer a product question like:

- "Is this mutation-response lane a normal happy path?"
- "Is it supported only as a precise denial?"
- "Is it a typed unavailable fallback instead of exact reconciliation?"
- "Is it intentionally out of scope?"

This page exists so "the runtime rejected it correctly" does not get mistaken
for "this is a normal feature path you should build around."

## What This Feature Is

The mutation-response closeout matrix is the support checklist for
response-owned create, update, and remove reconciliation.

It is published both as product documentation and as the machine-readable
runtime surface `signals.resource.mutationResponses.closeoutMatrix()`, so the
shipped support map can be read without scraping prose.

## Why You Use It

Use this page when you are:

- writing feature docs or product guides
- reviewing whether a route shape is a normal supported lane
- deciding whether a fallback posture is acceptable for your feature
- checking whether a denial means "not supported" instead of "supported, but
  you typed it wrong"
- choosing between exact collection-item update, summary patch, placement,
  identity migration, or explicit fallback

If a row is listed as denial-only, fallback-only, or out of scope, do not
describe it like a normal ergonomic path.

If you are trying to write the route itself, start with
[Mutation Response Reconciliation](../resources/mutation-response-reconciliation.md).
Use this page after that, when the real question is "what kind of support is
this lane?"

## Stable Entry Points

- `signals.resource.mutationResponses.closeoutMatrix()`
- `line.mutationResponse()`
- `line.summary().diagnostics.latest`
- `line.history().lifecycle.at(-1)`
- `line.history().verificationPackage()`
- [Mutation Response Reconciliation](../resources/mutation-response-reconciliation.md)

## Core Mental Model

There are four support categories:

1. **Supported ergonomic happy path**
   The route shape is accepted and the runtime gives you the normal feature
   path you would build app code around.
2. **Supported precise denial**
   The runtime or type surface rejects the route shape on purpose. This is
   useful because the boundary is enforced, but it is not a supported product
   workflow.
3. **Supported typed unavailable fallback**
   The route shape is accepted, but one target or lifecycle capability remains
   explicit as `refetchRequired`, `deliveryAwaited`,
   `placementUnavailable`, `deletionUnavailable`,
   `identityMigrationUnavailable`, or `partialReconciliation`.
4. **Intentionally out of scope**
   The product does not claim that lane at all. It should not be marketed as
   nearly supported.

The closeout question is not just "does the runtime behave honestly?" It is
"what kind of support is this, and should I build feature code around it?"

## How It Executes

The runtime still executes from one mutation-response plan.

The matrix is a product reading of that plan:

- exact reconciliation and exact placement/deletion count as ergonomic support
- compile or runtime rejection of an overclaimed declaration counts as precise
  denial
- an explicit fallback result counts as typed unavailable support
- missing declarations, hidden best-effort mutation, or undeclared read-truth
  rewriting remain out of scope
- `matrix.deferredErgonomics` lists any lanes that are intentionally left for
  future ergonomic support instead of being advertised as supported; on the
  shipped Phase 9 surface this list is empty

Use grouped diagnostics and verification reads to inspect which category a real
line fell into.

## Lane Selection Quick Guide

Use this matrix together with the main mutation-response doc when you need to
choose a lane:

- `saveDetailReplace`
  The response proves the whole detail value.
- `saveDetailGranular`
  The response proves only a declared detail field, JSON path, or region.
- `updateRelatedCollectionItem`
  The response proves one visible item inside a related collection or paged
  line.
- `updateRelatedSummary`
  The response proves collection-level metadata such as `version` or `total`.
- `createPlacement`
  The response proves where a newly created item belongs.
- `createIdentityMigration`
  The submitted identity differs from the server's real identity.
- `deleteExactRemoval`
  The response proves an item left the declared topology.
- `deleteCanonicalTombstone`
  The item stays visible as a deleted record instead of disappearing.

If the response does not prove exact local truth, expect a typed unavailable
fallback row instead of one of the exact lanes above.

## Small Example

```ts
const matrix = signals.resource.mutationResponses.closeoutMatrix();

console.log(matrix.rows.find((row) => row.lane === "deliveryAwaited"));
console.log(matrix.deferredErgonomics);

const latest = line.summary().diagnostics.latest;

console.log(line.mutationResponse().confirmation.kind);
console.log(latest.mutationResponseFallbackReasonDigest);
console.log(latest.mutationResponseTargetOutcomes);
```

Read it like this:

- exact confirmation plus exact target outcomes: ergonomic happy path
- explicit fallback digest: typed unavailable fallback
- no line at all because route construction was denied: precise denial
- empty `deferredErgonomics`: no deferred product ergonomics remain on the
  admitted surface

In practice, most teams use this API in one of two ways:

- docs and tooling check whether a lane is a happy path, fallback, or denial
- tests and closeout checks assert that the shipped support story matches the
  real runtime behavior

## Real Example

```ts
const saveTask = api.url("/tasks/:taskId")
  .response(signals.resource.response.detail()({
    status: "status",
    version: "version",
  }))
  .update({
    atomicity: "partialAllowed",
    reconciles: [{
      family: taskDetail,
      params: ({ taskId }) => ({ taskId }),
      fallback: "partialReconciliation",
      detail: { kind: "field", field: "status" },
    }, {
      family: taskList,
      params: () => ({}),
      fallback: "partialReconciliation",
      summary: { kind: "summary", summary: "version" },
    }],
    load: ({ taskId }) => ({
      id: taskId,
      status: "published",
    }),
  });

const line = saveTask.line({
  taskId: "task:1",
  body: {},
});

console.log(line.mutationResponse().partialAdmission);
console.log(line.summary().diagnostics.latest.mutationResponseFallbackReasonDigest);
```

This is not "broken support." It is an admitted partial lane:

- detail status is ergonomic exact reconciliation
- summary version is typed `partialReconciliation`
- the whole outcome remains one supported plan with explicit fallback evidence

If the same response had returned an updated visible list item instead of a
summary field, that would move the route into `updateRelatedCollectionItem`
instead of `updateRelatedSummary`.

That distinction matters in app code:

- collection-item update changes what the user sees in a list row
- summary update changes metadata around the list, such as count or version
- partial fallback means the route is still supported, but you should treat the
  missing part as intentionally unresolved

## Closeout Matrix

| Lane | Category | What the product claims |
| --- | --- | --- |
| Save detail line replace | Supported ergonomic happy path | Exact whole-detail reconciliation is supported when the route proves the whole detail value. |
| Save granular detail field/path/region | Supported ergonomic happy path | Exact narrow reconciliation is supported when the route declares the matching locus. |
| Update related collection item | Supported ergonomic happy path | Exact related collection-item reconciliation is supported for declared visible targets. |
| Update related summary | Supported ergonomic happy path | Exact related summary reconciliation is supported for declared visible targets. |
| Create with collection placement | Supported ergonomic happy path | Exact insert placement is supported for declared topologies. |
| Create with identity migration | Supported ergonomic happy path | Exact identity migration is supported for the accepted target classes and declared canonical params. |
| Delete with exact removal | Supported ergonomic happy path | Exact topology deletion is supported when the response proves the removed visible item. |
| Delete with canonical tombstone | Supported ergonomic happy path | Tombstone posture is supported when the route declares it and the response proves it. |
| Multi-family reconciliation | Supported ergonomic happy path | One write response may reconcile multiple declared families together. |
| Refetch required | Supported typed unavailable fallback | The write was accepted, but exact local truth still requires a later refresh. |
| Delivery awaited | Supported typed unavailable fallback | The write was accepted, but exact local truth is expected from a later delivery update. |
| Partial reconciliation | Supported typed unavailable fallback | Some declared targets were exact and others stayed explicit in fallback posture. |
| Placement unavailable | Supported typed unavailable fallback | The create route was accepted, but the response did not prove an exact insertion position. |
| Deletion unavailable | Supported typed unavailable fallback | The remove route was accepted, but the response did not prove an exact topology deletion. |
| Identity migration unavailable | Supported typed unavailable fallback | The write was accepted, but the declared target could not migrate exactly. |
| Overclaimed detail/path/region/summary/placement/deletion/identity declarations | Supported precise denial | The type surface or route lowering rejects declarations that claim stronger proof than the route actually has. |
| Hidden best-effort mutation of undeclared read truth | Intentionally out of scope | Forge does not claim silent convenience mutation outside declared targets. |
| Treating fallback posture as equivalent to exact reconciliation | Intentionally out of scope | Fallback is part of the contract, not an exact lane with softer wording. |
| Advertising denied-only lanes as normal ergonomics | Intentionally out of scope | A correct denial is still not a product happy path. |

## How It Relates To Other Features

- [Mutation Response Reconciliation](../resources/mutation-response-reconciliation.md)
  teaches the main authoring surface.
- [History And Restore](./history-and-restore.md) explains restore and replay
  posture.
- [Effect Closeout Matrix](./closeout-matrix.md) is the profile-level cousin of
  this page.

## Inspection And Debugging

Use these reads to classify a real result:

- `signals.resource.mutationResponses.closeoutMatrix()`
- `line.mutationResponse().confirmation.kind`
- `line.summary().diagnostics.latest.mutationResponseTargetOutcomes`
- `line.summary().diagnostics.latest.mutationResponseFallbackReasonDigest`
- `line.summary().diagnostics.latest.mutationResponseFreshnessPostureDigest`
- `line.history().verificationPackage().mutationResponse.plan`

If the route never materializes because declaration lowering rejects it, that
is a precise denial lane, not a runtime fallback lane.

## Anti-Patterns

- Do not describe `deliveryAwaited` or `refetchRequired` like exact
  reconciliation.
- Do not describe compile-denied declarations like supported ergonomics.
- Do not collapse denial-only and fallback-only lanes into one vague
  "sometimes supported" story.
- Do not hide out-of-scope lanes behind words like "advanced" or
  "manual for now" unless the runtime actually admits them.

## Current Limits

- This matrix is product guidance, not a second runtime planner.
- Some accepted lanes are honest fallback-only support, not exact support.
- Unsupported target classes and hidden best-effort mutation remain
  intentionally out of scope unless the public runtime and proof surface change.

## Related Docs

- [Mutation Response Reconciliation](../resources/mutation-response-reconciliation.md)
- [History And Restore](./history-and-restore.md)
- [Effect Closeout Matrix](./closeout-matrix.md)
- [Response Topology Proof](./response-topology-proof.md)
