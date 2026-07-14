# Graph Composition Authoring

## What This Feature Is

`workspace.compose_graph(...)` is Worth Query's first-class graph-shaped
mutation surface for one same-batch authoring program.

Use it when one logical change needs more than an ordered list of writes. The
runtime keeps the symbolic handles, resolution map, lifecycle meaning, and
denied-path diagnostics as explicit artifacts instead of making you reconstruct
graph intent from generic batch rows afterward.

This is not a nicer spelling of command-batch submission. It is a separate
public authoring family with its own receipts, inspection surfaces, support
rows, and typed denials.

Graph composition authors graph-shaped work. Graph Touch Obligation Authority
governs the semantic obligations created by graph-shaped work. When the
composition touch carries covered graph meaning, Query should select,
dispatch, budget, execute, and record registered graph obligations through the
authority path instead of making the caller remember validator callbacks.

## Why You Use It

- you need to declare entities or relations and reference them later in the
  same batch through symbolic handles
- you need to mix created targets and existing authoritative targets in one
  canonical program
- you need relation rewrites that preserve identity, retarget, or lineage
  explicitly instead of hiding behind plain update semantics
- you need backend-verified existing-truth checks inside the same authoring
  program
- you need denied-path diagnostics that say whether Query rejected the program,
  the registered graph obligation path rejected the touch, or a compatibility
  domain invariant pack rejected the topology

## Stable Entry Points

Stable public entry points:

- `workspace.compose_graph(...)`
- `workspace.compose_graph_with_invariant_pack(...)`

Stable admitted operations inside the composition closure today:

- `insert_entity(...)`
- `insert_relation(...)`
- `insert_symbolic_relation(...)`
- `update_entity(...)`
- `update_relation(...)`
- `delete_relation(...)`
- `update_existing(...)`
- `retarget_existing(...)`
- `supersede_existing(...)`
- `delete_existing(...)`
- `update_existing_verified(...)`
- `retarget_existing_verified(...)`
- `supersede_existing_verified(...)`
- `delete_existing_verified(...)`

Good to know:

- there is no preview-specific graph composition surface today
- broader graph workflows are not admitted just because a nearby lifecycle lane
  exists
- explicit command-batch submission remains the better fit when you do not need
  symbolic handles or graph-specific proof artifacts

## Core Mental Model

Graph composition is a typed program, not a bag of writes.

You author one composition-local program. The runtime retains:

- the canonical lowered step order
- which symbols were declared
- which component resolved which symbolic reference
- breadth counters for symbolic entities, symbolic relations, and total
  components
- lifecycle outcomes such as created, retargeted-identity-preserved, retired,
  and superseded-with-lineage
- assumption and read-set summaries when verified existing-truth lanes are part
  of the program
- lineage summaries when continuity-carrying retarget or supersession lanes are
  part of the program

Authority boundaries still matter:

- lower runtimes remain authoritative for truth identity, verification, naming,
  and continuity semantics
- Query owns the public authoring vocabulary, support posture, receipts,
  inspection, and denied-path evidence
- registered graph obligations are the primary covered path for graph touch
  legality, selector coverage, support rows, and executor verdict evidence
- manual invariant packs are compatibility/custom extension surfaces, not the primary covered graph obligation path
- domain invariants remain domain-owned; Query keeps "supported substrate",
  registered obligation denial, and compatibility topology rejection distinct

## How It Executes

1. Open a graph composition through `workspace.compose_graph(...)` or
   `workspace.compose_graph_with_invariant_pack(...)`.
2. Declare symbolic entities or relations in the order the subgraph needs them.
3. Add follow-up mutation against symbolic or existing authoritative targets
   using the admitted lanes.
4. Receive one canonical batch receipt.
5. Read graph-specific evidence from the receipt or `workspace.inspect(...)`.

Real graph-composition receipts can expose:

- `graph_composition_program()`
- `graph_composition_resolution_map()`
- `graph_composition_breadth()`
- `graph_composition_lifecycle_outcomes()`
- `graph_composition_evidence()`
- `graph_composition_assumption_summary()` when verified lanes participate
- `graph_composition_lineage_summary()` when retarget or supersession carries
  continuity meaning

Denied paths do not produce those receipt surfaces. Instead they expose:

- typed `GraphCompositionDenied(...)`
- `denial.admission_trace()`
- `denial.failure_stage()`

When a registered graph obligation applies, Query derives it from the graph
touch descriptor, operating world descriptor, and obligation index. The runtime
then executes the selected obligation through a dispatch envelope and preserves
the executor verdict in receipts, traces, support rows, or diagnostics.

When a compatibility domain invariant pack rejects an otherwise supported
program, the runtime uses:

- `WorthQueryRuntimeError::GraphCompositionDomainInvariantDenied(...)`
- `denial.domain_invariant_summary()`

That keeps Query-owned support denial distinct from registered graph obligation
denial and compatibility domain-invalid topology.

## Small Example

