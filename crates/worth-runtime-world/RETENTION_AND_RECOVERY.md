# Retention and recovery contract

Runtime World accounts for bounded populations with one installed
`RuntimeWorldBudgets` value. The limits cover live product branches, retained
composite commits, Runtime World metadata, active observations, active
publication attempts, retained product-unpublished records, retained-partial
metadata, unique exact component pins, in-flight pin reservations, and
owner-created component custody records. Zero limits are rejected during
installation; no `Default` can silently omit a bound.

An exact component basis is pinned by its full Runtime World owner-issued
composite admission identity and binding. Repeated reuse of a basis is a
dependency-count change, not a new lease per commit. A serializable Signal
descriptor is descriptive transport only and cannot authorize a pin or prove
owner affinity; currentness belongs to the Signal owner basis port.
`ProductBranchObservation` clones share one internal observation admission
obligation and never reread a head. The retention registry and owner lease
behavior are implemented in the Phase 2 retention lane.

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
