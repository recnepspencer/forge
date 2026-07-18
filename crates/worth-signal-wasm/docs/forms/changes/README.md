# Changes And Patch Plans

Dirty state answers whether effective value differs from source. A patch plan
answers how that difference can be represented with the identities and loci the
form actually knows.

```ts
form.fields.title.set("Published title");

const dirty = form.dirty();
const plan = form.patchPlan();

console.log(dirty.isDirty);
console.log(plan.operations);
console.log(plan.replacement);
```

Field writes never apply the plan. Host actions or resource-backed actions own
execution. Returning every field to its source-equivalent value produces an
empty plan. Nested fields, attachments, and repeated items can produce narrow
operations only when their paths and stable identities are declared.

Broad replacement is not automatically wrong; it is the honest result when a
narrower change cannot be proved. Inspect the replacement reason before
building an endpoint that expects field- or item-level operations.

Read next:

- [Dirty State](./dirty-state.md)
- [Patch Plans](./patch-plans.md)
- [Patching Complex Edit Forms](./patching-complex-edit-forms.md)
- [Unchanged Forms And Submit Readiness](./unchanged-forms-and-submit-readiness.md)
- [Broad Replacement Vs Narrow Patches](./broad-replacement-vs-narrow-patches.md)
