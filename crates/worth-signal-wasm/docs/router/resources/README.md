# Route Resources

Route resources bind route params to native resource-family lines. The router
does not own another cache. Projection can warm or prefetch a line; admission
hands the same line to the admitted route.

Start with [Route Resource Declarations](./route_resource_declarations.md), then
read:

- [Projected Resource Capabilities](./projected_resource_capabilities.md)
- [Admitted Resource Capabilities](./admitted_resource_capabilities.md)
- [Resource Prefetch](./resource_prefetch.md)
- [Resource Warmup](./resource_warmup.md)
- [Warmup Ingress](./warmup_ingress.md)

Prefetch artifacts own explicit resource lifecycles. Dispose them when the
preview ends; retained admitted lines continue through the resource system.
