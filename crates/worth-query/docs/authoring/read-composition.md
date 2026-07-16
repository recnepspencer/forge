# Read Composition

## What This Feature Is

`compose_read(...)` is the first public read-composition surface in
`worth-query`.

It lets you declare one bounded graph-shaped read, execute it through the
runtime, and get back both the derived payload and an attached receipt that
explains what kind of read actually ran.

Use this when you need a graph neighborhood or traversal-shaped read and you do
not want to rebuild that neighborhood manually from raw rows.

If the read shape carries graph obligation meaning, Graph Touch Obligation
Authority is the check-selection path. Read composition declares graph-shaped
access; obligation authority decides which registered obligations, diagnostic
postures, budgets, and evidence apply to that access in the current operating
world.

## Why You Use It

- you want one obvious happy path for bounded graph reads
- you want a canonical read artifact instead of ad hoc helper loops
- you want traversal breadth and fallback posture attached to the result
- you want a runtime surface that can grow toward richer graph-native reads
  without changing the mental model again
- you need graph-bearing read evidence to stay aligned with the same obligation
  vocabulary used by mutation, live-read, preview, branch, and construction
  lanes

This feature is especially useful when a domain layer would otherwise be
tempted to:

- drain raw rows from the runtime
- walk relations in caller-owned loops
- issue repeated neighbor lookups that quietly widen into N+1 behavior

## Stable Entry Points

Current stable entry points:

- `workspace.compose_read(...)`
- `workspace.compose_read_with_invariant_pack(...)`
- `workspace.define_read_family(...)`
- `workspace.define_read_family_with_invariant_pack(...)`
- `workspace.execute_read_family(...)`
- `workspace.execute_read_family_with_access_plan(...)`
- `workspace.execute_read_family_in_basis_context(...)`
- `workspace.execute_read_family_in_basis_context_with_access_plan(...)`
- `workspace.read_family_intent(&family).review()`
- `workspace.public_read_composition_support_report(...)`
- `workspace.public_read_composition_phase_one_closeout(...)`
- `workspace.public_read_composition_phase_gate(...)`
- `WorthQueryReadBuilder`
- `WorthQueryReadFamily`
- `WorthQueryReadGraph`
- `WorthQueryReadResult`
- `WorthQueryReadReceipt`
- `WorthQueryReadDenial`

Current builder surface in this first slice:

- `read.local_collection(...)`
- `read.local_detail(...)`
- `read.local_direct_edge_collection(...)`
- `read.local_direct_edge_detail(...)`
- `read.local_successor_walk_collection(...)`
- `read.local_successor_walk_detail(...)`
- `read.local_shared_endpoint_collection(...)`
- `read.local_shared_endpoint_detail(...)`
- `read.local_shared_attachment_collection(...)`
- `read.local_shared_attachment_detail(...)`
- `read.anchored_collection(...)`
- `read.anchored_detail(...)`
- `read.anchored_bounded_ancestor_collection(...)`
- `read.anchored_bounded_ancestor_detail(...)`
- `read.anchored_bounded_descendant_collection(...)`
- `read.anchored_bounded_descendant_detail(...)`
- `read.anchored_frontier_collection(...)`
- `read.anchored_frontier_detail(...)`
- `read.explicit_broad_search_frontier_collection(...)`
- `read.explicit_broad_search_frontier_detail(...)`
- `read.explicit_broad_search_collection(...)`
- `read.explicit_broad_search_detail(...)`

These are the first public scope-classed read shapes. They are the public
starting point, not the full finished read-composition kernel.

## Graph Read Access Accountability

Read composition is the authoring surface for graph-shaped reads. Graph read
access planning is the accountability surface beneath it.

For a covered graph read, the declaration should lower into:

```text
read declaration
-> graph read access requirement set
-> access admission
-> admitted access plan or typed denial
-> receipt access-plan consumption counters
```

Use [Graph Read Access Planning](./graph-read-access-planning.md) when you need
to inspect the access plan, understand why a broad read denied, prove a helper
did not perform caller-owned N+1 work, or explain required persistent index,
streaming, async materialization, store-backed capability, or domain capability
registration posture.

