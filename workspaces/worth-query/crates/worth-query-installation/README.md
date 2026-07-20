# worth-query-installation

`worth-query-installation` owns portable Query installation meaning. It is the
small package to compile when changing domain package, installed operation,
workflow, conditional-node, or canonical installation contracts.

Use its facade:

```rust
use worth_query_installation::facade::*;
```

Most application and domain consumers should use the same types through
`worth_query::facade::domain` or `worth_query_host::facade::domain`. Import this
crate directly only when working on installation meaning itself.

## Portable Meaning Only

This crate owns:

- domain package definitions and canonical package identity
- typed domain operation definitions and semantic closures
- workflow DAG declarations
- portable conditional-node declarations
- semantic truth dependencies
- validation, admission inputs, conflict denials, and rebuildable installed
  indexes

It does not own:

- executor callbacks
- graph providers
- Runtime Bridge instances
- Signal graphs, nodes, or aspect allocations
- runtime support profiles
- execution receipts or Query consequences

Portable definitions must remain callback-free and runtime-independent.

## Installed Operation Contract

`WorthQueryDomainOperationDefinition<D, O, F>` binds exact domain, operation,
and family marker types to `WorthQueryDomainOperationSemanticClosure`.

The closure states parameters, canonical query and result shape, graph reads,
touches/effects, workflow, conditional nodes, publication, support, terminal
states, failure classes, cost, and deterministic lowering identity.

Use typed `NotRequired` variants for absent capabilities. Provider absence and
empty labels are not semantic declarations.

## Conditional Authoring

`WorthQueryPortableConditionalNodeDeclaration::declare(...)` requires
dependencies, outputs, context, condition, trigger, comparison, artifact
policy, maintenance, and output relationship before `finish()` succeeds.

Dependencies carry Foundational aspect contracts and masks plus Relational
bindings, locality, relevant change kinds, and graph-read role. They never
contain runtime-local Signal allocation.

## Canonicality

Equivalent declaration order converges to one canonical identity. One-field
semantic drift produces a conflict. Derived lookup indexes must be rebuildable
from portable installed artifacts without changing identity or denial outcomes.

## Related Docs

- [Runtime-Installed Domains And Operations](../worth-query/docs/domain-capabilities/runtime-installed-domains.md)
- [Conditional Installed Operations](../worth-query/docs/domain-capabilities/conditional-installed-operations.md)
- [Worth Query Orientation](../worth-query/docs/AI_README.md)
