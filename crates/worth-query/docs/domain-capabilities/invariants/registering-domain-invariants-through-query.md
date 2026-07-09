# Registering Domain Invariants Through Query

## What This Feature Is

This feature gives domains one Query-owned ordinary path for invariant
registration while still lowering into relational invariant authority.

## Why You Use It

- you want geometry-kernel invariants registered through the same public Query
  runtime builder you already use elsewhere
- you have structural authoring legality to enforce â€” which owner kinds may
  contain which child kinds, what may move where, what may reference what,
  what may splice into what â€” and those rules should be registered invariants
  rather than a host-local legality graph beside the runtime
- you do not want every downstream domain importing relational builder plumbing
  directly
- you need registration artifacts that can flow from domain capabilities into
  the ordinary runtime builder

## Stable Entry Points

Contribution lane:

- `worth_query_domain(...).for_intent(...).register_invariant_catalog(...).because(...).materialize()`

Ordinary runtime lane:

- `WorthQueryRuntime::builder().invariant_catalog(...)`
- `WorthQueryRuntime::builder().custom_invariant(...)`
- `WorthQueryRuntime::builder().register_invariant(...)`
- `WorthQueryRuntime::builder().invariant_registration_artifact(...)`

## Core Mental Model

Query owns the public facade. Relational still owns invariant authority.

There are two honest ways to use that:

- register invariants directly through the ordinary Query runtime builder
- materialize a Query invariant-registration artifact through domain
  capabilities, then hand that artifact to the same runtime builder

Invariant content is not limited to geometric publication safety. Structural
authoring legality is the same category: containment and ownership rules,
move and placement eligibility, reference legality, and splice boundaries
for editor-grade authoring surfaces all belong in registered invariant
catalogs. When a registered structural invariant blocks an operation, the
denial arrives as the same typed graph-composition domain-invariant denial
the rest of the runtime emits â€” not as a verdict from a consumer-owned
validation layer.

## How It Executes

1. author an invariant catalog or custom rule
2. optionally materialize a domain capability registration artifact
3. queue the registration on `WorthQueryRuntime::builder()`
4. lower into relational runtime during backend assembly
5. execute with the resulting Query runtime

## Small Example

```rust
let runtime = WorthQueryRuntime::builder()
    .invariant_catalog(invariant_catalog)
    .runtime_bridge(bridge)
    .schema_adapter(schema)
    .write_authority(authority)
    .build_backend_from_parts()
    .build()?;
```

This is the smallest honest example because it uses the ordinary public Query
builder without making relational authority implicit.

## Real Example

```rust
let artifact = worth_query_domain("worth.spatial")
    .for_intent(&declaration)
    .register_invariant_catalog(
        "invariant.edge_split",
        invariant_catalog,
    )
    .because("non-manifold edge splits must be blocked before publication")
    .materialize()?;

let runtime = WorthQueryRuntime::builder()
    .invariant_registration_artifact(artifact)
    .runtime_bridge(bridge)
    .schema_adapter(schema)
    .write_authority(authority)
    .build_backend_from_parts()
    .build()?;
```

That pattern is useful when the domain capability seam already owns the
semantic reason for registration and you want the runtime builder to consume the
result without a second local adapter.

## How It Relates To Other Features

- use [Capability Gaps And Invariant Denials](./capability-gaps-and-invariant-denials.md)
  when the domain needs runtime-facing invariant evidence after registration
- use [Advisory And Violation Contributions](../admission/advisory-and-violation-contributions.md)
  when a specific operation should be advisory or violating even before it
  reaches invariant execution

## Inspection And Debugging

- the runtime builder rejects mixed authority paths such as queued Query-owned
  invariants plus an explicitly supplied relational runtime
- artifact-backed registration and direct builder registration both lower into
  the same relational authority seam

## Anti-Patterns

- importing relational builder APIs as the ordinary downstream path
- combining explicit relational runtime authority with queued Query-owned
  registrations
- teaching registration artifacts as if they were themselves executable
  invariant engines
- maintaining a host-local legality graph â€” ownership/containment edges, move
  eligibility tables, splice-boundary rules â€” beside the runtime and
  pre-validating commands against it, instead of registering those rules as
  domain invariants and consuming the runtime's typed denials

## Current Limits

- Query owns the facade, not invariant semantics
- relational runtime still decides invariant execution
- this feature does not make canonical invariant artifact constructors public

## Related Docs

- [Capability Gaps And Invariant Denials](./capability-gaps-and-invariant-denials.md)
- [Advisory And Violation Contributions](../admission/advisory-and-violation-contributions.md)
- [Writes And Intent Boundaries](../../execution/writes-and-intents.md)
