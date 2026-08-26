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
- consume fresh, recovered, partial-effect, and indeterminate commit outcomes;
- publish governed results and closed application-aftermath posture;
- bind an exact host predicate provider, named clock, and authoritative
  temporal-intent reconstruction contract before application-runtime
  publication;
- submit typed observations through a runtime-bound clock port while Signal
  owns wake eligibility and Query freshly admits the installed operation;
- reinstall derived temporal wakes from current authoritative domain truth and
  inspect non-authoritative lifecycle, work, and provenance evidence;
- inspect base-binding, complete runtime-installation, and fresh-admission
  canonical work through the carried clock, runtime-inspection, and provenance
  surfaces;
- dispatch declared external effects only from co-committed outbox facts;
- inspect, resolve, safely retry, dispose, or expire an exact receipt-bound
  runtime recovery handle;
- admit reconciliation or compensation against exact owner authority without
  claiming that Query executes the corrective effect;
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
- treat acknowledgement, timeout, disconnect, or lost response as external
  completion;
- serialize a recovery handle or treat its opaque wire identity as live
  authority;
- teach `facade::provisional_aftermath` as accepted undo/redo support.
- schedule temporal work locally, return raw Signal decisions, or invoke a
  conditional operation directly.
- invent a host-local temporal binding or idempotency hash, or derive either
  again during commit.

## Application Readiness For Editors

An editor or transport host that owns an installed primary-graph application
may inspect its current descriptive basis before presenting or submitting
work:

```rust
let readiness = application.inspect_application_readiness()?;
let optimistic_basis = readiness.basis_token();
```

The snapshot identifies the installed schema binding and current Query basis.
Query releases the inspection lease before returning it. The basis token is an
optimistic transport precondition only: it carries no query, mutation,
installation, or basis authority. Execution must still enter through the typed
Query application adapter, which performs fresh authorization, projection,
admission, and currentness checks.

## Related Docs

- [Ordinary Application Front Door](../worth-query/docs/foundations/ordinary-application-front-door.md)
- [WORTH Query Orientation](../worth-query/docs/AI_README.md)
- [Application Authorization And Emergency Elevation](../worth-query/docs/capabilities/application-authorization-and-emergency-elevation.md)
- [Runtime-Installed Domains And Operations](../worth-query/docs/domain-capabilities/runtime-installed-domains.md)
- [Canonical Graph Obligation Progression](../worth-query/docs/domain-capabilities/canonical-graph-obligation-progression.md)
- [Conditional Installed Operations](../worth-query/docs/domain-capabilities/conditional-installed-operations.md)
- [Installed Operation Re-Execution And Replay](../worth-query/docs/domain-capabilities/installed-operation-reexecution-and-replay.md)
- [Typed Stops And Remediation Guidance](../worth-query/docs/domain-capabilities/typed-stops-and-remediation-guidance.md)
- [Installed Operation Lineage And Promotion](../worth-query/docs/domain-capabilities/installed-operation-lineage-and-promotion.md)
- [Application Aftermath, External Effects, And Recovery](../worth-query/docs/execution/application-aftermath-and-recovery.md)
