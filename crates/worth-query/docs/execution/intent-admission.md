# Intent Admission

## What This Feature Is

Intent admission is the shared front door for the covered Query
families that need more than "just call the runtime and hope."

It gives those families one visible progression:

1. author a raw intent request
2. review the request and its eligibility facts
3. get an admitted, advisory, or violation decision
4. execute only through a sealed handoff when the family really crosses a
   runtime seam
5. receive a receipt, scoped artifact, bound contract, or typed denial that
   still points back to the same decision trace

This is the feature that keeps covered intent work honest. Callers do not have
to rediscover support posture, basis posture, routing posture, or mismatch
reasons from lower-runtime artifacts after the fact.

## Why You Use It

- you want one public way to ask whether a covered Query intent can proceed
- you want execution receipts to preserve a proof chain back to the admitted
  handoff
- you want basis observation and projection consumption to use the same
  admitted vocabulary as runtime-backed execution families
- you want the advanced path when framework or tooling code needs to inspect
  request, eligibility, decision, handoff, binding, and final receipt evidence

## Stable Entry Points

Common path:

- `runtime.intent(declaration).execute()`
- `runtime.intent(declaration).review()?.admit()?.execute()`
- `runtime.next_effect_write_intent(&effect, version, contract).execute()`
- `runtime.next_effect_write_intent(&effect, version, contract).review()?.admit()?.execute()`
- `runtime.write_intent(command).execute()`
- `runtime.write_intent(command).review()?.admit()?.execute()`
- `workspace.write_intent(command).execute()`
- `workspace.write_intent(command).review()?.admit()?.execute()`
- `runtime.write_batch_intent(commands).execute()`
- `runtime.write_batch_intent(commands).review()?.admit()?.execute()`
- `workspace.write_batch_intent(commands).execute()`
- `workspace.write_batch_intent(commands).review()?.admit()?.execute()`
- `workspace.read_family_intent(&family).execute()`
- `workspace.read_family_intent(&family).review()?.admit()?.execute()`
- `workspace.read_family_in_basis_context_intent(&family, &context).execute()`
- `workspace.read_family_in_basis_context_intent(&family, &context).review()?.admit()?.execute()`
- `workspace.read_live_intent(&view).execute()`
- `workspace.read_live_intent(&view).review()?.admit()?.execute()`
- `workspace.materialize_intent(&view).execute()`
- `workspace.materialize_intent(&view).review()?.admit()?.execute()`
- `workspace.inspect_intent(target).execute()`
- `workspace.inspect_intent(target).review()?.admit()?.execute()`
- `workspace.inspect_derived_intent(&view).execute()`
- `workspace.inspect_derived_intent(&view).review()?.admit()?.execute()`
- `runtime.probe_existing_intent(request).execute()`
- `runtime.probe_existing_intent(request).review()?.admit()?.execute()`
- `workspace.probe_existing_intent(request).execute()`
- `workspace.probe_existing_intent(request).review()?.admit()?.execute()`
- `worth_query_basis_observation_intent(RawBasisIntent::CurrentHead)?.admit()?.scope()`

Advanced path:

- `.review()?`
- `.admit()?`
- `.handoff()`
- `.execution_binding()`
- `.execute()?`

Existing-truth work covered by the same lattice:

- graph-composition existing-target update, retarget, supersession, and retirement lanes
- graph-composition verified-existing lanes
- `workspace.read(&view)`
- `workspace.materialize_result(&view)?`
- `workspace.inspect(&view)`
- `runtime.probe_existing(...)`
- `workspace.probe_existing_intent(request).execute()`

Good to know:

- coverage is concrete, not blanket. These named families are covered now.
- Generic materialization neighbors, temporal families, async/resource
  families, store-backed restart work, and other later neighbors remain explicitly deferred.
- unsupported or future families are still support-gated even when nearby
  covered families already have ordinary public entry points.

## Core Mental Model

Think of intent admission as a published contract that separates:

- naming work from executing work
- support posture from success posture
- admitted runtime execution families from admitted non-runtime families

Some covered families end in a runtime handoff:

- authoritative mutation intent execution
- batch mutation intent execution
- read-family execution
- basis-context read-family execution
- live read execution
- derived materialization
- generic and derived inspection
- existing-truth probe routing
- effect-triggered pending write intent execution

Graph touch obligation authority is covered by the same admission posture when
graph-shaped mutation or access needs obligation selection. Intent admission
must preserve the touch descriptor, operating world descriptor, selected
obligations, dispatch plan, executor verdict, and budget outcome instead of
flattening the work into a local validator callback.

For that effect-triggered family, the admitted review, binding, and final
effect-intent receipt can preserve one typed write-adjacent trigger class such
as ordinary effect follow-on, time-only wake, async completion, mixed-cause,
replay drift, remask drift, stale completion, or preview-crossed residue.
That trigger posture stays on the same admitted lane. It is not a second local
callback pipeline.

Some covered families are still admitted, but do not cross a runtime execution
seam:

- basis observation
- projection consumption

That difference is intentional. An admitted family with
`no-execution-handoff` is not less real. It just terminates in a scoped basis
artifact or a bound projection contract instead of route/evaluate execution.

## How It Executes

The common path is:

1. author the intent from a covered public entry point
2. let Query resolve eligibility and decision posture
3. if admitted, execute only through the sealed handoff for that family
4. receive a final receipt or artifact that still exposes the same trace and
   provenance chain

The advanced path keeps those phases visible:

1. `review()?`
2. inspect `request()`, `eligibility()`, `decision()`, and
   `decision_trace_envelope()`
3. if admitted, call `admit()?`
4. inspect `handoff()` and `execution_binding()` when the family crosses a
   runtime seam
5. execute or terminate through the family-owned admitted artifact

What "execute" means depends on the family:

- runtime-backed families return a receipt or result with trace and provenance
- basis observation returns a scoped basis artifact
- projection consumption returns a bound contract, then typed extracted facts

## Small Example

Use the common path when product code wants admitted execution and one
canonical receipt:

```rust
let receipt = runtime
    .intent(declaration)
    .execute()?;

let covered_entrypoint = receipt.covered_entrypoint_label();
let trace = receipt.decision_trace_envelope();
let provenance = receipt.execution_provenance();
```

That is the normal shape for a covered runtime-backed family: author intent,
execute, then inspect the receipt if you care about the proof chain.

## Real Example

Use the advanced path when framework or tooling code needs the proof chain
itself:

```rust
let review = workspace.read_live_intent(&view).review()?;

let request = review.request();
let eligibility = review.eligibility();
let decision = review.decision();
let trace = review.decision_trace_envelope();

let admitted = review.admit()?;
let handoff = admitted.handoff();
let binding = admitted.execution_binding();
let result = admitted.execute()?;

let receipt = result.receipt();
let provenance = receipt.execution_provenance();
```

The same advanced execute shape is also valid on the authoritative runtime
floor:

```rust
let admitted = runtime.intent(declaration).review()?.admit()?;
```

The same review/admit rhythm also covers the admitted families that do not
cross a runtime execution seam:

```rust
let scoped_basis = worth_query_basis_observation_intent(
    RawBasisIntent::CurrentHead,
)?
.admit()?
.scope();

```

Projection authority does not use this intent-admission rhythm. Its only
public route is a declared `ProjectionAuthorityContract` consumed directly by
the result or receipt that owns the projection facts.

Covered family examples:

```rust
let review = runtime.intent(declaration).review()?;

let request = review.request();
```

```rust
let consumer = receipt.consumer_inspection();

let outcome = consumer.outcome_class();
```

```rust
let scoped_basis = worth_query_basis_observation_intent(
    RawBasisIntent::CurrentHead,
)?
.admit()?
.scope();
```

```rust
let contract = ProjectionAuthorityContract::declare()
    .require_entity_identities()
    .build();

let authority = read_result
    .consume_projection_authority(&shape, &authorized_projection, contract)?
    .into_admitted()
    .map_err(|outcome| handle_projection_denial(outcome))?
    .0;
```

