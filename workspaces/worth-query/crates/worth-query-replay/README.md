# worth-query-replay

`worth-query-replay` is the cert-band audience facade for Query replay and
reconstruction authority. It re-exports the engine's replay types without
wrapping them or creating a second executor, comparator, or certification
model.

Ordinary application and host code must use `worth-query-decl` or
`worth-query-host`. Only cert-band crates may depend on this crate. That
boundary keeps replay and reconstruction authority out of ordinary execution
lanes while preserving exact type identity with `worth-query`.

Use the blessed surface:

```rust
use worth_query_replay::facade::{
    replay_installed_workflow,
    ScopedReplayBasis,
    WorthQueryCertificationReplayOutcome,
    WorthQueryReplayComparison,
};
```

Installed-operation replay is not receipt comparison. Query freshly executes
the retained installed workflow, compares exact effects, publication,
conditional observations, retained Signal evidence, lineage bindings, and the
domain-owned semantic result, then returns a typed equivalence or drift
outcome. Historical replay additionally requires an exact admitted
correspondence between the original and replay bases.

See [Installed Operation Re-Execution And Replay](../worth-query/docs/domain-capabilities/installed-operation-reexecution-and-replay.md)
for the execution model, examples, and current limits.
