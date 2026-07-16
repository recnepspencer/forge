# Graph Read Access Planning

Graph read access planning is the access, cost, and accountability lane for
declared graph reads. It is the part of Query that makes a graph-shaped read
prove which access structures it used instead of asking the caller to trust a
helper, a hidden index, or a local traversal loop.

Ordinary ORMs, graph helpers, reactive runtimes, and application frameworks may
run graph-shaped reads or generate indexes. Query's different claim is that the
read declaration lowers into a proof-bearing runtime lane: derived access
requirements, typed admission or denial, runtime-owned support rows, and
receipts that prove the admitted plan was consumed with exact counters.

The declaration is the authoring surface. The access plan is the accountability
surface. A caller describes graph read intent once; Query lowers that intent
into a proof-bearing access shape, admits or denies the shape against support
rows and budgets, and records the consumed access plan in the read receipt.

This is separate from [Graph Touch Obligation Authority](./graph-touch-obligation-authority.md).
Graph touch obligation authority decides which graph meaning must be checked.
Graph read access planning decides which access structures are required to read
that graph without caller-owned loops, hidden N+1 traversal, or RAM-expansive
work disguised as a cheap read.

This is not automatic index everything. The available read families, access
requirements, postures, and receipt fields remain visible. If Query cannot
prove a safe runtime-owned path, the result is a typed denial or required
posture, not an invisible fallback into caller-owned graph walking.

## Proof Chain

The ordinary proof chain is:

```text
authored read declaration
-> admitted schema/query references
-> graph read access shape
-> access requirement set
-> access admission
-> admitted access plan
-> execution binding
-> read receipt plus access-plan consumption counters
```

Callers should not skip from a declared read family to local graph traversal.
If a covered graph read needs adjacency, predicate support, ordering support,
frontier storage, visited sets, result buffers, materialization lifecycle, live
maintenance, or domain operation capability registration, those requirements
must appear before execution in the admitted access plan or in a typed denial.

## Public Flow

Use the high-level read declaration as the readable entry point, then inspect
the plan before execution when the cost or support posture matters.

```rust
fn tenant_face_neighborhood(
    workspace: &mut WorthQueryWorkspace,
    current_tenant: WorthQueryPredicateOperand,
) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
    let family = workspace.define_read_family("tenant-face-neighborhood", |read| {
        read.local_successor_walk_collection(
            "HalfEdge",
            topology_schema()?,
            topology_relation("HalfEdgeNext")?,
            4,
            |query| {
                query
                    .where_equal(topology_predicate_field("tenant", "tenant_id")?, current_tenant)
                    .project(topology_projection_field("identity", "half_edge_id")?)
                    .project(topology_projection_field("topology", "face_id")?)
                    .project(topology_projection_field("topology", "next_id")?)
            },
            |shape| {
                shape
                    .field(topology_result_field("identity", "half_edge_id", "half_edge_id")?)
                    .field(topology_result_field("topology", "face_id", "face_id")?)
                    .field(topology_result_field("topology", "next_id", "next_id")?)
            },
        )
    })?;

    let review = workspace.read_family_intent(&family).review()?;
    let access_plan = review.graph_read_access_plan()?;

    for requirement in access_plan.admission().requirement_set().rows() {
        tracing::debug!(
            requirement = requirement.kind().as_str(),
            contract = requirement.complexity_contract().as_str(),
            rebuild = requirement.rebuild_basis().as_str(),
            invalidation = requirement.invalidation_basis().as_str(),
        );
    }

let result = workspace.execute_read_family_with_access_plan(&family, access_plan.clone())?;
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

Ok(result)
}
```

For a read that executes in an admitted basis context, review the basis-context
intent and execute the reviewed access plan with
`workspace.execute_read_family_in_basis_context_with_access_plan(...)`. The
basis context owns which runtime world is read; the graph-read access plan owns
the proof of admitted access structures inside that world.

The helper functions in this example stand for domain-owned schema references.
Raw names may appear at declaration boundaries, but the runtime-facing plan must
carry admitted references, requirement rows, admission posture, and receipt
counters. Do not promote strings, comments, or local lookup tables into access
authority after declaration.

## Admission Postures

Graph read access admission has these postures:

- `inline_indexed`: the required access structures are already available inline.
- `bounded_ephemeral_index`: Query may provision a bounded per-execution index.
- `admitted_paged_streaming`: the read may execute through admitted streaming.
- `paged_streaming_required`: the shape must use paged streaming before it can
  execute.
