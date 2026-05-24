# Control-Level Availability

## What This Feature Is

This page covers availability rules that apply to one control surface rather
than to the field's semantic meaning.

## Why You Use It

- disable one control while leaving the field concept intact
- gate search widgets, pickers, or auxiliary controls separately from core
  field truth
- keep control disablement on the same form-owned availability surface

## Stable Entry Points

- `availability: ({ control }) => ...`
- `form.availability()`
- `form.readiness()`

## Core Mental Model

A control is not the same thing as a field.

The field is the semantic draft lane.
The control is one UI surface that may or may not currently be usable.

## How It Executes

1. declare control availability rules
2. the runtime evaluates those rules alongside field and action availability
3. availability reports keep the control posture explicit

## Small Example

```ts
availability: ({ control }) => ({
  searchOpen: control("searchOpen", ["online"], () => ({
    state: "unavailable",
    reason: "search is offline",
  })),
})
```

## Real Example

```ts
const report = form.availability();

console.log(report.artifacts.filter((artifact) => artifact.scope === "control"));
console.log(report.summary.byScope.control);
```

## How It Relates To Other Features

- Read [Field And Control Availability](../availability/field-and-control-availability.md)
  for the broader availability model.
- Read [Dropdowns, Comboboxes, And Search](./dropdowns-comboboxes-and-search.md)
  for common controls that benefit from this lane.

## Inspection And Debugging

- `availability().artifacts.filter((artifact) => artifact.scope === "control")`
  is the first read for control availability
- `availability().summary.byScope.control` shows how many control artifacts are
  present

## Anti-Patterns

- disabling a whole field when only one control surface needs to be unavailable
- hiding control blockers entirely inside renderer code

## Current Limits

- control availability does not replace action or route-coupled denial posture

## Related Docs

- [Field And Control Availability](../availability/field-and-control-availability.md)
- [Dropdowns, Comboboxes, And Search](./dropdowns-comboboxes-and-search.md)
