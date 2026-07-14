# Action Overview

## What This Feature Is

This page covers the action catalog behind submit, custom, and step actions.

## Why You Use It

- declare buttons and commands inside the form runtime
- inspect available actions without rebuilding planner logic yourself
- distinguish action declaration from action execution

## Stable Entry Points

- `actions: ({ submit, action, step }) => ...`
- `form.actions()`
- `form.actionPlan(actionId)`

## Core Mental Model

Actions come in three main kinds:

- `submit`
- `custom`
- `step`

The declaration says what exists. The action plan says whether it can run
right now.

## How It Executes

1. declare actions on the form
2. the runtime builds an action catalog
3. the planner derives accepted or denied action plans
4. execution can start later from those plans

## Small Example

```ts
const form = signals.form({
  source: { title: "" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  actions: ({ submit, action }) => ({
    submit: submit(),
    saveDraft: action("saveDraft", { patchPolicy: "allowEmpty" }),
  }),
});

console.log(form.actions().catalog);
```

## Real Example

```ts
const actions = form.actions();

console.log(actions.summary);
console.log(actions.plans.map((plan) => ({
  id: plan.id,
  status: plan.status,
  kind: plan.kind,
})));
```

## How It Relates To Other Features

- Read [Action Plans](./action-plans.md) for the planner surface.
- Read [Submit Actions](./submit-actions.md) for the common submit lane.

## Inspection And Debugging

- `actions().catalog` shows the declared actions
- `actions().plans` shows the current accepted or denied plans
- `actions().summary` shows how many actions are submit, step, denied, or
  fulfilled

## Anti-Patterns

- treating button declarations as if they were already executable
- duplicating action planner logic in app code

## Current Limits

- resource-backed action execution gets its own later forms section

## Related Docs

- [Action Plans](./action-plans.md)
- [Submit Actions](./submit-actions.md)
