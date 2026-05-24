# Source Truth, Draft, And Effective Values

## What This Feature Is

This page explains the three value layers every form controller exposes.

## Why You Use It

- understand what the form reads and what it owns
- debug why a visible value differs from the saved value
- keep source truth and draft truth separate in app code

## Stable Entry Points

- `form.source()`
- `form.draft()`
- `form.effective()`
- `form.sourceAuthority()`

## Core Mental Model

- `source()` is the authoritative input value
- `draft()` is the user's in-progress overlay
- `effective()` is the current merged read

`effective()` is not a second source of truth. It is a derived read.

## How It Executes

1. source truth is read from the declared source lane
2. draft writes are retained separately
3. effective values are derived by layering draft on top of source
4. dirty, patch, readiness, and validation all read from those layers

## Small Example

```ts
form.fields.title.set("Ship docs today");

console.log(form.source());
console.log(form.draft());
console.log(form.effective());
```

This is the smallest honest example because it shows the three layers after one
real write.

## Real Example

```ts
const form = signals.form({
  source,
  fields: ({ field }) => ({
    title: field("title"),
    owner: field("metadata.owner", { id: "owner" }),
  }),
});

form.fields.owner.set("docs-platform");

console.log(form.source().metadata.owner);
console.log(form.draft());
console.log(form.effective().metadata.owner);
```

The source remains unchanged. The draft stores only the edited part. The
effective read shows the merged value.

## How It Relates To Other Features

- Read [Dirty State](../changes/dirty-state.md) for the semantic comparison
  layer built on top of these values.
- Read [Patch Plans](../changes/patch-plans.md) for the write plan derived from
  these values.

## Inspection And Debugging

- compare `source()` and `effective()` first
- use `draft()` to see whether the difference actually lives in draft truth
- use `sourceAuthority()` if the wrong thing seems to be authoritative

## Anti-Patterns

- mutating source outside the declared source boundary and expecting the form
  to treat it like a draft write
- persisting `effective()` as if it were source truth

## Current Limits

- this page explains value layering only
- patch, readiness, and validation behavior live in later sections

## Related Docs

- [Choosing A Form Source](../getting-started/choosing-a-form-source.md)
- [Dirty State](../changes/dirty-state.md)
- [Patch Plans](../changes/patch-plans.md)
