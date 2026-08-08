# Gate 8.3 — Boundary Review And Implementation Plan

## Stage 1: Boundary Brief

### Semantic truth entering the slice
- Gate 8.1: installed aftermath two-axis contracts; recovery contract is
  `Admissible { posture }` vs `NotAdmitted`; compensate keys off
  `Compensation` mechanism / Compensatable posture; reconcile keys off
  `RuntimeWithExternalOwner` / Reconcilable.
- Gate 8.2: seven `ExternalEffectPosture` variants with causal links;
  C3/C4 repaired; `bank-external-rail` separate process; runtime time
  authority at `runtime_time/` (single source).
- Commit receipt (`WorthQueryApplicationCommitReceipt`) still lacks
  installed operation, principal scope, and idempotency binding (**C1**).
  Construction is crate-private via `Pending`/`from_recovered_provider`.
- G5 taxonomy `WorthQueryApplicationIdempotencyResolution` and admitted
  resolve at `resolve_admitted_application_idempotency` already exist.
- Managed-run module is lifecycle-per-run, not a central registry.
  Closest linear pattern: `WorthQueryMoveOnlyArtifactHandle` (no Clone,
  consume-by-value dispose). Artifact registry is the enumerate/terminate
  precedent.
- No `recovery_handle.rs` / `recovery_progression.rs` stubs exist.
- No named test-world construction authority (R8.65 / Q8.1 cause open).
- Q8.3: posture successor construction is visibility-only.

### What this slice owns
- C1 receipt strengthening (R8.62) — derive fields from admission.
- Linear recovery handle mint/bind/transition/dispose/expire.
- Registry under managed-run family (enumerate + force-terminate).
- Six distinct transition surfaces (R8.30); resolve via G5 (R8.32).
- Fresh authority re-entry before effect-producing transitions (R8.31).
- Zero-cost inspection counters (R8.33); wire opacity (R8.34);
  durability/publication posture (R8.12/R8.35).
- Named test-scope world-construction authority (R8.65).
- First cross-gate integration scenario through rail + aftermath (R8.64).
- Q8.3 posture construction residual: harden or name-and-bound.

### Adjacent ownership that continues
- 8.1 classification / next-action type absence — consume, do not redesign.
- 8.2 postures / rail — consume through, do not stub.
- Undo/redo (8.4/8.5), full courtroom (8.6), C2 mutation-work naming.

### Weaker representations that must become insufficient
- Handle that is Clone/Copy or re-readable after consume.
- Minting bound to invented fields not carried by the receipt.
- Resolve via provider memory or parallel taxonomy.
- Opaque wire identity that is a digest of known fields (forgeable).
- Support artifact / published posture readmitted as handle.
- Shared generic denial across binding axes.
- Fixture ctor on production facade (Q8.1 repeat).
- Transition after mint that trusts stored policy over current truth.

### Competing authorities / cutovers
- Register in existing managed-run family; do not invent a second lifecycle
  tree.
- One time source (`WorthQueryRuntimeClock`); first expiry consumer.
- Reuse G5 resolve; no new resolution enum.

### Downstream handoff
- 8.4 consumes handle + C1 receipt for inverse derivation.
- Cross-gate suite grows at 8.4/8.5 under the same named harness.

### Dirty-edge failure modes
- Handle-as-value (Clone, store-and-reuse, wire re-admit).
- Binding to ordinal instead of typed branch (equal-ordinal foreign branch).
- Inspect skipping disclosure admission.
- Compensate/reconcile keyed off posture name strings rather than installed
  mechanism/authority axes.
- One transition function with a mode parameter (R8.17 shape).
- Double-transition denied only at runtime when by-value consume could make
  it a compile failure.

### Unresolved facts verified before plan
- Receipt construction sites: `complete`, `from_recovered_provider` (+ callers
  in progression, entry, read_set, idempotency_resolution). Admission and
  idempotency binding are in scope at those sites.
- `WorthQueryOperationScopeBinding` is the admitted principal-scope carrier.
- `recovery_inspection` phase slot already exists (zero by default); minting
  accrues exactly one identity derivation; inspection must stay at zeros.
- Host must not expose raw handle internals.

---

## Stage 2: Implementation Plan

### Slice
Runtime Phase 8 Gate 8.3 — Recovery Handle And Resolution Lifecycle
(R8.28–R8.35, R8.62, R8.12, R8.6/R8.7 M2–M3, R8.10, R8.64, R8.65, Q8.3).

