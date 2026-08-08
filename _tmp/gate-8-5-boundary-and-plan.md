# Gate 8.5 — Boundary Review And Implementation Plan

## Stage 1: Boundary Review

### Semantic truth entering the slice

- **Proved undo** (Gate 8.4): committed undo outcome through production
  admission → progression → ordinary compare-and-commit / reverse-journal.
  Possession is a *derivation* precondition only.
- **Installed aftermath** (8.1), **recovery handle + fresh effect authority**
  (8.3), **external rail / outbox** (8.2) — consumed through, not re-authored.
- **Commit receipts** carry installed operation, principal scope, idempotency,
  commit id (C1/R8.62). Aftermath carries compatibility generation.
- **Foundational** lineage vocabulary already exists (`SingularContinuity` and
  forbidden siblings). Query lowers into it; Query does not invent Foundational
  kinds.

### What Gate 8.5 owns

| Artifact | Owns | Must not own |
|---|---|---|
| `WorthQueryRedoIntent` / identity | Descriptive binding: original meaning, undo receipt, bound linear head, principal scope, compatibility generation. One digest (1/1/0). | Validity vs current head; runtime authority; replay state; `into_admission` that authorizes |
| `redo_admission` | Fresh capability/policy/conflict/touched-graph/invariant/idempotency/provider/compare-and-commit; populates `redo_admission` phase slot | Treating proved undo as current authority |
| `linear_lineage` | Parent-causality edge type; one chain / one head; **linear-lane divergence invalidation policy**; Foundational lowering after Query transition | Branch/merge placeholders; baking linearity into the edge type; empty 9.18 slots |
| Denial taxonomy | Eight distinct production causes matching exit proof | Enum-dedup theatre |

### Adjacent ownership that continues

- Undo admission/progression (8.4) — produce proved undo; do not absorb redo.
- Recovery handle mint / fresh authority (8.3) — redo may re-admit through the
  same authority pattern; does not mint authority from the intent.
- Publication of lineage to consumers (8.6) — this gate builds the execution
  lineage product and lowering; public courtroom cutover waits.
- Store durability — explicit non-goal; process-local chain is honest.

### Weaker proxies that must become insufficient

- Caller-supplied `bool` / “already undone” flag standing in for proved undo.
- Copied intent bytes / foreign principal / foreign runtime as authority.
- Receipt that once authorized undo standing in for current redo authority
  (R8.43 twin of R8.37).
- Intent method that consults live head and self-invalidates (R8.45 trap).
- Empty `BranchSuccessor` placeholder module (R8.53).
- Relabeling replayed/reconstructed/restored/branch-local/partial/promoted as
  ordinary chain (R8.46).

### Competing paths to cut over / avoid

- No prior `redo_*` / `linear_lineage` modules — greenfield under §7 topology.
- Do not route redo through certification replay.
- Do not count requests as lineage rows — count **committed** edges only.

### Downstream handoff

- Host facade re-exports via `primary_graph` (existing pattern).
- Bank `admit_redo_*` / `progress_redo_*` assembly mirrors undo.
- Cross-gate suite (`phase8_cross_gate`) gains a redo-through-stack scenario
  (R8.64): redo through 8.4 undo, 8.3 handle, 8.2 rail, 8.1 aftermath.
- Closure ledger update (R8.63).

### Dirty-edge failure modes

- Intent that validates divergence → 9.18 rewrite.
- Fresh-admission shortcut via proved-undo possession.
- Tests that corrupt the intent instead of drifting the world (wrong R8.43).
- Eight-scenario proof as enum-dedup array (Gate 8.4 turn-2 defect).
- Divergence proved only for ordinary intervening op (need undo/redo shapes).
- Replay import in ordinary redo path without mechanical residue check.

### Unresolved facts verified before coding

1. §7 names flat `linear_lineage.rs` — keep that file; design the **edge type**
   as parent-causality (not LinearOnly) so a branch leaf can enter later
   without reshaping.
2. `redo_admission` slot already exists on `WorthQueryCanonicalWorkPhases`
   (R8.13); this gate populates it exactly 1/1/0.
