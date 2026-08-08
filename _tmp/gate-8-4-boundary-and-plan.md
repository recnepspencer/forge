# Gate 8.4 Turn 1 — Boundary Review And Implementation Plan

## Stage 1: Boundary Brief

### Slice selected
Fresh undo / inverse / compensation (Gate 8.4), entered after Gate 8.3
closure. Entry condition in the spec falsely lists R8.2 consumption and R8.9
as satisfied; both are this gate's work (Q8.10). Start with C2 — mutation work
must name touched records before any inverse can be derived — then instance-
scope the recovery registry (Q8.9 cause) before growing undo suites, then
build fresh undo admission that re-enters the ordinary progression.

### Semantic truth entering the slice
- **Committed receipt (C1)** — installed operation, principal scope,
  idempotency binding, already unforgeable and admission-derived.
- **Installed aftermath** — authority × mechanism axes, pre-image demand
  (R8.18), typed lowering correspondence slot (declaration-side; Bridge
  resolution still owed as R8.9).
- **Commit outcome** — `CommitResult.changed_records: Vec<RecordRef>` is the
  authoritative post-commit identity of what was mutated. Counters alone are
  insufficient (C2).
- **Current world authority** — capability, purpose, disclosure, conflict,
  graph, invariant, idempotency, provider, compare-and-commit. The receipt is
  evidence of *what happened*, never of *what is now lawful*.
- **Recovery handle (8.3)** — linear resource bound to receipt axes; transitions
  already re-admit. Undo must go *through* the handle, not beside it.

### What this slice owns
- Spec entry wording + ledger Q8.10.
- C2: `WorthQueryPrimaryMutationWorkEvidence` names touched records derived
  from the commit's `changed_records`; constructor stays non-public; every
  construction site must require the names.
- R8.2 consumption: retain the exact pre-image slice demanded by the installed
  inverse into the strengthened receipt; undo derives from that retention, not
  a live re-read.
- R8.9: installation resolves the typed Bridge correspondence; string slots
  remain diagnostic only.
- Undo admission + progression modules (destination topology already named):
  derive a *request* from mechanism/authority axes (not posture name); re-enter
  ordinary mutation entry; populate `undo_admission` at 1/1/0; one bounded
  undo intent identity independent of fan-out.
- Eight typed undo denials with no fallback mutation; money compensation with
  independent double-entry oracle; Foundational cannot authorize undo alone.
- Q8.9 cause: registry owned by the application runtime instance; retire
  `reset_for_integration_test` and `lock_for_test`.
- R8.64: cross-gate scenario driving undo through handle + rail + aftermath.
- R8.63 ledger update at closure.

### Adjacent ownership that continues
- Recovery inspect/resolve/compensate/reconcile transitions (8.3) — consume,
  do not redesign.
- External rail and posture ladder (8.2) — undo may observe, not reclassify.
- Aftermath installation axes (8.1) — consume; strengthen Bridge resolution
  only where R8.9 requires.
- Redo / lineage (8.5) — out of scope.
- Store durability — remains `StoreCapabilityRequired`.

### Weaker representations that must become insufficient
- Mutation work as six counters with no identities.
- Caller-supplied touched-record lists.
- Treating the receipt as current authority for any progression step.
- Parallel undo-only path that resembles progression.
- Posture-name branching for inverse vs compensation vs reconciliation.
- Process-global recovery registry + wipe/lock test affordances.
- Free-string lowering family as binding (G8).
- Foundational provenance as undo admission.

### Competing authorities / cutover
- Construct complete mutation work only on the commit path from
  `changed_records` + invariant counters; remove any path that can mint the
  public evidence without names.
- Registry: move from `OnceLock` static to `Arc<WorthQueryRecoveryHandleRegistry>`
  on `WorthQueryPrimaryGraphApplicationRuntime`; handle holds the Arc;
  delete static `register`/`reset*`/`lock_for_test`.
- Undo enters through the same `compare_and_commit_application` (or Bank
  ordinary mutation) entry as any other mutation, with an unusual derived
  input — not a free-standing undo executor.
- Inverse derivation keys off `InstalledCorrectionMechanism` /
  `InstalledCorrectionAuthority`, never `PublishedAftermathPosture` alone.

### Downstream handoff
- Receipt.mutation_work() exposes touched-record identities for undo
  derivation.
- Undo admission product carries one intent identity + original committed and
  aftermath identities; feeds ordinary progression.
- Cross-gate suite asserts undo through production Bank paths.
- Ledger rows move OPEN → PROVED / CLOSED with named evidence.

### Dirty-edge failure modes
- Names stored but inverse keys off counts or operation slot alone.
- Undo identity derivation that loops over postings/records (R8.40).
- World-drift test that corrupts the receipt instead of revoking capability.
- Path that writes then rolls back counted as “no mutation.”
- Double-entry oracle sharing production accounting code.
- Registry Arc leaked across runtimes in fixtures.
- Construction site that still compiles without names.

### Unresolved facts verified before plan
- `CommitResult` derefs to `CommitOutcome.changed_records: Vec<RecordRef>` —
  commit-derived names exist today; mutation work ignores them.
