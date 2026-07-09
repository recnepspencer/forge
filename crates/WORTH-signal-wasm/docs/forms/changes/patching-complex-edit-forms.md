# Patching Complex Edit Forms

## What This Feature Is

This page covers the patch behavior of nested, repeated, and mixed-field edit
forms.

## Why You Use It

- patch real edit forms with nested values instead of toy scalar examples
- understand how repeated items and attachment fields affect patch output
- inspect when one form produces several operations instead of one broad write

## Stable Entry Points

- `form.patchPlan()`
- `form.fieldContract()`
- repeated field handles such as `moveItem(...)`
- attachment or evidence field handles such as `attachmentIdentity()`

## Core Mental Model

Complex edit forms are still driven by the same rule: the form runtime lowers
the most honest patch it can from source to effective truth.

For nested, repeated, and attachment-shaped fields, that often means:

- several narrow operations
- attach or detach operations
- or an explicit broad replacement when narrow patch truth stops being honest

## How It Executes

1. field contracts define scalar, repeated, or attachment posture
2. writes change draft truth
3. repeated identity and attachment identity constrain what narrow patches are
   legal
4. the patch plan lowers operations or widens the scope

## Small Example

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

form.fields.displayName.set("Published docs");
form.fields.firstTag.set("released");

console.log(form.patchPlan().operations);
```

## Real Example

```ts
const form = signals.form({
  source: {
    profile: { displayName: "Ship docs" },
    evidence: { digest: "file-0", name: "draft.pdf" },
    reviewers: [{ id: "r1", name: "Ada" }],
  },
  fields: ({ field, evidence, repeated }) => ({
    displayName: field("profile.displayName"),
    evidence: evidence("evidence", { attachmentIdentity: "digest" }),
    reviewers: repeated("reviewers", { itemIdentity: "id" }),
  }),
});

form.fields.displayName.set("Published docs");
form.fields.evidence.set({ digest: "file-1", name: "audit.pdf" });
form.fields.reviewers.addItem({ id: "r2", name: "Grace" });

console.log(form.fieldContract());
console.log(form.patchPlan().operations);
```

This form can emit a scalar set, an attach operation, and a repeated-item
operation from one draft state because those lanes were declared explicitly.

## How It Relates To Other Features

- Read [Repeated Items](../state/repeated-items.md) for stable item identity.
- Read [Attachments And Evidence Fields](../state/attachments-and-evidence-fields.md)
  for attach and detach posture.
- Read [Broad Replacement Vs Narrow Patches](./broad-replacement-vs-narrow-patches.md)
  when the form widens unexpectedly.

## Inspection And Debugging

- `fieldContract()` shows which declared posture is shaping the patch result
- `patchPlan().operations` shows the current lowering
- `patchPlan().replacement` explains whole-value widening when it happens

## Anti-Patterns

- expecting nested collections to patch honestly without declared repeated
  identity
- treating attachment updates like ordinary scalar writes
- assuming one complex edit should always become one patch operation

## Current Limits

- resource-line write execution and reconciliation are outside this first doc
  batch

## Related Docs

- [Repeated Items](../state/repeated-items.md)
- [Attachments And Evidence Fields](../state/attachments-and-evidence-fields.md)
- [Broad Replacement Vs Narrow Patches](./broad-replacement-vs-narrow-patches.md)
