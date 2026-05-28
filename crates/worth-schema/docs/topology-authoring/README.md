# Topology Authoring

This section covers the curated topology-authoring namespace published through
`worth_schema::facade::topology_authoring`.

These docs are organized around the questions consumers actually ask:

- how do I build a small topology intent?
- how do I reference another thing I am creating in the same batch?
- which helpers are for tests and fixtures?
- where did schema-owned topology execution go?

Use this section when you need:

- a small topology intent builder
- symbolic same-batch references
- seed helpers for tests and fixtures
- migration guidance for topology execution that moved out of schema

Docs in this section:

- [Your First Topology Intent](./your-first-topology-intent.md)
- [Create Batch Builder](./create-batch-builder.md)
- [Verification](./verification.md)
- [Seed And Fixture Lane](./seed-and-fixture-lane.md)

Good to know:

- this is an authoring/support lane, not the public runtime entry
- it is especially useful for tests, certification support, fixtures, and small
  explicit topology examples
- seed helpers are support artifacts, not the stable runtime result story
