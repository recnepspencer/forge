# Retention and recovery contract

Runtime World accounts for bounded populations with one installed
`RuntimeWorldBudgets` value. The limits cover live product branches, retained
composite commits, Runtime World metadata, active observations, active
publication attempts, retained product-unpublished records, retained-partial
metadata, unique exact component pins, in-flight pin reservations, and
owner-created component custody records. Zero limits are rejected during
installation; no `Default` can silently omit a bound.

Phase 1 does not pin a component or issue a component-owner lease. It freezes
independent Relational and Signal exact-admission keys, the dependency classes,
and the opaque move-only handoffs consumed by observation, publication, and
recovery signatures. A composite identity is never a component registry key,
and a serializable Signal descriptor is descriptive transport only. The Phase
1 registry root contains no operational map, lease, dependency-count, transfer,
or release behavior; Phase 2 owns those bounded semantics and may add them
behind the sealed contracts without changing consumers.
`ProductBranchObservation` clones share one observation obligation and never
reread a head. Signal currentness still belongs to the Signal owner basis
port.

`ProductUnpublishedOwnerEffects` is not a commit, rollback token, or replay
record. It records the exact expected and last observed heads, both owner
progress postures, successor basis when present, live obligations, cause, legal
next actions, deadline/age, owner-effect count, and metadata-byte accounting.
Its `ProductUnpublishedRecoveryHandle` is a non-authorizing reference to that
record. Recovery cannot call an unperformed sibling or move a product
reference.

The Phase 1 retention root freezes the dependency classes, exact-pin request,
and obligation-transfer destinations for the later retention lane. It contains
no persistence, recovery cursor, background worker, or unbounded cleanup queue.
Close and explicit bounded reclamation are later lifecycle responsibilities.
