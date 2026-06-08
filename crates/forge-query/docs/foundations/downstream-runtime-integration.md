# Downstream Runtime Integration

## What This Feature Is

This page is the explicit contract for downstream runtimes and domain crates
that build on `forge-query` as their ordinary runtime surface.

If you are building something like a topology runtime, geometry kernel,
workflow engine, editor domain, or analysis runtime, this is the "how do I use
Query without reinventing Query?" guide.

## Why You Use It

- you want one concrete rule set for building on the stabilized runtime facade
- you need to know which Query surfaces are the ordinary entry points
- you want to avoid rebuilding local mutation, basis, inspection, or payload
  interpretation folklore above the runtime

## Core Rule

Ordinary downstream runtime code should enter through `ForgeQueryWorkspace`
and adjacent public crate-doc surfaces, not through lower-runtime plumbing.

The public contract is:

- declare retained runtime surfaces through the workspace
- mutate authoritative truth through the workspace mutation surfaces
- use graph composition when the authoring problem is graph-shaped
- use existing-truth surfaces when the target is already authoritative
- use preview and branch sessions for isolated work
- use inspection and projection consumption for explanations and typed facts
- use the support matrix and admission surfaces when a family may be deferred
  or unsupported

## Use These Surfaces

### Runtime front door

- `runtime.workspace(...)`
- `workspace.public_downstream_delivery_contract()`
- `workspace.live_view(...)`
- `workspace.computed(...)`
- `workspace.effect(...)`
- `workspace.preview(...)`
- `workspace.branch(...)`
- `workspace.read(...)`
- `workspace.observe(...)`
- `workspace.downstream_delivery(...)`
- `workspace.materialize(...)`
- `workspace.inspect(...)`

### Downstream delivery contract

- `workspace.public_downstream_delivery_contract()`
- `workspace.downstream_delivery(...)`

Use these when another runtime, server layer, or transport boundary needs one
Query-owned delivery envelope instead of reading raw live delivery batches and
guessing what they mean.

The contract tells downstream code:

- whether runtime-backed resume is admitted now
- that durable resume is still deferred debt
- which lower-runtime support posture governs each resume lane
- which support digest owns the current delivery/resume story

The projected delivery tells downstream code:

- whether the latest delivery is truth-patch, time-only, async-backed, or
  mixed-cause
- which basis digest the delivery is bound to
- whether remask posture makes the delivery supported, remasked, or denied
- whether a caller-supplied resume basis is admitted, missing, or stale

### Ordinary authoritative mutation

- `workspace.insert(...)`
- `workspace.update(...)`
- `workspace.delete(...)`
- `workspace.batch(...)`

Use these when the mutation is already fully known and does not need symbolic
same-batch graph authoring.

### Graph-shaped authoring

- `workspace.compose_graph(...)`
- `workspace.compose_graph_with_invariant_pack(...)`

Use these when one logical authoring step needs:

- symbolic same-batch handles
- follow-up mutation against just-created truth
- mixed existing and created targets
- graph-specific lifecycle evidence
- graph-specific denied-path diagnostics

Do not simulate this with caller-owned `workspace.batch(...)` choreography.

### Existing authoritative truth

- `workspace.bind_existing_entity(...)`
- `workspace.bind_existing_relation(...)`
- `workspace.update_existing(...)`
- `workspace.delete_existing(...)`
- `workspace.assert_existing(...)`
- `workspace.verify_existing(...)`
- `workspace.update_existing_verified(...)`
- `workspace.delete_existing_verified(...)`
- `workspace.probe_existing(...)`

Use these when the runtime must preserve or verify an authoritative target
binding instead of flattening everything into caller-owned identity strings.

### Covered intent families

- `runtime.intent(...)`
- `runtime.write_intent(...)`
- `runtime.write_batch_intent(...)`
- `runtime.next_effect_write_intent(...)`
- `workspace.read_live_intent(...)`
- `workspace.materialize_intent(...)`
- `workspace.inspect_intent(...)`
- `forge_query_basis_observation_intent(...)`
- `forge_query_projection_consumption_intent(...)`

Use the intent lattice only when the family genuinely belongs on the admitted
intent path. Do not treat every surface with `intent` in the name as the
default way to do ordinary work.

### Typed fact extraction

- `consume_projection_facts(...)`
- `declare_projection_fact_consumption(...)`
- `bind_contract()`

Use Projection Consumption when the caller needs typed identities, memberships,
targets, source references, provenance, or continuity facts from a read
result, write receipt, or query-context artifact.

Do not re-parse payload bags or reconstruct the same meaning in caller code.

## Do Not Build These Things Locally

Downstream runtimes should not recreate:

- a second mutation runtime above Query
- caller-owned graph authoring semantics where Query already has
  `compose_graph(...)`
- caller-owned authoritative target binding semantics where Query already has
  existing-truth surfaces
- custom branch or preview language when Query already owns basis and lane
  posture
- row or payload archaeology when Projection Consumption already exposes typed
  fact families
