# Query Iteration

Run ordinary Query behavior without compiler fixtures:

```text
cargo test -p worth-query --lib
```

Run the complete public compiler-boundary portfolio when a type or facade
boundary changes:

```text
cargo test -p worth-query --test compile_certification
```

The compiler target batches only selected compile-fail fixtures. Positive
facade journeys belong in ordinary integration tests or doctests; do not add a
compile-pass trybuild loop, a second trybuild target, or trybuild work to the
library suite.

## Observed On 2026-07-18

These elapsed times are development observations, not CI budgets or golden
performance tests:

- the former warm compiler-fixture portfolio took about `399.2 s`
- the retained compiler-certification target takes `4.34 s` warm (`64.15 s`
  after the final production cleanup rebuild)
- the final ordinary library suite contains 2,981 tests; the last all-up warm
  observation, immediately before deleting one zero-work matrix meta-test, was
  `118.14 s` test time (`118.54 s` wall with a `0.29 s` no-op build). Its
  previous warm observation was `126.6 s`
- the Worth UI-owned Query binding suite takes `47.35 s` after a rebuild, of
  which `3.32 s` is test and doctest execution

The compiler cut therefore removed the acute iteration failure. It did not
materially change the ordinary behavioral floor. That remaining floor is a
package-selection problem owned by Milestone 9.13.2, not a reason to add a
custom test runner or another fixture-selection system.
