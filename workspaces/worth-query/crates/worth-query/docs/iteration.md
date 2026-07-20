# Query Iteration

Choose the package that owns the edit. Declaration and installation are the
fastest loops and neither selects the remaining engine or cold certification:

```text
cargo check -p worth-query-declaration
cargo test -p worth-query-declaration

cargo check -p worth-query-installation
cargo test -p worth-query-installation
```

For behavior still in the remaining engine, run one command that answers the
current question. A check is useful while editing signatures; a test command
already performs the required build:

```text
cargo check -p worth-query --tests
cargo test -p worth-query
cargo test -p worth-query --lib domain_installation
```

Run the retained public compiler-boundary portfolio only when a type or facade
boundary changes:

```text
cargo test -p worth-query-certification --test compile_certification
```

Run the complete cold lane at milestone closeout:

```text
cargo test -p worth-query-certification -p worth-query-replay
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
- before the package extraction, the final ordinary library suite contained
  2,981 tests and took `118.14 s` of warm test time
- the Worth UI-owned Query binding suite takes `47.35 s` after a rebuild, of
  which `3.32 s` is test and doctest execution

Current owner-local and remaining-engine observations are recorded in
Milestone 9.13.1 Phase 8 closure evidence. They are review observations, not
portable thresholds. The remaining broad runtime boundary is a package-
selection problem owned by Milestone 9.13.2, not a reason to add a custom test
runner or another fixture-selection system.

The Phase 8 same-machine observations were:

- declaration warm check/test: `0.74 s` / `0.33 s`
- declaration package-invalidated check: `1.07 s`
- installation warm check/test: `0.36 s` / `0.22 s`
- installation package-invalidated check: `0.35 s`
- complete remaining engine after those invalidations: `43.55 s`
- explicitly selected cold compiler/replay lane after invalidation: `62.98 s`
