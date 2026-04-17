# Compatibility Surface Reference

`forge-signal-wasm` exposes a lower-level compatibility surface for advanced
and legacy consumers.

This surface is real and supported, but it is explicitly secondary to the
app-first `createSignals()` API.

## Why This Surface Exists

The compatibility surface preserves direct access to the older kernel-first
authoring model:

- `source`
- `recipe`
- `source_family`
- `recipe_family`
- lower-level runtime reads and keyed-family helpers
- low-level diagnostics/history/adapters access

Use it when:

- you need lower-level existing definitions
- you are porting prior wasm consumers
- you need keyed/grid helper paths the app-first surface does not foreground

Do not start new web app code here unless you specifically need the lower-level
shape.

## `SignalApp`

Create it directly:

```ts
const app = new SignalApp();
```

Or reach it from `Signals`:

```ts
const app = signals.compatibilityApp();
```

### Definition Registration

- `source(spec)`
- `recipe(spec)`
- `source_family(spec)`
- `recipe_family(spec)`

### Transaction And Batch

- `batch(ops)`
- `transaction_with_packed_grid_rgba(prefixOps, familyId, width, height, rgba, suffixOps)`

### Reads

- `read(id)`
- `read_many(ids)`
- `read_keyed(familyId, key)`
- `read_keyed_many(familyId, keys)`

### Keyed Grid And Packed Field Helpers

- `read_keyed_many_packed_fields(familyId, keys, fields)`
- `read_keyed_grid_packed_fields(familyId, columns, rows, fields)`
- `read_keyed_rect_packed_fields(familyId, columns, rows, row, startColumn, width, height, fields)`
- `prewarm_keyed_grid(familyId, columns, rows)`
- `seed_keyed_grid_coords(familyId, columns, rows)`

### Writes And Invalidation

- `set_keyed(familyId, key, value)`
- `set_keyed_many(familyId, values)`
- `mark_changed_with_regions(id, changedRegions)`
- `mark_keyed_changed_with_regions(familyId, key, changedRegions)`

### Diagnostics And Other Doors

- `take_debug_events()`
- `diagnostics()`
- `history()`
- `specialist()`
- `adapters()`

## `SignalRuntime`

Create it directly:

```ts
const runtime = new SignalRuntime();
```

Or reach it from `Signals`:

```ts
const runtime = signals.compatibilityRuntime();
```

### Definition Registration

- `define_source(spec)`
- `define_recipe(spec)`
- `define_source_family(spec)`
- `define_recipe_family(spec)`

### Reads

- `read(id)`
- `read_many(ids)`
- `read_keyed(familyId, key)`
- `read_keyed_many(familyId, keys)`
- `read_keyed_many_packed_fields(familyId, keys, fields)`
- `read_keyed_grid_packed_fields(familyId, columns, rows, fields)`
- `read_keyed_rect_packed_fields(familyId, columns, rows, row, startColumn, width, height, fields)`

### Keyed Grid Helpers

- `prewarm_keyed_grid(familyId, columns, rows)`
- `seed_keyed_grid_coords(familyId, columns, rows)`

### Writes And Mutation

- `set_keyed(familyId, key, value)`
- `set_keyed_many(familyId, values)`
- `clear_keyed_family_cache(familyId)`
- `mark_changed_with_regions(id, changedRegions)`
- `mark_keyed_changed_with_regions(familyId, key, changedRegions)`
- `transaction(ops)`
- `transaction_with_packed_grid_rgba(prefixOps, familyId, width, height, rgba, suffixOps)`

### Runtime Policy

- `set_runtime_policy(policy)`

This is the primary low-level way to switch runtime mode from web code:

- `operational`
- `webDevelopment`
- `development`
- `forensic`
- plus the broader preset set still exposed by the runtime policy model

### Diagnostics And Other Doors

- `take_debug_events()`
- `diagnostics()`
- `history()`
- `specialist()`
- `adapters()`

## Semantics Notes

- the compatibility surface should converge to the same committed runtime truth
  as the app-first surface where they overlap
- this surface is not allowed to become a separate semantic engine
- performance and read breadth still contribute to the same web perf cert
  surface

## Related Docs

- [app_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/app_surface_reference.md)
- [diagnostics_and_history_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/diagnostics_and_history_reference.md)
