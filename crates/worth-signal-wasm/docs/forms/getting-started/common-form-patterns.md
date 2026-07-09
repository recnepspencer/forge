# Common Form Patterns

## What This Feature Is

This page maps common product tasks to the first forms feature you should reach
for.

## Why You Use It

- find the correct lane quickly
- avoid starting in a lower or broader feature than you need
- get from "task" to "feature doc" fast

## Stable Entry Points

- `signals.form(...)`
- `signals.form.source.*(...)`
- `form.patchPlan()`
- `form.readiness()`
- `form.validation()`

## Core Mental Model

Most forms work starts in one of four patterns:

- ordinary local form
- complex edit form
- validated form
- schema-drifting long-lived form

Pick the pattern first, then the deeper feature page.

## How It Executes

1. identify who owns source truth
2. decide whether the main problem is state, patching, or validation
3. start on the narrowest matching doc
4. layer adjacent features only when the form actually needs them

## Small Example

```ts
const form = signals.form({
  source: { title: "" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
});
```

This is the normal pattern when the form problem is just local draft state.

## Real Example

```ts
const form = signals.form({
  source,
  fields: ({ field, repeated }) => ({
    title: field("title"),
    reviewers: repeated("reviewers", { itemIdentity: "id" }),
  }),
  validation: ({ field }) => ({
    titleRequired: field("title", (value) => (
      value.length > 0 || {
        kind: "invalid",
        message: {
          code: "title.required",
          severity: "error",
          audience: "user",
          visibility: "visible",
        },
      }
    )),
  }),
});
```

This is the pattern where state, repeated identity, and validation all matter,
so the right follow-up docs are in `state`, `changes`, and `validation`.

## How It Relates To Other Features

- complex edit form:
  [Patching Complex Edit Forms](../changes/patching-complex-edit-forms.md)
- unchanged submit denial:
  [Unchanged Forms And Submit Readiness](../changes/unchanged-forms-and-submit-readiness.md)
- async validation:
  [Async Validation](../validation/async-validation.md)

## Inspection And Debugging

- `sourceAuthority()` when source ownership is unclear
- `patchPlan()` when change semantics are unclear
- `validation()` and `visibleMessages()` when correctness posture is unclear

## Anti-Patterns

- starting with the broadest doc every time
- using one giant feature page as a substitute for clear lookup paths

## Current Limits

- this page is a router, not a full API reference

## Related Docs

- [Your First Form](./your-first-form.md)
- [Changes And Patching](../changes/README.md)
- [Validation](../validation/README.md)
