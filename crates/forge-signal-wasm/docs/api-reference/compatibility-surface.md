# Compatibility Surface Reference

## What This Feature Is

The compatibility surface is the lower-level runtime-facing lane in
`forge-signal-wasm`.

It exposes `SignalApp`, `SignalRuntime`, lower-level definition registration,
keyed-family helpers, aspect-aware reads, and lower-level diagnostics/history
doors.

This surface is supported, but it is intentionally secondary to the main
`createSignals()` app lane.

## Why You Use It

- port older wasm consumers
- work with lower-level source/recipe definitions directly
- use keyed-family or packed-grid helpers the app surface does not foreground
- control explicit aspect declarations and aspect-targeted invalidation
- consume lower-level runtime artifacts intentionally

## What Distinguishes This

This surface gives you direct access to the lower-level structural runtime
shape instead of the newer handle-first app lane.

What distinguishes it is not “more power” in some vague sense. It is that this
lane exposes:

- explicit named source and recipe definitions
- keyed-family and packed-grid helpers
- lower-level aspect-aware invalidation
- runtime policy and lower-level artifact doors

That makes it useful for migration and specialized lower-level consumers, even
though it is not the recommended starting point for normal browser app code.

## Stable Entry Points

The compatibility surface is available only from an explicitly constructed
compatibility root:

```ts
const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});
```

From that explicitly compatibility-owned root:

- `signals.compatibilityApp()`
- `signals.compatibilityRuntime()`

Within those surfaces:

- definition registration
- lower-level reads
- keyed-family helpers
- aspect-aware writes and invalidation
- lower-level diagnostics/history/adapters

## Core Mental Model

This is still the same runtime truth model.

The compatibility surface is not a separate engine. It is a lower-level
doorway into the same runtime.

So the rule is:

- use the main app surface for ordinary app code
- use the compatibility surface when you truly need lower-level runtime shapes

If you are starting a normal browser app today, begin with:

```ts
const signals = await createSignals();
```

Do not assume a worker-first root exposes `signals.compatibilityApp()` or
`signals.compatibilityRuntime()`. Those compatibility doors belong to the
explicit `mainThreadCompatibility` construction lane.

## How It Executes

The compatibility surface registers and reads lower-level definitions directly:

- sources
- recipes
- source families
- recipe families

It also exposes aspect-aware invalidation and keyed-family helpers directly.

These operations still feed the same underlying runtime truth as the app
surface. They do not create their own semantic model.

## Small Example

```ts
const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});
const app = signals.compatibilityApp();

const count = app.source({
  id: "count",
  value: 1,
});

console.log(app.read("count"));
```

This is the smallest honest example because it shows the compatibility posture
directly:

- explicit named definition
- lower-level registration
- lower-level read

## Real Example

```ts
const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});
const runtime = signals.compatibilityRuntime();

runtime.define_source({
  id: "inventory",
  value: {
    quantity: 10,
    reserved: 2,
  },
  producesAspects: [1, 2],
});

runtime.define_recipe({
  id: "availableInventory",
  reads: [
    { id: "inventory", aspect: 1 },
    { id: "inventory", aspect: 2 },
  ],
  expr: {
    kind: "call",
    fn: "subtract",
    args: [
      { kind: "read", id: "inventory" },
      { kind: "value", value: 2 },
    ],
  },
});

runtime.markChanged("inventory", [1]);

console.log(runtime.read("availableInventory"));
```

What this gives you:

- explicit structural naming
- direct aspect-aware definition work
- lower-level mutation and invalidation
- the same runtime truth model the app surface uses

## `SignalApp`

Use `signals.compatibilityApp()` when you want the lower-level app-oriented
compatibility door:

```ts
const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});
const app = signals.compatibilityApp();
```

Important capability families on `SignalApp`:

- definition registration:
  - `source(spec)`
  - `recipe(spec)`
  - `source_family(spec)`
  - `recipe_family(spec)`
- reads:
  - `read(id)`
  - `read_many(ids)`
  - `read_keyed(familyId, key)`
  - `read_keyed_many(familyId, keys)`
- keyed grid / packed field helpers
- writes and invalidation
- diagnostics/history/specialist/adapters access

## `SignalRuntime`

Use `signals.compatibilityRuntime()` when you want the runtime-oriented lower
door:

```ts
const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});
const runtime = signals.compatibilityRuntime();
```

Important capability families on `SignalRuntime`:

- definition registration:
  - `define_source(spec)`
  - `define_recipe(spec)`
  - `define_source_family(spec)`
  - `define_recipe_family(spec)`
- reads and keyed-family helpers
- direct mutation and invalidation
- runtime policy changes with `set_runtime_policy(policy)`
- lower-level diagnostics/history/specialist/adapters access

## Aspect Model

The compatibility surface is fully aspect-aware.

Definition specs can declare produced aspects:

- `SourceSpec.producesAspects`
- `KeyedSourceFamilySpec.producesAspects`
- `RecipeSpec.producesAspects`
- `KeyedRecipeFamilySpec.producesAspects`

Read specs can also select aspects:

- `RecipeReadSpec` object reads use `aspect` or `aspects`
- `RecipeFamilyReadSpec` uses `aspects: { aspect?, aspects? }`

If you omit aspect metadata, the lower-level compatibility behavior still falls
back to default aspect `0`.

## Keyed Families And Packed Helpers

The compatibility lane still exposes direct family/grid helpers for advanced
consumers:

- `read_keyed_many_packed_fields(...)`
- `read_keyed_grid_packed_fields(...)`
- `read_keyed_rect_packed_fields(...)`
- `prewarm_keyed_grid(...)`
- `seed_keyed_grid_coords(...)`
- `transaction_with_packed_grid_rgba(...)`

Use these when you genuinely need them. They are not the normal app-facing
starting point.

## Runtime Policy

`SignalRuntime` is also where lower-level runtime policy changes remain
explicit:

- `operational`
- `webDevelopment`
- `development`
- `forensic`

Use this lane when you are deliberately managing runtime policy. Do not teach
ordinary application code to depend on it casually.

## How It Relates To Other Features

- Use the app surface for most browser application code.
- Use aspects with the compatibility surface when you need explicit structural
  aspect contracts.
- Use diagnostics/history together with compatibility only when you need
  lower-level artifacts rather than graph-shaped inspection.
- Use the compatibility lane for migration or specialized lower-level control,
  not because the newer app surface feels unfamiliar.

## Inspection And Debugging

The compatibility surface still exposes:

- `diagnostics()`
- `history()`
- `specialist()`
- `adapters()`

If you are debugging a published graph, the graph-scoped inspection surfaces
are usually better. Drop to compatibility only when the question is genuinely
about lower-level runtime structure.

## Anti-Patterns

- starting new ordinary app code on `SignalApp` or `SignalRuntime`
- treating the compatibility surface as a more “real” state model than the app
  surface
- using lower-level keyed/grid helpers when ordinary graph/controller code
  already fits
- mixing app-surface and compatibility definitions without being explicit about
  who owns the boundary

## Current Limits

- this is a lower-level lane by design
- it is supported, but it is not the primary product path
- explicit structural naming remains part of the contract here
- this surface should converge to the same committed truth as the app surface
  where they overlap

## Related Docs

- [App Surface Overview](../app-surface/overview.md)
- [Diagnostics And History](../app-surface/diagnostics-and-history.md)
- [Aspects](../app-surface/aspects.md)