The declaration is the authoring surface. The access plan is the accountability
surface. Friendly calls such as `execute_read_family(...)` may plan and execute
in one step only when the receipt exposes the same admitted access-plan evidence
available through explicit planning.

The explicit accountability shape is:

```rust
let review = workspace.read_family_intent(&family).review()?;
let access_plan = review.graph_read_access_plan()?;

let result = workspace.execute_read_family_with_access_plan(
    &family,
    access_plan.clone(),
)?;

let consumed = result
    .receipt()
    .graph_read_access_plan_consumption()
    .ok_or_else(|| missing_access_plan_consumption())?;

assert_eq!(consumed.plan_digest(), access_plan.digest());
assert_eq!(
    consumed
        .execution_counters()
        .per_result_neighbor_lookup_count(),
    0,
);
```

For historical, preview, branch, or other admitted basis-context reads, use the
same accountability shape with
`workspace.execute_read_family_in_basis_context_with_access_plan(...)`. The
basis context decides which runtime world is read; the access plan still proves
which graph-read structures were admitted and consumed.

Do not treat a broad boolean graph read, high-fanout traversal, or missing
required index as a reason to rebuild the graph locally. It should return a
typed access posture such as `persistent_index_required`,
`paged_streaming_required`, `async_materialization_required`, or `denied`.

Good to know:

- the first traversal-backed runtime proof currently covers `local_detail(...)`
- the first anchored-expansion proof currently covers `anchored_detail(...)`
- the first explicit broad-search proof currently covers
  `explicit_broad_search_detail(...)`
- ordered collection reads are now admitted through the same surface across
  local, anchored-expansion, and explicit broad-search scope classes
- unordered collection reads now inherit canonical identity ordering from the
  shared declarative request path instead of denying by default

## Core Mental Model

Think of `compose_read(...)` as the read-side sibling of graph composition.

You are not asking the runtime to hand you raw rows and hoping you rebuild the
graph correctly afterward. You are declaring one bounded read graph:

1. choose a root
2. declare projections, predicates, ordering, and traversal
3. bind that declaration to a schema view
4. execute the admitted read once
5. receive the derived payload and a receipt together

The canonical artifact underneath that flow is `WorthQueryReadGraph`.

The lower request story matters too. `compose_read(...)` now derives the same
declarative request shape used by live views before canonicalization. That
means read composition and live declarations no longer carry separate
traversal-, predicate-, or ordering-shaped truth.

That artifact is important because it freezes:

- the read family (`Detail` or `Collection`)
- the scope class
- the schema basis
- the planned execution bundle

The result is not just data. It is data plus an execution explanation.

Good to know:

- if the attached `WorthQueryReadResult` payload is not a strong enough
  boundary for the caller, feed that result into
  [Projection Consumption](../capabilities/projection-consumption.md) to extract typed facts
  with their own receipt and envelope
- that fact lane now preserves temporal, async, mixed-cause, and remask-bound
  materialized posture when the read materialization carried it, instead of
  flattening the result into ordinary row folklore

Some entrypoints now own traversal mechanically instead of asking the caller to
declare it by convention:

- direct-edge operators add the single-hop traversal themselves
- successor-walk operators add the repeated successor traversal themselves and
  reserve the local repeated-walk lane for bounded walks deeper than one hop
- shared-attachment operators add one single-hop traversal per shared relation
  and require at least two distinct relations
- shared-endpoint operators add the same bounded one-hop traversal shape, but
  keep a distinct built-in operator identity for endpoint/vertex-style local
  neighborhood questions
- bounded ancestor and bounded descendant operators add the bounded traversal
  themselves and reserve that lane for walks deeper than one hop
- anchored-frontier operators add one traversal clause per frontier relation
  and require `max_depth > 1`, so one-hop shared reads stay on the local
  shared-attachment lane
- frontier-search operators add one traversal clause per frontier relation and
  require at least one predicate, so broad search is an explicit graph-native
  lane rather than an unlabeled deeper traversal
