# Your First Form

## What This Feature Is

This is the smallest honest entrypoint into `signals.form(...)`.

## Why You Use It

- create one runtime-owned form without building your own draft store
- get field handles, effective values, dirty checks, and readiness reads
- start from the normal lane before adding validation or actions

## Stable Entry Points

- `signals.form(...)`
- `field(...)`
- `form.fields.<id>.set(...)`
- `form.source()`
- `form.draft()`
- `form.effective()`
- `form.readiness()`

## Core Mental Model

The runtime owns three value layers:

- source: the authoritative input value
- draft: the user's edits
- effective: the value you read right now

You work through the form controller. You do not create a second draft store
next to it.

## How It Executes

1. declare a source
2. declare fields
3. the runtime builds field handles and draft storage
4. field writes update draft truth
5. reads such as `effective()` and `readiness()` derive from the current form
   state

## Small Example

```ts
const source = signals.input({ title: "Ship docs", done: false });

const form = signals.form({
  source,
  fields: ({ field }) => ({
    title: field("title"),
    done: field("done"),
  }),
});

form.fields.title.set("Ship docs today");

console.log(form.effective());
```

This is the smallest honest example because it stays on the public controller
surface and already shows source vs effective truth.

## Real Example

```ts
const source = signals.input({
  title: "Ship docs",
  metadata: { owner: "docs-team" },
});

const form = signals.form({
  id: "task-form",
  source,
  fields: ({ field }) => ({
    title: field("title"),
    owner: field("metadata.owner", { id: "owner" }),
  }),
});

form.fields.title.set("Publish docs");

console.log(form.source());
console.log(form.draft());
console.log(form.effective());
console.log(form.readiness());
```

The source input still owns source truth. The form owns draft truth and the
derived reads above it.

## How It Relates To Other Features

- Read [Choosing A Form Source](./choosing-a-form-source.md) when the source is
  graph-owned, resource-owned, or external.
- Read [Dirty State](../changes/dirty-state.md) when you need semantic change
  truth instead of just the effective value.
- Read [Validation Overview](../validation/validation-overview.md) when the
  form should produce typed validation artifacts.

## Inspection And Debugging

- `declaration()` shows form id, source kind, and field family counts
- `sourceAuthority()` shows which boundary owns source truth
- `fieldContract()` shows stable field paths and declared posture
- `source()`, `draft()`, and `effective()` let you compare the three value
  layers directly

## Anti-Patterns

- storing another draft object outside the form
- treating `effective()` as if it were source truth
- starting with actions, validation, and lifecycle features before the core
  form lane is clear

## Current Limits

- this page covers the normal controller lane only
- route coupling, resource-backed writes, and lifecycle lanes live in later
  sections

## Related Docs

- [Choosing A Form Source](./choosing-a-form-source.md)
- [Source Truth, Draft, And Effective Values](../state/source-truth-draft-and-effective-values.md)
- [Dirty State](../changes/dirty-state.md)
