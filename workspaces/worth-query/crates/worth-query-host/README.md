# worth-query-host

`worth-query-host` is the entry-band audience facade for applications and host
runtimes that install and execute Query behavior.

Use:

```rust
use worth_query_host::facade::{domain, runtime};
```

The host facade exposes the same installed-domain and portable conditional
authoring contracts as Query’s public domain facade. It does not expose Query
implementation modules, certification-only replay, or lower-runtime internals.

## Installed Domain Use

Host code may:

- declare portable domain packages and typed operations
- author semantic truth dependencies and conditional nodes
- register runtime executors, graph providers, Runtime Bridge, Signal graph,
  and exact conditional providers
- obtain a workspace and installed domain handle
- bind through the workspace-issued `workspace.observe_operating_world()` or
  `workspace.prepare_mutation_operating_world()` root
- execute the move-only operation or workflow progression
- run ordinary installed workflow re-execution
- admit and execute installed exact-inverse or compensation aftermath
- inspect trace-bound lineage and request sparse promotion from an exact
  carrying publication

Host code must not:

- create a second operating-world root
- call operation executors directly
- expose raw Signal node/aspect allocation as domain authoring
- reconstruct Query authority from receipts or digests
- import cert-only replay through the host facade
- construct lineage outcomes or promoted graph identities from raw identities,
  report rows, or digests

## Related Docs

- [Worth Query Orientation](../worth-query/docs/AI_README.md)
- [Runtime-Installed Domains And Operations](../worth-query/docs/domain-capabilities/runtime-installed-domains.md)
- [Conditional Installed Operations](../worth-query/docs/domain-capabilities/conditional-installed-operations.md)
- [Installed Operation Re-Execution And Replay](../worth-query/docs/domain-capabilities/installed-operation-reexecution-and-replay.md)
- [Installed Operation Aftermath](../worth-query/docs/domain-capabilities/installed-operation-aftermath.md)
- [Installed Operation Lineage And Promotion](../worth-query/docs/domain-capabilities/installed-operation-lineage-and-promotion.md)
