# Authority

This section covers the public authority vocabulary that belongs in
`worth-schema::facade::platform::authority`.

The key word is adjacent.

These docs do not teach a schema-owned runtime execution lane. They cover the
write-side truth vocabulary and authority semantics that belong in the Worth
platform layer.

Use this section when you need:

- write-side topology truth vocabulary
- authority semantics such as invalidation, interpretation, and precision
  classification

Docs in this section:

- [Topology Mutations](./topology-mutations.md)
- [Derived Invalidation Declarations](./derived-invalidation-declarations.md)
- [Geometry Binding Vocabulary](./geometry-binding-vocabulary.md)
- [Interpretation Vocabulary](./interpretation-vocabulary.md)
- [Precision Fallbacks](./precision-fallbacks.md)
- [Verified Commits And Read Basis](./verified-commits-and-read-basis.md)

Good to know:

- the stable consumer surface here is the write-side vocabulary
- the published import lane is `worth_schema::facade::platform::authority`
- runtime execution, inspection, and recovery belong to Query