- lower-runtime debugging folklore when `workspace.inspect(...)` already owns
  the public explanation surface
- edge-side delivery folklore when Query already exposes
  `workspace.downstream_delivery(...)`
- "method exists, therefore supported" logic instead of using the support
  matrix and admission gate

## Support And Admission Rules

Method presence is not a support claim.

Before teaching a family as normal downstream runtime DX, use:

- `workspace.public_support_matrix()`
- `workspace.public_api_contract()`
- `workspace.public_handle_contract()`
- `workspace.public_mutation_surface_report()`
- `workspace.admit_public_api_family(...)`

This is especially important around:

- intent-shaped families
- temporal and async neighbors
- downstream delivery and resume posture
- preview or branch-local behavior
- backend-verified existing-truth lanes

In practice:

- teach rows with `ordinary_downstream_dx() == true` as normal runtime entry
  points
- treat `visible-but-deferred` rows as published future vocabulary that must
  still deny through admission today
- treat `visible-vocabulary-only` rows as public language, not blanket runtime
  support
- treat `support-gate-only` rows as support or certification boundaries, not
  ordinary runtime entry points

## Basis And Lane Rules

Downstream runtimes must reuse Query's basis and authority-lane semantics.

That means:

- current, branch, preview-derived, and historical posture are Query basis
  capabilities, not caller-defined snapshot folklore
- preview and branch work are lane shifts over retained surfaces, not separate
  local runtimes
- authoritative truth, branch-local truth, preview truth, derived runtime
  state, delivery state, and pending write intent remain Query-owned public
  lane vocabulary
- downstream delivery resume negotiation is still basis-bound; missing or stale
  basis must deny explicitly instead of silently becoming "best effort"

## Downstream Delivery

If you are publishing live updates to another process, this is the contract to
use instead of opening retained runtime batches directly.

Small example:

```rust
let workspace = runtime.workspace("server.push").unwrap();
let contract = workspace.public_downstream_delivery_contract();

assert!(contract.runtime_backed_resume_supported());
assert!(contract.durable_resume_deferred());
assert_eq!(
    contract.runtime_resume_support_posture().as_str(),
    "admitted"
);
assert_eq!(
    contract.durable_resume_support_posture().as_str(),
    "deferred"
);
```

Real example:

```rust
let delivery = workspace
    .downstream_delivery(&tasks)
    .unwrap()
    .expect("latest retained delivery should exist");

match delivery.delivery_class().as_str() {
    "time-only" | "async-backed" | "mixed-cause" | "truth-patch" => {}
    other => panic!("unexpected delivery class: {other}"),
}

let resume = delivery.negotiate_runtime_resume(Some(delivery.basis_digest()));
assert_eq!(resume.kind().as_str(), "runtime-backed-admitted");

let durable = delivery.durable_resume_posture();
assert_eq!(durable.kind().as_str(), "durable-deferred-debt");
```

Good to know:

- `workspace.downstream_delivery(...)` projects the latest retained live
  delivery, not the drained patch queue
- runtime-backed resume is supported now only when the basis digest matches
- durable replay/restart resume is still deferred debt and stays typed as debt
- remasked or denied live meaning stays explicit on the projected delivery

## Recommended Reading Order

If you are onboarding a downstream runtime to Query, read in this order:

1. [Workspace Overview](workspace-overview.md)
2. [Support Matrix And Admission](support-matrix-and-admission.md)
3. [Branches and Previews](branches-and-previews.md)
4. [Aspects And Authority Lanes](../modeling/aspects-and-authority-lanes.md)
5. [Writes and Intent Boundaries](../execution/writes-and-intents.md)
6. [Graph Composition Authoring](../authoring/graph-composition-authoring.md)
7. [Existing Truth](../capabilities/existing-truth.md)
8. [Reads, Observation, and Materialization](../runtime-surfaces/reads-observe-materialize.md)
9. [Inspection](../capabilities/inspection.md)
10. [Projection Consumption](../capabilities/projection-consumption.md)
11. [Historical Basis, Diff, And Comparison Queries](../capabilities/historical-diff-and-basis.md)

## Anti-Patterns

- treating Query like a thin transport layer instead of the actual public
  runtime contract
- using lower-runtime seams because they feel "more direct"
- rebuilding target identity, branch posture, or inspection logic in the
  downstream crate
- making rows the permanent downstream contract when Query already exposes a
  stronger typed fact or receipt surface

## Related Docs

- [Workspace Overview](workspace-overview.md)
- [Support Matrix And Admission](support-matrix-and-admission.md)
- [Writes and Intent Boundaries](../execution/writes-and-intents.md)
- [Graph Composition Authoring](../authoring/graph-composition-authoring.md)
- [Existing Truth](../capabilities/existing-truth.md)
- [Workspace Overview](workspace-overview.md)
- [Inspection](../capabilities/inspection.md)
- [Projection Consumption](../capabilities/projection-consumption.md)
