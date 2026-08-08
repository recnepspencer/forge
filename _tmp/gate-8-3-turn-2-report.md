# Gate 8.3 Turn 2 — Report

**Gate 8.3 is still not closed.** Turn 2 fixed the R8.31 API inversion and
wired production + expiry; the exit-proof matrix is stronger but incomplete.

## Boundary reviewed / slice built
Who in production calls recovery, and where does compared truth come from?

- **Callers:** `BankIdentityRuntime::{open_commit_recovery, admit_commit_recovery_effect,
  admit_commit_recovery_inspect, dispose/reconcile/resolve/inspect_commit_recovery,
  evaluate_commit_recovery_expiry}`. Cross-gate lost-response uses these — not
  a hand-filled FreshAuthority.
- **Truth:** `admit_recovery_effect_authority` / `admit_recovery_inspect_authority`
  on `WorthQueryPrimaryGraphApplicationRuntime` derive current axes from
  (1) live admitted operation, (2) recovery-target receipt, (3) installed
  aftermath, (4) runtime clock for expiry. Capability/disclosure are privately
  constructed proofs — no booleans.

Plan: `_tmp/gate-8-3-turn-2-boundary-and-plan.md`.

## Material changes
- Removed `WorthQueryRecoveryFreshAuthority` public field bag.
- Added `WorthQueryRecoveryEffectAuthority`, `WorthQueryRecoveryInspectAuthority`,
  `WorthQueryRecoveryDisclosureAdmission` (private mint).
- Distinct `CompensationNotAdmitted` / `ReconciliationNotAdmitted`.
- Expiry evaluation on runtime clock; mint attaches TTL via runtime mint.
- Q8.3: earlier ladder posture ctors sealed `pub(crate)` (dated residual).
- trybuild: no Clone; no duplicate transition; inspect ≠ effect authority.

## Cutover
- Deleted bool-based `fresh_authority_for` from cross-gate world.
- Tests go through production Bank admit paths.

## Verification
- `phase8*` bank tests: 7 pass
- Gate 8.1 aftermath: 13 pass
- Gate 8.2 external_effect (execution) + bank rail + undeclared: pass
- recovery_handle unit tests: pass
- application_aftermath_compile_fail (incl. 3 new trybuild): pass
- boundary-check, agent-context: pass
- Dirty files ≤ 400 lines (manual count; bash line-cap script unavailable on this host)

## Still open (honest)
- Full per-axis drift matrix (foreign runtime, foreign-branch-equal-ordinal, …)
- Clock-advanced expiry → Expired terminal + leak proof for that path
- Capability-grant revocation (vs foreign-principal) as CurrentPolicyDenied twin
- Gate closure itself

## Production call sites (the question turn 1 missed)
| Site | Role |
|---|---|
| `BankIdentityRuntime::open_commit_recovery` | mints via runtime clock |
| `admit_commit_recovery_effect` | re-admits notify-death → EffectAuthority |
| `admit_commit_recovery_inspect` | capability + disclosure → InspectAuthority |
| `dispose/reconcile/resolve/inspect_commit_recovery` | drive transitions |
| `evaluate_commit_recovery_expiry` | R8.7 M2/M3 sample |
| Compared truth | admitted operation + receipt + aftermath + clock — not caller fields |
