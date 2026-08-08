# Gate 8.5 Turn 2 — Boundary Review And Plan

## Stage 1: Boundary Review

### What turn 2 owns

The residual A9 / X2 / X3 row only. Turn 1 proved the type/policy centre
(R8.45), six of eight exit causes through Bank, and the mapping unit tests for
`Stale`/`NewlyUnauthorized`. Turn 2 must prove those two causes through the
**Bank production path** in the world-drift shape:

- world drifts after proved undo
- intent remains completely honest
- denial names the exact cause
- positive twin admits without the drift

### Semantic truth entering

- `WorthQueryRedoIntent` is descriptive; possession authorizes nothing.
- Bank `admit_redo_disbursement_recovery` re-admits capability first, then
  recovery effect authority, then `admit_redo`.
- `map_redo_admission_denial`: Authorization → `NewlyUnauthorized`.
- `map_redo_path_recovery_denial`: `Expired` → `Stale`;
  `CurrentPolicyDenied` / `FreshAuthorityDenied` → `NewlyUnauthorized`.
- Recovery handle TTL = 3_600_000 ms from mint sample.
- Grant validity is `CapabilityValidity` compared against the runtime
  authorization clock (Gate 8.4 A10 pattern).

### What this slice may own

- Disbursement fixture clock + grant-validity delta (narrow world compiler
  extension — same pattern as notify-death / cross-gate).
- Bank courtroom tests that drift the world after `commit_and_prove_undo`,
  leave the intent untouched, and assert the typed cause + intent equality.
- Closure-ledger correction: Gate 8.5 is not CLOSED until these land; record
  string-typed derivation `Result` residue for Gate 8.6 sweep.

### What adjacent continues to own

- Intent / lineage / admission types (turn 1 — do not reshape).
- Unit `map_recovery_denial` proofs (remain; no longer the sole evidence for
  these two causes).
- Gate 8.6 publication / residue sweep — not started.

### Weaker proxies that must stay insufficient

- Corrupting the intent instead of drifting the world.
- Asserting only that *some* denial maps to the kind (unit map table).
- Foreign-principal / terminal-handle admits standing in for world-drift.
- Closing the ledger before A9 and X2 land.

### Dirty-edge risks

- Clock advance past both grant and handle TTL → ambiguous cause (Stale vs
  NewlyUnauthorized). Separate windows: grant expires inside handle TTL for
  A9; handle expires under still-valid grant for X2.
- Fixture extension that invents a parallel disbursement authority path.
- File growth of `phase8_redo_denials.rs` past 400 lines — new module for
  world-drift family.

### Unresolved facts verified

1. Handle TTL is one hour from mint; Gate 8.4 stale advances 2000 → 5601.
2. Grant validity uses epoch seconds on `EstateMoment`; Gate 8.4 world-drift
   uses grant until 600, clock 500 → 601.
3. `WorthQueryRedoIntent: PartialEq` — clone-before / assert-after is honest
   "intent unchanged" evidence.

---

## Stage 2: Implementation Plan

### Slice

**Gate 8.5 turn 2 — A9 / X2 / X3 world-drift redo denials**

### Steps

1. Extend `disburse_estate` fixture with
   `disbursement_world_with_clock_and_grant_validity` threading
   `AuthorizationTimeController` and optional grant `valid_until`.
2. Add `phase8_redo_world_drift.rs`:
   - `newly_unauthorized_after_grant_expiry_with_honest_intent` (+ twin)
   - `stale_after_handle_expiry_with_honest_intent` (+ twin)
   Each: prove undo → snapshot intent → drift → deny exact kind → intent eq;
   twin admits without drift.
3. Wire module; update module docs in `phase8_redo_denials.rs`.
4. Correct Gate 8.5 ledger status; mark A9/X2/X3 PROVED; record string-typed
   derive `Result<&'static str>` as Gate 8.6 residue finding (not fix here).
5. Standing verification set in full.

### Out of scope

- Gate 8.6; fixing string-typed derivation errors; reshaping intent/lineage.
