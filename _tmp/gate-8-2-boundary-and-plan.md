# Gate 8.2 — Boundary Review And Implementation Plan

## Stage 1: Boundary Brief

### Semantic truth entering the slice
- Gate 8.1 installed aftermath with `InstalledExternalEffectContract::{None, Declared}` and R8.57 rejection of Reversible+external.
- Provider commit path co-commits Query-owned idempotency entities via `MutationIntent` (`provider/idempotency.rs` → `provider.rs::register_application_attempt`).
- Provider layer already distinguishes `CommitRecoveryRequired` / `AbortRecoveryRequired` and carries `Indeterminate(WorthQueryProviderSessionFailure)`; application boundary discards that into unit `Indeterminate` / `PartialEffect` (C3).
- Host-published time exists only as authorization-scoped `WorthQueryAuthorizationTimeSource` / `WorthQueryAuthorizationClock` (`authorization/time_*.rs`).
- Provider `plan_identity` / `token_identity` / `provider_receipt` are `String` (C4/G4) — diagnostic grade.
- Bank world crates: courtroom, domain, estate-certification, http-adapter, server. **No external-effect process.** Bank Phase 5 user-node processes remain blocked; Gate 8.2 must create the first real external boundary process itself.
- Relational CDC exists for live/collection delivery; Query already denies raw CDC as application delivery authority. CDC is **not** an application-authority owner.

### What this slice owns
- Query: external-effect posture ladder (7 distinct typed postures), typed correlation identity, transactional outbox co-commit, R8.13 `external_dispatch` slot (plus unused-but-required `undo_admission`/`redo_admission` slots), time-authority generalization (PB3), carrying C3 payloads, classification of timeout/disconnect/lost/duplicate/unknown without guessing completion.
- Bank world: real controllable external service in its own OS process over a network boundary, with injectable faults for exit proof.

### Adjacent ownership that continues
- Installation aftermath classification (8.1) — consume, do not redesign.
- Recovery handle / resolve lifecycle (8.3) — out of scope; postures may name unknown/recovery need but mint no handle.
- Undo/redo (8.4/8.5), publication cutover (8.6).
- Relational commits truth only; no lower runtime calls the world.
- CDC remains live-delivery substrate only.

### Weaker representations that must become insufficient
- Unit `Indeterminate` / `PartialEffect` at application commit outcome.
- Provider `String` identities used for any admission/transition equality for external-effect correlation.
- Authorization-named clock as the only trusted-time owner when dispatch timeout needs the same source.
- Any in-process fake sharing the runtime truth source as gate exit proof.

### Competing authorities / cutovers
- No existing outbox lane to retire; first construction.
- Do **not** use CDC as dispatch delivery (R8.8 evaluate → decline): would blur Relational subscription with application authority. Do **not** build a second Relational change stream either. Delivery derives from Query-owned outbox records after co-commit.
- Single time source: rename/generalize, do not fork.

### Downstream handoff
- Gate 8.3 will consume correlation evidence + indeterminate payloads to mint recovery handles.
- Bank courtroom later consumes the external rail for Phase 8.6 proofs.

### Dirty-edge failure modes
- In-process double that “looks like” timeout/ack/duplicate.
- Inferring ExternalCompletion from local commit, HTTP 2xx, or possession of an earlier posture.
- Opt-in outbox leaving mutation-free escaping effects unanchored.
- Defaulted-to-zero `external_dispatch` counter pretending lifecycle completeness.
- CDC checkpoint readmitted as dispatch posture.

### Unresolved facts verified before plan
- Entry condition unmet → Bank process crate is in causal closure.
- Idempotency co-commit is the mechanical precedent for outbox.
- `WorthQueryCanonicalWorkPhases::new` currently zeros later phases; R8.13 requires construction to demand the three new slots (arch law 9).
- CDC: decline as dispatch substrate; document/prove no CDC checkpoint admits as Query dispatch posture.

---

## Stage 2: Implementation Plan