```rust
let receipt = workspace
    .compose_graph(|graph| {
        let task = graph.insert_entity("draft-task", "Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Draft task")
        })?;

        let relation = graph.insert_symbolic_relation("draft-edge", "TaskEdge", |edge| {
            edge.aspect("edge.kind", "depends_on")
                .symbolic_entity_identity("edge.source_identity", &task)
                .existing_entity_identity("edge.target_identity", "task-existing")
        })?;

        graph.update_entity(&task, |task| task.aspect("title.value", "Published task"))?;
        graph.delete_relation(&relation, |delete| {
            delete.touches(["edge.kind", "edge.source_identity", "edge.target_identity"])
        })?;
        Ok(())
    })
    .unwrap();

let program = receipt.graph_composition_program().unwrap();
let lifecycle = receipt.graph_composition_lifecycle_outcomes().unwrap();
let evidence = receipt.graph_composition_evidence().unwrap();

assert_eq!(program.component_count(), 4);
assert_eq!(lifecycle.entries().len(), 4);
assert_eq!(evidence.symbolic_resolution_count(), 3);
```

Why this is the smallest honest example:

- it declares a symbolic entity
- it declares a relation that points at that entity
- it performs same-batch follow-up mutation
- it retires a symbolic relation
- it reads graph-specific receipt evidence afterward

That is enough to show why this is not ordinary `batch(...)`.

## Real Example

```rust
let receipt = workspace
    .compose_graph(|graph| {
        let successor = graph.insert_entity("draft-half-edge", "HalfEdge", |half_edge| {
            half_edge
                .aspect("identity.id", "he-3")
                .aspect("kind.value", "half_edge")
        })?;

        graph.retarget_existing_verified(
            existing_loop_next_binding,
            |verify| {
                verify
                    .aspect("source.id", "he-1")
                    .aspect("target.id", "he-2")
            },
            |update| {
                update
                    .aspect("source.id", "he-1")
                    .continuity_rebind_existing_target(
                        "authority:loop-next-rel",
                        "authority:loop-next-rel-successor",
                    )
                    .symbolic_entity_identity("target.id", successor.reference().clone())
            },
        )?;

        Ok(())
    })
    .unwrap();

let program = receipt.graph_composition_program().unwrap();
let lifecycle = receipt.graph_composition_lifecycle_outcomes().unwrap();
let assumptions = receipt.graph_composition_assumption_summary().unwrap();
let resolution_map = receipt.graph_composition_resolution_map();

assert_eq!(program.component_count(), 2);
assert_eq!(
    program.steps()[1].kind(),
    WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget
);
assert_eq!(
    lifecycle.entries()[1].outcome_kind(),
    WorthQueryGraphCompositionLifecycleOutcomeKind::RetargetedIdentityPreserved
);
assert_eq!(assumptions.verified_step_count(), 1);
assert_eq!(resolution_map.entries()[0].aspect_path(), Some("target.id"));
```

What is authoritative here:

- the existing relation binding
- the backend-verified precondition on `source.id` and `target.id`
- the continuity rebind request

What is derived here:

- the symbolic successor identity resolution
- the composition-level assumption summary
- the lifecycle classification

What gets retained automatically:

- the verified precondition summary
- the symbolic resolution evidence
- the exact lifecycle outcome
- the component ordering

This is the right surface for geometry-style rewires where the runtime must
preserve both verification meaning and continuity meaning in one receipt story.

If that same shape is structurally valid but domain-invalid, use
`workspace.compose_graph_with_invariant_pack(...)` so the domain can reject it
through the distinct invariant-pack lane instead of forcing callers to flatten
the rejection into generic graph denial.

## Custom Hooks And Domain Extensions

Graph composition is extensible, but only at a few explicit boundaries.

The runtime does **not** support arbitrary middleware hooks, callback-based
execution injection, or a second domain-owned graph engine layered beside the
public Query contract.

The allowed hook classes today are:

- `domain_lowering_hook`
  - use this when your domain wants to lower its own higher-level intent into
    the generic graph-composition program
- `domain_invariant_pack_hook`
  - use this when the runtime supports the graph shape, but your domain still
    needs to reject it as invalid topology or invalid local semantics
- `domain_interpretation_hook`
  - use this when your domain wants to interpret canonical Query receipts and
    inspection into domain diagnostics, certification output, or UX-facing
    summaries

The mental model is:

- Query owns graph authoring, symbolic resolution, lifecycle meaning, support
  posture, receipts, inspection, and denied-path artifact families
- hooks may add **domain meaning**
- hooks may **not** replace generic runtime truth

Use a lowering hook when:

- the domain starts from a richer edit vocabulary than `insert_*`,
  `update_*`, `retarget_existing(...)`, or `supersede_existing(...)`
- the right end-state is still one canonical Query graph-composition program

Use an invariant-pack hook when:

- the graph program is structurally supported by Query
- the domain still needs to reject it as invalid
- you want the denial to remain
  `GraphCompositionDomainInvariantDenied(...)`
  instead of collapsing into generic graph-composition support denial

Use an interpretation hook when:

