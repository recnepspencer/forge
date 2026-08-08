# Gate 8.3 — Turn 2

I re-ran the audit against a closure ledger written before your turn 1 existed.
Your self-report was accurate and your "still open" list is real. But the list
is incomplete in a way that matters: **everything you listed is test coverage,
and the actual blocker is the shape of the API those tests would be written
against.** Writing them now would bake the defect in behind eleven green tests.

Read this whole brief before changing anything.

## What is genuinely good — do not disturb it

These are proved and I re-verified each by reading the code, not your report:

- **C1 / R8.62.** `WorthQueryApplicationCommitAuthorityBinding::from_admission`
  derives all three fields from the admitted operation. The constructor is
  `pub(in crate::domain_computation::primary_graph)`, fields are private with
  accessors. This is exactly the "derived, not asserted" standard the brief
  asked for.
- **Linearity.** `WorthQueryRecoveryHandle` has no `Clone`, no `Copy`;
  `consume(mut self, ...)` takes ownership; every terminal transition takes the
  handle by value. A double transition is genuinely unrepresentable. This is
  structural enforcement, which is what the gate wanted.
- **Six separate transition functions, no mode parameter.** R8.17's shape is
  not reproduced.
- **The two axes are genuinely consumed.** `compensate_recovery_handle` matches
  on `aftermath.mechanism()`; `reconcile_recovery_handle` matches on
  `aftermath.authority()`. Different axes, correctly. This is the sharpest
  available evidence that Gate 8.1's model was used rather than stored, and you
  got it right.
- **R8.64.** `phase8_cross_gate/world.rs` spawns the real `bank-external-rail`
  process via `spawn_rail()` and holds `RailProcessHandle`. The lost-response
  scenario runs through the real process. That is a real cross-gate proof, not
  a stub.

## The blocker: R8.31 is inverted

R8.31 says every transition that can produce effect authority **re-establishes
current provider truth and current application authority first**.

`require_fresh_effect_authority` re-establishes nothing. It receives
`WorthQueryRecoveryFreshAuthority`, a struct of public fields, and reads:

```rust
pub capability_currently_grants: bool,
pub disclosure_admitted: bool,
```

The caller supplies the authority decision as a boolean. There is no capability
admission, no policy evaluation, no provider inquiry, and no read of current
truth anywhere in the module. `check_binding_axes` then compares the handle's
eleven bindings against **eleven more caller-supplied values** in the same
struct.

Three consequences, in increasing order of seriousness:

1. **The eleven drift attacks you have queued would be vacuous.** The test
   constructs both the handle and the `FreshAuthority` it is compared against.
   A passing drift test proves the test corrupted a field on purpose. It cannot
   fail for an adversarial reason because there is no adversary — there is no
   independent source of truth to disagree with.

2. **`current_policy_revocation_after_mint_denies_for_current_policy` does not
   test what its name says.** `fresh_authority_for(...)` takes
   `capability_currently_grants: bool` as a parameter from the test. Passing
   `false` and asserting denial tests the `if !flag` statement. Nothing was
   revoked and nothing was re-admitted.

3. **Nothing in production calls any of this.** `mint_recovery_handle` and all
   six transitions have exactly one caller in the repository: the test. The
   only construction site of `WorthQueryRecoveryFreshAuthority` is
   `phase8_cross_gate/world.rs:96`. Gate 8.3's product is currently a
   correctly-shaped module that the runtime never invokes.

Point 3 is why point 1 is not merely a testing gap. There is no production path
that would supply real current truth, so the boolean is not a seam awaiting a
caller — it is the design.

## What turn 2 must do, in this order

**Do not start with the missing tests.** Fix the API first, or you will write
eleven tests that must then be rewritten.

### 1. Make current authority impossible to assert

`capability_currently_grants: bool` and `disclosure_admitted: bool` must go.
Replace them with evidence that can only exist if admission actually ran —
the same discipline you already applied correctly to C1.

The runtime already has the machinery: capability admission, policy evaluation,
and disclosure admission all produce typed, privately-constructed proofs today
(Phase 7's `R7.6`/`R7.7` access proofs and disclosure admission are the
precedent). A transition should require one of those values, not a description
of one. If a caller cannot fabricate the proof, the drift attacks become real
tests, and the same test file becomes adversarial evidence instead of
self-confirmation.

Whatever you choose, the standard is: **a test must not be able to construct a
passing authority without going through the production admission path.**

### 2. Make the compared truth independent of the caller

`check_binding_axes` should compare the handle against truth the *runtime*
establishes at transition time, not against a struct the caller filled in.
Where an axis genuinely can only come from the request (the branch being
operated on, say), that is fine — but it must be the request's real value as
the runtime sees it, not a field the test set.

The distinction to hold onto: the handle says what it was bound to; the runtime
says what is true now; the check compares those two. A caller-supplied struct
collapses both sides into one voice.

### 3. Wire it to production

At least one real path must mint a handle and drive a transition. Until the
runtime calls this module, R8.31's "re-establishes" has no implementation site
and R8.33's counter claims measure a path nothing takes.

This is also what makes the §10.4 world-construction authority meaningful:
`phase8_cross_gate/world.rs` is a good test-scope world builder, but a world
builder that assembles authority by hand is only honest if production assembles
it the same way. Right now there is no production assembly to match.

### 4. Wire expiry into the lifecycle

`evaluate_expiry` carries `#[allow(dead_code)]`. R8.29 says a handle is
"consumed, expired, or disposed" — expiry is one of three terminal paths and it
is currently unreachable. A dead function is not a lifecycle branch. Once it is
wired, M2 and M3 become testable: M2 that no caller can supply a sample or
choose the evaluation moment, M3 that every expiry decision records its exact
sample.

### 5. Then the tests you already listed

Per-axis drift attacks with positive twins; already-completed resolve twin
through an admitted graph read; expiry M2/M3; compile-fail for the cloned
handle and duplicate transition; foreign principal / foreign runtime /
foreign-branch-equal-ordinal; leak detection across all four terminal paths
(consumed, expired, disposed, force-terminated).

Two notes on these:

- The compile-fail cases for clone and duplicate transition should already hold
  structurally — write them as `trybuild` cases so the guarantee is mechanical
  rather than incidental.
- The seven denial causes must stay distinguishable. `compensate` and
  `reconcile` both currently return
  `WorthQueryRecoveryHandleDenialKind::TransitionNotAdmitted`. That is one
  cause for two different axis failures. Decide deliberately: either they are
  genuinely the same fact, and say so, or they are two causes and need two
  kinds.

## Also worth fixing while you are here

**Q8.3, your own residual.** You noted Compensation and Reconciliation now
require `ExternalEffectPostureEvidence` but that earlier ladder postures remain
constructible inside `external_effect`. That is honest and it is progress.
Finish it or bound it in writing — a named, dated note stating which postures
retain internal constructibility and why is acceptable; silence is not.

## Standard

Everything from the turn 1 brief still applies: 400-line cap, `boundary-check`,
`agent-context`, Gate 8.1 and 8.2 suites green, positive twin for every
negative case, no `String`-keyed identity.

Report honestly again — your turn 1 report was accurate about what you had
built, and that made this audit fast. The thing to add next time is the
question I had to ask on your behalf: *who in production calls this, and where
does the truth it checks against actually come from?*