### Slice
Runtime Phase 8 Gate 8.2 — External-Effect Causality And Indeterminate Posture (R8.22–R8.27, R8.4/R8.5/R8.7/R8.8/R8.13/R8.25/R8.26/R8.55, PB3).

### Boundary constraints that drive design
- Outbox co-commit is structural (D1/R8.55); zero cost when `InstalledExternalEffectContract::None` (R8.4).
- Seven postures are sealed distinct types/enum variants with predecessor links; no upgrade From/Into.
- Correlation is typed Query identity, never provider String equality.
- One host time source, renamed for runtime use; callers cannot supply samples.
- Bank external rail is a separate process; exit proof faults cannot be closed by an in-process fake.

### Intended result (DX sketch)
```rust
// After commit of an operation declaring an external effect:
let intent = outbox.committed_dispatch_intent()?; // Query-owned, co-committed
let attempt = dispatch.attempt(intent, &runtime_clock)?; // records time sample
// Faults classify to typed postures — never ExternalCompletion by guess:
match rail.exchange(attempt) {
    RailFault::LostResponse => ExternalEffectPosture::Unknown { .. },
    RailFault::AckWithoutCompletion => ExternalEffectPosture::ExternalAcknowledgement { .. },
    // ...
}
```

### Destination topology
```
worth-query-execution/src/domain_computation/application_aftermath/
  external_effect/
    mod.rs
    posture.rs              // 7 postures; no upgrade
    correlation.rs          // typed Query correlation identity
    identity.rs             // posture event identities + predecessor link
    classification.rs       // timeout/disconnect/lost/dup/unknown
    outbox.rs               // MutationIntent create for dispatch record
    dispatch.rs             // post-commit attempt progression (no CDC)

worth-query-execution/.../primary_graph/
  schema_layout/provider_dispatch_outbox.rs
  provider/dispatch_outbox.rs   // co-commit push (mirrors idempotency)

worth-query-installation/src/canonical_work.rs
  + external_dispatch, undo_admission, redo_admission (required construction)

authorization/ → runtime time authority rename (same source)

worth-query-bank-world/crates/bank-external-rail/
  lib + bin: TCP fault-controllable external service process
```

### Ordered steps
1. **R8.13** — Add three phase slots; change construction so every site must supply them (break Default-zero completeness). Fix construction sites; prove external_dispatch remains zero for no-effect commits.
2. **R8.7/PB3** — Generalize authorization time → Query runtime time authority (type rename + visibility for dispatch classification); keep single host-published source; record sample on expiry/timeout decisions.
3. **R8.5 + R8.22/R8.23** — Typed correlation identity + seven postures with exact identities and predecessor links; residue that provider Strings are not used in transition equality.
4. **R8.25/R8.4/R8.55** — Outbox layout + `dispatch_intent_create` into the same `MutationIntent` batch as idempotency iff external effect declared; prove zero intents/counters when None.
5. **R8.26** — Carry session failure / recovery-required kind on `Indeterminate` and `PartialEffect` through progression → application outcome; preserve Commit vs Abort recovery distinction.
6. **R8.24 + R8.8** — Classification module; explicit non-use of CDC; type/residue proof that CDC checkpoint cannot be a dispatch posture.
7. **Bank half** — `bank-external-rail` process + spawn harness proving five exit-proof faults over the network; Query-side classification consumes those outcomes. If incomplete: stop and report Query half built, gate not closeable.
8. **Evidence** — owner tests per package; line-cap dirty; boundary-check; agent-context; three worth-query consumer targets stay green.

### Cutover / deletion
- No predecessor outbox to delete.
- Authorization-scoped names become runtime time authority aliases only during cutover if needed — leave one name as the ordinary path.
- Do not leave an in-process fake as ordinary proof path.

### Out of scope
- Gates 8.3–8.6; PB1/PB2/PB4; re-derivation mechanism; `_docs/` edits; recovery handle minting.

### Blockers
- Gate exit requires the Bank process half. Honest incomplete report is allowed if Query authority lands first without a false in-process close.
