# Gate 8.4 Turn 2 — Status

## Verdict

Turn 2 is a coherent vertical slice closing the load-bearing honesty risks
named in the brief, **not** Gate 8.4 closure. A10 denial precision, R8.2
pre-image retention/consumption, R8.9 Bridge correspondence resolution,
A11 ordinary compensation progression, and R8.38 money with an independent
oracle are in. R8.39/R8.40/R8.41 are substantially proved; a few Bank
mutation-boundary no-write twins and RecordedInverse end-to-end remain
honestly open.

## Boundary reviewed / slice built

See `_tmp/gate-8-4-turn-2-boundary-and-plan.md`. Built against turn 1 audit:
tightened A10 arm; receipt-retained pre-image; install-time typed
correspondence catalog; undo progression through reverse-journal
`compare_and_commit_application`; money courtroom + independent activity-sum
oracle; denial matrix + fan-out intent twin.

## Material artifacts

- A10: `phase8_cross_gate` asserts
  `CapabilityAuthorizationMissing` on the Authorization arm
- R8.2: `undo_preimage` → receipt `retained_preimage`; RecordedInverse
  admission consumes it
- R8.9: `AftermathLoweringCorrespondenceCatalog` /
  `InstalledLoweringCorrespondence`; unresolved / wrong-generation /
  mismatched-participation denials
- A11: `progress_admitted_undo` + Bank `progress_undo_commit_recovery`
- R8.38: `phase8_undo_money` (committed journal rows, originals preserved,
  retry once, independent oracle)
- R8.39–R8.41: `phase8_undo_denials`, `undo_admission_tests` (EscapedEffect
  classification, Stale/AlreadyConsumed/Conflicted mapping, fan-out twin)

## Cutover removed

- Permissive A10 `Authorization(_)` empty arm
- String-only lowering correspondence as binding identity
- Admission-only undo counted as progression
- Process-global recovery wipe surfaces (already gone at turn 1; residue
  check still clean)

## Still owed (honest)

- R8.39 Bank mutation-boundary no-write for Conflicted / AlreadyConsumed
  after admit (install-time no-write and kind mapping proved)
- RecordedInverse Bank end-to-end consuming retained pre-image through
  ordinary progression (compensation path proved)
- Optional: receipt-backed R8.40 twin in addition to part-based fan-out twin

## Standing verification (all reported)

| Target | Result |
|---|---|
| `cargo test -p bank-server --test ordinary_mutations` | **56 passed** |
| `installed_operating_world` | **313** |
| `public_declarative_journeys` | **37** |
| `runtime_public_journeys` | **22** |
| `compile_certification` | **14** |
| `worth-query-execution --lib` × 5 | **562 × 5** |
| warning-clean (`RUSTFLAGS=-D warnings` on touched crates) | clean |
| boundary-check | pass |
| agent-context check | pass |
| dirty line-cap (Git Bash) | PASS |

## Best next QA target

RecordedInverse end-to-end through ordinary progression (retained pre-image
must be required and used), then Conflicted/AlreadyConsumed no-write at the
Bank mutation boundary — after those, Gate 8.4 can be closed honestly.
Do not start Gate 8.5.
