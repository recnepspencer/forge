# Gate 8.4 Turn 1 — Status

## Verdict

Turn 1 is a coherent vertical slice, **not** Gate 8.4 closure. Entry defect
(Q8.10), C2, Q8.9 cause, and fresh undo admission with cross-gate evidence are
in. Money compensation, full denial matrix, R8.2 pre-image retention, and R8.9
Bridge resolution remain for later turns.

## Boundary reviewed / slice built

See `_tmp/gate-8-4-boundary-and-plan.md`. Built: C2 touched-record names;
instance-scoped recovery registry; undo admission/intent/denial; Bank
`admit_undo_commit_recovery`; R8.64 undo scenarios.

## Material artifacts

- Spec: Gate 8.4 entry rewritten (“builds its own entry condition”)
- `provider/mutation_work.rs` — C2 identities from commit `changed_records`
- `managed_run/recovery_registry.rs` — instance `Arc` registry; reset/lock gone
- `application_aftermath/undo_{admission,intent,denial}.rs` + tests
- Bank `estate_progression/recovery.rs` — `admit_undo_commit_recovery`
- `phase8_cross_gate` — undo through stack + world-drift denial
- Closure ledger updated (R8.63)

## Cutover removed

- Counter-only mutation work construction
- Process-global recovery registry, `reset_for_*`, `lock_for_test`
- Free `mint_recovery_handle` (runtime method only)

## Still owed (honest)

- R8.2 consumption (retain pre-image into receipt)
- R8.9 Bridge correspondence resolution at install
- R8.38 money journals + independent oracle
- R8.39 full eight denials with no-write proof
- R8.40 fan-out twins; R8.41 positive Foundational description
- Ordinary compare-and-commit progression for derived inverse/compensation

## Standing verification (all reported)

| Target | Result |
|---|---|
| `cargo test -p bank-server --test ordinary_mutations` | 51 passed |
| `installed_operating_world` | 313 |
| `public_declarative_journeys` | 37 |
| `runtime_public_journeys` | 22 |
| `compile_certification` | 14 |
| `worth-query-execution --lib` × 5 | 554 × 5 |
| warning-clean check | clean |
| boundary-check / agent-context / dirty line-cap | pass |
| residue | no reset/lock symbols |

## Best next QA target

R8.2 pre-image retention into the receipt + R8.9 Bridge resolution, then
money compensation (R8.38) with an independent double-entry oracle — those are
the remaining load-bearing honesty risks before claiming Gate 8.4 closed.
