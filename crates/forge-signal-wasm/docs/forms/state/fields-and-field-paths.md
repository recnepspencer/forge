# Fields And Field Paths

## What This Feature Is

This page covers form field declarations, field ids, and field paths.

## Why You Use It

- declare stable field handles
- target nested values without building your own path layer
- inspect field-level diagnostics and write posture

## Stable Entry Points

- `field(...)`
- `form.fields.<id>`
- `fieldHandle.locus()`
- `form.fieldContract()`
- `form.fieldWritePosture(fieldId)`

## Core Mental Model

The path points to the value location. The field id identifies the form field.
They are often the same, but they do not have to be.

The runtime turns field declarations into stable field handles and field
contracts.

## How It Executes

1. declare fields by path
2. optional field ids and options are normalized into field contracts
3. the runtime exposes typed handles through `form.fields`
4. later diagnostics and patch operations refer back to those contracts

## Small Example

```ts
const form = signals.form({
  source: { profile: { displayName: "Ship docs" } },
  fields: ({ field }) => ({
    displayName: field("profile.displayName", { id: "displayName" }),
  }),
});

console.log(form.fields.displayName.locus());
```

## Real Example

```ts
const form = signals.form({
  source: {
    profile: { displayName: "Ship docs" },
    tags: ["regulated"],
  },
  fields: ({ field }) => ({
    displayName: field("profile.displayName"),
    firstTag: field(["tags", 0], { id: "firstTag" }),
  }),
});

console.log(form.fieldContract());
console.log(form.fieldWritePosture("firstTag"));
```

## How It Relates To Other Features

- Read [Repeated Items](./repeated-items.md) when the field is collection-shaped
  instead of scalar.
- Read [Attachments And Evidence Fields](./attachments-and-evidence-fields.md)
  when the field carries attachment identity.

## Inspection And Debugging

- `locus()` shows the normalized field, path, and segments
- `fieldContract()` shows the declared family and resource-locus posture
- `fieldWritePosture(...)` shows edit or patch denial at field scope

## Anti-Patterns

- reusing one field id for multiple declarations
- inventing a second path abstraction beside the field contract

## Current Limits

- this page covers ordinary field declarations only
- layout, accessibility, and input capability posture belong in later sections

## Related Docs

- [Repeated Items](./repeated-items.md)
- [Attachments And Evidence Fields](./attachments-and-evidence-fields.md)
- [Patch Plans](../changes/patch-plans.md)
