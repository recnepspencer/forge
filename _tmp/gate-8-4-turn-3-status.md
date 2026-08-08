# Gate 8.4 Turn 3 — Status

## Verdict

**Gate 8.4 is closeable.** Turn 3 replaced R8.39 theatre with eight production
undo scenarios (typed cause + no-write + positive twins), completed
RecordedInverse end-to-end consumption of retained pre-image through ordinary
compare-and-commit, and kept Gate 8.5 unopened.

## Boundary reviewed / slice built

See `_tmp/gate-8-4-turn-3-boundary-and-plan.md`. Built against turn-3 audit:
delete enum-dedup; real denial scenarios; undo-attempt no-write snapshots;
RecordedInverse restore using retained `Status` bytes; Compensation divergence.

## Material artifacts

- `deny_irreversible_undo_attempt` on Bank undo admission (before mint)
- Undo-path Recovery `Expired`→`Undo(Stale)`, `AlreadyTerminal`→`AlreadyConsumed`
- EffectProgram `preimage_demand` wire through progression registration
- Freeze attaches declared RecordedInverse demand (`Status` field slot)
- `progress_undo_recorded_inverse` restores prior status via compare_and_commit
- Courtroom: `phase8_undo_denials`, `phase8_undo_denials_lifecycle`,
  `phase8_undo_recorded_inverse` (deleted install-only / enum-dedup tests)

## Rows considered proved

| Row | Evidence |
|---|---|
| R8.2 consumption | freeze retains pre-image; admit consumes; restore writes retained prior |
| R8.9 | turn 2 catalog install (unchanged) |
| R8.36–R8.37 | compensation + recorded-inverse ordinary progression |
| R8.38 | `phase8_undo_money` (turn 2; still green) |
| R8.39 | eight scenarios in `phase8_undo_denials*` with graph snapshots |
| R8.40 | turn 2 fan-out twin (unchanged) |
| R8.41 | turn 2 (unchanged) |

## Standing verification (all reported by name)

| Target | Result |
|---|---|
| `cargo test -p bank-server --test ordinary_mutations` | **62 passed** |
| `installed_operating_world` | **313** |
| `public_declarative_journeys` | **37** |
| `runtime_public_journeys` | **22** |
| `compile_certification` | **14** |
| `worth-query-execution --lib` × 5 | **562 × 5** (all five) |
| warning-clean (`RUSTFLAGS=-D warnings` on touched crates) | clean |
| boundary-check | pass |
| agent-context check | pass |
| dirty line-cap (Git Bash) | PASS |

## Cutover removed

- Enum-dedup “eight kinds distinguishable” theatre
- Install-as-irreversible counted as undo no-write
- Hardcoded `preimage_demand: None` on attempt registration
- Bank RecordedInverse → `CorrectionNotAdmitted` as the only inverse path

## Not started

Gate 8.5. Optional R8.40 receipt-backed fan-out twin remains polish only.