```rust
let read_result = workspace.read_family_intent(&family).execute()?;
```

```rust
let basis_result = workspace
    .read_family_in_basis_context_intent(&family, &context)
    .execute()?;
```

```rust
let write_receipt = runtime.write_intent(command).execute()?;
```

```rust
let batch_receipt = runtime.write_batch_intent(commands).execute()?;
```

```rust
let batch_review = runtime.write_batch_intent(commands).review()?;
```

```rust
let live_result = workspace.read_live_intent(&view).execute()?;
```

```rust
let live_rows = workspace.read(&view);
```

```rust
let inspection_result = workspace.inspect_intent(&view).execute()?;
```

```rust
let materialization = workspace.materialize_result(&view)?;
```

```rust
let inspection = workspace.inspect(&view)?;
```

```rust
let probe_result = runtime.probe_existing_intent(request).execute()?;
```

```rust
let probe = runtime.probe_existing(request.clone())?;
```

```rust
let basis_review = worth_query_basis_observation_intent(RawBasisIntent::CurrentHead)?
    .review()?;
```

```rust
let write_review = runtime.write_intent(command).review()?;
```

```rust
let live_review = workspace.read_live_intent(&view).review()?;
```

```rust
let inspection_review = workspace.inspect_intent(&view).review()?;
```

```rust
let probe_review = runtime.probe_existing_intent(request).review()?;
```

## How It Relates To Other Features

- Use [Writes And Intent Boundaries](writes-and-intents.md) when you are
  deciding whether a direct mutation path or an intent path is the right author
  surface.
- Use [Existing Truth](../capabilities/existing-truth.md) when the intent work
  is specifically about verified existing-target mutation or probing.
- Use [Inspection](../capabilities/inspection.md) when you need the retained
  explanation surface over receipts, denials, preview artifacts, or branch
  artifacts.
- Use [Projection Consumption](../capabilities/projection-consumption.md) when
  the terminal admitted artifact is a bound contract and typed fact extraction,
  not runtime execution.
- Use [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
  when you need to explain whether a family is fully supported, deferred, or
  still support-gated before teaching it as everyday production DX.

## Inspection And Debugging

Every covered receipt or denial/failure evidence carries a
`WorthQueryIntentDecisionTraceEnvelope`.

The most useful public inspection surfaces are:

- `decision_trace_envelope()`
- `consumer_inspection()`
- `execution_provenance()`
- `execution_provenance_chain_digest()`
- `covered_entrypoint_label()`
- `outcome_class()`
- `terminal_stage_label()`

The consumer lane is the right fallback when downstream code wants shared
admission vocabulary and does not want to branch on family-specific receipt
types.

## Anti-Patterns

- teaching every method with "intent" in its name as equally stable everyday
  DX without checking support posture
- treating the convenience wrappers as independent execution systems when they
  are really thin delegates over the same lattice
- reconstructing support, mismatch, or routing posture from lower-runtime
  details instead of reading the public decision trace
- treating basis observation or projection consumption as second-class because
  they terminate without a runtime handoff

## Current Limits

- the covered families listed above are real now, but they are not the whole
  future Query surface
- generic materialization neighbors, temporal families, async/resource
  families, durable restart work, and store-backed replay remain explicitly deferred
- support posture still matters. Method presence is not a promise that every
  runtime profile admits every intent family

## Related Docs

- [Writes And Intent Boundaries](writes-and-intents.md)
- [Graph Touch Obligation Authority](../authoring/graph-touch-obligation-authority.md)
- [Graph Obligation Consumer Kit](../authoring/graph-obligation-consumer-kit.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
- [Existing Truth](../capabilities/existing-truth.md)
- [Inspection](../capabilities/inspection.md)
- [Projection Consumption](../capabilities/projection-consumption.md)
- [Workspace Overview](../foundations/workspace-overview.md)
