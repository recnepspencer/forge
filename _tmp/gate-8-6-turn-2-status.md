# Gate 8.6 turn 2 — status (Phase 8 close)

## Verdict

**Gate 8.6 turn 2 CLOSED. Phase 8 is CLOSED.**

No production code changed. Two durable-artifact defects from the auditor's
turn-2 brief are repaired.

## What landed

### 1. Exit condition rewritten (true under Q8.3)

Was self-contradictory: required every `Q8.*` CLOSED and no PARTIAL evidence,
then claimed two PARTIAL rows with deadlines earlier than Phase 8 close.

Now: every `R8.*` PROVED; every `Q8.*` CLOSED **except** the single named
deliberate carry **Q8.3**; no *unnamed* PARTIAL.

- **Q8.3** owner: Runtime Hardening Track (`external_effect`)
- Resolves no later than **Runtime Phase 9** (before facade snapshot)
- Sound because: type not consumer-exported; Compensation/Reconciliation
  require evidence; earlier ladder ctors `pub(crate)`; residual bounded in
  writing
- Q8.9 noted as already CLOSED at Gate 8.4 (not a Phase 8 carry)

### 2. §11 courtroom traceability map

Fifteen-row table in the Gate 8.6 ledger section, each scenario → named
test(s). Honest gaps recorded:

| Row | Gap |
|---|---|
| 11 | Handle non-leak proved; separate user-node session/queue crash is Bank Phase 5 TCP courtroom |
| 14 | Co-commit / lost-response / exactly-once covered via NotifyDeath; **no named mutation-free (O2) e2e** |

## Standing verification (turn 2)

| Check | Result |
|---|---|
| `cargo test -p bank-server --test ordinary_mutations` | **79** |
| `cargo test -p worth-query --test installed_operating_world` | **313** |
| `cargo test -p worth-query --test public_declarative_journeys` | **37** |
| `cargo test -p worth-query --test runtime_public_journeys` | **22** |
| `cargo test -p worth-query-certification --test compile_certification` | **14** |
| `cargo test -p worth-query-execution --lib` × 5 | **578, 578, 578, 578, 578** |
| `RUSTFLAGS=-Dwarnings cargo check` (execution + bank-server) | clean |
| `boundary-check` / `agent-context check` | exit 0 |
| Dirty line-cap (dirty scope; PowerShell equivalent) | PASS |

## Artifacts

- `_tmp/gate-8-6-turn-2-boundary-and-plan.md`
- `_tmp/gate-8-6-turn-2-status.md` (this file)
- `_docs/WORTH-query/milestone-9.16-runtime-phase-8-closure-ledger.md`
