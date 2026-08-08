# Task Brief: Implement Runtime Phase 8 Gate 8.7 (Safe-Retry Re-Dispatch)

You are implementing one gate of a specified milestone phase in the WORTH
platform. Read this brief fully before doing anything else.

## Mandatory reading order

Read these before you write any code. They are not optional context.

1. `AGENTS.md` — the engineering constitution and hard invariants.
2. `_docs/coding_guidelines/` — read every file. `MENTALITY.md`,
   `arch_laws.md`, `composition_laws.md`, `domain_structure_laws.md`,
   `dx_laws.md`, `perf_laws.md`, `testing_laws.md`.
3. `workspaces/worth-query/crates/worth-query/docs/AI_README.md` — the runtime
   authority model.
4. `_docs/WORTH-query/milestone-9.16-runtime-phase-8.md` — your governing
   specification. **§9 Gate 8.7** is your gate. Also directly binding: §6.4
   (D8 / R8.55), §8 (canonical work), §9 Gate 8.2 and Gate 8.3 (the closed
   gates this corrects), §11 rows 16–18. §5 G10, §5 G11, and §12 constrain
   what you may not do.
5. `_docs/WORTH-query/milestone-9.16-runtime-phase-8-closure-ledger.md` —
   findings **Q8.13** (yours to close) and **Q8.14** (explicitly not yours).

## Mandatory skill

You must follow `skills/implementation-batch/SKILL.md` and execute its four
ordered stages in order:

1. Select the slice — already selected: **Gate 8.7 only**.
2. **Boundary review** — produce the boundary brief before planning.
3. **Implementation plan** — write it before editing any code.
4. **Implement and verify.**

Do not edit code before stages 2 and 3 are complete and stated in your output.

Your code must also satisfy `skills/code-quality-qa/SKILL.md` (composition and
domain topology, the 400-line file cap) and `skills/qa-tests/SKILL.md` (test
and fixture honesty). Read both. Do **not** read or use
`skills/spec-designer/SKILL.md` — the specification is already written and is
not yours to change.

## The finding you are closing

`dispatch_external_effect` has **exactly one call site**:
`worth-query-execution/src/domain_computation/primary_graph/application_attempt/provider_execution/external_dispatch.rs`,
in-process, immediately after commit. Nothing ever re-dispatches.

`safe_retry_recovery_handle`
(`.../application_aftermath/recovery_progression/safe_retry.rs`) returns an
*admission* to retry, performs no dispatch, and is the only one of R8.30's six
transitions with **no production consumer**. The other five are called from
`workspaces/worth-query-bank-world/crates/bank-server/src/estate_progression/recovery.rs`.
Verify both of those claims yourself before you plan; if either is false, say
so and stop.

The reachable failure needs no crash. A transport fault on a live process
yields `WorthQueryExternalDispatchPosture::Unresolved`, a receipt that still
carries its `dispatch_outbox()` record, and a mintable recovery handle. The
handle admits the retry. Nothing retries.

## Layering — get this right before you write anything

An earlier draft of this work was stopped because it got the layering wrong.
Two facts, both verified:

1. **`worth-query-execution` does not depend on `worth-signal`.** Only the
   top-level `worth-query` crate does, and it reaches Signal **through Bridge
   lowering** (see `worth-query/src/domain_installation/conditional_execution/bridge_lowering.rs`
   for the sanctioned pattern). The aftermath lane is currently both
   Bridge-free and Signal-free. **Keep it that way.** Do not import Signal
   retry-policy types, and do not route re-dispatch through the runtime bridge.
2. **§5 G10 / R8.11**: no Signal decision, slot value, or explanation may
   classify an external effect, admit a recovery transition, or authorize an
   inverse. A mechanical residue test already asserts zero `worth_signal` /
   `WorthSignal` in `application_aftermath/**` (`phase8_residue::r8_11_*`).
   Your work must leave that test green.

Re-dispatch is a **host-port** operation, not a policy one. The transport is
already installed on the runtime
(`WorthQueryPrimaryGraphApplicationRuntime::external_effect_transport`), which
is where the existing post-commit dispatch reads it from. Scheduling policy —
backoff, budgets, when the next attempt is due — is **out of scope for this
gate**. Safe-retry is operator-driven and explicit: one handle, one retry,
called by the consumer.

## The shape to build

`resolve` is your template. It is the existing transition that needs the
runtime to perform a privileged operation before the pure transition function
consumes the result. Read `resolve_commit_recovery` in the Bank recovery file
and `resolve_recovery_handle` in `recovery_progression/resolve.rs`, and mirror
that split:

- The **runtime** performs the re-dispatch and returns typed evidence.
- The **transition function** consumes that evidence, re-checks authority, and
  consumes the handle.

Four pieces:

**1. R8.68 — the binding must carry the outbox record.**
`WorthQueryRecoveryHandleBinding` (`recovery_handle/binding.rs`) carries
`correlation: Option<ExternalEffectCorrelationIdentity>` but **not** the
correlation family. `dispatch_external_effect` derives its request from
correlation *and* family (`WorthQueryExternalDispatchRequest::for_correlation`),
so the binding as it stands cannot form a dispatch request at all. Bind the
co-committed `WorthQueryDispatchOutboxRecord` — that is what R8.28's
"correlation evidence" axis was always naming. Preserve the existing
`correlation()` accessor's meaning for current callers, and keep the axis-probe
fixture (`#[cfg(test)] axis_probe`) coherent so the per-axis drift proofs still
bind every axis.

