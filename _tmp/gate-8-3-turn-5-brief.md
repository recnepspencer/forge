# Gate 8.3 — Turn 5 (verification integrity)

Your turn 4 fix is verified and correct. I read it rather than trusting the
report:

- `WorthQueryAdmittedIdempotencyRead` now carries its
  `WorthQueryApplicationIdempotencyBinding` and no longer derives `Clone`.
- `resolve_recovery_handle` denies with `ForeignIdempotencyRead` when the
  read's binding is not the handle's — correctly distinct from
  `IdempotencyMismatch`, because there the handle is wrong and here the read is.
- `impl Drop` marks the slot `Disposed` while live, so the new denial path does
  not leak — and your negative test asserts `assert_no_live_handles()` to prove
  it, which I had not asked for and which is the right instinct.
- Negative and positive twin both present.

Gate 8.3's product is sound. Two verification-integrity problems remain, and
neither is about the recovery handle. Both are about whether our evidence can
be trusted.

## 1. `worth-query-execution --lib` is not reliably green

I ran it four times. It failed twice:

```
---- domain_computation::primary_graph::application_query::live::lease::
     lifecycle::lifecycle_tests::tests::
     cancellation_and_deadline_terminalize_all_live_resources ----
panicked at .../live/lease/lifecycle_tests.rs:120:18:
called `Result::unwrap()` on an `Err` value:
WorthQueryApplicationLiveOpenDenial { kind: Admission(DeadlineExceeded), ... }
```

**This is not yours and not Gate 8.3's.** Phase 8 never touched
`application_query/live/` — I checked. It is a pre-existing Phase 7 test whose
setup opens a live query under a wall-clock deadline and `.unwrap()`s the
result. Under CPU load the setup exceeds its own deadline before the open
completes, and the test panics.

It matters anyway, for a reason worth stating plainly: your "549 passed" and my
own earlier "547 passed" were both single lucky runs on an idle machine. A
target that is green half the time under load is not evidence, and every gate
from here to 8.6 rests on that target.

Fix it deterministically. The test's intent is to prove that cancellation and
deadline expiry terminalize all live resources — that intent has nothing to do
with how fast the machine is. Options, in order of preference:

1. Drive the deadline from the same injectable runtime clock Gate 8.3 wired for
   expiry, so the test advances time explicitly instead of racing it. This is
   the right answer and you already built the machinery.
2. If the live-lease deadline path cannot yet take an injectable clock, give
   the *setup* a deadline that cannot expire and apply the tight deadline only
   to the phase under test.

Do not fix it by widening the deadline to a bigger wall-clock number. That
converts a fast flake into a slow one.

Confirm the fix by running the target **at least five times** and reporting
every result, not the best one.

## 2. The registry exposes a production-callable reset

```rust
#[doc(hidden)]
pub fn reset_for_integration_test() {
    let mut state = registry().lock()...;
    *state = RecoveryRegistryState::new();
}
```

This wipes the framework's managed-resource registry and is callable from any
consumer crate. `#[doc(hidden)]` hides it from documentation; it is not access
control.

R8.29 makes that registry the framework's authority over live recovery
resources, and R8.65's rule is that test affordances do not live on production
surfaces. A production-callable "erase all handle tracking" is a sharper
version of the Gate 8.1 defect, because calling it silently orphans every live
handle's terminal record — the exact thing your leak tests exist to detect.

I understand why it exists: the registry is process-global, integration tests
live in another crate, and `#[cfg(test)]` does not reach them. That constraint
is real, so pick one:

- **A cargo feature** (e.g. `test-support`) that the bank test target enables
  and production builds never do. Standard, honest, cheap.
- **Scope the registry to the runtime instance** rather than a process global,
  so isolation falls out of construction and no reset is needed.

The second is architecturally better and is what I would want before 8.4 and
8.5 add more registry-touching tests. The first is acceptable if the second is
too large for this turn — but if you take the first, say so explicitly and note
the second as owed, rather than leaving it implied.

`assert_no_live_handles()` is fine to leave public: it is a thin read over
`enumerate_live()`, which R8.29 requires the framework to expose anyway.

## Then close

Update `_docs/WORTH-query/milestone-9.16-runtime-phase-8-closure-ledger.md`.
Add the flaky-test finding as a new `Q8.*` row with its evidence — pre-existing,
Phase 7 origin, surfaced during Gate 8.3 — and record whichever registry
option you took.

Re-run the full named set, including `worth-query-execution --lib` five times.
Report every run. Do not start Gate 8.4.
