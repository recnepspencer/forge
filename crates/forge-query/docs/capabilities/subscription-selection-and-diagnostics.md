# Automatic Subscription Family Selection And Diagnostics

## What This Feature Is

Automatic subscription family selection is the part of Forge Query that decides
which subscription family a query should use based on its live meaning, view
shape, future-bearing live posture, authority posture, relationship-proof
posture, and work budget.
Diagnostics make that decision auditable across selection, declaration, bridge
lowering, runtime-backed admission, support reporting, and certification.

## Why You Use It

- you want the runtime to choose the right subscription family without
  hand-picking one in product code
- you need grouped, table, detail, inspector, and bounded-materialization
  subscriptions to stay semantically distinct
- you want typed denials when a budget, basis, or bridge posture makes the
  subscription illegal
- you need support reports and diagnostic bundles that tell you exactly where a
  subscription failed

## Stable Entry Points

- `select_query_subscription_family(...)`
- `QuerySubscriptionFamilySelection`
- `QuerySubscriptionFamilySelectionError`
- `QuerySubscriptionFamilySelectionFailureClass`
- `QuerySubscriptionFutureSelection`
- `QuerySubscriptionFutureSelectionClass`
- `report_query_subscription_support(...)`

Important public vocabulary:

- `QuerySubscriptionFamily`
- `LiveQueryAdmissionArtifact`
- `QuerySubscriptionSupportSubject`
- `QuerySubscriptionSupportEvidence`
- `QuerySubscriptionSupportReport`
- `QuerySubscriptionSupportClass`
- `QuerySubscriptionSupportPosture`
- diagnostic traces and diagnostic bundles exported from `subscription`

## Core Mental Model

Selection is derived from query meaning, not from whatever family happens to be
convenient.

Forge Query looks at:

- live query family and detail vs collection posture
- admitted view shape such as table, inspector, or grouped Kanban
- retained future-bearing live posture such as ordinary, temporal, async, or
  temporal+async
- basis posture and bridge posture
- relationship-proof posture
- work-budget ceilings
- declaration widths such as projection, ordering, grouping, and relation scope

That produces a semantic subscription family such as:

- `DetailExact`
- `InspectorDetailExact`
- `CollectionMembership`
- `GroupedCollectionMembership`
- `BoundedMaterialization`

The family vocabulary stays stable even when live meaning becomes temporal or
async. Future-bearing live posture is preserved as a retained selection fact,
not as a product-chosen family label.

Diagnostics then preserve how the system moved from selection to activation,
which future-bearing live posture was active, or why it denied before getting
there.

Selection is not the final identity boundary.

Once Forge Query declares the subscription, the retained future-bearing live
posture and basis posture become part of the canonical subscription declaration
digest before activation begins. If temporal basis, async request identity,
policy, or tenant meaning changes, Forge Query must mint a new declaration
instead of patching the old one in place.

Selection is also not the final delivery contract.

After activation, Forge Query now retains one mixed-cause delivery stream for
truth patches, time-only wakes, async completions, retries, cancellations, and
other future-bearing delivery members. Bridge owns the canonical ordering law;
Query owns the public coalesced delivery artifact. Product code does not pick a
"mixed-cause mode," and host callback order is not semantic order.

## How It Executes

1. `select_query_subscription_family(...)` classifies the query, admitted view
   shape, and retained future-bearing live posture.
2. Selection verifies required widths, basis posture, bridge families,
   allocation budgets, and relationship-proof posture.
3. If selection succeeds, the runtime lowers the declaration and continues
   through bridge and admission stages.
4. Active lifecycle then reuses that retained selection and declaration truth
   to decide whether one active subscription lane can be opened, shared, or
   denied.
5. Diagnostic traces record the stage-by-stage outcomes.
6. `report_query_subscription_support(...)` can emit a support matrix and
   posture report for a declaration, activation, active lifecycle,
   continuation, or preview closeout subject.

Selection can deny before declaration or bridge lowering when the semantic
contract is already impossible.

## Small Example

```rust
use forge_query::subscription::select_query_subscription_family;

let selection = select_query_subscription_family(live_query, work_budget)?;

assert_eq!(selection.family().as_str(), "collection_membership");
assert_eq!(selection.cost_posture().as_str(), "bounded_membership");
```

This is the smallest honest example because it shows that the family is
computed from admitted meaning, not chosen manually.

