# Intent Admission

## What This Feature Is

Intent admission is the proof-bearing front door for covered Query intent
admission and execution. It turns a raw intent request into one explicit
progression:

1. raw request authoring
2. eligibility resolution
3. admitted, advisory, or violation decision
4. typed execution handoff for covered seams, or an explicit
   `no-execution-handoff` admitted plan for non-runtime families
5. receipt, scoped artifact, bound contract, or typed denial/failure evidence
   with a decision trace envelope

The current covered entrypoints are:

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
- `forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)?.admit()?.scope()`
- `forge_query_projection_consumption_intent(declaration)?.admit()?.bind_contract()`

Generic materialization neighbors and other future-neighbor intent families
remain explicitly deferred in the support matrix rather than being quietly
treated as partial support.

## Why You Use It

- you want the runtime to reject unsupported or mismatched intent work before
  execution
- you want execution receipts to retain one canonical provenance chain back to
  the admitted handoff
- you want basis observation and projection consumption to enter through the
  same lattice instead of family-local preflight booleans
- you want a public advanced path that can inspect request, eligibility,
  decision, handoff, binding, and final receipt evidence without spelunking
  internal modules

## Common Path

Use the common path when product code wants admitted execution and a canonical
receipt:

```rust
let receipt = runtime
    .intent(declaration)
    .execute()?;

let covered_entrypoint = receipt.covered_entrypoint_label();
let trace = receipt.decision_trace_envelope();
let provenance = receipt.execution_provenance();
```

That path reads as intent first, execution second. The receipt carries the
decision trace envelope, execution handoff digest, execution binding digest,
and execution provenance chain digest directly.

Effect-triggered pending write intent uses the same shape:

```rust
let receipt = runtime
    .next_effect_write_intent(&effect, "1.0", "effect.intent.input.v1")
    .execute()?;

let effect_name = receipt.effect_name();
let intent_trace = receipt.decision_trace_envelope();
let intent_provenance = receipt.execution_provenance();
```

Basis observation is now a covered admitted family even though it does not
cross a Query execution seam:

```rust
let scoped_basis = forge_query_basis_observation_intent(
    RawBasisIntent::CurrentHead,
)?
.admit()?
.scope();

let basis_digest = scoped_basis.scoped_basis_digest();
```

Projection consumption is also covered as a no-execution-handoff admitted
family:

```rust
let contract = forge_query_projection_consumption_intent(declaration)?
    .admit()?
    .bind_contract();

let contract_digest = contract.contract_digest();
```

Authoritative mutation now uses the same lattice for both scalar and batch
surfaces:

```rust
let write_receipt = runtime.write_intent(command).execute()?;
let batch_receipt = runtime.write_batch_intent(commands).execute()?;

let workspace_write_receipt = workspace.write_intent(command).execute()?;
let workspace_batch_receipt = workspace.write_batch_intent(commands).execute()?;

let write_trace = write_receipt.decision_trace_envelope();
let batch_trace = batch_receipt.decision_trace_envelope();
```

That includes verified-existing convenience surfaces too. `workspace.verify_existing(...)`,
`workspace.update_existing_verified(...)`, and `workspace.delete_existing_verified(...)`
remain thin convenience wrappers over authoritative mutation intent execution rather
than parallel bridge-backed routing families.

Read-family execution now uses the same lattice for both runtime-current and
admitted basis-context execution. In other words, read-family execution now uses the same lattice
as the covered mutation and effect paths:

```rust
let read_result = workspace.read_family_intent(&family).execute()?;
let basis_result = workspace
    .read_family_in_basis_context_intent(&family, &context)
    .execute()?;

let read_trace = read_result.receipt().decision_trace_envelope();
let basis_trace = basis_result.receipt().decision_trace_envelope();
```

Retained live-view reads now use the same lattice too. `workspace.read(&view)` is
a thin wrapper over the explicit live-read intent path:

```rust
let live_rows = workspace.read(&view);
let live_result = workspace.read_live_intent(&view).execute()?;

let live_trace = live_result.receipt().decision_trace_envelope();
let live_provenance = live_result.receipt().execution_provenance();
```

Derived-view materialization and inspection now use the same lattice as well.
`workspace.materialize(&view)` and `workspace.inspect(&view)` are thin wrappers
over the explicit intent paths for derived views:

```rust
let materialized_rows = workspace.materialize(&view);
let materialization = workspace.materialize_intent(&view).execute()?;
let inspection = workspace.inspect_derived_intent(&view).execute()?;

let materialization_trace = materialization.receipt().decision_trace_envelope();
let inspection_trace = inspection.receipt().decision_trace_envelope();
```

Non-derived inspection now uses the same lattice too. `workspace.inspect(&target)`
and `runtime.inspect(&target)` are thin wrappers over the explicit generic
inspection path for covered live, effect, receipt, denial, preview, and branch
inspection subjects:

```rust
let inspection = workspace.inspect(&view)?;
let inspection_result = workspace.inspect_intent(&view).execute()?;

let inspection_trace = inspection_result.receipt().decision_trace_envelope();
let inspection_provenance = inspection_result.receipt().execution_provenance();
```

Bridge-backed existing-truth probes now use the same lattice too.
`runtime.probe_existing(request)` and `workspace.probe_existing(binding, paths)`
are thin wrappers over the explicit routing intent path:

```rust
let probe = runtime.probe_existing(request.clone())?;
let probe_result = runtime.probe_existing_intent(request).execute()?;

let workspace_probe = workspace.probe_existing(binding.clone(), ["identity.id"])?;
let workspace_probe_result = workspace
    .probe_existing_intent(ForgeQueryExistingTruthProbeRequest::new(
        binding,
        ["identity.id"],
    )?)
    .execute()?;

let probe_trace = probe_result.receipt().decision_trace_envelope();
let probe_provenance = probe_result.receipt().execution_provenance();
```

## Advanced Path

Use the advanced path when you need the proof chain explicitly:

```rust
let review = runtime.intent(declaration).review()?;

let request = review.request();
let eligibility = review.eligibility();
let decision = review.decision();
let admitted_plan = review.admitted_plan();
let handoff = review.admitted_handoff();
let non_admitted_trace = review.decision_trace_envelope();
```

If the review admits, you can materialize the sealed handoff and execution
binding before execution:

```rust
let admitted = runtime.intent(declaration).review()?.admit()?;

let handoff = admitted.handoff();
let binding = admitted.execution_binding();
let receipt = admitted.execute()?;
```

The advanced path is the honest way to inspect:

- request family and covered entrypoint
- structured eligibility posture
- admitted versus advisory versus violation decision
- sealed execution handoff for the covered seam
- final receipt provenance and trace once execution completes

Family-specific advanced paths follow the same review, admit, and execute
rhythm:

```rust
let write_review = runtime.write_intent(command).review()?;
let write_handoff = write_review.admitted_handoff();
let write_receipt = write_review.admit()?.execute()?;
```

```rust
let live_review = workspace.read_live_intent(&view).review()?;
let live_handoff = live_review.admitted_handoff();
let live_result = live_review.admit()?.execute()?;
```

```rust
let inspection_review = workspace.inspect_intent(&view).review()?;
let inspection_handoff = inspection_review.admitted_handoff();
let inspection_result = inspection_review.admit()?.execute()?;
```

```rust
let probe_review = runtime.probe_existing_intent(request).review()?;
let probe_handoff = probe_review.admitted_handoff();
let probe_result = probe_review.admit()?.execute()?;
```

```rust
let basis_review = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)?
    .review()?;
let basis_plan = basis_review.admitted_plan();
let scoped_basis = basis_review.admit()?.scope();
```

```rust
let projection_review =
    forge_query_projection_consumption_intent(declaration)?.review()?;
let projection_plan = projection_review.admitted_plan();
let contract = projection_review.admit()?.bind_contract();
```

## Consumer Lane

Use the consumer lane when downstream code wants only the shared admission
surface and does not want to branch on family-specific receipt or denial
types. Reach it through `consumer_inspection()`:

```rust
let receipt = runtime.intent(declaration).execute()?;
let consumer = receipt.consumer_inspection();

let outcome = consumer.outcome_class();
let trace_digest = consumer.decision_trace_digest();
let terminal_stage = consumer.terminal_stage_label();
let provenance = consumer.execution_provenance_chain_digest();
```

The same consumer shape is available on review, receipt, denial, and
execution-failure evidence. It stays on the shared lattice vocabulary:

- outcome class
- canonical decision trace envelope and digest
- family and covered entrypoint when a trace exists
- terminal stage, cause, and detail
- execution provenance chain when execution had already begun

## Decision Trace Envelope

Every covered receipt or denial/failure evidence carries a
`ForgeQueryIntentDecisionTraceEnvelope`.

Each row exposes:

- `stage()`
- `cause()`
- `detail()`
- `evidence_owner()`
- `evidence()`
- `artifact_digest()`

The eligibility row now carries structured posture rather than only one opaque
digest. Offline consumers can inspect support, capability, policy, basis,
invariant, projection/source, routing-support, source-lane, and authority-lane
posture from the public envelope.

## Outcome Classes

Admitted:

- execution can proceed only through a sealed handoff and binding
- receipts preserve decision trace and execution provenance

Advisory:

- the caller receives a typed non-admitted decision plus a canonical trace
- current advisory examples include warning-bearing projection admission,
  deferred-neighbor posture, and explicit review-only stops rather than only
  runtime execution lanes

Violation:

- execution does not proceed
- denial or failure evidence still preserves the decision trace, and
  execution-time violations retain provenance when execution had already begun

Deferred:

- deferred neighbors remain explicit support posture, not best-effort support
- callers should consult the support matrix before teaching those families as
  ordinary runtime-backed flows

## Current Limits

- intent admission is public and executable for the covered authoritative,
  effect-triggered, authoritative-mutation, read-execution, derived
  inspection-materialization, lower-runtime capability-routing,
  basis-observation, and projection-consumption families
- it is still support-gated vocabulary rather than part of the stable everyday
  mutation surface
- generic materialization, temporal, async/resource, store-backed, and durable
  restart neighbors remain deferred work

## Related Docs

- [Writes and Intent Boundaries](./writes-and-intents.md)
- [Writes and Intent Examples](./writes-and-intents-examples.md)
- [Support Matrix and Admission](./support-matrix-and-admission.md)
- [Workspace Overview](./workspace-overview.md)
