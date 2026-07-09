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
