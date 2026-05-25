# Dropdowns, Comboboxes, And Search

## What This Feature Is

This page covers the common input-tool families that usually need adapter and
interaction wiring instead of plain `set(...)` calls.

## Why You Use It

- integrate dropdowns and comboboxes honestly
- handle search boxes with raw input and commit boundaries
- keep control availability and interaction state tied to the form

## Stable Entry Points

- `form.bindInput(fieldId, options?)`
- adapter capabilities such as `reportsRawInput`, `reportsCommitBoundary`,
  `reportsFocus`
- field handle methods:
  `input(...)`, `commitInput(...)`, `focus()`, `blur()`, `set(...)`
- `availability: ({ control }) => ...`

## Core Mental Model

Different controls usually need different integration shapes:

- dropdown: often `set(...)` plus focus/blur
- combobox: often raw input, focus, and a later selection commit
- search box: often raw input, composition, and explicit commit boundaries

## How It Executes

1. declare the adapter for the field
2. wire the control's events into the field handle or controller
3. if needed, declare control-level availability separately from field
   availability

## Small Example

```ts
statusSelect.onChange((value) => {
  form.fields.status.set(value);
});
```

## Real Example

```ts
const query = form.bindInput("query", {
  source: "typing",
  parse: (raw) => raw.trim(),
});

searchBox.onInput(query.input);
searchBox.onCommit(query.commit);
searchBox.onFocus(query.focus);
searchBox.onBlur(query.blur);
```

## How It Relates To Other Features

- Read [Raw Input, Compose, And Commit](./raw-input-compose-and-commit.md) for
  commit-boundary behavior.
- Read [Control-Level Availability](./control-level-availability.md) for
  control-only gating.
- Read [Layout Configuration Reference](../layout/layout-configuration-reference.md)
  when the same control also needs label, track, or sizing configuration.

## Inspection And Debugging

- `form.bindInput(...)` is the shortest common path for search, combobox, and
  similar controls
- `fieldHandle.diagnostics()` is the best first read for one control
- `form.inputCapabilities()` shows whether the control is overclaiming support
- `form.actionReadiness(...)` helps when search or selection controls affect
  submit readiness

## Anti-Patterns

- using `set(...)` alone when the control really has raw-input and commit
  phases
- forcing every dropdown or search widget through the same event model

## Current Limits

- the form runtime does not prescribe your widget rendering layer

## Related Docs

- [Raw Input, Compose, And Commit](./raw-input-compose-and-commit.md)
- [Control-Level Availability](./control-level-availability.md)
