# Gate 8.7 Pre-Written Oracle

Written **before** reading any Gate 8.7 diff. Every prediction below is a
falsifiable claim about what a defective implementation would look like. Graded
after the turn: HIT (defect present), CLEAN (prediction wrong — calibration
evidence), or N/A.

## State snapshot at brief time (verified by me, 2026-08-06)

- `dispatch_external_effect` call sites: **1**
  (`provider_execution/external_dispatch.rs`, `dispatch_committed_external_effect`).
- `safe_retry_recovery_handle` consumers: **0**. Other five R8.30 transitions:
  called from `bank-server/src/estate_progression/recovery.rs`
  (lines 106, 122, 139, 156, 273).
- `WorthQueryRecoveryHandleBinding` fields: 13, including
  `correlation: Option<ExternalEffectCorrelationIdentity>`. **No family field.**
- `WorthQueryDispatchOutboxRecord` = correlation + correlation_family +
  outcome_identity.
- `safe_retry.rs` is 42 lines; signature
  `(handle, &authority, InstalledAftermathRecoveryContract) -> Result<Admission, Denial>`.
- `WorthQueryRecoverySafeRetryAdmission` has exactly one field: `binding`.
- `phase8_residue::r8_11_*` asserts zero `worth_signal` in `application_aftermath/**`.
- `WorthQueryAdmittedApplicationOperation::mint` is `pub(super)`.

## Predicted defects, most likely first

**P1 — Permission dressed as proof (R8.66).** The highest-probability defect
and the one this gate exists to prevent. Watch for `safe_retry_recovery_handle`
gaining a parameter that a *caller* supplies rather than one only a real
dispatch can produce: a `bool`, a posture enum passed by value, an
`Option<...>` that defaults to `None`, or a `WorthQueryExternalEffectDispatch`
with a public constructor. This is the exact shape of the Gate 8.3 turn-1
defect (`capability_currently_grants: bool`). **Test: can I, from a test module
outside the crate, construct the retry-evidence type without a transport?** If
yes, R8.66 is not proved.

**P2 — Second classification site (R8.67).** A new `match` on
`WorthQueryExternalTransportOutcome` anywhere outside
`dispatch.rs::classify_observation`. Also watch for a "lighter" re-dispatch
helper that calls `transport.dispatch(...)` directly and maps the outcome
itself, skipping the canonical identity derivation. Grep: `transport.dispatch`
should have exactly the sites that `dispatch_external_effect` owns.

**P3 — Rail assertion at the wrong layer (R8.70).** The test asserts on the
returned posture, on `receipt.external_dispatch()`, or on a request counter,
rather than on the rail's own attempt count. The spec names this explicitly
because a request-layer assertion proves only that Query declined to ask. Row 2
of the test list is where this hides: "no second emission" is easy to fake by
returning early before the transport call and then asserting the early return.

**P4 — Denial tests that never prove ordering (R8.69).** The expired/terminal/
foreign-principal cases assert the denial and stop, without asserting the rail
recorded nothing. That would leave "authority precedes dispatch" unproved while
looking like three passing negative cases. Named in the brief; if it still
happens, the brief's explicitness was not the binding constraint.

**P5 — Binding change breaks axis-probe coherence (R8.68).** `axis_probe` is
`#[cfg(test)]` and constructs all 13 fields positionally-by-name. Adding or
replacing the correlation axis without updating the per-axis drift proofs would
leave one axis unbound — a hole in R8.28's proof that every axis denies on
drift. Watch for the probe compiling because the new field was given a
`Default`-ish value in the fixture rather than being driven by a drift case.

**P6 — Scope creep into Q8.14.** A `pending_dispatch` query, a `drain`,
`sweep`, `relay`, or `scan` function; a "retry policy" struct; a backoff field.
Explicitly forbidden. Grep the diff for those nouns.

**P7 — Signal or Bridge leakage.** `worth_signal` import in
`worth-query-execution`, or a Bridge lowering call in the aftermath lane. The
residue test should catch it; if the residue test was *modified*, that is a
worse finding than the import.

**P8 — `#[allow(...)]` reappearing.** Count is currently 0 in new aftermath
code. Any new suppression, especially one with a comment asserting the item is
used, is a HIT regardless of whether it compiles.

**P9 — The Bank consumer is a test-only method.** `safe_retry_commit_recovery`
added under `#[cfg(test)]`, or added to a test support module rather than
`estate_progression/recovery.rs`. That would leave the transition without a
production consumer while appearing to close Q8.13.

**P10 — Durability posture asserted, not published (R8.71).** A doc comment
saying "in-memory only" instead of a typed posture. R8.12's precedent is a real
enum variant (`StoreCapabilityRequired`) reachable through inspect. A comment
is not a posture.

## Ledger-attack predictions (expect some CLEAN)

These are checks I run against the *ledger* rather than the code, and I expect
several to come back clean — that is the calibration evidence.

- **L1** — Does the Gate 8.7 evidence column name a test that actually exists
  and actually runs? (Q8.6 class: a target that is red or absent.)
- **L2** — Does any R8.66–R8.71 row read PROVED on evidence that belongs to a
  different row? (Q8.11 class: O2 proved on O1 evidence.)
- **L3** — Does the reported `--lib` run count equal five, with every result
  reported rather than the best one?
- **L4** — Is `compile_certification` named by target in the report, not folded
  into "trybuild passed"?
- **L5** — Does Q8.14 stay deferred, with R8.71's posture actually built rather
  than the deferral quietly widening to cover R8.66 too?

## What I will do regardless of the report

Read the diff directly. Gate 8.3 turn 1, Gate 8.4 turn 2, and the skeptical-QA
pass all found defects that the report described as complete and green. The
report is a hypothesis about the code, not evidence of it.
