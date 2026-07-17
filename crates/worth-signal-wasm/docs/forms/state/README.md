# State, Fields, And Changes

Use this guide when you need to answer three questions precisely: who owns the
original value, what has the user changed, and what can be written back.

## Source, Draft, And Effective Value

```ts
const profile = signals.input({
  name: "Ada",
  tags: [{ id: "systems", label: "Systems" }],
});

const form = signals.form({
  source: signals.form.source.signal(profile, { id: "profile" }),
  fields: ({ field, repeated }) => ({
    name: field<string>("name"),
    tags: repeated<Array<{ id: string; label: string }>>("tags", {
      itemIdentity: "id",
    }),
  }),
});
```

The signal owns source truth. The controller owns the draft. Effective value,
dirty state, and patch plans are derived and can be rebuilt from those two
inputs.

```ts
form.fields.name.set("Ada Lovelace");
form.fields.tags.addItem({ id: "math", label: "Mathematics" });

console.log(form.source());
console.log(form.draft());
console.log(form.effective());
console.log(form.dirty());
console.log(form.patchPlan().operations);
```

## Field Families

- `field(...)` declares one scalar or structured value path.
- `repeated(...)` declares a collection and requires stable item identity.
- `attachment(...)` and `evidence(...)` require stable attachment identity or a
  digest.

Array positions are not identity. Reordering, removing, and replacing repeated
items remain meaningful only when an item can be recognized across versions.
Attachments and evidence fields describe attach/detach posture; they do not
upload a file by themselves.

## Semantic Dirty Truth

Dirty state is value-based. Calling `set(...)` does not permanently mark a
field dirty:

```ts
form.fields.name.set("Countess Lovelace");
form.fields.name.set("Ada");

console.log(form.fields.name.dirty().isDirty); // false
console.log(form.patchPlan().empty);            // true
```

For deep structured values, equality and aggregate reports can require
structural scans. The reports expose counters and `notIncremental` posture
where that cost is not field-incremental; do not assume every read is O(1).

## Patch Plans Are Plans, Not Writes

`patchPlan()` describes the narrow operations the declaration can prove. It
does not call an endpoint or mutate a resource. A broad replacement appears
when the declared identity is insufficient for a narrower operation.

Use an action to execute the plan. With a resource-line source, declare an
honest resource locus for fields whose form path does not directly lower to the
resource shape.

## Go Deeper

- [Source Truth, Draft, And Effective Values](./source-truth-draft-and-effective-values.md)
- [Fields And Field Paths](./fields-and-field-paths.md)
- [Repeated Items](./repeated-items.md)
- [Attachments And Evidence Fields](./attachments-and-evidence-fields.md)
- [Changes And Patch Plans](../changes/README.md)
- [Inputs And Controls](../inputs/README.md)
