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
- `forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)?.admit()?.scope()`
- `forge_query_projection_consumption_intent(declaration)?.admit()?.bind_contract()`

Read execution, inspection-materialization, and other future-neighbor intent
families remain explicitly deferred in the support matrix rather than being
quietly treated as partial support.

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
  effect-triggered, basis-observation, and projection-consumption families
- it is still support-gated vocabulary rather than part of the stable everyday
  mutation surface
- read execution, inspection-materialization, temporal, async/resource,
  store-backed, and durable restart neighbors remain deferred work

## Related Docs

- [Writes and Intent Boundaries](./writes-and-intents.md)
- [Writes and Intent Examples](./writes-and-intents-examples.md)
- [Support Matrix and Admission](./support-matrix-and-admission.md)
- [Workspace Overview](./workspace-overview.md)
