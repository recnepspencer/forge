# Aspects And Dependencies

Dependencies are about who cares about whom.

Aspects are about what part changed.

That matters when one node can change in more than one way.

## Real-World Meaning

Imagine a product node that can change in different ways:

- price changed
- inventory changed
- title changed

Not every downstream node cares about all three.

A checkout summary may care about price and inventory.
A search result card may care about title and price.

Aspects let you express that difference instead of treating every change like a
full reset.

## Main Surfaces

- `Aspect`
- `DependencyEdge`
- `mark_changed(...)`
- `mark_changed_with_regions(...)`
- `PartitionSubscription`

## Practical Rule

Use aspects when you want the runtime to know what kind of change happened.

Use changed regions or partition-aware subscriptions when part of a result can
change without forcing all downstream work to act like the whole thing changed.

## Runtime-Local Identity

Signal `Aspect` values are local slots within one installed graph and node.
They do not replace portable Foundational aspect contracts or Relational aspect
bindings.

When Query hosts conditional operations, Runtime Bridge installs the exact
correspondence from semantic dependencies to these local slots. Domain packages
never author or persist Signal aspect numbers.

See [Conditions And Comparators](../reference/conditions-and-comparators.md).