- the runtime receipt is already sufficient as truth
- the domain wants a richer explanation layer on top of that retained evidence

Do **not** use custom hooks for:

- alternate target identity semantics
- alternate relation-update semantics that disguise replacement as update
- alternate symbolic resolution rules
- private support widening that teaches a domain workflow as "stable" when the
  public support rows still deny it
- callback-style execution bypass that skips the canonical program and lowering
  model

If you think you need one of those, the missing capability belongs in Query
first. The hook should consume widened runtime substrate after it exists, not
smuggle it in privately.

Before teaching a custom extension as ordinary behavior, read the public
support surface:

- `graph_composition_capability_support_rows()`
- `graph_composition_extension_hook_support_rows()`

Those rows are the machine-readable answer to:

- what graph lifecycle or target-combination classes are admitted
- which extension-hook boundaries are allowed

## How It Relates To Other Features

- Use [Writes and Intent Boundaries](../execution/writes-and-intents.md) when the workflow
  is ordinary direct mutation and does not need graph-shaped symbolic meaning.
- Use explicit command-batch submission when ordering is enough and you do not
  need symbolic handles, graph lifecycle surfaces, or graph-specific denial.
- Use `update_existing(...)` when you need an admitted existing-target update
  and no backend verification is required first.
- Use `retarget_existing(...)` when the runtime should preserve one continuing
  identity under explicit rebind semantics.
- Use `supersede_existing(...)` when the runtime should preserve lineage under
  split or merge semantics.
- Use the verified variants when the backend must prove current authoritative
  truth immediately before the existing-target mutation.
- Use registered graph obligations when the program shape should select covered
  legality, sequencing, capability, context, schema-contract, or advisory
  checks from the touched graph shape.
- Use `compose_graph_with_invariant_pack(...)` only when the program shape may
  be substrate-valid but still needs compatibility/custom domain-owned
  invariant rejection outside the covered graph obligation path.

## Inspection And Debugging

Start here when a composition succeeds:

- `graph_composition_program()`
  - shows the canonical lowered step kinds and component ordering
- `graph_composition_resolution_map()`
  - shows which symbolic reference resolved where
- `graph_composition_lifecycle_outcomes()`
  - shows whether a component created, updated identity, retargeted identity,
    retired truth, or superseded with lineage
- `graph_composition_assumption_summary()`
  - shows aggregate verified precondition and read-set breadth when verified
    lanes participate
- `graph_composition_lineage_summary()`
  - shows prior and successor authoritative identities when continuity-carrying
    lanes participate

Start here when a composition denies:

- `denial.kind()`
  - tells you what family failed
- `denial.failure_stage()`
  - tells you where admission stopped
- `denial.admission_trace()`
  - shows the ordered denied-path stages

Start here when a registered graph obligation denies:

- graph obligation dispatch envelope evidence
- graph obligation executor verdict evidence
- graph obligation support row evidence
- graph obligation budget evidence such as `BudgetExceeded`, state-load
  counters, cost classes, and artifact-policy-gated diagnostics

Start here when a compatibility domain invariant pack rejects a supported
program:

- `WorthQueryRuntimeError::GraphCompositionDomainInvariantDenied(...)`
- `denial.domain_invariant_summary().declared_collections()`
- `denial.domain_invariant_summary().declared_symbols()`
- `denial.domain_invariant_summary().target_combination_families()`
- `denial.domain_invariant_summary().lifecycle_families()`

That summary is the shortest honest answer to "what supported graph shape did
the domain reject?"

## Anti-Patterns

- teaching graph authoring as "command-batch submission plus careful strings"
- treating symbolic handles as stable cross-composition identities
- using plain `update_existing(...)` when the real contract is retarget or
  supersession
- assuming a nearby admitted lifecycle lane implies broader graph workflow
  support
- collapsing domain-invalid topology into `GraphCompositionDenied(...)`
- reconstructing graph meaning from generic component rows when composition
  evidence already exists
- presenting manual invariant packs as the primary covered graph obligation
  path
- hiding budget overflow by completing local graph walks after `BudgetExceeded`

## Current Limits

- graph composition is stable only for the admitted lifecycle and capability
  rows exposed by the public support surface
- broader graph workflows still need explicit admission and hostile proof
- no preview-specific graph composition surface is admitted today
- temporal, async/resource, store-backed, and durable graph-authoring semantics
  remain outside this feature boundary
- Query does not own topology validity or domain-local invariant semantics
- graph touch obligation authority is bounded by admitted support rows, budget
  posture, and executor verdict evidence

## Related Docs

- [Graph Touch Obligation Authority](graph-touch-obligation-authority.md)
- [Graph Obligation Consumer Kit](graph-obligation-consumer-kit.md)
- [Writes and Intent Boundaries](../execution/writes-and-intents.md)
- [Existing-Truth Verified Updates](../capabilities/existing-truth.md)
- [Existing-Truth Verified Deletes](../capabilities/existing-truth.md)
- [Inspection](../capabilities/inspection.md)
- [Support Matrix and Admission](../foundations/support-matrix-and-admission.md)