### Boundary constraints that drive design
1. C1 first — handle cannot bind to fields the receipt does not carry.
2. Handle is move-only; transitions take `self` by value where they terminate
   or consume the live resource; registry tracks live set for enumerate /
   force-terminate / leak detection.
3. Eleven binding axes each deny with a distinct typed cause.
4. Six transitions are six functions/types, not one with a mode.
5. Effect-producing transitions re-admit current capability/policy first;
   inspect still requires disclosure admission.
6. Resolve returns inherited G5 taxonomy; never upgrades unresolved external
   posture to completed.
7. World construction lives in test scope and uses production derivation.

### Intended result (DX sketch)
```rust
let handle = runtime.mint_recovery_handle(receipt, &installed_aftermath)?;
// move-only; no Clone
let view = handle.inspect(&current_admission)?; // no effect authority
let resolution = handle.resolve(&current_admission)?; // G5 taxonomy
let _ = handle.dispose()?; // terminal; second call unrepresentable
```

### Destination topology
```
worth-query-execution/.../application_aftermath/
  recovery_handle/
    mod.rs
    identity.rs          // opaque unforgeable identity (framework secret)
    binding.rs           // eleven-axis binding record derived from receipt
    mint.rs              // mint only when installed recovery admits
    denial.rs            // distinct denial causes per axis + lifecycle
    registry.rs          // managed-run-family live-set; enumerate/force-term
    handle.rs            // move-only handle; Drop leak accounting
  recovery_progression/
    mod.rs
    inspect.rs
    resolve.rs
    safe_retry.rs
    compensate.rs        // keys InstalledCorrectionMechanism::Compensation
    reconcile.rs         // keys RuntimeWithExternalOwner / Reconcilable
    dispose.rs
    fresh_authority.rs   // re-establish provider + application authority
    expiry.rs            // runtime clock sample recorded on decision
  publication/
    durability.rs        // R8.12 Store-capability-required posture
    support_projection.rs // R8.35 Foundational support-truth vocab

worth-query-execution/.../compare_and_commit.rs
  + installed_operation, principal_scope, idempotency_binding on receipt

test-support (test cfg or dedicated test crate module):
  worth-query-execution/.../application_aftermath/test_world/
    OR bank-server / worth-query tests:
    phase8_world_construction.rs  // R8.65 named authority

cross-gate suite:
  bank-server/tests/.../phase8_cross_gate/  // R8.64 first scenario
```

### Ordered steps
1. **R8.62 / C1** — Add three required fields to receipt; thread through
   `Pending`/`complete`/`from_recovered_provider` from admission + binding.
   No public ctor; no defaults. Accessors read-only. Prove every construction
   site breaks until supplied.
2. **Q8.3** — Restrict successor posture construction: make Compensation /
   Reconciliation constructors `pub(crate)` requiring provider-held evidence
   token, OR document residual bound if structural seal is incomplete.
   Prefer: remove public constructors for late postures; require
   `ExternalEffectPostureEvidence` minted only by dispatch/classification.
3. **R8.28/R8.29** — Identity + binding + registry + move-only handle; mint
   derives exactly one identity; register in managed-run-family registry.
4. **R8.30–R8.32** — Six transition modules; resolve calls admitted
   idempotency; compensate/reconcile check installed mechanism/authority.
5. **R8.31** — Fresh authority gate shared by effect-producing transitions;
   inspect requires disclosure admission without producing effect authority.
6. **R8.7 M2/M3 + expiry** — Expiry samples runtime clock; records sample;
   caller cannot supply sample.
7. **R8.33/R8.34/R8.35/R8.12** — Counter assertions; opaque identity (not
   digest of public fields); publication support projection; durability
   explicit.
8. **R8.65** — Named test-scope world construction authority.
9. **Evidence** — Eleven exit scenarios + leak detection + drift per axis +
   cost zeros (inspect twice) + cross-gate lost-response through real rail
   with balance oracle.
10. **Ledger** — Update phase-8 closure ledger for 8.3 rows.
11. **Verify** — format, line-cap dirty, boundary-check, agent-context,
    Gate 8.1/8.2 suites, three consumer targets.

### Cutover / deletion
- No predecessor recovery handle to retire.
- Do not leave Clone on handle or identity.
- Do not export fixture constructors from production facades.

### Out of scope
- Gates 8.4–8.6 behavior (undo/redo/courtroom full).
- C2 touched-record naming.
- Durable/restart-stable handles (explicit Store-required posture only).

### Blockers
- None structural; rail and aftermath already exist for R8.64.