When the live meaning is temporal or async, callers still use the same
selection API. The future-bearing posture stays on the returned selection
artifact instead of forcing product code to pick a second subscription-family
surface.

## Real Example

```rust
use forge_query::subscription::{
    report_query_subscription_support, select_query_subscription_family,
    QuerySubscriptionSupportEvidence, QuerySubscriptionSupportSubject,
};

let selection = select_query_subscription_family(kanban_live_query, work_budget)?;
assert_eq!(selection.family().as_str(), "grouped_collection_membership");

// Later lifecycle stages produce `declaration`, `admission`, and `closeout`
// artifacts through declaration, bridge lowering, runtime admission, and active
// lifecycle closeout.

let subject = QuerySubscriptionSupportSubject::preview_closeout(
    &declaration,
    &admission,
    &closeout,
);
let evidence = QuerySubscriptionSupportEvidence::admission(&declaration, &admission)?;
let (report, receipt) = report_query_subscription_support(subject, evidence)?;

assert_eq!(
    report.support_posture().as_str(),
    "runtime_backed_certified"
);
assert_eq!(receipt.resolution_posture().as_str(), "indexed_family_lookup");
```

And when the meaning is illegal, the system should fail early and specifically:

```rust
let error = select_query_subscription_family(temporal_inspector_query, work_budget)
    .expect_err("temporal inspector live meaning is not admitted");

assert_eq!(
    error.failure_class().as_str(),
    "unsupported_temporal_live_shape"
);
```

That is the real payoff: grouped, table, detail, inspector, and bounded
materialization semantics do not collapse into one vague "subscription" bucket,
and temporal or async live meaning does not force product code to guess at a
separate family selector.

## How It Relates To Other Features

- View family from [Scopes, Templates, Saved Queries, And View Shapes](../authoring/scopes-templates-saved-queries-and-view-shapes.md)
  is a major input to selection.
- Authority and basis posture connect directly to
  [Aspects And Authority Lanes](../modeling/aspects-and-authority-lanes.md).
- Runtime-facing consumers eventually observe the selected family through
  [Live Views](../runtime-surfaces/live-views.md), [Computed](../runtime-surfaces/computed.md), and
  [Inspection](inspection.md).

## Inspection And Debugging

The strongest debugging artifacts are the diagnostic traces, diagnostic
bundles, and support reports.

Inspect:

- selected family, live family, and optional view family
- selected future-bearing live posture
- active future-bearing lane posture and active checkpoint identity when the
  subscription has already crossed into runtime-backed lifecycle
- mixed-cause delivery posture when one retained delivery combined truth, time,
  or async members under one canonical ordered window
- cost posture, basis posture, bridge posture, and equivalence basis
- required slice count and declared widths
- failure stage such as `ViewMismatch`, `BasisBinding`, `DeliveryIntent`,
  `BridgeFamily`, `BridgeSlice`, or `DurableReloadOverclaim`
- support subject class such as declaration, activation, continuation, or
  preview closeout
- support posture: certified, denied, deferred, or uncertified-denied

One important law: denied bundles omit later-stage claims. A selection-denied
trace should not pretend bridge lowering or certification succeeded.

## Anti-Patterns

- Hard-coding a subscription family in product code instead of letting meaning
  drive selection.
- Hand-picking a "temporal subscription family" or "async subscription family"
  in product code instead of letting future-bearing live meaning stay on the
  same selector.
- Treating grouped collections as plain tables or inspector detail as plain
  detail.
- Ignoring relationship-proof posture drift or assuming it can be fixed after
  declaration.
- Emitting diagnostics that blur failure stage boundaries.

## Current Limits

- Runtime-backed family selection and diagnostic staging are the supported
  center of gravity today.
- Future-bearing live meaning now stays visible through selection, declaration,
  activation, active lifecycle, continuation, and preview closeout, while still
  sharing the same family vocabulary as ordinary live meaning.
- Durable replay and store-backed restart appear in support reporting vocabulary
  but are not equivalent to certified runtime-backed lifecycle support.
- Some support classes require admission evidence by contract; reporting them
  without matching admission artifacts is denied.

## Related Docs

- [Live Views](../runtime-surfaces/live-views.md)
- [Inspection](inspection.md)
- [Aspects And Authority Lanes](../modeling/aspects-and-authority-lanes.md)
- [Scopes, Templates, Saved Queries, And View Shapes](../authoring/scopes-templates-saved-queries-and-view-shapes.md)


