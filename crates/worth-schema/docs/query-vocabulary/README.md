# Query Vocabulary

This section covers the `worth-schema` surfaces that help you name schema truth
for Query-facing work without turning `worth-schema` into the runtime itself.

These docs answer the questions people actually hit first:

- how do I name the truth slice I care about?
- how do I name the collection and schema basis behind a live or computed view?
- how do I name delivered fields without inventing strings?

Docs in this section:

- [Overview](./index.md)
- [Query Aspect Paths](./query-aspect-paths.md)
- [Query Aspect Family](./query-aspect-family.md)
- [Query Collections And Bases](./query-collections-and-bases.md)
- [Live Fields](./live-fields.md)
- [Field Selection Recipes](./field-selection-recipes.md)

Good to know:

- use these surfaces to name what truth you want
- use `forge-query` to decide whether that truth can run, how it is admitted,
  and how it is inspected or recovered
