# Attachments And Evidence Fields

## What This Feature Is

Attachments and evidence fields are the forms lane for values that carry
attachment identity and metadata proof.

## Why You Use It

- keep attachment identity explicit
- distinguish attachment-shaped fields from ordinary scalar fields
- support attach and detach patch behavior honestly

## Stable Entry Points

- `attachment(...)`
- `evidence(...)`
- `fieldHandle.attachmentIdentity()`
- `fieldHandle.diagnostics()`

## Core Mental Model

These fields are ordinary form fields plus declared attachment identity. The
runtime does not guess which property is the file digest. You declare it.

## How It Executes

1. declare an attachment or evidence field with `attachmentIdentity` or
   `digest`
2. the runtime records attachment identity posture in the field contract
3. field writes keep attachment identity inspectable
4. patch planning can lower attach or detach operations later

## Small Example

```ts
const form = signals.form({
  source: { evidence: { digest: "file-1", name: "audit.pdf" } },
  fields: ({ evidence }) => ({
    evidence: evidence("evidence", {
      attachmentIdentity: "digest",
      metadata: { required: true },
    }),
  }),
});

console.log(form.fields.evidence.attachmentIdentity());
```

## Real Example

```ts
form.fields.evidence.set({ digest: "file-2", name: "signed.pdf" });
console.log(form.patchPlan().operations);

form.fields.evidence.set(null);
console.log(form.patchPlan().operations);
```

The first write can lower to attach posture. The second can lower to detach
posture. The runtime keeps that distinction public.

## How It Relates To Other Features

- Read [Patch Plans](../changes/patch-plans.md) for the attach/detach operation
  layer.
- Read [Patching Complex Edit Forms](../changes/patching-complex-edit-forms.md)
  for mixed scalar and attachment edit forms.

## Inspection And Debugging

- `attachmentIdentity()` shows the current attachment digest and metadata
- `fieldContract()` shows declared attachment posture
- `diagnostics().attachment` shows current attachment diagnostics at field
  scope

## Anti-Patterns

- treating file-shaped values like ordinary scalar fields
- relying on ambient property names instead of declared attachment identity

## Current Limits

- transfer and media visibility lanes are outside this first doc batch

## Related Docs

- [Patch Plans](../changes/patch-plans.md)
- [Patching Complex Edit Forms](../changes/patching-complex-edit-forms.md)
