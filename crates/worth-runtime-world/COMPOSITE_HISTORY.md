# Composite history contract

Runtime World history has two separate meanings:

- `CompositeBasisKey` binds one Runtime World owner to the exact
  owner-issued Relational, Signal, and Bridge admission identities. It is the
  composite equivalence key; descriptors are descriptive only.
- `CompositeCommitIdentity` names one immutable commit occurrence.

Equal bases may therefore appear in distinct commits. A commit carries its
owner-issued identity, one `Root` or `OrdinaryParent`, the exact admitted
composite basis, Relational and Signal change postures, changed-owner
publication identities, the admitted Bridge basis, root/publication
provenance, and optional descriptive caller correlation. Caller correlation
does not authorize a commit.

`ProductBranchIdentity`, lifecycle incarnation, reference generation, and
selected commit remain separate meanings. `ProductBranchObservation` compares
all of them together with the exact owner-issued composite admission. A branch
name, commit id, generation, digest, or descriptor alone is not a product-head
observation. Signal currentness is admitted only through the Signal owner basis
port's exact current comparison.

Repeated admission of the same live owner-issued tuple reuses its composite
identity. Equal descriptors with a distinct component admission identity do
not compare as the same composite basis.

Root bootstrap is the only operation that can establish the initial product
reference. Later history is single-parent in this milestone. The mutable
product reference is not the immutable commit, and history insertion alone is
not a product-reference movement.

The Phase 1 contract intentionally exposes no catalog, traversal, persistence,
or reclamation implementation. Those responsibilities must consume these
types without introducing a second history or currentness authority.
