# Retention and recovery contract

Runtime World accounts for bounded populations with one installed
`RuntimeWorldBudgets` value. The limits cover live product branches, retained
composite commits, Runtime World metadata, active observations, active
publication attempts, retained product-unpublished records, retained-partial
metadata, unique exact component pins, in-flight pin reservations, and
owner-created component custody records. Zero limits are rejected during
installation; no `Default` can silently omit a bound.

The retention owner keys its unique leases by independent exact Relational and
Signal bases. A composite identity is not a component registry key. Repeated
use of a pinned component basis shares its existing lease; acquiring a fresh
obligation does not re-observe the component's current head.

Fresh product observations reserve the active-observation budget. Clones share
one observation obligation and its charge until the final clone drops. Creation
reserves the returned observation before an owner fork. Close reports outstanding
observations while their caller-owned obligations remain live.

`ProductUnpublishedOwnerEffects` is not a commit, rollback token, or replay
record. It records the exact expected and last observed heads, both owner
progress postures, successor basis when present, live obligations, cause, legal
next actions, deadline/age, owner-effect count, and metadata-byte accounting.
Its `ProductUnpublishedRecoveryHandle` is a non-authorizing reference to that
record. Recovery cannot call an unperformed sibling or move a product
reference.

Close exposes retained records and returns owner-created component retirement
work; it does not delete component branches. `OwnerLost` requires an actual
owner-unavailable denial. Retention capacity, arithmetic, or acquisition-panic
failures instead retain `RetentionAdmissionDenied`; destination authority
mismatches retain `DestinationAdmissionDenied`. Neither justifies owner closure.

The active-custody owner preinstalls an attempt record in the same bounded
recovery catalog. Caller abandonment atomically converts its reservation to
retained accounting before releasing operation admission. Inspection moves the
existing evidence and resources into the retained view without component calls.
`BindingReserved` means the component pair has not been bound;
`PublicationPinsRetained` preserves the original `ActivePublicationAttempt` pins
without claiming a dependency transfer. `ProductHeadPinsRetained` preserves
the original head-class claims when materialization preceded caller loss but
the product cell did not move. Abandonment retains any unused history capacity
until explicit cleanup, and close counts that capacity as a live obligation.

Ordinary production reservation, settlement, readiness, and final product
movement now carry this custody throughout their transitions. Tests cover real
two-owner settlement Drop, an unwind inside Signal's apply callback, invalid
ready-basis evidence, materialization and committed-boundary unwind, concurrent
inspection, close, and exact Relational identity repair. An explicit terminal
acquires its caller view while the catalog still owns the accounting conversion
lock, preventing cleanup from removing it before delivery.

Branch creation retains each actual fork and its exact destination in the same
record. Abandonment after destination assembly preserves its original head pair,
head history, observation pair, observation history, and recovery slot: seven
live obligations. Explicit cleanup releases those claims and returns the exact
component-retirement work. Registry-issued installation evidence survives
retirement and name reuse, so caller loss after insertion cannot expose a false
unpublished record. Recovery continues to settle or clean up only.

After a successful CAS, the history entry owns the canonical performed facts.
The owner can recover a caller's lost delivery without another component call
or product movement. Normal delivery and recovery claim the same exclusive
lane. Dropping an unconsumed delivery makes it available again; consumption
permanently closes that lane. A live claim keeps the entry and its metadata
charge retained even after branch retirement. Its old head is an exact
snapshot, not a permanently held active-observation obligation.
