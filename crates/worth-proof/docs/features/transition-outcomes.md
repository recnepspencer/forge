# Transition Outcomes

## What This Feature Is

Transition outcomes are the result vocabulary for progression in `worth-proof`. They preserve success and non-success categories as distinct typed cases rather than flattening them into one generic error path.

## Why You Use It

- you need to preserve denial separately from deferment
- you need stale and rebind-required to remain visible
- you want success mapping without losing the non-success category

## Stable Entry Points

- `SuccessfulTransitionOutcome<S>`
- `TransitionOutcome<S, D, De, St, R, F>`
- `DenialTransitionOutcome<S, D, F>`
- `DeferredTransitionOutcome<S, D, De, F>`
- `FreshnessTransitionOutcome<S, St, R, F>`
- constructors:
  - `TransitionOutcome::success(...)`
  - `TransitionOutcome::denied(...)`
  - `TransitionOutcome::deferred(...)`
  - `TransitionOutcome::stale(...)`
  - `TransitionOutcome::rebind_required(...)`
  - `TransitionOutcome::failed(...)`
- helpers:
  - `SuccessfulTransitionOutcome::value()`
  - `SuccessfulTransitionOutcome::into_value()`
  - `TransitionOutcome::is_success()`
  - `TransitionOutcome::map_success(...)`

## DX Posture

This feature has a mixed DX story.

- the pleasant checked lane exposes `ProofOutcome` and `ProofOutcomeKind` as narrow inspectors
- raw `TransitionOutcome` remains the semantic source of truth
- when you need full topology or direct pattern matching, prefer `use worth_proof::raw::*;`

## Core Mental Model

The outcome type is part of the law.

The categories mean different things:

- denied
  - progression is not permitted
- deferred
  - progression is not permitted yet
- stale
  - the input is still readable, but freshness is no longer strong enough
- rebind-required
  - semantic rebinding is required before trusted progression resumes
- failed
  - some failure lane outside the previous categories occurred

If you collapse those too early, you lose exactly the information `worth-proof` exists to preserve.

## How It Executes

Outcome surfaces usually appear in this order:

1. a checked transition or composition helper returns `TransitionOutcome`
2. callers inspect or pattern match the exact category
3. callers may `map_success(...)` to continue a success lane without disturbing other cases
4. richer helpers compose outcomes without flattening them

## Small Example

```rust
use worth_proof::TransitionOutcome;

let denied: TransitionOutcome<u64, &'static str> = TransitionOutcome::denied("denied");
assert!(matches!(denied, TransitionOutcome::Denied("denied")));
```

This is the smallest honest example because it shows one explicit non-success lane without pretending all outcomes are interchangeable.

## Real Example

```rust
use worth_proof::{SuccessfulTransitionOutcome, TransitionOutcome};

fn preserve_non_success() {
    let denied: TransitionOutcome<u64, &'static str> = TransitionOutcome::denied("nope");
    let remapped = denied.map_success(|value| value + 1);

    assert!(matches!(remapped, TransitionOutcome::Denied("nope")));

    let success = SuccessfulTransitionOutcome::new(9_u64);
    let widened: TransitionOutcome<u64, &'static str> = success.into();
    assert!(matches!(widened, TransitionOutcome::Success(9)));
}
```

What this shows:

- success mapping touches only the success lane
- non-success categories stay intact
- narrower success-only surfaces can be widened into full outcome surfaces when needed

## How It Relates To Other Features

- Pair this with [Checked Transitions](./checked-transitions.md) because checked progression is built around these categories.
- Pair this with [Preconstruction And Readiness Gates](./preconstruction-and-readiness-gates.md) when the categories first appear before actual progression runs.
- Pair this with [Ready Recipe Join](./ready-recipe-join.md) because joined success lanes short-circuit non-success cases intentionally.

## Inspection And Debugging

- pattern matching is the clearest inspection surface
- the narrower aliases are useful when some categories are impossible by construction
- `map_success(...)` is the right tool when you want to compose success lanes without rewriting the rest of the match

## Anti-Patterns

- Do not flatten `TransitionOutcome` into `Result<T, E>` when stale or rebind-required still matter.
- Do not use `Success` plus stringly typed failure categories when the typed aliases already express the topology.
- Do not call something "failed" when it is semantically denied or deferred instead.

## Current Limits

- generic parameters can get verbose in raw form
- domain crates still choose the actual denial, deferment, or failure payload types
- the crate preserves topology, but it does not impose one universal error payload

## Related Docs

- [Checked Transitions](./checked-transitions.md)
- [Preconstruction And Readiness Gates](./preconstruction-and-readiness-gates.md)
- [Ready Recipe Join](./ready-recipe-join.md)
