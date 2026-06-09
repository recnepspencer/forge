# Forge Query Orientation For AI Agents

This document is the orientation map for AI agents building real applications
and downstream runtimes on top of `forge-query`.

It is not an API reference. Its job is to answer three questions:

1. What category of thing am I touching?
2. What does that category actually do?
3. Which docs should I read next for the real API details?

If you need exact signatures, types, or examples, use the linked docs. This
file is the mental model and navigation layer.

## Runtime Stack

Forge Query lives in this stack:

```text
domain crate / application
-> forge-query
-> forge-runtime-bridge
-> forge-relational + forge-signal
```

That layering matters. Query is not a thin read helper over lower runtime
systems. It is the ordinary domain-facing runtime layer. It owns the public
runtime facade, domain entry, declaration pipelines, support posture, binding,
orchestration, recovery, inspection, and the public domain capability
contribution seam.

`forge-runtime-bridge` owns the causal protocol layer that wires authoritative
truth to derived computation without collapsing either runtime into the other:
patch-to-invalidation and snapshot-backed evaluation over committed truth,
aspect mapping and fine-grained subscriptions, lineage-aware continuity across
replace/split/merge-like identity evolution, historical and branch-aware
evaluation, planned bulk routing with canonical reduction, change-stream and
reactive-source protocols, structural-identity-assisted remapping, merge-bearing
history consumption, speculative branch coordination and preview flows,
cross-runtime policy propagation, bridge-mediated commit strategies and
extensible writeback families, subscription declaration/admission/lifecycle with
checkpointed delivery and shared fanout, temporal basis binding for mixed
truth-and-clock causality, and async/resource completion causality with
generation-safe stale-completion rejection—all expressed through deterministic,
replay-safe routing plus machine-checkable receipts, envelopes, and diagnostics
that record how work crossed boundaries while leaving truth authority,
invariants, merge execution, and signal scheduling to their owning runtimes.
`forge-relational` owns lower truth semantics and authoritative state mechanics:
transactional commit authority, savepoints and rollback, literal MVCC snapshots,
version and history substrate, deterministic patch and replay publication, CDC
and subscriber recovery, schema contracts and schema evolution/reconciliation,
structural identity and historical inspection, relational invariants, joins,
lineage/correspondence, merge-ready history and merge execution, bulk query and
bulk mutation surfaces, and the constraint-bearing truth model that keeps
concurrent reads, historical views, recovery, and authoritative mutation
coherent without collapsing into caller-owned bookkeeping.
`forge-signal` owns derived-computation infrastructure beneath the bridge: an explicit
dependency graph with aspect-aware and maybe-stale invalidation, conditional
and policy-aware evaluation gates, transactional invalidation with hard rewind,
lazy pull and reactive diff propagation with partial recomputation boundaries,
structural memoization and query-style keyed incremental execution, deterministic
versus optimized scheduling with cost and priority shaping and parallel-ready
planning, speculative branchable execution with snapshot/replay/time-travel
state, fixed-point and convergence policies, temporal and previous-value
dependencies, comparator and adaptive tolerance propagation, and first-class
observation with extensible delivery strategies—all over host snapshots without
owning truth storage, while exposing execution traces, graph inspection, and
metrics that explain why work ran, deferred, or delivered change.

Ordinary domain work starts at Query. Use lower layers to understand semantics,
not as permission to bypass Query.

## The Core Rule

The governing Query rule is:

```text
declare intent once
lower it once
execute or inspect it through canonical runtime-owned artifacts
```

That rule explains most of the architecture.

Query wants domain code to express work once, keep that work canonically
identified, and let the runtime lower it through public, typed lanes instead of
forcing every downstream crate to invent local wrappers, local status enums,
local recovery folklore, or local “smart” adapters around lower layers.

If you are about to invent a local pseudo-Query surface, a hidden recovery path,
or a caller-owned translation layer that duplicates a Query lane, stop and
check whether the category you need already exists below.

## How To Use This File

Read this file in two passes.

First, find the category that matches the problem you are solving. Each section
explains what that category is for, when to reach for it, and what mistake to
avoid.

Second, jump to the linked docs at the end of that category. Those docs are the
source of truth for the exact surfaces and examples.

If you have no idea where to start, read these first:

- [Docs README](./README.md)
- [Choosing The Right Surface](./domain-capabilities/choosing/README.md)
- [Workspace Overview](./foundations/workspace-overview.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Async Resources And Result State](./capabilities/async-resources-and-result-state.md)
- [Downstream Runtime Integration](./foundations/downstream-runtime-integration.md)

## Choosing Guides

Choosing guides exist for the moment when several Query surfaces look equally
plausible and you need the shortest honest chooser before writing code.

They do not replace feature docs. They answer overlap questions such as binding
versus orchestration versus helpers, inspection versus readiness versus
recovery, grouped authoring versus grouped products versus grouped
contributions, and signal compatibility versus continuation.

Use this category when you already know the rough job but two or more Query
lanes still feel interchangeable.

The mistake to avoid is picking a surface because the name sounds familiar.
Query separates lanes that often look similar from the outside.

Read next:

- [Choosing The Right Surface](./domain-capabilities/choosing/README.md)
- [Binding Vs Orchestration Vs Helpers](./domain-capabilities/choosing/binding-vs-orchestration-vs-helpers.md)
- [Inspection Vs Readiness Vs Recovery](./domain-capabilities/choosing/inspection-vs-readiness-vs-recovery.md)
- [Grouped Authoring Vs Grouped Products Vs Grouped Contributions](./domain-capabilities/choosing/grouped-authoring-vs-grouped-products-vs-grouped-contributions.md)
- [Signal Compatibility Vs Continuation Pipeline](./domain-capabilities/choosing/signal-compatibility-vs-continuation-pipeline.md)

## Workflow Guides

Workflow guides are task-first paths across multiple Query surfaces. They show
how declaration work, retained artifacts, grouped neighborhoods, preview
steps, signal or continuation moves, and recovery fit together for one job.

Use this category when you know the end-to-end task but do not want to assemble
the path from isolated feature pages alone.

The mistake to avoid is treating a workflow guide as the authority boundary.
It is a navigation shortcut; feature docs still own the contracts.

Read next:

- [Workflow Guides](./domain-capabilities/workflow/README.md)
- [Single Declaration To Envelope](./domain-capabilities/workflow/single-declaration-to-envelope.md)
- [Retained Artifact To Next Step](./domain-capabilities/workflow/retained-artifact-to-next-step.md)
- [Envelope To Signal Or Continuation](./domain-capabilities/workflow/envelope-to-signal-or-continuation.md)
- [Grouped Neighborhood Workflow](./domain-capabilities/workflow/grouped-neighborhood-workflow.md)
- [Stop To Recovery](./domain-capabilities/workflow/stop-to-recovery.md)

## Recipes

