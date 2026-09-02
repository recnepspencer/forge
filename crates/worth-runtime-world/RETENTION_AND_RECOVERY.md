# Retention and recovery contract

Runtime World accounts for bounded populations with one installed
`RuntimeWorldBudgets` value. The limits cover live product branches, retained
composite commits, Runtime World metadata, active observations, active
publication attempts, retained product-unpublished records, retained-partial
metadata, unique exact component pins, in-flight pin reservations, and
owner-created component custody records. Zero limits are rejected during
installation; no `Default` can silently omit a bound.

Each component is pinned by its own Runtime World owner-issued admission
identity and binding. Relational and Signal keys are independent; a composite
identity is never used as their registry key, and a serializable Signal
descriptor is descriptive transport only. One exact component key has one
external owner lease; further commits or observations add bounded dependency
counts to that key. The opaque observation/publication/partial-retention
obligations are bound to the issuing Runtime World owner, own release through
RAII, and retain no component owner by themselves. A publication obligation
names the exact prospective basis and is transferred by the ready token; it
cannot be replaced by a caller-supplied receipt. The Phase 1 registry root
freezes this protocol; Phase 2 supplies the complete bounded map and owner
lease calls.
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