3. No lineage append exists in undo yet — Gate 8.5 owns appending edges for
   original, undo, and redo outcomes on the linear chain.
4. Compatibility generation lives on installed aftermath / handle binding;
   redo intent binds it descriptively and admission re-checks currentness.

---

## Stage 2: Implementation Plan

### Slice

**Gate 8.5 — Fresh redo intent and linear lineage** covering R8.42–R8.46,
exit proof (eight scenarios), courtroom rows 7–9 seed, R8.63 ledger update,
R8.64 cross-gate redo scenario, §8 `redo_admission` population, mechanical
replay import residue.

### Boundary constraints on the design

1. **`WorthQueryRedoIntent` has no method that reads current head to decide
   validity.** Divergence invalidation lives in
   `linear_lineage` lane policy consumed by `admit_redo`.
2. A 9.18 rebasing lane must reuse `WorthQueryRedoIntent` unchanged — answer
   in the status report against the real signature.
3. Edge type = parent→child causality identities; chain + policy enforce
   linearity. No empty branch placeholder.
4. R8.43: world-drift test with honest intent.

### Intended DX

```rust
// After proved undo commit:
let intent = WorthQueryRedoIntent::derive(&proved_undo, chain.bound_head_at_derivation());
// intent is descriptive — no authority:
let _ = intent.original_operation();
let _ = intent.bound_linear_head();

// Fresh admission — lane policy may deny DivergenceInvalidation:
let admission = admit_redo(handle, &authority, &intent, &chain, aftermath)?;
// chain.append_committed(...); // only on successful commit
```

### Module shape (§7)

```
application_aftermath/
  redo_intent.rs       // identity + descriptive intent (R8.42 / R8.10)
  redo_denial.rs       // eight distinct causes
  redo_admission.rs    // fresh admit; populates redo_admission 1/1/0
  redo_progression.rs  // handoff into ordinary mutation
  linear_lineage.rs    // edge, chain, divergence policy, Foundational lower
```

Plus bank assembly + tests; certification residue UI if needed.

### Ordered steps

1. **linear_lineage** — `WorthQueryAftermathParentCausalityEdge`,
   `WorthQueryLinearLineageChain` (one head),
   `evaluate_linear_divergence` policy, `lower_completed_transition` →
   `SingularContinuity` only; six negative lowering denials + positive twin.
2. **redo_intent** — derive from `WorthQueryProvedUndo` + bound head; digest
   1/1/0; getters only; no authority / replay fields; no head-check methods.
3. **redo_denial / redo_admission / redo_progression** — mirror undo shape;
   admit consults fresh authority + linear divergence policy + meaning/
   principal/generation currentness; never authorizes from intent alone.
4. **Wire** — `mod.rs`, facade, `admit_redo` on application runtime, bank
   `admit_redo_*` / `progress_redo_*`, lineage append at commit handoffs for
   original/undo/redo outcomes.
5. **Unit evidence** — R8.42 descriptive; R8.45 intent unchanged under
   divergence (policy denies); R8.44 committed edge counts; R8.46 six×2;
   counters 1/1/0 + fan-out twin.
6. **Bank eight scenarios** through production path, each own cause, no-write
   on denial; divergence ≥2 shapes (ordinary + intervening undo/redo).
7. **R8.64** cross-gate redo-through-stack scenario.
8. **Residue** — mechanical check: ordinary redo modules do not import
   `worth_query_replay` (compile-fail or ripgrep-cert style).
9. **Ledger + standing verification** — update closure ledger; run full set.

### Out of scope

- Gate 8.6 publication cutover / bank courtroom full matrix
- Branch-shaped lineage population
- Store-durable lineage
- Deterministic re-derivation mechanism leaf

### Verification commands (standing set)

- `cargo test -p bank-server --test ordinary_mutations`
- consumer targets named in ledger
- `cargo test -p worth-query-certification --test compile_certification`
- `cargo test -p worth-query-execution --lib` × 5
- `RUSTFLAGS=-Dwarnings cargo check` (scoped)
- boundary-check, agent-context, dirty line-cap
