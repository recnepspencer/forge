# Using Resources In Routes

Use this section when a route declares resources and you need to understand the
resource side of route prefetch, warmup, or admitted-route reuse.

## Start Here

- [Declare Route Resources](./declare-route-resources.md)
  Bind a real resource family to a route.
- [Prefetch And Warmup Route Resources](./prefetch-and-warmup-route-resources.md)
  Materialize route-local lines before admission without inventing a second
  cache.
- [Read Projected And Admitted Resource Capabilities](./read-projected-and-admitted-resource-capabilities.md)
  Understand the projected/admitted route capability surfaces that expose those
  lines.

## Main Mental Model

The route does not own a second resource runtime.
It owns a declaration that resolves route truth into one real resource family
line.

## Related Docs

- [Router Route Resources](../../router/resources/README.md)
- [Fetching Data](../fetching/README.md)
- [Inspecting And Debugging Resources](../debugging/README.md)
