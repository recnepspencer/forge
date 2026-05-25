# Choosing A Form Source

## What This Feature Is

This page explains the source-authority choices behind `signals.form(...)`.

## Why You Use It

- choose the right owner for source truth before adding draft behavior
- keep graph, resource, and external authority explicit
- avoid treating every form like local signal state

## Stable Entry Points

- `signals.form(...)`
- `signals.form.source.signal(...)`
- `signals.form.source.graphPublicInput(...)`
- `signals.form.source.resourceLine(...)`
- `signals.form.source.external(...)`
- `form.sourceAuthority()`
- `form.sourceAdmission()`
- `form.draftRestore()`

## Core Mental Model

The form owns draft truth. It does not always own source truth.

Choose the source lane based on who is authoritative:

- `signal(...)` for local signal-owned source truth
- `graphPublicInput(...)` when source truth belongs to a public graph input
- `resourceLine(...)` when source truth belongs to a resource line
- `external(...)` when source truth is readable but not signal-native

## How It Executes

1. declare one source lane
2. the runtime records source-authority diagnostics
3. the form derives draft and effective truth on top of that source
4. bootstrap or restore posture may appear through `sourceAdmission()` and
   `draftRestore()`

## Small Example

```ts
const source = signals.input({ title: "Ship docs" });

const form = signals.form({
  source: signals.form.source.signal(source, { id: "task-signal" }),
  fields: ({ field }) => ({
    title: field("title"),
  }),
});

console.log(form.sourceAuthority());
```

This is the smallest honest example because it shows explicit source ownership
without introducing any unrelated feature lane.

## Real Example

```ts
const form = signals.form({
  source: signals.form.source.resourceLine(taskLine, {
    id: "task-resource",
  }),
  fields: ({ field }) => ({
    title: field("title"),
    status: field("status"),
  }),
});

console.log(form.sourceAuthority().kind);
console.log(form.sourceAdmission());
console.log(form.draftRestore());
```

Here the resource line still owns source truth. The form runtime only owns the
draft and the reads derived from it.

## How It Relates To Other Features

- Read [Your First Form](./your-first-form.md) if you only need the basic form
  lane.
- Read [Source Truth, Draft, And Effective Values](../state/source-truth-draft-and-effective-values.md)
  for the value layers above this source choice.
- Read [Source Compatibility And Draft Migration](../validation/source-compatibility-and-draft-migration.md)
  when source schema drift is part of the source contract.

## Inspection And Debugging

- `sourceAuthority()` is the first read when source ownership is unclear
- `sourceAdmission()` explains pending or unavailable source bootstrap posture
- `draftRestore()` explains draft restore posture during bootstrap
- `declaration().source` shows the admitted source kind and source id

## Anti-Patterns

- hiding graph or resource ownership behind an ordinary local source
- treating bootstrap posture as if the form had already admitted source truth
- choosing a source lane based only on convenience syntax

## Current Limits

- this page explains source ownership, not resource-backed write behavior
- route-coupled source authority lives in the router/forms seam, not this lane

## Related Docs

- [Your First Form](./your-first-form.md)
- [Source Truth, Draft, And Effective Values](../state/source-truth-draft-and-effective-values.md)
- [Source Compatibility And Draft Migration](../validation/source-compatibility-and-draft-migration.md)
