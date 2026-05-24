# Broad Replacement Vs Narrow Patches

## What This Feature Is

This page explains when the form can lower a narrow patch and when it must widen
to a broader replacement.

## Why You Use It

- debug surprising whole-value replacements
- understand why reorder or mixed-placement collection edits widen
- choose declarations that preserve honest narrow patch behavior

## Stable Entry Points

- `form.patchPlan().broadReplacement`
- `form.patchPlan().replacement`
- `form.patchPlan().operations`

## Core Mental Model

The runtime prefers narrow patches when they stay honest. It widens when a
smaller patch would hide real structural change.

## How It Executes

1. the runtime tries to lower narrow operations
2. if repeated or structural edits make that dishonest, it sets
   `broadReplacement`
3. the replacement artifact explains the wider write scope

## Small Example

```ts
console.log(form.patchPlan().broadReplacement);
console.log(form.patchPlan().replacement);
```

## Real Example

```ts
form.fields.reviewers.moveItem("r2", "r1");

const patch = form.patchPlan();

console.log(patch.broadReplacement);
console.log(patch.replacement);
console.log(patch.operations);
```

If the repeated edit cannot stay a narrow operation honestly, the replacement
artifact tells you why.

## How It Relates To Other Features

- Read [Repeated Items](../state/repeated-items.md) for the identity lane that
  usually controls this outcome.
- Read [Patching Complex Edit Forms](./patching-complex-edit-forms.md) for the
  mixed edit-form case.

## Inspection And Debugging

- `replacement.reason` is the most important read
- compare `operations` and `replacement` together instead of assuming only one
  will matter

## Anti-Patterns

- assuming broad replacement means the runtime failed
- forcing narrow patch semantics in app code when the form widened honestly

## Current Limits

- resource-backed merge and delivery consequences live in later docs

## Related Docs

- [Repeated Items](../state/repeated-items.md)
- [Patching Complex Edit Forms](./patching-complex-edit-forms.md)
