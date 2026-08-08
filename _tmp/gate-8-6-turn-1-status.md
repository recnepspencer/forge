# Gate 8.6 turn 1 — status

## Verdict

**Gate 8.6 is CLOSED. Phase 8 is closeable.**

## What landed

### R8.47–R8.51

| ID | Disposition |
|---|---|
| R8.47 | Typed host/installation facade inventory + Bank production paths |
| R8.48 | Paired protected AccountDetails worlds; equal posture/next-action/lineage; vacuity asserted |
| R8.49 | HTTP adapter mechanical residue — no recovery authority |
| R8.50 | **Removed** monolith aftermath, bank `EstateAftermath`, Phase-8 rollback door (not privatized) |
| R8.51 | Live transport asserted, then ordinary commit 0/0/0 on all three aftermath slots |

### Carried PARTIAL rows trued up

R8.0, R8.3, R8.10, R8.11, R8.14, R8.15, R8.59–R8.61, R8.63–R8.65, Q8.4, Q8.7
moved to PROVED/CLOSED with evidence columns updated. Q8.3 remains deliberate
PARTIAL. `&'static str` derivation → `WorthQueryAftermathDerivationFailure`.

### PB1/PB2/PB4

**Recorded** in Bank front-door gap ledger. Rename **out of scope** (spec §12).
PB1/PB2 deadline: before Phase 9 facade snapshot. Owner: Phase 9.

### R8.50 remove vs privatize

| Path | Action |
|---|---|
| Monolith `operation_aftermath` | **Removed** |
| Bank `EstateAftermath` | **Removed** |
| Phase-8 generic rollback door | **Removed** (absent); managed-run cleanup rollback retained (not aftermath) |

## Standing verification

| Check | Result |
|---|---|
| bank-server ordinary_mutations | **79** |
| installed_operating_world | **313** |
| public_declarative_journeys | **37** |
| runtime_public_journeys | **22** |
| compile_certification | **14** |
| worth-query-execution --lib × 5 | **578, 578, 578, 578, 578** |
| RUSTFLAGS=-Dwarnings cargo check | clean |
| boundary-check / agent-context | exit 0 |
| dirty line-cap | PASS |

## Artifacts

- `_tmp/gate-8-6-boundary-and-plan.md`
- `_docs/WORTH-query/milestone-9.16-runtime-phase-8-closure-ledger.md` (Gate 8.6 section)
- `workspaces/worth-query-bank-world/docs/front-door-closure-ledger.md` (PB rows)