Recipes are short, copy-oriented examples for common Query jobs. They help you
see one practical call shape before diving into the full mental model.

Use this category when you want a working shape first and will move to feature
docs for support posture, authority boundaries, and alternate lanes.

The mistake to avoid is treating a recipe as the complete contract. Recipes
compress; they do not replace admission, basis, or recovery semantics.

Read next:

- [Recipes](./domain-capabilities/recipes/README.md)
- [Prepare Preview From Active Face Selection](./domain-capabilities/recipes/prepare-preview-from-active-face-selection.md)
- [Attach Material With Declaration-Scoped Contributions](./domain-capabilities/recipes/attach-material-with-declaration-scoped-contributions.md)
- [Author A Grouped Neighborhood With Contributions](./domain-capabilities/recipes/author-a-grouped-neighborhood-with-contributions.md)
- [Turn A Stop Into A Recovery Action](./domain-capabilities/recipes/turn-a-stop-into-a-recovery-action.md)

## Public Runtime Facade

The public runtime facade is stabilized **per support-matrix row** on the
runtime-backed path. It is the part of Query that says “downstream runtimes can
build on this now” for families that are actually admitted—not every visible
export.

Conceptually, the workspace facade turns Query into a real platform layer. It
gives downstream code one public context for live views, computed state,
effects, reads, observation, materialization, preview and branch work, writes,
state snapshots, inspection, and support posture. Instead of forcing callers to
orchestrate lower runtime layers directly, the facade gives them one stable
surface and one stable vocabulary.

The most important thing to understand here is that the facade is not just a
nice naming pass. It is a support contract. Some concepts are public now so
later milestones can extend the same model, but not every visible concept is
already admitted as a stable production lane. That is why support posture and
admission belong beside the facade instead of after it.

Reach for this category when the task sounds like ordinary runtime-backed
product behavior: declaring retained surfaces, reading them, mutating truth,
opening preview or branch sessions, inspecting retained handles, or deciding
whether a public family is really supported today.

The main mistake to avoid is teaching support from autocomplete. Query
deliberately separates “public vocabulary” from “runtime-backed support.”

Read next:

- [Workspace Overview](./foundations/workspace-overview.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Query Operating Modes](./foundations/query-operating-modes.md)

## Query Operating Modes

Query deliberately supports more than one honest execution posture. The same
canonical query meaning can run runtime-backed against relational snapshots,
later run store-backed where admitted without changing query semantics, promote
to live maintenance without changing the query expression, or exist as
ephemeral saved-query or host-bound artifacts before durable store support
closes.

Use this category when the question is not “how do I author the query?” but
“which execution posture is real today, and what completion debt is still open?”

The mistake to avoid is claiming store-backed, restart-stable, or durable
cursor semantics because a helper name sounds persistent. Ephemeral and
store-gated debt must stay explicit.

Read next:

- [Query Operating Modes](./foundations/query-operating-modes.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Workspace Overview](./foundations/workspace-overview.md)

## Support And Admission

Support and admission explain what the runtime actually promises today.

Query exposes some public vocabulary before every neighbor is fully closed. That
is intentional: it lets downstream runtimes plan against the final public shape
without pretending everything is already implemented. The support matrix exists
to make that distinction explicit. It tells you what is stable, what is
deferred debt, what is visible-but-not-admitted yet, and what must fail closed.

Use this category whenever the real question is not “does this method exist?”
but “can I build on this honestly right now?” This is especially important near
intent-shaped families, temporal neighbors, async/resource neighbors, and
anything that looks like a future extension point.

The mistake to avoid here is assuming that visibility implies support. Query
wants support posture to be machine-checkable, not guessed from API surface
shape.

Read next:

- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Async Resources And Result State](./capabilities/async-resources-and-result-state.md)
- [Downstream Runtime Integration](./foundations/downstream-runtime-integration.md)
- [Writes And Intent Boundaries](./execution/writes-and-intents.md)

## Aspects And Authority Lanes

Aspects and authority lanes are one of the core concepts an AI has to
understand to use Query correctly.

Aspects are the semantic names Query uses for what a surface reads, produces,
writes, routes, or inspects. They are not casual dotted field names. They are
the auditable contract for semantic dependency and change. Authority lanes are
the ownership side of the same story: they tell Query whether state is
authoritative truth, branch-local truth, preview truth, derived runtime state,
effect delivery state, pending write intent, bridge external state, or an
explicit future temporal/async neighbor.

These concepts show up everywhere. Computeds declare what they read and
produce through aspects. Effects declare what they watch and where delivery
goes. State and inspection surfaces explain which lane a result belongs to.
Write receipts preserve aspect operations so later code can see what was
changed without reconstructing it from raw deltas.

Reach for this category when the real question is about dependency, production,
writes, triggers, or ownership. If the task depends on understanding what a
surface semantically reads or produces, or whether a result is authoritative,
derived, preview-local, or delivery-local, start here.

The mistake to avoid is treating aspects like incidental syntax and lanes like
debug labels. In Query, both are part of the runtime contract.

Read next:

- [Aspects And Authority Lanes](./modeling/aspects-and-authority-lanes.md)
- [Computed](./runtime-surfaces/computed.md)
- [Writes And Intent Boundaries](./execution/writes-and-intents.md)
- [Inspection](./capabilities/inspection.md)

## Query Expressions, Validation, Planning, And Execution

This category is the foundation for honest reads: typed query intent before
execution, schema-aware validation, proof-carrying plans, and snapshot-backed
one-shot execution.

Query expression families give collection/detail reads, aspect projection,
bounded traversal, and typed result shapes one canonical identity. Validation
fails illegal, over-broad, or schema-dishonest queries before planning.
Planning lowers intent once; execution consumes the plan without rediscovering
legality, projection, or scope on the hot path.

Use this category when you are defining what is being asked for, whether it is
legal, how it will be planned, or how one-shot execution binds to truth.

The mistake to avoid is host-local query builders, string predicates, or
execution paths that widen silently when validation should have failed closed.

Read next:

- [Query Expressions And Result Shapes](./authoring/query-expressions-and-result-shapes.md)
- [Read Composition](./authoring/read-composition.md)
- [Workspace Overview](./foundations/workspace-overview.md)

## Collections, Cursors, Ordering, And Aggregation Reads

Large-surface reads need first-class collection semantics, not loops of detail
queries or offset/limit pretending to be pagination.

Query owns typed ordering, opaque cursor pagination, bounded traversals,
aggregation and rollup families where admitted, query-time derived fields as
part of the canonical artifact, and CDC-shaped output that stays query-shaped
rather than raw runtime CDC.

Use this category when the job is list/table scale, stable pages, rollups,
aggregations, or integration-facing change-shaped output.

The mistake to avoid is unstable pagination, unbounded graph walks, or host
post-processing that re-derives fields Query should have planned.

Read next:

