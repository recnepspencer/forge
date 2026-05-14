# Aspects Reference

## What This Feature Is

Aspects let one signal carry multiple semantic change lanes.

Instead of one broad "this changed" channel, aspects let the runtime track
which kind of change happened and let downstream definitions subscribe only to
the lanes they care about.

## Why You Use It

- distinguish change kind on one value
- keep derivation more precise under multi-lane state
- reduce unnecessary downstream reevaluation
- make version and invalidation reporting more specific
- express explicit aspect-aware recipes on the spec and compatibility lanes

## What Distinguishes This

Aspects let one node carry multiple semantic change lanes inside the runtime
itself.

That is different from most frontend state tools:

- Zustand can model multiple concerns, but it does not give the runtime a
  first-class semantic lane model for invalidation and dependency tracking
- Angular signals give you fine-grained reactive values, but they do not make
  "change kind" a runtime-owned dimension on one signal

What this enables here is:

- one value with multiple invalidation lanes
- dependency contracts that can subscribe to only the lanes they actually need
- version and diagnostics surfaces that can explain which semantic lane moved

## Stable Entry Points

Main app surface:

- `signals.input(value, { producesAspects })`
- `tx.setWithAspects(...)`
- `tx.setWithRegionsAndAspects(...)`

Explicit named/spec lane:

- `signals.spec.input(...)`
- `signals.spec.computed(...)`
- `signals.spec.output(...)`

Compatibility lane:

- `SourceSpec.producesAspects`
- `RecipeSpec.producesAspects`
- aspect-aware read specs
- lower-level aspect-targeted invalidation helpers

## Core Mental Model

Aspects shape derivation and invalidation, not observation identity.

That split is important:

- aspects decide which semantic lane advanced
- subscriptions and observation are still node-scoped by default

So aspects make callback or recipe dependency tracking more precise without
forcing the ordinary observation model to become aspect-scoped everywhere.

## How It Executes

At write time:

- a source may declare which aspects it produces
- writes may mark only the aspects that changed

At read time:

- explicit spec reads can subscribe to one aspect or several aspects
- downstream reevaluation only happens for matching lanes

If you do nothing aspect-specific, the runtime falls back to default aspect `0`.

## Small Example

```ts
const sensor = signals.input(10, {
  producesAspects: [1, 2],
});

signals.transaction((tx) => {
  tx.setWithAspects(sensor, 11, [2]);
});
```

This is the smallest honest example because it shows:

- aspect-producing input state
- aspect-targeted mutation

## Real Example

```ts
const sensor = signals.spec.input("sensor", 10, {
  producesAspects: [1, 2],
});

const display = signals.spec.output("display", {
  reads: [
    {
      id: "sensor",
      aspect: 1,
    },
  ],
  expr: { kind: "read", id: "sensor" },
});

const summary = signals.spec.computed("summary", {
  reads: [{ id: "sensor", aspect: 1 }],
  expr: { kind: "read", id: "sensor" },
  producesAspects: [7],
});

signals.transaction((tx) => {
  tx.setWithRegionsAndAspects(sensor, 42, [], [1]);
});
```

What this gives you:

- explicitly named aspect-producing source state
- explicit aspect-filtered spec reads
- explicit produced-aspect declaration on derived named definitions
- targeted invalidation through the normal transaction model

## How It Relates To Other Features

- Use callback-first authoring when ordinary dependency capture is enough.
- Use `signals.spec.*` when you need explicit aspect-filtered read contracts.
- Use the compatibility lane when you need lower-level structural aspect work.
- Pair aspects with diagnostics when you need more precise version or
  invalidation explanation.

## Observation And Debugging

Observation is still node-scoped by default.

That means:

- `watch(...)` and `effect(...)` observe committed node truth
- aspects influence whether derivation reruns
- aspects do not automatically turn observation into per-aspect subscriptions

For version reporting, use the specialist surface:

- `specialist().read_versions(...)`

That can expose both:

- `version`
- `aspectVersions`

## Anti-Patterns

- reaching for aspects when separate nodes would express the truth more clearly
- expecting ambient callback closure reads to become aspect-aware automatically
- assuming node-level observation becomes per-aspect observation by default
- using the explicit spec lane when ordinary callback authoring is already
  sufficient

## Current Limits

- explicit aspect-filtered derived reads live on the named/spec or
  compatibility lanes
- ordinary callback-first app code still reads whole handles, not explicit
  aspect contracts
- if you omit aspect metadata, default aspect `0` remains the fallback

## Related Docs

- [App Surface Overview](./overview.md)
- [Compatibility Surface](../api-reference/compatibility-surface.md)
- [Diagnostics And History](./diagnostics-and-history.md)