- the declaration closure on those paths intentionally has no `.traverse(...)`
  method, so callers cannot smuggle a second traversal shape into an
  operator-owned read

When repetition becomes real, you do not need to keep rewriting the same
closure wrapper by hand. The kernel now exposes `WorthQueryReadFamily` as a
reusable read artifact:

1. define one family from a canonical `ReadGraph`
2. optionally gate it through an invariant pack
3. execute that same frozen family repeatedly

That keeps the one-shot happy path beautiful while still giving repeated reads
their own first-class product surface.

Reusable families now also carry their admission history explicitly:

- plain reusable families report `KernelOnly`
- invariant-gated families report `DomainInvariantAdmitted(...)`

That admission history is part of the family digest, so a plain reusable family
and an invariant-admitted family with the same read graph do not silently
collapse into the same identity.

## How It Executes

The current execution path is:

1. `compose_read(...)` opens a `WorthQueryReadBuilder`
2. the builder authors a collection or detail query plus result shape
3. the runtime derives one declarative lower request from that authored shape
4. that request canonicalizes into one `WorthQueryReadGraph`
5. validation and planning admit that read graph against the schema view
6. the runtime resolves the current runtime snapshot basis
7. the current execution substrate runs the preflight bundle
8. the runtime returns `WorthQueryReadResult { payload, receipt }`

If a domain needs to reject an otherwise supported read graph before
execution, `workspace.compose_read_with_invariant_pack(...)` inserts one extra
pre-execution step:

1. the runtime builds the same admitted `WorthQueryReadGraph`
2. it exposes that graph through `WorthQueryReadInvariantPackContext`
3. a domain hook either admits the graph or returns a typed violation
4. admitted graphs execute normally
5. denied graphs surface through
   `WorthQueryRuntimeError::ReadCompositionDomainInvariantDenied(...)`

The receipt currently reports:

- read-graph digest
- graph family
- query digest
- basis digest
- result digest
- snapshot token
- scope class
- execution engine
- fallback class and count
- operator families
- built-in operator coverage
- relationship-proof posture
- breadth counters

The invariant-pack path adds one more typed inspection surface:

- a `WorthQueryReadDomainInvariantSummary`
- the graph family and scope class the domain hook inspected
- operator families, built-in operator coverage, and declared traversal breadth
- the planned read-surface count and summary digest for the denied graph

The first real relationship-proof contract is now:

- reads with no traversal report `NotRequired`
- traversal-bearing reads report `DescriptorAdmittedSyntheticRuntime`
- that admitted posture carries a synthetic runtime-read descriptor admission
  identity plus a read-shape-specific relationship-proof support profile digest
  and verified/deferred/forbidden surface counts
- bounded ancestor and bounded descendant operators keep distinct topology
  proof identity on that receipt surface; descendant reads do not collapse back
  into ancestor proof labeling

This is intentionally honest. The read kernel now admits that traversal-shaped
reads freeze a real descriptor-backed proof artifact before execution, but the
receipt still distinguishes deferred runtime proof evaluation from the admitted
descriptor topology.

For collection reads, those breadth counters now include the first
collection-specific execution posture:

- page width
- page truncation count
- cursor advance count
- materialized relation count

That means you can inspect the shape of the read without reverse-engineering it
from helper code.

The current public scope classes are:

- `LocalNeighborhood`
- `AnchoredExpansion`
- `ExplicitBroadSearch`

They are not just labels. The builder enforces basic shape honesty:

- local neighborhood currently admits only queries that remain local under the
  kernel classifier
- anchored expansion currently requires a query shape that classifies as
  anchored expansion
- explicit broad search currently requires a query shape that classifies as
  explicit broad search

The important rule is that callers do not get to relabel the same admitted
query arbitrarily. The kernel classifies the query shape and denies mismatched
builder requests.

## Small Example

