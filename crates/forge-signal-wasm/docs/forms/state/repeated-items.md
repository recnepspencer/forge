# Repeated Items

## What This Feature Is

Repeated items are the forms lane for collection fields with stable item
identity.

## Why You Use It

- add, remove, move, and replace collection items safely
- preserve item identity through draft edits
- support honest collection patch behavior later

## Stable Entry Points

- `repeated(...)`
- `fieldHandle.addItem(...)`
- `fieldHandle.removeItem(...)`
- `fieldHandle.replaceItem(...)`
- `fieldHandle.moveItem(...)`
- `fieldHandle.collectionIdentity()`

## Core Mental Model

Repeated fields are not just arrays. They are arrays with explicit item
identity, because later patch and reconciliation behavior depends on that
identity staying stable.

## How It Executes

1. declare a repeated field with `itemIdentity` or `key`
2. the runtime tracks stable item ids
3. collection edits update draft truth through repeated-field operations
4. patch and diagnostics layers consume that collection identity later

## Small Example

```ts
const form = signals.form({
  source: { reviewers: [{ id: "r1", name: "Ada" }] },
  fields: ({ repeated }) => ({
    reviewers: repeated("reviewers", { itemIdentity: "id" }),
  }),
});

form.fields.reviewers.addItem({ id: "r2", name: "Grace" });
console.log(form.fields.reviewers.collectionIdentity());
```

## Real Example

```ts
form.fields.reviewers.moveItem("r2", "r1");
form.fields.reviewers.replaceItem("r1", { id: "r1", name: "Ada Lovelace" });
form.fields.reviewers.removeItem("r2");

console.log(form.patchPlan().operations);
```

Stable item ids are what make those collection edits safe to lower later.

## How It Relates To Other Features

- Read [Patching Complex Edit Forms](../changes/patching-complex-edit-forms.md)
  for how repeated identity affects patch plans.
- Read [Broad Replacement Vs Narrow Patches](../changes/broad-replacement-vs-narrow-patches.md)
  when reorder or mixed-placement behavior widens the patch scope.

## Inspection And Debugging

- `collectionIdentity()` shows the current item ids and digests
- `fieldContract()` shows repeated collection posture
- `patchPlan()` shows whether repeated edits stayed narrow or widened

## Anti-Patterns

- declaring repeated data without stable item identity
- replacing one item with a different identity under the same item id

## Current Limits

- resource-backed collection write behavior lives in the later resource-backed
  section

## Related Docs

- [Fields And Field Paths](./fields-and-field-paths.md)
- [Patching Complex Edit Forms](../changes/patching-complex-edit-forms.md)
- [Broad Replacement Vs Narrow Patches](../changes/broad-replacement-vs-narrow-patches.md)
