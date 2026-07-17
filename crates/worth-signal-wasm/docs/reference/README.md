# Reference

Use these pages when you already understand the feature and need an exact
answer about entrypoints, support, deployment, results, or compatibility.

## Start With The Question

- **What should I import?** [Package Entrypoints And Runtime Contracts](./package-entrypoints-and-contracts.md)
- **Is this surface stable in my deployment?** [Support Status](./support-status.md)
- **Why did a supported operation return unavailable or denied?** [Typed Results, Denials, And Unavailability](./typed-results-and-unavailability.md)
- **How does runtime construction select worker-first or compatibility?** [Construction API](../api-reference/construction.md)
- **What is on the callable facade and its handles?** [Callable Signals API](../api-reference/callable-signals.md)
- **When is the lower-level surface appropriate?** [Lower-Level Compatibility Surface](../api-reference/compatibility-surface.md)

## Domain Reference

- [Form API](../api-reference/forms.md)
- [Complete Form Export Catalog](../api-reference/form-export-catalog.md)
- [Resource API](../api-reference/resources.md)
- [Resource Family Authoring](../api-reference/resource-family-authoring.md)
- [Resource Line](../api-reference/resource-line.md)
- [API Route Authoring](../api-reference/route-authoring.md)
- [Resource Transfers](../api-reference/resource-transfers.md)
- [Resource Binary And Download](../api-reference/resource-binary-and-download.md)

## How To Read The Reference

The reference uses five product statuses: stable, mixed, deferred,
unsupported, and compatibility-only. A sixth word, `unavailable`, describes a
runtime result rather than product support.

That distinction matters. Exact restore is a stable API, for example, but an
individual restore can be unavailable when the active runtime does not retain
the required same-runtime evidence. Returning unavailable is the supported
behavior; fabricating a plausible restore is not.

The public declarations remain the signature authority. The reference explains
which facade to choose, who owns the state, how the lifecycle behaves, and what
failure means.
