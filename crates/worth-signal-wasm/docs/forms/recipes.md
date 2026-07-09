# Forms Recipes

This page is the forms-only task-first companion to the feature docs.

## Quick Answers

- "I need to start a normal form."  
  [Your First Form](./getting-started/your-first-form.md)
- "I need to choose between signal, graph, resource, or external source
  authority."  
  [Choosing A Form Source](./getting-started/choosing-a-form-source.md)
- "I need a complex edit form with nested patches."  
  [Patching Complex Edit Forms](./changes/patching-complex-edit-forms.md)
- "I need submit disabled when nothing changed."  
  [Unchanged Forms And Submit Readiness](./changes/unchanged-forms-and-submit-readiness.md)
- "I need async validation or visible validation messages."  
  [Async Validation](./validation/async-validation.md) and
  [Visible Messages](./validation/visible-messages.md)

## Recipe: Ordinary Local Form

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
console.log(form.readiness());
```

## Recipe: Complex Edit Form With Nested Patch Truth

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

## Recipe: Async Validation

```ts
const form = signals.form({
  source: { slug: "ship-docs" },
  fields: ({ field }) => ({
    slug: field("slug"),
  }),
  validation: ({ asyncField }) => ({
    slugUnique: asyncField("slug", {
      id: "slugUnique",
      triggers: ["submit"],
    }),
  }),
});

const pending = form.startAsyncValidation("slugUnique");
form.fulfillAsyncValidation(pending.operationId, {
  reason: "slug is unique",
});
```

## Related Docs

- [Forms Overview](./index.md)
- [Getting Started](./getting-started/README.md)
- [Form State](./state/README.md)
- [Changes And Patching](./changes/README.md)
- [Validation](./validation/README.md)
