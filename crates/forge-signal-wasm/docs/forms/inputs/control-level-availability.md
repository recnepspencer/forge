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
- `form.controlAvailabilities()`
- `form.controlAvailability(controlId)`
- `form.readiness()`

## Core Mental Model

A control is not the same thing as a field.

The field is the semantic draft lane.
The control is one UI surface that may or may not currently be usable.

## How It Executes

1. declare control availability rules
2. the runtime evaluates those rules alongside field and action availability
3. availability reports keep the control posture explicit
4. control-specific reads give you the common-path lookup without manual
   filtering

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
console.log(form.controlAvailabilities());
console.log(form.controlAvailability("searchOpen"));
```

## How It Relates To Other Features

- Read [Field And Control Availability](../availability/field-and-control-availability.md)
  for the broader availability model.
- Read [Dropdowns, Comboboxes, And Search](./dropdowns-comboboxes-and-search.md)
  for common controls that benefit from this lane.

## Inspection And Debugging

- `controlAvailabilities()` is the first read for all control availability
- `controlAvailability(controlId)` is the first read when you already know the
  control id
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
