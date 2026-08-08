# Gate 8.3 — Turn 3

Turn 2 fixed the R8.31 inversion properly. I verified the fix by reading the
code, and it holds:

- `WorthQueryRecoveryEffectAuthority` / `InspectAuthority` are privately minted
  (`pub(crate) fn mint`, `_private: ()`), so no caller can fabricate one.
- `admit_recovery_effect_authority` requires a real
  `WorthQueryAdmittedApplicationOperation`, whose only constructor is
  `pub(super) fn mint`. **A test cannot forge one.** That is the linchpin, and
  it is what makes the drift attacks meaningful rather than circular.
- `check_binding_axes` now compares the handle against *current admission* and
  *the recovery-target receipt* — runtime-held truth on both sides.
- `ensure_admission_belongs_to_runtime` calls `validate_current_authority()`,
  and expiry runs off the runtime's own clock.
- Production path exists: `resolve_commit_recovery` performs
  `resolve_admitted_application_idempotency(...)` before consuming the handle.

Verification I re-ran myself, not read from your report: bank
`ordinary_mutations` **42 pass** (was 35 at Gate 8.2), and the three Query
consumer targets **313 / 37 / 22** — exactly the Gate 8.2 baseline.

Gate 8.3 still does not close. Six items, two of which are new and one of which
is not your fault.

## 1. Constitution violation — the 400-line cap (blocking)

Two files you touched are over:

- `.../authorization/admitted_operation.rs` — **408** (was 397 at HEAD)
- `.../primary_graph/application_attempt/compare_and_commit.rs` — **407**
  (was 283 at HEAD)

Both were compliant before Phase 8. Split them. `compare_and_commit.rs` more
than doubled across Gates 8.2 and 8.3 and is the clearer decomposition target —
the receipt type, its accessors, and the commit progression are separable.

## 2. Gate 8.1's retirement was not exact — append-only corrective (blocking)

This is not a turn 2 defect. It has been latent since Gate 8.1 and neither my
audits nor yours caught it, because none of us ran the certification
compile-fail target. §9 says a discovery that strengthens a closed gate becomes
an append-only corrective and **blocks unfinished dependents** — Gate 8.3 is
the unfinished dependent, so it lands on you.

Gate 8.1 retired `domain_installation/operation_aftermath/` correctly in
production terms, but the retirement left two pieces of residue:

**(a) An orphaned authority witness.** `query_effect_lifecycle_authority()` in
`identity_authority/authority.rs:62` had exactly one caller —
`operation_aftermath/authority.rs`, which Gate 8.1 deleted. It is now dead code
with an unused import in `identity_authority/mod.rs:13`, and the build emits
two warnings. Decide deliberately: if the effect-lifecycle authority has a
successor in the new aftermath topology, wire it; if it does not, retire it.
Do not silence it with an attribute.

**(b) A stale certification fixture.**
`worth-query-certification/tests/ui/replay/ordinary_facade_cannot_import_replay_capability.stderr`
still expects rustc to suggest `WorthQueryCompensationCapability` as "a similar
name" in `facade::domain`. Gate 8.1 retired that symbol, so the suggestion is
gone and the fixture no longer matches.

`cargo test -p worth-query-certification --test compile_certification` is
**red** and has been since Gate 8.1: `13 passed; 1 failed`.

Read this next part carefully before fixing it. The guarantee itself is intact
— the test still fails to compile with `E0432` for exactly the right reason
(no such symbol in `facade::domain`). Only rustc's incidental suggestion list
changed. So the correct fix is to re-bless the fixture, having confirmed the
actual output is right. **Confirm it yourself before blessing** — `TRYBUILD=overwrite`
on a fixture you have not read is how a real regression gets laundered into a
green test.

While you are there: check the other `.stderr` fixtures for the same staleness.
A fixture that encodes a facade's contents is a witness to that facade, and
Phase 8 has been changing facades for three gates.

## 3. `resolve` still takes its answer as a parameter (blocking)

```rust
pub fn resolve_recovery_handle(
    handle: WorthQueryRecoveryHandle,
    authority: &WorthQueryRecoveryEffectAuthority,
    resolution: WorthQueryApplicationIdempotencyResolution,   // <- supplied
) -> Result<WorthQueryApplicationIdempotencyResolution, ...>
```

The file's own doc comment says "Resolve via admitted idempotency read (R8.32)."
The function performs no read. It receives the taxonomy value and echoes it.

Your production wrapper `resolve_commit_recovery` does the right thing — it
calls `resolve_admitted_application_idempotency` and passes the result in. But
`resolve_recovery_handle` is exported through `worth-query-execution`'s facade,
and it will accept *any* `WorthQueryApplicationIdempotencyResolution` a caller
constructs. The read is not bound to its result.

This is the same shape as turn 1's `capability_currently_grants: bool`, one
layer further in: the honest caller does the right thing and the type system
does not require it. You already know the fix, because you applied it to
authority in turn 2 — make the read produce a privately-minted,
non-constructible value that `resolve_recovery_handle` demands. Then a caller
cannot supply a resolution it did not read.

If you conclude the inner function should not be exported at all, that is an
acceptable alternative — say so explicitly rather than leaving it public.

## 4. The queued adversarial tests (blocking)

From your own list, plus two of mine:

- **Per-axis drift with positive twins.** Now meaningful, because the admission
  cannot be forged. Every one of the eleven axes, each denying for its own
  distinct kind.
- **Foreign branch with an equal version ordinal.** The ordinal must not be
  what distinguishes the branches.
- **Grant revocation → `CurrentPolicyDenied`**, distinguishable from
  `ForeignPrincipal`. These are different facts and must not collapse.
- **Clock-advanced `Expired` terminal.** Expiry as a *terminal path*, not just
  a denial — R8.29 names three terminals and expiry is one.
- **T4b — `RuntimeWithExternalOwner` + `RecordedInverse`.** The one
  configuration where both axes are simultaneously load-bearing: `reconcile`
  must admit (authority axis) while `compensate` must deny (mechanism axis) on
  the *same* installed contract. If anything collapsed to posture-name
  dispatch, this is the only case that reveals it. No other test in the gate
  covers it.
- **Leak detection across all four terminal paths** — consumed, expired,
  disposed, force-terminated. All four, not the three that are easy.

## 5. Counters (verify, then assert)

R8.33 requires 0 basis preparation, 0 digest derivation, 0 digest-text
comparison for handle lookup, provider inquiry, and repeated inspection, and
exactly one identity derivation at mint. You assert inspect-twice at 0/0/0.
Add lookup and provider inquiry, and state the exact values inline rather than
comparing two runs against each other.

## 6. Warnings

The build must be warning-clean when you are done. Two exist today, both from
item 2(a).

## Standard

Everything prior still applies. When you report, list the exact test targets
you ran, by name. Turn 2's report said "trybuild pass" — the new
`application_aftermath_compile_fail` target does pass, but `compile_certification`
does not, and the narrower claim read as the broader one. Naming the target
would have surfaced it.