```rust
use worth_query::facade::read::{
    AspectFieldSelector, AspectName, AuthoredResultShapeField, FieldName, QuerySchemaView,
    RelationName, ScalarAspectType, SchemaFieldView, SchemaRelationView, TraversalSelector,
};

let mut workspace = runtime.workspace("editor")?;

let result = workspace
    .compose_read(|read| {
        read.anchored_detail(
            "Task",
            QuerySchemaView::new(
                "task-read",
                [
                    SchemaFieldView::new(
                        AspectName::new("identity")?,
                        FieldName::new("id")?,
                        ScalarAspectType::String,
                    ),
                    SchemaFieldView::new(
                        AspectName::new("title")?,
                        FieldName::new("value")?,
                        ScalarAspectType::String,
                    ),
                ],
                [SchemaRelationView::new(RelationName::new("depends_on")?, 2)],
            ),
            |query| {
                query
                    .project(AspectFieldSelector::new("identity", "id")?)
                    .project(AspectFieldSelector::new("title", "value")?)
                    .traverse(TraversalSelector::bounded("depends_on", 2)?)
            },
            |shape| {
                shape
                    .field(AuthoredResultShapeField::new("identity", "id", "id")?)
                    .field(AuthoredResultShapeField::new("title", "value", "title")?)
            },
        )
    })
    ?;

assert_eq!(result.receipt().scope_class().as_str(), "anchored_expansion");
```

If you want an operator-owned anchored walk instead of an open-coded traversal,
use the bounded-walk entrypoints directly:

```rust
let result = workspace
    .compose_read(|read| {
        read.anchored_bounded_descendant_detail(
            "Task",
            task_schema,
            depends_on_relation,
            2,
            |query| query.project(AspectFieldSelector::new("title", "value")?),
            |shape| shape.field(AuthoredResultShapeField::new("title", "value", "title")?),
        )
    })?;
```

That keeps the walk on a typed built-in operator lane, reports
`BoundedDescendant` coverage on the receipt, and denies one-hop uses that
should really be written as `local_direct_edge_*`.

This is the smallest honest example because it shows the full loop:

- one declared read
- one canonical read graph
- one execution
- one receipt

If your domain needs to reject some admitted read shapes, use the invariant
pack variant instead of rebuilding those rules outside the kernel:

```rust
use worth_query::facade::{
    read::{
        AspectFieldSelector, AspectName, AuthoredResultShapeField, FieldName, QuerySchemaView,
        RelationName, ScalarAspectType, SchemaFieldView, SchemaRelationView, TraversalSelector,
    },
    runtime::WorthQueryReadInvariantPackViolation,
};

let mut workspace = runtime.workspace("org-chart")?;

let manager_chain = workspace
    .compose_read_with_invariant_pack(
        |read| {
            read.anchored_detail(
                "User",
                QuerySchemaView::new(
                    "org-chart-read",
                    [
                        SchemaFieldView::new(
                            AspectName::new("identity")?,
                            FieldName::new("id")?,
                            ScalarAspectType::String,
                        ),
                        SchemaFieldView::new(
                            AspectName::new("profile")?,
                            FieldName::new("display_name")?,
                            ScalarAspectType::String,
                        ),
                    ],
                    [SchemaRelationView::new(RelationName::new("manager")?, 2)],
                ),
                |query| {
                    query
                        .project(AspectFieldSelector::new("identity", "id")?)
                        .project(
                            AspectFieldSelector::new("profile", "display_name")?,
                        )
                        .traverse(TraversalSelector::bounded("manager", 2)?)
                },
                |shape| {
                    shape
                        .field(AuthoredResultShapeField::new("identity", "id", "id")?)
                        .field(
                            AuthoredResultShapeField::new(
                                "profile",
                                "display_name",
                                "display_name",
                            )
                            ?,
                        )
                },
            )
        },
        |context| {
            let summary = context.read_domain_invariant_summary();
            if summary.declared_traversal_depth_limit() > 2 {
                Err(WorthQueryReadInvariantPackViolation::new(
                    "manager-depth-budget",
                    "manager-chain reads may not traverse deeper than 2 hops",
                ))
            } else {
                Ok(())
            }
        },
    )
    ?;

assert_eq!(
    manager_chain.receipt().scope_class().as_str(),
    "anchored_expansion"
);
```

