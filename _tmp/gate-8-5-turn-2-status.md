# Gate 8.5 turn 2 — status

## Verdict

**Gate 8.5 is CLOSED.** Turn 1's type/policy centre held under audit. Turn 2
lands the residual A9 / X2 / X3 world-drift proofs through the Bank production
path.

## What turn 2 owed and landed

1. **A9 / X3** — `newly_unauthorized_after_grant_expiry_with_honest_intent`:
   grant expires after proved undo; intent unchanged; Bank denies
   `NewlyUnauthorized`; positive twin admits.
2. **X2** — `stale_after_handle_expiry_with_honest_intent`: recovery handle
   clock-expires after proved undo; intent unchanged; Bank denies `Stale`;
   positive twin admits.

Fixture delta: `disbursement_world_with_clock_and_grant_validity` (same shape as
notify-death / cross-gate clock worlds).

## Finding recorded for Gate 8.6 (not fixed here)

`WorthQueryRedoIntent::derive` and peers return `Result<Self, &'static str>` for
internal digest failures while consumer denials are typed kinds. Predates this
gate; owed at 8.6 residue sweep.

## Standing verification (by name)

| Check | Result |
|---|---|
| `cargo test -p bank-server --test ordinary_mutations` | **72** |
| `cargo test -p worth-query --test installed_operating_world` | **313** |
| `cargo test -p worth-query --test public_declarative_journeys` | **37** |
| `cargo test -p worth-query --test runtime_public_journeys` | **22** |
| `cargo test -p worth-query-certification --test compile_certification` | **14** |
| `cargo test -p worth-query-execution --lib` × 5 | **578, 578, 578, 578, 578** |
| `RUSTFLAGS=-Dwarnings cargo check` (execution + bank-server) | clean |
| `boundary-check` | exit 0 |
| `agent-context check` | exit 0 |
| `check_workspace_rust_line_caps.sh dirty` | PASS |

Gate 8.6 not started.
