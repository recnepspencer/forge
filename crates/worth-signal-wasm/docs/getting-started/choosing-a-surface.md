# Choose The Right Surface

Use the smallest Worth Signals surface that honestly owns your problem. Moving
up the stack should remove responsibility from application code, not merely add
more nouns.

## Use Core Signals When

- state is browser-local;
- values derive from other values;
- coordinated writes matter;
- a feature needs explicit graph inputs and outputs.

Start with `signals.input`, `signals.computed`, and `signals.transaction`.

## Use Resources When

- values are identified by request parameters;
- loading, freshness, retry, or invalidation matter;
- writes receive server canonicalization;
- optimistic requests may overlap;
- uploads, processing jobs, or downloads have lifecycle state.

A resource line owns those concerns together. Rebuilding them with ordinary
signals usually creates a cache that forgot to call itself a cache.

## Use Forms When

- source truth and draft truth differ;
- validation and visible messages matter;
- readiness, approval, or submission policy matter;
- actions need repeat-attempt and recovery behavior.

A form controller owns form state. It does not replace the authority that
supplied the source value.

## Use Router When

- raw browser location must become an admitted route;
- transitions need pending visibility or recovery;
- history and back behavior must remain explainable;
- forms or resources participate in navigation.

Do not use the router merely to parse a string if the application has no route
authority problem.

## Use Local Truth When

- two browser-local branches edit the same application value;
- disjoint edits should compose automatically;
- overlapping aspects need manual resolution;
- historical inspection must identify the exact basis and decision.

Local Truth is process-local. Use server authority or the wider Worth platform
for durable shared workflows.

## Use The Compatibility Surface When

- migrating lower-level code;
- authoring explicit structural specs;
- using specialist keyed or packed helpers;
- deliberately managing main-thread runtime policy.

Compatibility is not "more real" than the callable surface. It is lower-level
and therefore asks the caller to own more ceremony.

## A Useful Test

Ask: if this layer disappeared, which responsibility would fall back into my
application code?

If the answer is "none," you probably do not need the layer. If the answer is
"we would have to rebuild resource identity, form drafts, route admission, or
merge authority," you have found the right abstraction.

## Related Docs

- [Core Signals](../core/README.md)
- [Resources](../resources/index.md)
- [Forms](../forms/index.md)
- [Router](../router/index.md)
- [Local Truth](../local-truth/README.md)