**2. R8.67 — a runtime re-dispatch method, one classification site.**
Add it beside the existing dispatch code, in the same impl that owns the
transport. It must call the **same** `dispatch_external_effect`.
`classify_observation` stays the single place a transport observation becomes a
typed posture. A second classification path would give R8.24's fault taxonomy
two owners — that is exactly the "second, weaker effect lane" this phase exists
to prevent.

**3. R8.66 / R8.69 — `safe_retry` consumes proof, not permission.**
`safe_retry_recovery_handle` must take the completed
`WorthQueryExternalEffectDispatch` as an argument and surface it on the
admission. A caller must not be able to reach the transition without a real
dispatch having happened. Authority is established first, through the existing
`admit_recovery_effect_authority` path — a handle minted before the fault
authorizes nothing by itself (R8.31). **Order matters and is provable: the
denial cases must deny before any transport call is made.**

Note the linchpin already in place: `WorthQueryAdmittedApplicationOperation::mint`
is `pub(super)`, so a test cannot forge an admission. Do not weaken that to
make anything convenient.

**4. Q8.13's other half — give it a Bank consumer.**
Add `safe_retry_commit_recovery` to
`bank-server/src/estate_progression/recovery.rs`, following the five methods
already there. A transition with no consumer is how this defect survived six
gate closures.

## R8.71 — the durability limit is typed, not implied

We are in-memory today; `worth-store` is still being built. Publish the limit
as an explicit posture, exactly as R8.12 does with `StoreCapabilityRequired`.
Do not imply durability we do not have, and **do not add a `worth-store`
dependency to any Query crate.**

**Explicitly out of scope: the crash-recovery drain.** Scanning committed
outbox rows after process death is **Q8.14, deferred to Store integration**. It
is unreachable in-memory — process death takes the outbox row with the process
— and building it now means guessing Store's durability semantics. Do not build
a scanner, a sweeper, a relay, a pending-row query, or a placeholder for one.
If you find yourself needing one to make safe-retry work, you have the wrong
design: safe-retry starts from a **live handle**, not from a table scan.

## Hard boundaries

- **Gate 8.7 only.** No scheduling policy, no backoff, no retry budgets, no
  drain, no Store.
- Do not modify any file under `_docs/`. The specification is authoritative and
  is not yours to edit. If you believe it is wrong or incomplete, **stop and
  report the conflict** instead of working around it or amending it.
- Do not weaken a type, widen an API, add a fallback, or add a compatibility
  shim to make something compile. If the honest path is blocked, report the
  blocker.
- **No `#[allow(...)]` in new or touched code.** A suppression carrying a
  comment that asserts the item is used is how three earlier defects hid.
- Respect the 400-line file cap. Split touched oversized files.

## Tests — §11 rows 16–18, all provable in-memory today

Use the real `bank-external-rail` process, as Gate 8.2's scenarios do. An
in-process fake sharing the runtime's truth source does not close this gate.

1. **Faulted dispatch, then safe-retry.** Commit with the transport faulting →
   the outbox row commits and no completion posture is published. Mint a
   handle, safe-retry through production admission → the effect escapes.
   **Assert at the rail:** exactly one attempt across both attempts combined.
2. **Retry of an already-completed effect** → no second emission; the rail's
   attempt count is unchanged. This is the row that matters most.
3. **Denials observe nothing at the rail.** Expired handle, terminal handle,
   foreign principal — three distinct denial causes, and the rail records no
   attempt for any of them. Asserting the denial alone is not enough; assert
   the rail stayed untouched, which is what proves authority precedes dispatch.
4. **Zero-cost positive twin (R8.4):** an operation declaring no external
   effect leaves the retry path nothing to find, with the transport live.
5. **Compile-fail evidence** for the R8.66 construction boundary: safe-retry
   must be unreachable without a real dispatch. Put it in
   `worth-query-certification`, with a positive twin.

Every negative case needs a positive twin (`testing_laws.md`). Do not write a
test whose assertion is something the compiler already guarantees — asserting
that N enum variants are distinct is a tautology, not a proof.

## Verification — the standing set, every target by name

Phase 8 had two red certification targets that a narrower "trybuild passes"
report concealed. Run and report **all** of these:

```
cargo test -p bank-server --test ordinary_mutations
cargo test -p worth-query-certification --test compile_certification
cargo test -p worth-query-execution --lib          # five runs, report every one
cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .
cargo run --manifest-path tools/agent-context/Cargo.toml -- check
scripts/ci/check_workspace_rust_line_caps.sh
```

Plus `RUSTFLAGS=-Dwarnings cargo check`, the Query consumer targets
(`installed_operating_world`, `public_declarative_journeys`,
`runtime_public_journeys`), and `cargo test --workspace --no-fail-fast` in
**both** workspaces.

Two reading rules that cost this phase more than the missing commands did:

1. **Report the target you ran, by name.** "trybuild passed" was true of a
   narrow new target while a broader one was red.
2. **A single green run of a timing-sensitive target proves nothing.** Report
   every run, not the best one.

## Reporting

End your turn with the report `implementation-batch` Stage 4 requires: the
boundary reviewed, the slice built, material files changed, verification
results including anything you did not run, and remaining work. Be explicit
about what you did not finish or could not prove. An honest incomplete report
is correct; a confident false one is the worst possible outcome.

You will be reviewed after this turn by an auditor following
`skills/qa-loop/SKILL.md`, who will read your diff directly and attempt to
falsify both your implementation and your report.