When you need to run the same read more than once, define a reusable read
family instead of building a hand-rolled wrapper around `compose_read(...)`:

```rust
let family = workspace
    .define_read_family("manager-chain", |read| {
        read.anchored_detail("User", schema, declare_query, declare_result_shape)
    })
    ?;

let first = workspace.execute_read_family(&family)?;
let second = workspace.execute_read_family(&family)?;

assert_eq!(
    first.receipt().read_graph_digest(),
    second.receipt().read_graph_digest()
);
```

When the caller already holds an admitted query basis context, execute the same
family through `workspace.execute_read_family_in_basis_context(...)` instead.
That preserves the reusable read graph while letting the receipt report whether
the admitted basis was current, branch, historical, or preview-derived. If the
workspace is not itself bound to the admitted non-current basis, the returned
rows are derived from the admitted query-context execution artifact instead of
from current-head runtime materialization. The runtime denies the call before
materialization if the context query digest does not match the family read
graph, so an admitted basis cannot be substituted for an unrelated reusable
read family.

Domain runtimes that need to bind a read-only runtime snapshot into that path
should resolve the basis through `resolve_runtime_current_snapshot_basis(...)`
and then preflight the reusable family execution plan. That keeps
`ResolvedSnapshotIdentity` sealed while still producing a basis digest and
snapshot token that `execute_read_family_in_basis_context(...)` can preserve on
the historical read receipt.

If the reusable family must carry domain admission proof, define it through the
invariant-packed variant and give that admission a stable family name:

```rust
let family = workspace
    .define_read_family_with_invariant_pack(
        "manager-chain",
        "manager_depth_budget",
        |read| read.anchored_detail("User", schema, declare_query, declare_result_shape),
        |context| {
            let summary = context.read_domain_invariant_summary();
            if summary.declared_traversal_depth_limit() > 2 {
                Err(WorthQueryReadInvariantPackViolation::new(
                    "manager_depth_budget",
                    "manager-chain families may not traverse deeper than 2 hops",
                ))
            } else {
                Ok(())
            }
        },
    )
    ?;

assert!(matches!(
    family.admission(),
    worth_query::facade::runtime::WorthQueryReadFamilyAdmission::DomainInvariantAdmitted(_)
));
```

Use the family form when:

- the same read shape recurs in one subsystem
- you want one frozen reusable artifact
- you want repeated execution without reauthoring the closure every time

## Real Example

```rust
use worth_query::facade::read::{
    AspectFieldSelector, AspectName, AuthoredResultShapeField, EqualityPredicate, FieldName,
    QuerySchemaView, RelationName, ScalarAspectType, SchemaFieldView, SchemaRelationView,
    WorthQueryPredicateOperand,
};

let mut workspace = runtime.workspace("org-chart")?;

let manager_detail = workspace
    .compose_read(|read| {
        read.explicit_broad_search_detail(
            "User",
            QuerySchemaView::new(
                "org-chart-read",
                [
                    SchemaFieldView::new(
                        AspectName::new("identity")?,
                        FieldName::new("id")?,
                        ScalarAspectType::String,
                    ),
                    SchemaFieldView::new(
                        AspectName::new("profile")?,
                        FieldName::new("display_name")?,
                        ScalarAspectType::String,
                    ),
                ],
                [SchemaRelationView::new(RelationName::new("manager")?, 1)],
            ),
            |query| {
                query
                    .project(AspectFieldSelector::new("identity", "id")?)
                    .project(AspectFieldSelector::new("profile", "display_name")?)
                    .where_equal(
                        EqualityPredicate::new(
                            "profile",
                            "display_name",
                            WorthQueryPredicateOperand::string("Ada".to_string()),
                        )
                        ?,
                    )
            },
            |shape| {
                shape
                    .field(AuthoredResultShapeField::new("identity", "id", "id")?)
                    .field(
                        AuthoredResultShapeField::new(
                            "profile",
                            "display_name",
                            "display_name",
                        )
                        ?,
                    )
            },
        )
    })
    ?;

assert_eq!(
    manager_detail.receipt().scope_class().as_str(),
    "explicit_broad_search"
);
assert!(manager_detail
    .receipt()
    .operator_families()
    .contains(&worth_query::facade::runtime::WorthQueryReadOperatorFamily::Predicate));
```

