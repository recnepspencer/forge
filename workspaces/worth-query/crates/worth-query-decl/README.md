# worth-query-decl

`worth-query-decl` is the declaration-audience facade for application schema
and Query meaning. Application and entry-band domain crates use it to declare
typed entities, relations, aspects, fields, queries, operations, capabilities,
policies, and principal bindings.

```rust
use worth_query_decl::facade::{
    application_capability,
    application_query,
    application_schema,
};
```

Declarations describe portable application intent. They do not install a
runtime, authenticate a caller, authorize a request, execute a provider, or
publish a result. Application hosts perform those transitions through
`worth-query-host`.

Pure reusable schema-meaning crates remain Query-agnostic. Query declaration
integration belongs in the application entry band.

## Related Docs

- [Ordinary Application Front Door](../worth-query/docs/foundations/ordinary-application-front-door.md)
- [WORTH Query Orientation](../worth-query/docs/AI_README.md)
- [Declarative Query Experience](../worth-query/docs/capabilities/declarative-query-experience.md)
- [Query Expressions And Result Shapes](../worth-query/docs/authoring/query-expressions-and-result-shapes.md)
