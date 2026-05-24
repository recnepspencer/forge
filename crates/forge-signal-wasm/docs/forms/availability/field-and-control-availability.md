# Field And Control Availability

## What This Feature Is

This page covers the form rules that make fields, actions, controls, groups,
 or sections enabled, disabled, hidden, readonly, required, omitted, blocked,
or unavailable.

## Why You Use It

- disable or hide parts of a form from declared runtime rules
- keep draft-handling behavior explicit when availability changes
- inspect which scope is blocking the current UI

## Stable Entry Points

- `availability: ({ field, action, control, group, section }) => ...`
- `form.availability()`
- `form.fieldWritePosture(fieldId)`

## Core Mental Model

Availability is broader than "can edit this field." It can apply to a field,
an action, a UI control, a group of fields, or a whole section.

Each availability result also carries a draft policy such as `preserve`,
`clear`, `freeze`, or `omit`, so availability changes do not silently rewrite
draft truth.

## How It Executes

1. declare availability rules for one or more scopes
2. rules read current form values and dependencies
3. the runtime emits availability results
4. write posture, readiness, and later action planning consume those results

## Small Example

```ts
const form = signals.form({
  source: { title: "", archived: false },
  fields: ({ field }) => ({
    title: field("title"),
    archived: field("archived"),
  }),
  availability: ({ field }) => ({
    titleEditable: field("title", ["archived"], ({ archived }) => (
      archived ? { state: "readonly", draftPolicy: "freeze" } : "enabled"
    )),
  }),
});

console.log(form.availability().artifacts);
```

## Real Example

```ts
const report = form.availability();

console.log(report.summary);
console.log(report.summary.byScope);
console.log(form.fieldWritePosture("title"));
```

## How It Relates To Other Features

- Read [Admission Rules](./admission-rules.md) when the rule is about
  capability or approval posture instead of availability state.
- Read [Readiness Blockers](./readiness-blockers.md) when you need the
  submit-facing consequence.

## Inspection And Debugging

- `availability().artifacts` shows each declared result
- `availability().summary.byScope` tells you which scope is contributing
- `fieldWritePosture(...)` tells you whether availability is actually denying
  field edits

## Anti-Patterns

- using hidden UI state as a substitute for declared availability
- changing availability without thinking about draft policy

## Current Limits

- host-specific offline and capability facts get their own later section

## Related Docs

- [Admission Rules](./admission-rules.md)
- [Readiness Blockers](./readiness-blockers.md)