Good to know:

- this is explicitly a broad-search read, so the broad-search entrypoint is
  required and the receipt must not claim local neighborhood posture
- schema views still control which traversals and predicate targets are legal
- the returned receipt tells you which operator families actually shaped the
  read
- if the same authored query shape is actually local or anchored, the
  broad-search builder denies instead of relabeling it

The first admitted ordered collection shape looks like this:

```rust
let team = workspace
    .compose_read(|read| {
        read.local_collection(
            "User",
            schema,
            |query| {
                query
                    .project(AspectFieldSelector::new("identity", "id")?)
                    .project(AspectFieldSelector::new("profile", "display_name")?)
                    .traverse(TraversalSelector::bounded("manager", 1)?)
                    .order_by(OrderingSelector::ascending("profile", "display_name")?)
            },
            |shape| {
                shape
                    .field(AuthoredResultShapeField::new("identity", "id", "id")?)
                    .field(
                        AuthoredResultShapeField::new(
                            "profile",
                            "display_name",
                            "display_name",
                        )
                        ?,
                    )
            },
        )
    })
    ?;

assert_eq!(
    team.receipt().graph_family(),
    &WorthQueryReadGraphFamily::Collection
);
assert_eq!(team.receipt().breadth().execution_cursor_advance_count(), 1);
```

Use this pattern when you want a one-shot ordered collection read with the
same attached receipt model as detail reads.

If you omit `order_by(...)` on a collection read, the runtime still
canonicalizes the request with identity ordering so the collection stays
stable under the shared declarative contract.

If your read is really “one direct edge hop” or “one bounded ancestor walk,”
prefer the operator-owned entrypoints instead of open-coded traversal:

```rust
let result = workspace
    .compose_read(|read| {
        read.anchored_bounded_ancestor_detail(
            "User",
            schema,
            RelationName::new("manager")?,
            2,
            |query| {
                query.project(AspectFieldSelector::new("identity", "id")?)
            },
            |shape| {
                shape.field(AuthoredResultShapeField::new("identity", "id", "id")?)
            },
        )
    })
    ?;

assert_eq!(result.receipt().scope_class().as_str(), "anchored_expansion");
```

Use these operator-owned entrypoints when:

- you want the graph operator itself to own traversal shape
- you want the declaration closure to stay projection/predicate/order only
- you want the kernel, not convention, to enforce that this read remains a
  direct-edge, successor-walk, bounded-ancestor, or anchored-frontier form
- you want the attached receipt to report the exact built-in operator coverage
  instead of only generic traversal presence

If you mean “follow one successor relation repeatedly, but still treat that
bounded walk as one local neighborhood question,” use the successor-walk
entrypoints:

```rust
let result = workspace
    .compose_read(|read| {
        read.local_successor_walk_detail(
            "User",
            schema,
            RelationName::new("next")?,
            3,
            |query| {
                query.project(AspectFieldSelector::new("identity", "id")?)
            },
            |shape| {
                shape.field(AuthoredResultShapeField::new("identity", "id", "id")?)
            },
        )
    })
    ?;

assert_eq!(result.receipt().scope_class().as_str(), "local_neighborhood");
assert_eq!(
    result.receipt().built_in_operator_coverage(),
    &[WorthQueryReadBuiltInOperator::SuccessorWalk]
);
```

Use successor-walk when:

- one repeated relation is the operator shape
- the walk is bounded and still semantically local
- you want the kernel to distinguish this from a generic anchored-expansion
  builder

If you mean “expand from this root through a small explicit frontier of
relations,” use the anchored-frontier entrypoints instead of hand-writing
multiple `.traverse(...)` calls:

```rust
let result = workspace
    .compose_read(|read| {
        read.anchored_frontier_collection(
            "User",
            schema,
            [
                RelationName::new("manager")?,
                RelationName::new("mentor")?,
            ],
            2,
            |query| {
                query
                    .project(AspectFieldSelector::new("identity", "id")?)
                    .order_by(OrderingSelector::ascending("profile", "display_name")?)
            },
            |shape| {
                shape.field(AuthoredResultShapeField::new("identity", "id", "id")?)
            },
        )
    })
    ?;

assert_eq!(result.receipt().scope_class().as_str(), "anchored_expansion");
assert_eq!(
    result.receipt().built_in_operator_coverage(),
    &[WorthQueryReadBuiltInOperator::AnchoredFrontier]
);
```

## How It Relates To Other Features

- Use `workspace.live_view(...)` when you want a retained live surface whose
  rows and patches stay installed in the runtime.
- Use `workspace.read(...)` when you already have a retained live view and only
  need its current entities.
- Use `compose_read(...)` when you need one bounded graph read without
  materializing a named retained live view first.
- Use [Projection Consumption](../capabilities/projection-consumption.md) when the returned
  `WorthQueryReadResult` must become typed consumed facts instead of staying a
  payload-plus-receipt artifact.
- Policy, tenant, and relationship-proof narrowing detail lives in
  [policy, tenant, and relationship-proof narrowing](../foundations/policy-tenant-and-relationship-proof-narrowing.md)
  (masking, descriptors, deferred policy-aware live/historical parity)—not duplicated here.
- Collection cursors, ordering, aggregates, and CDC-shaped collection planning live in
  [collections, cursors, ordering, and aggregations](collections-cursors-ordering-and-aggregations.md).
- Reusable composition posture for named scopes and templates lives in
  [scopes, templates, saved queries, and view shapes](scopes-templates-saved-queries-and-view-shapes.md).
  The application support report now publishes `named_scope_expansion:verified`
  and `template_instantiation:verified` there. Observed-inspector and
  focused-inspector template neighbors remain explicitly deferred, while
  grouped collection templates are part of the admitted runtime-backed
  template lane.
- Use graph composition when you are authoring writes or mutation intent, not
  reads.

So the split is:

- retained query surfaces: `live_view(...)`
- one-shot bounded graph reads: `compose_read(...)`
- reusable bounded graph reads: `define_read_family(...)` plus
  `execute_read_family(...)`
- basis-aware reusable bounded graph reads:
  `define_read_family(...)` plus `execute_read_family_in_basis_context(...)`
- authoritative mutation composition: graph composition / write surfaces

Later domain adoption is expected to plug into the read kernel through four
frozen extension seams:

- `domain_read_family_lowering`
- `domain_invariant_pack`
- `domain_decoder`
- `domain_result_certification`

Those are the sanctioned kernel extension hooks for later Worth adoption. They
exist so topology, trim, carrier, NURBS, fillet, or branch-history domains do not
need to invent a second local read stack around raw rows.

## Inspection And Debugging

Start with the attached receipt.

The receipt answers most of the first debugging questions:

- what read graph ran?
- which query digest and basis digest were used?
- did traversal participate?
- what was the planned traversal depth limit?
- did fallback happen?
- how many read operations and records were involved?
- was relationship proof actually needed for this read?
- if it was needed, what runtime-backed proof support profile applied?

If a read fails before execution, the error comes back as
`WorthQueryReadDenial`, which is then surfaced through
`WorthQueryRuntimeError::ReadCompositionDenied(...)`.

That means denial remains typed at the public runtime boundary.

Built-in operator denials now carry structured evidence too. For example,
anchored-frontier failures expose:

- the denied built-in operator
- the operator-specific denial reason
- the user-facing denial message

So callers do not have to infer frontier failure meaning from message text
alone.

Relationship-proof admission denials are structured too. If synthetic
runtime-context admission or descriptor admission fails, the denial exposes:

- the relationship-proof denial stage
- the exact policy admission failure class, when synthetic runtime context was
  the failing boundary
- the exact relationship-proof failure class, when descriptor admission was
  the failing boundary

If a domain invariant pack rejects an admitted graph, the runtime uses
`WorthQueryRuntimeError::ReadCompositionDomainInvariantDenied(...)` instead.
That denial carries the rejected graph summary through
`denial.domain_invariant_summary()`.