- Only production ctor of mutation work is `invariant_execution.rs` (pre-
  commit). Names must be attached at commit in `session_lifecycle`, or
  construction must move there — otherwise “from the commit” is a lie.
- `DeclaredLoweringCorrespondenceRef` is typed but installation copies the
  slot string without Bridge resolution — R8.9 still open.
- Pre-image demand is validated at install (R8.18); retention into the receipt
  is not implemented — R8.2 consumption is open.
- Destination modules `undo_admission.rs` / `undo_progression.rs` are named in
  §7 and do not exist yet.
- `WorthQueryPrimaryGraphApplicationRuntime` already owns instance-local
  registries for result buffers and basis leases — recovery registry follows
  that pattern.
- Bank recovery already goes through `application_runtime().mint_recovery_handle`
  — instance cutover has a production assembly site.

---

## Stage 2: Implementation Plan

### Slice name
Gate 8.4 turn 1 — C2, instance registry, fresh undo admission vertical slice.

### Boundary constraints carried into the plan
- Names from commit `changed_records` only.
- Undo is ordinary progression with unusual input.
- Receipt ≠ current authority.
- Q8.9 before new registry-touching undo suites.
- Axes, not posture names, select inverse/compensation/reconciliation.

### Intended developer-facing result

```rust
// C2 — names derived at commit; no public ctor
let work = receipt.mutation_work().expect("committed");
assert!(!work.touched_records().is_empty());

// Undo — through recovery handle + ordinary mutation entry
let admission = runtime.admit_undo(
    &handle, &effect_authority, &receipt, &aftermath, &operation_admission,
)?;
let outcome = runtime.compare_and_commit_application(
    admission.into_admitted_operation(),
    undo_idempotency,
);
```

### Module shape (destination topology)
```
worth-query-execution/.../provider/mutation_work.rs     # C2 identities
worth-query-execution/.../provider/session_lifecycle.rs # attach names at commit
worth-query-execution/.../managed_run/recovery_registry.rs  # instance Arc
worth-query-execution/.../application_aftermath/
    undo_admission.rs      # derive request + intent identity + counters
    undo_progression.rs    # denials + handoff into ordinary entry
    undo_preimage.rs       # retained pre-image slice (R8.2 consumption)
worth-query-installation/.../correction_mechanism/recorded_inverse.rs
    # R8.9 Bridge correspondence resolution at install
bank-server/.../estate_progression/undo.rs              # production assembly
bank-server/tests/.../phase8_cross_gate.rs              # R8.64 undo scenario
bank-server/tests/.../phase8_undo_*.rs                  # courtroom 1-5 shape
```

### Ordered steps
1. **Spec + ledger Q8.10** — rewrite Gate 8.4 entry like Gate 8.2's "builds its
   own entry condition"; mark Q8.10 CLOSED when wording is fixed.
2. **C2** — introduce sealed touched-record identity list on mutation work;
   construct complete evidence at commit from `changed_records` + invariant
   counters; break the old counter-only ctor; prove construction sites require
   names; prove inverse derivation consumes `touched_records()`.
3. **Q8.9** — registry becomes instance state on the application runtime;
   handle holds `Arc<Registry>`; delete reset/lock; update all bank and unit
   tests to assert against the runtime's registry.
4. **R8.2 consumption + R8.9** — retain pre-image per installed demand into the
   receipt; install-time Bridge correspondence resolution with rejection
   cases.
5. **Undo admission** — derive request from mechanism/authority; mint one
   undo intent identity (1 basis / 1 digest / 0 text); populate
   `undo_admission`; hand off to ordinary compare-and-commit.
6. **Denials + money + Foundational** — eight typed causes; compensating
   journals with independent oracle; no Foundational-only undo.
7. **R8.64** — cross-gate undo through handle + rail + aftermath.
8. **Verify** — Standing verification set in full; code-quality-qa + qa-tests;
   update ledger (R8.63).

### Cutover / deletion
- Counter-only `WorthQueryPrimaryMutationWorkEvidence::new` signature.
- Process-global registry, `reset_for_*`, `lock_for_test`, `test-support`
  wipe surface once instance scope lands.
- Any undo helper that skips admission steps.

### Acceptance evidence
- Unit: C2 names match commit records; fan-out twins leave undo counters at
  1/1/0; construction without names does not compile.
- Integration: world drift after honest commit denies undo on current policy;
  money compensation once; cross-gate undo through 8.3/8.2/8.1 products.
- Compile-fail: irreversible / released estate has no undo method (already
  8.1); strengthen if new outcome types appear.
- Residue: no test affordance without `test-support` after wipe retirement
  (feature may shrink to empty or be removed if unused).

### Verification commands (Standing set)
- `cargo test -p bank-server --test ordinary_mutations`
- Query consumers: `installed_operating_world`, `public_declarative_journeys`,
  `runtime_public_journeys`
- `cargo test -p worth-query-certification --test compile_certification`
- `cargo test -p worth-query-execution --lib` × 5, all reported
- warning-clean build, boundary-check, agent-context, dirty line-cap
- production-surface residue check

### Out of scope
- Redo / lineage (8.5)
- Full courtroom rows 6–12 except as already owned by 8.2/8.3
- Store-durable handles
- Deterministic re-derivation mechanism sibling
