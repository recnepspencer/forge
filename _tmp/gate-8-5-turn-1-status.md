# Gate 8.5 turn 1 — status

## Verdict

**Gate 8.5 is closeable.** Fresh redo intent and linear lineage landed; the
type/policy centre (R8.45) holds by signature inspection, not only by tests.

## Answers the brief demanded by reading signatures

1. **Where does invalidation live?**
   `WorthQueryLinearLineageChain::evaluate_divergence(bound_head)` in
   `linear_lineage.rs`. `admit_redo` consults it. `WorthQueryRedoIntent` has
   only getters for `bound_linear_head` — no validity method against the live
   head.

2. **Could a 9.18 rebasing lane reuse `WorthQueryRedoIntent` unchanged?**
   **Yes.** Real signature:
   `WorthQueryRedoIntent::derive(proved: &WorthQueryProvedUndo, bound_linear_head: WorthQueryAftermathLineageNode) -> Result<Self, &'static str>`.
   The bound head is descriptive input. A rebasing lane supplies a different
   lane policy at admission and never needs to unpick this type. Symmetrically,
   `WorthQueryAftermathParentCausalityEdge` is parent→child only — no linear
   invariant; a branch-shaped successor can enter as a leaf without reshaping.

## Rows proved

| Row | Evidence |
|---|---|
| R8.42 | Descriptive intent; no authority/replay; residue excludes `worth_query_replay` |
| R8.43 | Fresh admit; proved undo is derivation only; no caller bools for copied/duplicate |
| R8.44 | Committed lineage rows (not requests); one parent edge per successor |
| R8.45 | Policy on chain; intent unchanged under divergence |
| R8.46 | `SingularContinuity` only; six forbidden postures × positive twin |
| R8.63 | Ledger updated in-slice |
| R8.64 | `redo_through_undo_handle_rail_and_aftermath` |

## Standing verification (by name)

- bank `ordinary_mutations` **70**
- consumers **313 / 37 / 22**
- `compile_certification` **14**
- `worth-query-execution --lib` **578 × 5** (all green)
- `RUSTFLAGS=-Dwarnings` check on execution + bank-server: clean
- boundary-check / agent-context: exit 0
- Gate 8.5 dirty files all ≤ 400 lines

## Residual

Dedicated disbursement clock-expiry twin for `Stale` after proved undo (exact
8.4 A10 shape) not landed; `Stale`/`NewlyUnauthorized` proved via production
`map_recovery_denial` on the bank path and foreign/terminal production admits.
Gate 8.6 not started.

## Material artifacts

- `redo_intent.rs`, `redo_admission.rs`, `redo_denial.rs`, `redo_progression.rs`
- `linear_lineage.rs`
- Bank `estate_progression/redo.rs`
- Tests: `redo_admission_tests`, `phase8_redo_*`, cross-gate redo scenario
- Closure ledger Gate 8.5 section