If you need the closeout/readiness answer for the generic kernel itself, the
public support artifacts are part of this feature too:

- `workspace.public_read_composition_support_report()` freezes the admitted
  generic kernel surface
- `workspace.public_read_composition_phase_one_closeout()` freezes the safe
  assumptions, remaining non-assumptions, adoption guidance, and required
  verification commands
- `workspace.public_read_composition_phase_gate()` turns the readiness answer
  into typed runtime evidence for:
  - generic-kernel completion
  - Worth-adoption readiness
  - the fact that the broader topology-first aggregate closeout gate is now
    satisfied for resume

## Anti-Patterns

Avoid these:

- draining raw rows and reconstructing a graph neighborhood by hand
- wrapping repeated neighbor lookup loops around `compose_read(...)`
- building your own ad hoc "read family" wrapper once the same read clearly
  repeats
- open-coding `.traverse(...)` when a direct-edge or bounded-ancestor operator
  entrypoint already expresses the real read shape more honestly
- reaching for `local_shared_attachment_*` when the neighborhood question is
  really shared endpoint / shared vertex semantics and should stay on the
  dedicated shared-endpoint lane
- pretending a broad search is a local neighborhood read
- using invariant packs to paper over missing generic kernel capability
- treating the payload as the whole product and ignoring the receipt
- using `compose_read(...)` as a hidden alias for unrestricted whole-view scans

If a workflow needs retained patches, live invalidation, or recurring
subscription semantics, `live_view(...)` is usually the better fit.

## Current Limits

This is the first public slice, not the end state.

Current limits:

- public builder coverage is still small and centered on detail/collection
  shapes across three scope classes
- the `public_read_composition_phase_gate()` artifact now freezes the generic
  operator surface as complete
  for initial domain adoption, including both shared-neighborhood lanes:
  `SharedEndpoint` and `SharedAttachment`
- `WorthQueryReadFamily` is currently a process-owned reusable artifact, not a
  runtime-persisted registry entry
- invariant-admitted reusable families now carry explicit admission evidence,
  but family admission is still an artifact-level proof surface rather than a
  separate compile-time family type
- the `public_read_composition_phase_one_closeout()` artifact now freezes the
  extension-hook posture for later domain adoption, but those hooks still
  certify the generic kernel boundary, not completed Worth-side domain
  vocabularies
- the `public_read_composition_phase_gate()` artifact now freezes that the
  generic kernel is complete, that Worth may adopt it through domain-owned
  facades, and that the broader topology-first aggregate proof has closed for
  resume
- read-family execution can now use an already-admitted current, branch,
  historical, or preview-derived query basis context, while the one-shot
  `compose_read(...)` path still resolves the current runtime snapshot basis
- the lower declarative request can now preserve hidden query-only projection,
  delivered result fields, non-equality predicates, traversal, and ordering
- collection reads no longer require an explicit ordering clause to stay
  stable, but the default identity ordering may still be less expressive than
  an explicit app-level ordering
- anchored expansion and explicit broad search currently classify the request
  and enforce basic scope-shape honesty, but they still execute on the same
  underlying runtime path as the first local-neighborhood slice
- the current scope classifier is intentionally simple and deterministic:
  predicates classify broad search, while wider traversal shapes classify
  anchored expansion
- relationship-proof posture is now real enough to distinguish non-traversal
  reads from traversal-bearing reads; traversal reads now carry a real
  synthetic-runtime descriptor admission, while runtime proof evaluation still
  remains explicitly deferred in the receipt support profile
- this does not automatically migrate every future domain-level neighborhood
  helper in downstream crates; each later domain still needs its own
  domain-owned facade, decoded views, and aggregate closeout proof

In other words, the surface is real and stable enough to build against, but it
is still growing toward the full read-composition kernel described in the wider
Query roadmap.

## Related Docs

- [Workspace Overview](../foundations/workspace-overview.md)
- [Read Composition Closeout](../../../../_docs/worth-query/read-composition-phase1-closeout.md)
- [Projection Consumption](../capabilities/projection-consumption.md)