- [Collections, Cursors, Ordering, And Aggregations](./authoring/collections-cursors-ordering-and-aggregations.md)
- [Read Composition](./authoring/read-composition.md)
- [Query Expressions And Result Shapes](./authoring/query-expressions-and-result-shapes.md)

## Scopes, Templates, View Shapes, And Saved Queries

This category is Query’s productization layer for reusable query meaning:
named scopes, parameterized templates, admitted view shapes, and frozen
saved-query artifacts with explicit reuse posture.

View shapes are not display tags. They affect planning, invalidation narrowing,
delivery formatting, and live patch semantics. Scopes and templates must expand
to the same canonical query meaning as direct construction.

Use this category when you need reusable fragments, table/detail/grouped/
timeline/inspector presentation intent, or saved-query freeze and reuse
decisions.

The mistake to avoid is treating view shape as UI-only sugar or saved queries
as durable product completion before store-backed reload is honestly admitted.
On the runtime-backed application support profile, core view-family support
rows are already verified for `table`, `detail`, inspector detail, and
`kanban_grouped` surfaces, and grouped reusable composition/template support is
now admitted on the same runtime-backed product lane. Remaining grouped
follow-on work in this neighborhood is about later durable/store-backed
neighbors, not the grouped view-family row.

Read next:

- [Scopes, Templates, Saved Queries, And View Shapes](./authoring/scopes-templates-saved-queries-and-view-shapes.md)
- [Read Composition](./authoring/read-composition.md)
- [Live Views](./runtime-surfaces/live-views.md)

## Policy, Tenant, And Relationship-Proof Narrowing

Policy masking, tenant truth/schema basis, and relationship-proof queries are
structural query concerns, not post-read filters glued on by hosts.

Masking happens before execution so masked aspects never enter the plan or live
path. Tenant scoping narrows truth basis and schema basis explicitly.
Relationship-proof families stay typed query semantics with explicit denial when
the proof chain breaks. Delivery-shape metadata preserves the same masked and
projected meaning across one-shot, live, and historical lanes where admitted.

Use this category when reads must respect policy, tenant variation, branch
access, or proof-gated relationship access.

The mistake to avoid is over-reading and redacting later, ambient tenant
filters, or authorization callbacks that bypass canonical query artifacts.

Read next:

- [Policy, Tenant, And Relationship-Proof Narrowing](./foundations/policy-tenant-and-relationship-proof-narrowing.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Read Composition](./authoring/read-composition.md)
- [Basis Capability Lifecycle](./capabilities/basis-capability-lifecycle.md)

## Platform Entry For Serious Downstream Work

Platform entry is the Query-as-beginning seam for serious downstream domains:
one public boundary where declaration, progression, authority routing,
preparation, continuation, inspection, and ordinary product work start inside
Query instead of above it in local pseudo-Query layers.

Configured domain handles are the typed operating world you get after entry.
Platform entry is the broader “enter Query honestly” contract that those handles
assume.

Use this category when the product layer should treat Query as the daily-driver
runtime, not as a late adapter over relational, bridge, or signal crates.

The mistake to avoid is rebuilding declaration, preparation, or handoff
worlds locally while only calling Query for the final read or write.

Read next:

- [Platform Entry](./domain-capabilities/platform-entry.md)
- [Configured Domain Handles](./domain-capabilities/configured-domain-handles.md)
- [Canonical Domain Declarations](./domain-capabilities/canonical-domain-declarations.md)
- [Declaration Entry Orchestration](./domain-capabilities/declaration-entry-orchestration.md)

## Domain Entry And Configured Handles

Configured domain handles are the typed operating world you work in after Query
entry: one admitted handle with support posture attached, rather than raw
strings or ad hoc bootstrapping.

Platform entry (see the section above) is the broader serious-downstream boundary;
configured handles are the concrete handle your app or domain crate holds day to
day.

Use this category when the question is “which admitted handle should this product
layer hold for ordinary work?”

The mistake to avoid is expecting the handle alone to replace declarations,
basis, orchestration, or recovery lanes.

For runtime-backed read bring-up specifically, Query now also ships one simple
public bridge-backed bootstrap lane for obtaining a valid read runtime without
custom minimal assembly folklore. Use that ordinary builder-owned path for
hostile tests and downstream examples instead of rebuilding one-off bridge
fixtures above Query.

Read next:

