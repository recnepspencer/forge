# worth-query-host

`worth-query-host` is the entry-band audience facade for applications and host
runtimes that install and execute Query behavior.

Use:

```rust
use worth_query_host::facade::{admission, domain, primary_graph, publication, runtime};
```

The host facade exposes the production Query authority graph without exposing
Query implementation modules, certification-only replay, or raw lower-runtime
internals.

## Installed Domain Use

Host code may:

- install portable domain packages, application schemas, typed queries, and
  typed operations;
- register the exact providers and lower-runtime adapters required by installed
  meaning;
- obtain the installed application and domain handles;
- resolve authenticated principals through installed principal bindings;
- admit application queries and operations through current capability,
  purpose, disclosure, conflict, and graph-obligation evidence;
- execute managed provider sessions and consume typed terminal outcomes;
- publish governed results;
- run ordinary installed workflow re-execution;
- inspect trace-bound lineage and request sparse promotion from an exact
  carrying publication.

Host code must not:

- create a second operating-world or application-authority root;
- call operation executors directly;
- expose raw Relational, Bridge, or Signal handles to application code;
- reconstruct Query authority from receipts, reports, projections, or digests;
- import certification replay through the host facade;
- construct lineage outcomes or promoted graph identities from raw identity
  material.

## Related Docs

- [WORTH Query Orientation](../worth-query/docs/AI_README.md)
- [Application Authorization And Emergency Elevation](../worth-query/docs/capabilities/application-authorization-and-emergency-elevation.md)
- [Runtime-Installed Domains And Operations](../worth-query/docs/domain-capabilities/runtime-installed-domains.md)
- [Canonical Graph Obligation Progression](../worth-query/docs/domain-capabilities/canonical-graph-obligation-progression.md)
- [Conditional Installed Operations](../worth-query/docs/domain-capabilities/conditional-installed-operations.md)
- [Installed Operation Re-Execution And Replay](../worth-query/docs/domain-capabilities/installed-operation-reexecution-and-replay.md)
- [Typed Stops And Remediation Guidance](../worth-query/docs/domain-capabilities/typed-stops-and-remediation-guidance.md)
- [Installed Operation Lineage And Promotion](../worth-query/docs/domain-capabilities/installed-operation-lineage-and-promotion.md)
