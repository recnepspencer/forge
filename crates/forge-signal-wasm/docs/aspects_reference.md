# Aspect Reference

`forge-signal-wasm` supports real Forge Signal aspects on the web surface.

That means web consumers are not limited to a single "this node changed"
channel. Nodes can declare multiple semantic aspects, reads can subscribe to
only the aspects they care about, and writes/invalidation can mark only the
aspects that actually changed.

## Why Aspects Matter

Aspects let the runtime distinguish change kind, not just changed value.

That matters when a node represents multiple semantic lanes, for example:

- `layout` vs `content`
- `price` vs `inventory`
- `geometry` vs `lighting`
- `visible rows` vs `selected rows`

Without aspects, every change becomes one broad invalidation channel.

With aspects:

- downstream nodes can ignore irrelevant churn
- node-level watchers/effects stay simpler because derivation is already more
  precise
- diagnostics and version reporting can tell you which semantic lane advanced

That is one of the important differences between Forge Signal on the web and a
single-channel store like Zustand or lightweight frontend signal systems that
do not carry semantic change kind through the runtime itself.

## App-First Shape

### Input Aspects

`input(...)` accepts optional input options:

```ts
const sensor = signals.input(10, {
  id: "sensor",
  producesAspects: [1, 2],
});
```

If you omit `producesAspects`, wasm preserves backwards-compatible default
single-aspect behavior through aspect `0`.

### Aspect-Filtered Reads

Callback-first `computed(() => ...)` and `output(() => ...)` stay the normal
product lane when plain callable signal reads are enough.

When you need explicit aspect-filtered read contracts, use the spec lane:

```ts
const display = signals.outputSpec("display", {
  reads: [
    {
      id: "sensor",
      aspect: 1,
    },
  ],
  expr: { kind: "read", id: "sensor" },
});
```

Object-form reads also allow:

```ts
{
  id: "sensor",
  aspects: [1, 2],
}
```

### Produced Aspects On Derived Nodes

Produced-aspect declarations on derived nodes also belong on the explicit spec
lane today:

```ts
const summary = signals.computedSpec("summary", {
  reads: [{ id: "sensor", aspect: 1 }],
  expr: { kind: "read", id: "sensor" },
  producesAspects: [7],
});
```

### Aspect-Targeted Transactions

`SignalsTransaction` supports aspect-targeted writes:

```ts
signals.transaction((tx) => {
  tx.setWithAspects(sensor, 99, [2]);
});
```

Or with changed regions:

```ts
signals.transaction((tx) => {
  tx.setWithRegionsAndAspects(sensor, 42, [], [1]);
});
```

## Compatibility Surface Shape

Lower-level compatibility specs are also aspect-aware:

- `SourceSpec.producesAspects`
- `KeyedSourceFamilySpec.producesAspects`
- `RecipeSpec.producesAspects`
- `KeyedRecipeFamilySpec.producesAspects`
- `RecipeReadSpec` object reads can select `aspect` or `aspects`
- `RecipeFamilyReadSpec` uses `aspects: { aspect?, aspects? }`

Compatibility writes and invalidation are also explicit:

- `setKeyedWithAspects(...)`
- `markChanged(...)`
- `markChangedWithRegionsAndAspects(...)`
- `markKeyedChanged(...)`

## Observation Model

Subscriptions remain node-scoped by default.

That is intentional.

The runtime uses aspects to decide whether downstream derivation should react.
`watch(...)` and `effect(...)` still observe committed node truth, not
individual aspect lanes.

So the architectural split is:

- aspects shape derivation and invalidation
- node-level observation shapes delivery

This keeps the default app model simple while still letting wasm use the real
Forge Signal aspect substrate.

## Version Reporting

`specialist().read_versions(...)` now exposes both:

- `version`
- `aspectVersions`

`version` preserves the default public summary lane.
`aspectVersions` exposes per-aspect advancement for multi-aspect nodes.

## Practical Guidance

- Prefer callback-first authoring for ordinary app code.
- Switch to `computedSpec(...)` or `outputSpec(...)` when you need explicit
  aspect-filtered reads or produced-aspect declarations on derived nodes.
- Callback tracking follows callable signal reads only. Ordinary closure
  variables are not reactive dependencies.
- If the distinction is part of app truth, model it with aspects or separate
  nodes.
- If the node is still one coherent public thing, keep `watch(...)` and
  `effect(...)` node-scoped.
- If you do nothing aspect-specific, wasm keeps the previous aspect `0`
  behavior for backwards compatibility.
