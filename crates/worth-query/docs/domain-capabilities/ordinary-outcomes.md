# Ordinary Outcomes

## What This Feature Is

`WorthQueryOrdinaryOutcome<T>` is the shared bound/stopped result vocabulary for
domain-capability operations that intentionally expose an outcome lane. It
preserves a successful artifact or a typed stop without inventing a local
status enum.

## Why You Use It

- keep success and typed stop posture in one inspectable result
- pass a stopped operation to the recovery boundary without flattening it
- share outcome handling across contribution, grouped, continuation, and
  Signal-compatibility orchestration

## Stable Entry Points

- `WorthQueryOrdinaryOutcome::{Bound, Stopped}`
- `WorthQueryOrdinaryOutcome::{bound, stop, into_bound}`
- `WorthQueryInstalledDomainDeclarationContext::recover_from_outcome(...)`
- `orchestrate_declaration_with_contributions_outcome(...)`
- `orchestrate_signal_compatibility_outcome(...)`

Declaration-entry convenience methods that return `Result` should keep their
typed error. Convert to an ordinary outcome only when the owning public API
explicitly exposes that lane.

## Core Mental Model

An ordinary outcome does not erase the boundary. `Bound` retains the public
artifact; `Stopped` retains the stop that explains why no artifact was bound.

## How It Executes

1. Execute through the installed declaration context.
2. Preserve the returned ordinary outcome.
3. Branch on `bound()` or `stop()` when the caller needs immediate handling.
4. Ask `recover_from_outcome(...)` for a typed recovery brief when remediation
   is needed.

## Small Example

```rust
let outcome = context.orchestrate_signal_compatibility_outcome(input);

if let Some(bound) = outcome.bound() {
    use_compatibility(bound);
}
```

## Real Example

```rust
let outcome = context.orchestrate_declaration_with_contributions_outcome(input);

match (outcome.bound(), context.recover_from_outcome(&outcome)) {
    (Some(bound), _) => publish(bound),
    (None, Some(recovery)) => schedule(recovery.next_action()),
    (None, None) => retain_for_inspection(outcome),
}
```

## How It Relates To Other Features

- [Recovery Boundary](./recovery-boundary.md) derives actionable next steps.
- [Contribution-Composed Orchestration](./contribution-composed-orchestration.md)
  exposes an ordinary outcome lane.
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  exposes an ordinary outcome lane.

## Inspection And Debugging

Inspect the retained stop, linked artifact identities, and installed authority.
Do not infer the stop class from display text.

## Anti-Patterns

- converting the outcome to a boolean
- replacing typed stops with a local retry enum
- manufacturing a bound value from receipt or digest fields

## Current Limits

Not every domain-capability method returns `WorthQueryOrdinaryOutcome`; many
return a boundary-specific `Result` or checked enum. Preserve the type selected
by the owning API.

## Related Docs

- [Domain Capabilities](./README.md)
- [Runtime-Installed Domains](./runtime-installed-domains.md)
- [Recovery Boundary](./recovery-boundary.md)
