# Partial Updates And Derived Views

Use this section when you want one part of a resource to update without
replacing the whole value.

This is also the user-facing home for the feature the runtime calls
`aspects`. You do not need that word to use the feature, but it helps when you
go looking for the exact API later.

## Start Here

- [How Partial Resource Updates Work](./how-partial-resource-updates-work.md)
  The normal mental model for narrow patches and deliveries.
- [Automatic Derived Views](./automatic-derived-views.md)
  Per-item derived views and collection summaries that update automatically.
- [Update One Region, Field, Or Item](./update-one-region-field-or-item.md)
  Choosing the right narrow patch lane.
- [When To Declare Derived Views Explicitly](./when-to-declare-derived-views-explicitly.md)
  When the built-in object-field lane is enough and when it is not.
- [How Partial Updates Affect Caching And Delivery](./how-partial-updates-affect-caching-and-delivery.md)
  What stays stable when only part of the line changes.

## Common Questions

- "How do I update one row without replacing everything?"
  [Update One Region, Field, Or Item](./update-one-region-field-or-item.md)
- "What is an aspect, and do I need to care?"
  [Automatic Derived Views](./automatic-derived-views.md)
- "When do I need explicit derived-view declarations?"
  [When To Declare Derived Views Explicitly](./when-to-declare-derived-views-explicitly.md)

## Deeper History

- [Collections And Delivery](../collections-and-delivery.md)
- [JSON Path Effects](../json-effects.md)