- `persistent_index_required`: the shape needs a persistent graph index.
- `async_materialization_required`: the shape must run through admitted async
  materialization.
- `store_backed_capability_required`: the shape requires store-backed support.
- `access_capability_registration_required`: the shape requires a domain or
  lower-runtime capability registration.
- `denied`: the shape is not admitted.

These are not documentation labels. They are the public labels returned by
`WorthQueryGraphReadAccessAdmissionPosture::as_str()`.

## Denial Kinds

Typed denials are part of the public contract:

- `budget_exceeded`
- `required_async_materialization`
- `required_access_capability_registration`
- `required_persistent_index`
- `unsupported_graph_index_support`

A broad boolean predicate, high-fanout traversal, or large result shape may
deny with `budget_exceeded` and suggest `async_materialization_required`.
That is not a prompt to increase an arbitrary limit and retry. It is a typed
statement that the operation needs a different admitted access posture.

## Requirement Rows

Representative complex graph reads expose exact requirement rows before
execution. The core requirement vocabulary is:

- `directional_adjacency`
- `reverse_adjacency`
- `predicate_support`
- `ordering_support`
- `traversal_workset`
- `visited_set`
- `dedup_set`
- `proof_support`
- `result_buffer`
- `materialization_lifecycle`
- `live_maintenance_support`
- `domain_operation_capability_registration`

For a tenant-filtered face-neighborhood walk, the representative requirement
set should make the derived structures visible before execution:

| Requirement | Why It Exists |
| --- | --- |
| `directional_adjacency` | traverse `HalfEdgeNext` in the declared direction |
| `predicate_support` | filter by `tenant.tenant_id` before expansion |
| `traversal_workset` | bound and account for frontier work |
| `visited_set` | avoid repeating already-seen topology nodes |
| `dedup_set` | keep result identity canonical when paths converge |
| `result_buffer` | account for resident result memory |

If this shape becomes too large for inline or bounded ephemeral execution, the
same declaration must return a typed required posture such as
`persistent_index_required`, `paged_streaming_required`, or
`async_materialization_required`. It must not fall back to caller-owned frontier
loops or ad hoc adjacency maps.

## Required Capability Owners

When the runtime cannot satisfy a requirement directly, the owner is explicit:

- `query_runtime`
- `lower_runtime`
- `persistent_store`
- `domain_registration`
- `async_materializer`

Required capability owners are support evidence, not hints. A domain operation
that needs registration must register capability; a persistent index requirement
must be backed by a store support row; an async materialization posture must
cross the materialization API instead of pretending the read completed inline.

## Receipts And Counters

The read receipt is the execution proof. It should expose:

- the admitted graph access plan digest
- the access admission digest
- the access requirement-set digest
- the plan-consumption digest
- `graph_read_access_plan_consumption`
- `ephemeral_graph_index_receipt`
- `graph_read_streaming_receipt`
- `live_graph_read_access`
- `graph_read_access_summary`
- execution counters
- materialized row count
- per-result neighbor lookup count
- edge-scan count
- strategy recompute count

The important no-N+1 claim is not "the helper is efficient." The claim is:

```rust
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

If the receipt cannot prove the access plan was consumed, the execution is not
evidence for this lane.

## Broad Reads

Broad graph reads are legitimate only when their posture is honest.

Allowed outcomes:

- admitted inline indexed execution
- bounded ephemeral indexing
- admitted paged streaming
- required persistent index
- required async materialization
- typed denial

Forbidden outcomes:

- local relation-row loops after access denial
- per-node neighbor lookups hidden in helper fronts
- surface-local graph caches that bypass Query support rows
- broad boolean scans that materialize all graph state in ordinary runtime RAM
- retry guidance that says only to increase a limit

If a read needs a materialization job, use the materialization request and job
surfaces. If it needs persistent index support, admit that support explicitly.
If it denies, preserve the denial evidence.

## Relationship To Read Composition

[Read Composition](./read-composition.md) is still the authoring guide for
`compose_read(...)`, `define_read_family(...)`, and reusable read families.
This document owns the graph access accountability layer below those friendly
surfaces.

Use both together:

- read composition answers "how do I declare this read?"
- graph read access planning answers "what structures, budgets, and receipts
  prove this graph read is safe to execute?"