- [Configured Domain Handles](./domain-capabilities/configured-domain-handles.md)
- [Platform Entry](./domain-capabilities/platform-entry.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## Declarations And Family Contracts

Declarations are how Query gives domain work stable identity and stable family
meaning.

If a request can be repeated, checked, routed, denied, deferred, grouped,
inspected, or used to avoid future dead ends, it needs canonical declaration
identity instead of a display string or host-local hash. Declaration family
contracts then describe what sort of thing the work is: relational,
descriptive, grouped, signal-compatible, legality-constrained, route-sensitive,
and so on.

Use this category when the problem is “define the work honestly.” That means
new request families, canonical identity, declaration family taxonomy, or any
place where lower-runtime posture should be part of declared meaning rather than
buried in local branching logic.

The mistake to avoid is treating declarations like passive DTOs. In Query they
are the stable identity boundary for runtime work the system can reason about
later.

Read next:

- [Canonical Domain Declarations](./domain-capabilities/canonical-domain-declarations.md)
- [Declaration Family Taxonomy](./domain-capabilities/declaration-family-taxonomy.md)
- [Declaration Family Capability Matrix](./domain-capabilities/declaration-family-capability-matrix.md)
- [Declaration Legality](./domain-capabilities/declaration-legality.md)
- [Declaration Progression](./domain-capabilities/declaration-progression.md)
- [Declaration Entry Inspection](./domain-capabilities/declaration-entry-inspection.md)
- [Declaration Foundational Evidence](./domain-capabilities/declaration-foundational-evidence.md)

## Readiness, Orchestration, Route, Receipt, And Envelope

This category answers the question: what happens after I have a declaration?

Query separates several concerns here on purpose. Readiness tells you whether a
runtime seam is actually available before you pretend it is. Orchestration gives
you a public lowering path from declaration work into route, receipt, and
envelope truth. Receipts and envelopes retain what happened so later code does
not need to reverse-engineer the path from side effects and logs.

Use this category when the problem is “I have declaration-shaped work and need
to know what Query can do with it now.” Use readiness for seam posture before
execution. Use orchestration when you want Query to lower the work through its
public path. Use route/receipt/envelope artifacts when you need the result to
stay structured and inspectable.

The mistake to avoid is treating readiness as a debug extra or receipts as
optional output. In Query they are part of the public knowledge model.

Read next:

- [Declaration Entry Readiness](./domain-capabilities/declaration-entry-readiness.md)
- [Declaration Entry Orchestration](./domain-capabilities/declaration-entry-orchestration.md)
- [Declaration Route Plan](./domain-capabilities/declaration-route-plan.md)
- [Declaration Boundary Receipts](./domain-capabilities/declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./domain-capabilities/declaration-boundary-envelopes.md)
- [Declaration Bridge Continuation Routing](./domain-capabilities/declaration-bridge-continuation-routing.md)

## Ordinary Outcomes

Ordinary outcomes are the compact public result vocabulary for binding,
declaration-entry orchestration, continuation preparation, and signal-
compatibility orchestration.

They keep non-success categories distinct—denied, refused, stale, rebind-
required, wrong-world, wrong-handle, basis mismatch, authority mismatch,
unsupported, ambiguous—without collapsing into one local `Result` or string.

Use this category when you need one concise outcome value that still links back
to the checked topology underneath, especially before handing a stop to recovery.

The mistake to avoid is flattening ordinary outcomes into booleans or inventing
a parallel status enum Query already represents elsewhere.

Read next:

- [Ordinary Outcomes](./domain-capabilities/ordinary-outcomes.md)
- [Typed Binding Pipeline](./domain-capabilities/typed-binding-pipeline.md)
- [Recovery Boundary](./domain-capabilities/recovery-boundary.md)

## Typed Binding And Retained Artifact Reuse

This category exists for the moment when the next explicit step should come from
an already-retained Query artifact.

Without a real binding pipeline, this is where callers start inventing ambient
recovery: they pull route meaning, receipt meaning, workspace meaning, or basis
meaning out of host context or object relationships. Query instead makes that
reuse explicit and typed, with stale, rebind-required, wrong-world, and
wrong-handle posture built into the public lane.

Use this category when the next step starts from a retained route, receipt,
envelope, continuation, or other artifact rather than from a fresh declaration.
If your instinct is “I can probably infer the next input from this object,” that
usually means you should look here first.

The mistake to avoid is hidden dependency injection. Query wants reuse to stay
auditable and machine-checkable.

Read next:

- [Typed Binding Pipeline](./domain-capabilities/typed-binding-pipeline.md)
- [Retained Artifact To Next Step](./domain-capabilities/workflow/retained-artifact-to-next-step.md)
- [Binding Vs Orchestration Vs Helpers](./domain-capabilities/choosing/binding-vs-orchestration-vs-helpers.md)

## Basis Capability Lifecycle

A Query basis is a phase-typed capability lifecycle, not a raw branch head,
snapshot id, preview handle, or tenant label passed through host context.

Basis intent normalizes, becomes eligible or denied, admits as a capability,
scopes execution or observation, binds lower-runtime truth, emits use receipts,
and returns self-describing envelopes. Read, mutate, replay, inspect, and
materialize surfaces consume basis proofs rather than rediscovering authority
from identifiers alone.

Use this category when the job depends on which truth world, preview world,
historical world, or tenant/policy world a surface is allowed to use—and what
transition is legal next.

The mistake to avoid is threading raw relational or bridge ids through product
code when Query expects basis capability artifacts.

Read next:

- [Basis Capability Lifecycle](./capabilities/basis-capability-lifecycle.md)
- [Historical Diff And Basis](./capabilities/historical-diff-and-basis.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## State Readiness Vs Inspection

`workspace.state(...)` answers typed readiness posture for a retained surface
or public facade family: ready, pending, unsupported, or otherwise not in a
normal ready lane. Inspection answers richer **per-target retained evidence**
after work has run. Cross-runtime causal explanation is a separate lane—see
[Cross-Runtime Causal Inspection](./capabilities/cross-runtime-causal-inspection.md).

Declaration entry readiness is a third neighbor: it tells you whether a
declaration seam is available before you orchestrate, not what a live handle’s
runtime posture is right now.

Use state when you need a digest-bound posture snapshot without full
explanation. Use inspection when you need why. Use declaration readiness before
you lower new declaration work.

The mistake to avoid is guessing support from handle behavior, or using
inspection when you only needed readiness—or the reverse.

Read next:

- [State And Readiness Surfaces](./foundations/state.md)
- [Inspection](./capabilities/inspection.md)
- [Declaration Entry Readiness](./domain-capabilities/declaration-entry-readiness.md)
- [Inspection Vs Readiness Vs Recovery](./domain-capabilities/choosing/inspection-vs-readiness-vs-recovery.md)

## Recovery

This category is for paths that stopped or narrowed instead of simply
continuing. For explanation **contributions** (domain declaration posture), see
[Explanation Contributions](./domain-capabilities/explanation/lower-runtime-explanation-contributions.md)—not this section.

Query uses recovery surfaces so stop states do not collapse into one vague local
error. Denied, deferred, stale, rebind-required, unsupported, and other stop
classes are part of the runtime’s knowledge model. Recovery turns those stops
into typed next-step posture instead of leaving them as comments, strings, or
host-local exception handling.

Use this category when you need to explain why a path stopped, or when the next
step is a repair action instead of a normal continuation.

The mistake to avoid is inventing one local error family that erases the
runtime’s distinctions.

Read next:

- [Recovery Boundary](./domain-capabilities/recovery-boundary.md)
- [Recovery Requests And Next-Step Actions](./domain-capabilities/recovery/recovery-requests-and-next-step-actions.md)
- [Stop To Recovery](./domain-capabilities/workflow/stop-to-recovery.md)
- [Inspection Vs Readiness Vs Recovery](./domain-capabilities/choosing/inspection-vs-readiness-vs-recovery.md)

## Grouped And Neighborhood Work

This category exists because some operations are grouped by meaning, not by
iteration convenience.

Sometimes the correct abstraction is not “many isolated declarations,” but one
grouped or neighborhood-shaped operation whose members, outputs, contributions,
and support posture belong together semantically. Query treats that as a real
category instead of letting it dissolve into loops and caller-owned grouping
folklore.

Use this category when grouped shape affects support, orchestration, products,
or contributions. If removing the grouped shape changes the meaning of the
operation, this is probably the right category.

The mistake to avoid is pretending grouped work is only a batching helper. In
Query it is a semantic category.

Read next:

- [Grouped Authoring](./domain-capabilities/grouped-authoring.md)
- [Grouped Products](./domain-capabilities/grouped-products.md)
- [Grouped Contributions](./domain-capabilities/grouped-contributions.md)
- [Grouped Support And Readiness](./domain-capabilities/grouped-support-readiness.md)
- [Grouped Neighborhood Workflow](./domain-capabilities/workflow/grouped-neighborhood-workflow.md)

## Domain Capability Contributions

This is the public domain capability contribution seam. It exists so downstream
domains can contribute typed semantic posture while Query remains the owner of
canonical runtime artifacts.

That ownership split is the whole point. Domains contribute meaning and
evidence. Query materializes canonical artifacts. This prevents downstream
domains from solving contribution problems by minting local pseudo-Query
artifacts or by exposing canonical constructors directly.

This category is proof-bearing from the start. The progression from request to
eligibility to admitted contribution to materialization-ready contribution to
canonical runtime materialization is a real typed lifecycle. On top of that,
Query documents contribution lanes across admission, support/traceability,
invariant/capability posture, workflow, continuity, aftermath, and explanation—
**not every orchestration row is fully closed**; check per-lane support reports.

Use this category when the domain needs to add semantic posture to Query-owned
runtime truth. If the problem is “the domain needs to say something important
about runtime posture, but Query should still own the final artifact,” this is
where you start.

The mistake to avoid is solving contribution problems by making canonical
artifact construction local or by flattening contribution meaning into generic
strings.

Read next:

- [Contributions Hub](./domain-capabilities/contributions/README.md)
- [Contribution-Composed Orchestration](./domain-capabilities/contribution-composed-orchestration.md)
- [Invariant And Capability Contributions](./domain-capabilities/invariants/invariant-and-capability-contributions.md)
- [Registering Domain Invariants Through Query](./domain-capabilities/invariants/registering-domain-invariants-through-query.md)

## Lower-Runtime Capability Routing

All ordinary Query contact with relational, runtime bridge, signal, and later
store-adjacent surfaces should pass through capability-routed boundary envelopes,
not scattered direct imports or convenience shortcuts.

Routing names authority, route plan, capability, cost posture, failure topology,
and retained evidence. Compatibility debt for any remaining direct path must
stay explicit rather than becoming a silent escape hatch around basis, admission,
projection, effect, or inspection contracts.

Use this category when work must touch lower runtimes and you need the honest
Query-owned route rather than “import bridge/relational/signal for speed.”

The mistake to avoid is choosing a lower crate by convenience and bypassing
basis, admission, and envelope contracts Query already owns publicly.

Read next:

- [Lower-Runtime Capability Routing](./domain-capabilities/lower-runtime-capability-routing.md)
- [Declaration Bridge Continuation Routing](./domain-capabilities/declaration-bridge-continuation-routing.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## Live Views And Live Promotion

A live view is a durable, query-shaped runtime surface over authoritative truth.
Live promotion means the same canonical query expression can be maintained
incrementally through query-shaped patches rather than as a separate reactive
product with different meaning.

Live maintenance must converge to the same result as re-executing the canonical
one-shot query on the same basis, with suppression and invalidation explained in
query terms—not raw CDC events or host observer folklore.

Use this category when you need current rows or view-shaped records, query-
shaped write patches, or a retained surface that computeds, effects, previews,
and inspection can reuse.

The mistake to avoid is treating live views as thin subscriptions to raw truth
streams or as a different query language from one-shot reads.

Read next:

- [Live Views](./runtime-surfaces/live-views.md)
- [Reads, Observe, And Materialize](./runtime-surfaces/reads-observe-materialize.md)
- [Scopes, Templates, Saved Queries, And View Shapes](./authoring/scopes-templates-saved-queries-and-view-shapes.md)

## Subscriptions

Subscriptions are first-class query artifacts, not ambient observer glue around
a live view handle.

Query lowers admitted live meaning into subscription declaration families with
their own identity, basis binding, bridge lowering, sharing, continuation,
preview isolation, and family-aware delivery. Automatic family selection must
remain bridge-honest and diagnostically sufficient rather than smuggling a fake
default subscription kind.

Use this category when work is long-lived observation: shared equivalent
subscriptions, continuation after identity evolution, preview-scoped
subscriptions, or understanding which bridge and signal strategies were selected.

The mistake to avoid is hand-rolling observers, inferring subscription meaning
from CDC, or collapsing all live families into one generic runtime lane.

Read next:

- [Subscription Selection And Diagnostics](./capabilities/subscription-selection-and-diagnostics.md)
- [Live Views](./runtime-surfaces/live-views.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## Region-Scoped Live Invalidation And Stream Contracts

When truth changes only touch a bounded region or partition of a query’s
declared scope, live maintenance should narrow to that region and emit delivery
metadata that stays query-shaped.

Change-stream-backed delivery contracts may lower where the bridge admits them,
but the consumer contract remains query-shaped result maintenance—not raw
partition events or transport-local stream glue.

Use this category for geometry-grade locality, integration feeds, or large
collections where broad aspect invalidation would be disproportionate.

The mistake to avoid is widening to full-aspect or full-collection refresh when
planner-owned region narrowing was available.

Read next:

- [Region-Scoped Live Invalidation And Stream Contracts](./runtime-surfaces/region-scoped-live-invalidation-and-stream-contracts.md)
- [Live Views](./runtime-surfaces/live-views.md)
- [Live View Vs Subscription](./domain-capabilities/choosing/live-view-vs-subscription.md)

## Workspace Runtime Surfaces

The workspace is the unified facade context for retained runtime-backed product
work: computed state, preview or branch sessions, reads, observation,
materialization, state snapshots, and inspection entrypoints that share one
configuration and support contract.

Live views, subscriptions, effects, basis lifecycle, and write paths have their
own categories because they carry extra identity, lowering, or authority rules.
This section is the umbrella for everything else you declare or consume through
`workspace` without re-deriving lower-runtime wiring.

The important mental model is retained handles and digest-bound evidence, not
throwaway callbacks or host-local stores.

That same category now includes the simple public bridge-backed read-runtime
bootstrap closed in Milestone 9.5: hostile tests and downstream bring-up can
obtain a valid raw read runtime through the ordinary Query builder-owned lane
instead of inventing custom scaffolding first.

Use this category when you are operating inside the stabilized facade and need
the overview of which workspace methods belong to which retained surface family.

The mistake to avoid is treating the workspace as permission to skip category-
specific contracts for live, subscription, effect, basis, or mutation work.

Read next:

- [Workspace Overview](./foundations/workspace-overview.md)
- [Computed](./runtime-surfaces/computed.md)
- [Reads, Observe, And Materialize](./runtime-surfaces/reads-observe-materialize.md)
- [Branches And Previews](./foundations/branches-and-previews.md)
- [State And Readiness Surfaces](./foundations/state.md)

## Authority-Scoped Effects

Effects are retained delivery or staging surfaces that react to live or computed
changes. They are not a general truth-mutation lane.

Query lowers effect intent once through an authority-scoped pipeline: eligibility,
scoped plan, lowered execution plan, receipt, and self-describing envelope.
Executors must consume lowered proofs rather than re-deciding authority, basis,
invariant scope, preview policy, route strategy, or artifact policy.

Use this category when you need conditional delivery, meaningful-change
suppression, staged pending work, or covered write-intent residue that later
admits through the intent lattice.

The mistake to avoid is hiding business logic inside effects or mutating truth
directly from an effect callback.

Read next:

- [Effects](./execution/effects.md) — authoring and staging
- [Authority-Scoped Effect Execution](./execution/authority-scoped-effect-execution.md) — lifecycle matrix
- [Intent Admission](./execution/intent-admission.md)

## Writes And Intent Boundaries

This category answers the question: how should truth change happen, and when
does that change belong on a direct write path versus an intent path?

Query is explicit here because runtime-heavy domains need more than “a mutation
happened.” Direct writes are the stable everyday path when product code already
knows the mutation to perform. Covered intent families exist too, but they
belong on the admitted intent lattice instead of in a vague “everything is an
intent now” story.

Write receipts are important in their own right. They preserve aspect
operations, target evidence, existing-truth binding evidence, causality, batch
evidence, continuity-aware authority evidence, and touched surface routing.
That is how downstream code can explain what actually happened without
rebuilding the story from raw deltas or lower-runtime logs.

Use this category when you are performing authoritative mutation now, when you
need graph-shaped same-batch authoring, when you need existing-truth binding or
verification, or when you need covered mutation intent families.

The mistake to avoid is teaching `workspace.write(...)` as the ordinary public
mutation story. It exists as a lower-level seam, but the preferred public lane
is the aspect-native mutation vocabulary plus the higher-level graph/existing-
truth lanes when they are the honest fit.

Read next:

- [Writes And Intent Boundaries](./execution/writes-and-intents.md)
- [Intent Admission](./execution/intent-admission.md)
- [Graph Composition Authoring](./authoring/graph-composition-authoring.md)
- [Existing Truth](./capabilities/existing-truth.md)

## Intent Admission Decision Lattice

**Covered intent families** resolve through a structured admission decision
lattice before construction, lowering, or covered execution—not every public
`Intent` export is admitted; check the matrix per family.

Success, advisory, and violation outcomes carry decision traces and typed
context rather than collapsing into a binary wall. Covered families cross into
real bridge-backed execution through typed admitted handoffs.

Use this category when you need to know whether an intent may proceed, proceed
with advisory posture, or stop with violation evidence—and what trace to
preserve for inspection or recovery.

The mistake to avoid is treating admission as “it returned Ok” or rebuilding
admission logic locally after Query already classified the intent.

Read next:

- [Intent Admission](./execution/intent-admission.md)
- [Writes And Intent Boundaries](./execution/writes-and-intents.md)
- [Advisory And Violation Contributions](./domain-capabilities/admission/advisory-and-violation-contributions.md)

## Authoritative Mutation Evidence

Write-heavy work needs more than a mutation succeeded. The public mutation-
evidence contract preserves target identity, causality, existing-truth binding,
aspect operations, batch and continuity evidence, and authority explanation so
downstream code does not rebuild carry-forward folklore above Query.

Write receipts are one visible part of that story; the broader contract keeps
Query and bridge carry-forward aligned for replay and inspection.

Use this category when you must explain what was touched, under which authority,
against which existing truth, and with what continuity-sensitive evidence.

The mistake to avoid is reconstructing mutation stories from raw deltas, logs, or
local side maps when Query already issued receipt-grade evidence.

Read next:

- [Authoritative Mutation Evidence](./capabilities/authoritative-mutation-evidence.md)
- [Writes And Intent Boundaries](./execution/writes-and-intents.md)
- [Inspection](./capabilities/inspection.md)

## Signal Compatibility And Continuation

This category is for the work that sits between declared Query meaning and lower
reactive execution.

Its job is to make signal-facing posture explicit before execution and to
provide a continuation pipeline instead of local callback folklore. Rather than
assuming “reactive behavior exists somewhere below,” Query gives public,
retained artifacts that say whether a path is compatible, prepared, denied, or
still stopped.

Use this category when the feature needs invalidation, recomputation, signal
compatibility review, prepared continuation artifacts, or the next-step move
from envelope truth into signal-facing execution.

The mistake to avoid is treating signal compatibility as something the caller
can safely infer from lower behavior without using Query’s public posture.

Read next:

- [Declaration Signal Compatibility](./domain-capabilities/declaration-signal-compatibility.md)
- [Signal Compatibility Orchestration](./domain-capabilities/signal-compatibility-orchestration.md)
- [Continuation Pipeline](./domain-capabilities/continuation-pipeline.md)
- [Envelope To Signal Or Continuation](./domain-capabilities/workflow/envelope-to-signal-or-continuation.md)

## Bridge-Facing Workflow, Merge, And Writeback

This category covers the point where Query-authored domain work needs preview,
workflow, mutation planning, merge inspection, or writeback lowering into lower
authority lanes.

The important distinction here is that Query may own the public planning and
inspection lane without owning lower truth mutation, merge semantics, or
writeback execution themselves. This is why workflow declarations, mutation
lowering, merge inspection, and writeback declarations can be public Query
surfaces without turning Query into the owner of all lower bridge semantics.

Use this category when the job sounds like workflow declaration, preview-bound
inspection, mutation lowering, merge analysis, or writeback planning.

The mistake to avoid is either bypassing Query for bridge semantics or
pretending Query now owns lower truth mutation just because it owns the planning
surface.

Read next:

- [Workflow README](./domain-capabilities/workflow/README.md)
- [Preview Inspection And Mutation Planning](./domain-capabilities/workflow/preview-inspection-and-mutation-planning.md)
- [Runtime-Preflight Workflow Contributions](./domain-capabilities/workflow/runtime-preflight-workflow-contributions.md)
- [Branches And Previews](./foundations/branches-and-previews.md)

## Relational Truth And Invariants Through Query

This category exists because downstream domains often need relational truth or
invariant posture, but that does not mean the right answer is to import
Relational directly from ordinary domain code.

Query gives public surfaces for relational truth routing, invariant
registration, capability gaps, and invariant denials so that domains can use
lower truth semantics through a real public lane. The ownership split is that
Relational remains the authority, but Query owns the ordinary public access and
orchestration shape.

Use this category when the feature needs invariants, relational truth, joins,
capability-gap posture, or lower truth reasoning that should be visible in the
public domain surface.

The mistake to avoid is deciding “this is about truth or invariants, so I
should skip Query.” In this architecture, a major part of the point is that
domains should not have to invent their own relational-entry folklore above the
runtime.

Read next:

- [Declaration Relational Truth Routing](./domain-capabilities/declaration-relational-truth-routing.md)
- [Registering Domain Invariants Through Query](./domain-capabilities/invariants/registering-domain-invariants-through-query.md)
- [Capability Gaps And Invariant Denials](./domain-capabilities/invariants/capability-gaps-and-invariant-denials.md)

## Read Composition And Graph Authoring

Read composition covers collection/detail authoring, validated query bundles, and
planning hooks over canonical read shape. Graph composition authoring covers
graph-shaped read and mutation authoring that stays on the same runtime
substrate.

Expression, validation, planning, and collection semantics have their own
categories above; this section is where you compose reads and graph work that
should remain Query-owned rather than host-local.

Use this category when you are building read bundles or graph-shaped authoring
that must lower through Query’s canonical artifacts.

Graph-owned lookup is part of this authority boundary. If a Query-owned feature
repeatedly needs to find nodes by canonical identity, resolve owner or placement
kind, enumerate legal children or targets, check scope or boundary posture, or
splice/move within a structured authoring graph, first establish one canonical
Query-owned graph/index view and consume that view. Do not start with recursive
tree walks, per-call registry scans, or surface-local lookup helpers and treat a
later index as a mere performance cleanup. In Query, the graph/index is part of
the proof boundary, not just an optimization.

The mistake to avoid is duplicating query legality or planning in domain helpers
when read composition or graph composition already owns the lane. A closely
related mistake is host-local traversal folklore: helper loops that reconstruct
graph meaning separately inside commands, scoped editing, legality, or
UI-adjacent surfaces.

Read next:

- [Read Composition](./authoring/read-composition.md)
- [Graph Composition Authoring](./authoring/graph-composition-authoring.md)
- [Query Expressions And Result Shapes](./authoring/query-expressions-and-result-shapes.md)

## Frontier-Aware Planning And Parallel Admission

Planning can consume frontier and cost posture so bulk queries, live
maintenance, and multi-query bundles admit parallel preparation only where
legality and canonical meaning stay explicit.

Serial and admitted-parallel paths must remain parity-safe: parallelism changes
dispatch, not query semantics.

Use this category when scale pressure risks opaque planner heuristics or
executor rediscovery of planning decisions.

The mistake to avoid is treating parallel admission as a host thread-pool concern
outside proof-carrying plan artifacts.

Read next:

- [Planner Parallel Admission And Scale Posture](./authoring/planner-parallel-admission-and-scale-posture.md)
- [Read Composition](./authoring/read-composition.md)

## Structural Correspondence And Historical Materialization

Structural correspondence and historical materialization-path metadata make
historical evaluation explicit and ambiguity-honest when truth identity or
materialization path affects what a query may read.

This sits beside branch/historical/diff contexts and lineage work: it is about
how Query names correspondence and historical materialization without host cache
repair or silent basis substitution.

Use this category when historical reads depend on structural match posture,
materialization path identity, or explicit rejection of ambiguous historical
targets.

The mistake to avoid is reconstructing history through ambient host caches
instead of declared basis and materialization contracts.

Read next:

- [Structural Correspondence And Historical Materialization](./capabilities/structural-correspondence-and-historical-materialization.md)
- [Historical Diff And Basis](./capabilities/historical-diff-and-basis.md)
- [Lineage And Correspondence](./capabilities/lineage-and-correspondence.md)

## Cross-Runtime Causal Inspection

This is the **`CausalInspection` lane** (`admit_causal_inspection`,
`request_causal_inspection`) for cross-runtime causal explanation—not
`workspace.inspect`, which is per-target retained evidence only.

`CrossRuntimeCausalExplanation` at reference-only richness is **supported**;
materialized detail is **advisory**. Durable causal archive and store-backed
replay reconstruction are **deferred**.

Use this category when the question is end-to-end “why across runtimes?”—not
“what does inspect retain for this handle?”

The mistake to avoid is calling `workspace.inspect` cross-runtime causal inspection,
or using explanation contributions instead of the causal inspection API.

Read next:

- [Cross-Runtime Causal Inspection](./capabilities/cross-runtime-causal-inspection.md)
- [Inspection](./capabilities/inspection.md)
- [Inspection Vs Cross-Runtime Explanation](./domain-capabilities/choosing/inspection-vs-cross-runtime-explanation.md)
- [Lower-Runtime Explanation Contributions](./domain-capabilities/explanation/lower-runtime-explanation-contributions.md)

## Projection Consumption And Typed Facts

Projection consumption is the declared, receipt-backed lane for using
materialized query facts without reopening source authority.

Consumers declare which identities, memberships, labels, topology facts,
workflow facts, or view-local facts they consume; Query returns typed fact
receipts bound to the materialization digest, basis, policy, and view shape that
produced them.

Use this category when retained rows or payload bags are too weak and you need
typed facts Query already materialized.

The mistake to avoid is fishing in relational truth, bridge internals, or domain
caches for IDs and memberships that should have been declared as consumed
projection facts.

Retained derived-artifact bindings and live-artifact bindings now participate as
first-class projection-consumption declaration, support-discovery, and typed
fact-consumption sources. Ordinary callers should use
`consume_projection_facts(...)` on those bindings instead of falling back to
older runtime-owned retained/live helper seams.

Read next:

- [Projection Consumption](./capabilities/projection-consumption.md)
- [Async Resources And Result State](./capabilities/async-resources-and-result-state.md)
- [Projection Consumption Vs Inspection](./domain-capabilities/choosing/projection-consumption-vs-inspection.md)
- [Policy, Tenant, And Relationship-Proof Narrowing](./foundations/policy-tenant-and-relationship-proof-narrowing.md)

## Family Helpers And Declaration Progression

Family helpers expose family-native ergonomics over the same canonical
declaration, orchestration, binding, and recovery surfaces—they are not a second
execution engine.

Declaration progression moves declaration work forward through typed phases
without rebuilding earlier identity or re-deriving route meaning from host state.

Use helpers when the honest fit is ergonomic composition over an admitted family.
Use progression when you already have declaration identity and need the next
phase in the public pipeline.

The mistake to avoid is helpers that smuggle alternate semantics, or progression
that skips readiness, receipts, or envelopes when the pipeline requires them.

Read next:

- [Family Helpers](./domain-capabilities/family-helpers.md)
- [Declaration Progression](./domain-capabilities/declaration-progression.md)
- [Orchestration Inventory](./domain-capabilities/orchestration-inventory.md)
- [Binding Vs Orchestration Vs Helpers](./domain-capabilities/choosing/binding-vs-orchestration-vs-helpers.md)

## Temporal And Time-Aware Live Queries

Temporal query basis and time-aware subscriptions now ship on the
runtime-backed path as an extension of the same canonical query and
subscription model.

The shipped shape distinguishes truth time-travel from signal execution time,
admits time-only deliveries where no truth patch occurred, and lowers temporal
basis through bridge and signal authorities without making Query the owner of
clocks, wake queues, or reactive scheduling.

Use the support matrix and admission docs to see which ordinary runtime-backed
lanes are shipped and which sibling facade-family roots remain intentionally
support-gated. Do not invent parallel temporal APIs or ambient host timers.

The mistake to avoid is implementing stale-after, interval, deadline, or
rolling-window behavior as UI timers outside canonical query artifacts, or
confusing historical truth reads with clock-driven live reevaluation.

Read next:

- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Historical Diff And Basis](./capabilities/historical-diff-and-basis.md)
- [Subscription Selection And Diagnostics](./capabilities/subscription-selection-and-diagnostics.md)

## Async Resources And Result State

Async capabilities are the Query-owned way to model resource-backed or
completion-driven meaning without inventing a second async facade, a local
`loading` taxonomy, or host-owned retry folklore.

The important split is:

- declaration-side async meaning lives on canonical declaration input through
  async clauses such as source family, request identity, loading posture, and
  failure posture
- runtime-backed async state lives on the same live/state/inspection surfaces
  as the rest of Query, where retained result-state can become `pending`,
  `current`, `stale`, `cancelled`, `retried`, `revalidating`, `superseded`, or
  `denied`
- projection consumption, continuation, recovery, and downstream delivery carry
  that async posture forward instead of asking callers to reopen lower-runtime
  artifacts or transport callbacks

Use this category when the question is:

- how do I declare async/resource meaning honestly?
- where do I read current async result-state?
- how does async posture survive materialization or downstream delivery?
- what does replay, stale completion, or async-request drift look like?

The mistake to avoid is assuming async support means “there must be a
`workspace.async(...)` API somewhere.” Query does not work that way. Async
meaning is part of existing declaration, live, inspection, projection, and
continuation lanes.

Read next:

- [Async Resources And Result State](./capabilities/async-resources-and-result-state.md)
- [Canonical Domain Declarations](./domain-capabilities/canonical-domain-declarations.md)
- [Inspection](./capabilities/inspection.md)
- [Projection Consumption](./capabilities/projection-consumption.md)
- [Continuation Pipeline](./domain-capabilities/continuation-pipeline.md)

## Decision Rules

Need the shortest path between close surfaces:

- use choosing guides, then workflow guides, then recipes

Need platform entry or operating world:

- use platform entry, configured domain handles, support snapshot, and
  operating-mode honesty

Need typed query read meaning:

- use query expressions, validation, planning, collections, scopes/templates/view
  shapes, and read composition—in that dependency order

Need policy, tenant, or proof-gated access:

- use policy/tenant/relationship-proof narrowing before execution, not post-read
  filters

Need domain work/request:

- use typed Query declaration input, family marker, progression, and helpers only
  over the same canonical surfaces

Need identity/deduplication:

- use canonical declaration entries and canonical declaration artifacts

Need family support:

- use family taxonomy, capability matrix, readiness, inventory, and support
  admission

Need basis for read, mutate, replay, inspect, or materialize:

- use basis capability lifecycle, not raw branch or snapshot ids

Need posture before or after a run:

- use state/readiness before guessing; use inspection after; use declaration
  entry readiness before orchestration

Need compact orchestration/binding/continuation result:

- use ordinary outcomes before flattening to local errors

Need relational invariants/truth:

- use Query relational truth, invariant contribution, or invariant registration
  surfaces first

Need one-shot or live read execution:

- use planning and snapshot-backed execution first; use live views for durable
  query-shaped surfaces; use subscriptions for long-lived admitted live meaning

Need signal/reactive behavior:

- use signal compatibility, continuation, and subscription surfaces first

Need async/resource-backed declaration meaning or retained async runtime state:

- use async declaration clauses, retained live async result-state, projection
  consumption posture, and continuation/recovery drift surfaces instead of
  inventing host-local loading or retry models

Need effects or staged delivery:

- use authority-scoped effects and intent admission, not ad hoc callbacks

Need graph mutation/writeback/bridge routing:

- use workflow, bridge routing, writeback lowering, lower-runtime capability
  routing, and receipt/envelope surfaces first

Need intent admission or mutation evidence:

- use intent admission lattice and authoritative mutation evidence, not local Ok/
  Err wrappers

Need domain-authored capability posture:

- use the domain capability contribution seam and the relevant admission,
  support, workflow, continuity, aftermath, or explanation lane

Need serious runtime-backed product work:

- use the workspace facade and support/admission contract **per admitted family row**

Need retained artifact to become next input:

- use typed binding/resolver surfaces

Need failure/recovery:

- use ordinary outcomes, checked stops, recovery brief, and recovery boundary

Need grouped/neighborhood semantics:

- use grouped authoring/products/contribution surfaces, not local loops over
  isolated declarations

Need cross-runtime why:

- use cross-runtime causal inspection

Need materialized facts without reopening authority:

- use projection consumption declarations and receipts

Need lower-runtime contact:

- use lower-runtime capability routing and boundary envelopes
- obtain boundary envelopes from real Query boundary receipts or other
  `ForgeQueryLowerRuntimeBoundaryEnvelopeSource` values; do not construct or
  synthesize envelopes from strings

Need temporal or time-aware live behavior:

- check support matrix first; do not invent parallel temporal APIs because the
  shipped runtime-backed temporal surface lives on ordinary Query handles while
  sibling facade-family roots remain intentionally support-gated

Need async capabilities:

- read the async capabilities doc first; do not assume there is a separate
  async facade or that blanket async family visibility means every runtime
  profile admits ordinary async DX

Need public DX:

- expose a domain facade that forwards to Query instead of teaching raw lower
  runtime plumbing

## Hard Prohibitions

- Do not start from lower runtime crates for ordinary domain work.
- Do not build local pseudo-Query layers.
- Do not create a second admission path.
- Do not bypass admitted Query handles.
- Do not erase canonical declaration identity.
- Do not flatten Query outcomes into booleans.
- Do not invent local status enums for states Query already represents.
- Do not log failures when they can become structured Query/runtime facts.
- Do not mint proof/theorem/writeback/bridge/signal/relational authority
  locally.
- Do not expose internal domain module topology when a facade should own the
  public surface.
- Do not add crates merely to mirror Query, bridge, relational, or signal
  layers.
- Do not assume a public method is supported because it compiles.
- Do not teach `workspace.write(...)` as the default runtime mutation story.
- Do not add sibling public APIs for future async or temporal work; check support
  matrix for admitted neighbors instead.
- Do not replace Query async result-state with local `loading`, `retrying`, or
  `cancelled` enums unless you are intentionally projecting it for product UX.
- Do not implement temporal or time-aware live semantics with ambient host
  clocks or timers outside the shipped Query runtime-backed temporal surface.

## AI Checklist Before Editing Code

Before building on a Query category, answer these:

1. What category am I actually in?
2. What is the public entrypoint for that category?
3. What is the canonical identity boundary?
4. What Query artifact or outcome should be preserved instead of flattened?
5. What support row or admission gate decides whether the surface is real now?
6. Am I using Query to carry lower-runtime semantics, or am I bypassing Query
   and inventing a local runtime path?

If you cannot answer those, read the owning docs before writing code.

## When In Doubt

Use this decision order:

1. Query public docs
2. Query facade surface
3. support matrix / admission
4. inspection
5. lower-layer docs only to understand semantics

If the current public lane cannot do the job honestly, do not invent a local
runtime above the lower layers. Stop, read the owning docs, and choose the
nearest honest public Query lane first.

