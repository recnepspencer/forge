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
observation. The product reference selects its retained exact tuple even if a
component owner has since advanced. Only a requested component mutation checks
that owner's current exact basis; unchanged components require no owner contact.

The same exact owner-issued tuple has the same `CompositeBasisKey`; no separate
lookup index authorizes equivalence. Equal descriptors with distinct component
admission identities do not compare as the same composite basis.

Root bootstrap is the only operation that can establish the initial product
reference. Later history is single-parent in this milestone. The mutable
product reference is not the immutable commit, and history insertion alone is
not a product-reference movement.

Ordinary publication reserves a canonical performed envelope beside its history
entry before component effects. Its logical metadata charge includes the
preallocated facts and a conservative per-envelope charge for the retained
shared branch name; it is not a measurement of unique allocator-resident bytes.
The immutable commit remains separate from the later movement. The envelope
retains the exact old/new snapshots, full component results, transfer receipt,
late cancellation, and final publication counters only after the cell commits.
It is reclaimed with its entry, with final evidence destructors outside the
history lock. A live delivery claim carries explicit history protection; the
envelope stored inside the entry does not protect itself.

Admission allocates pending slots in the eventual ordered history and
reachability maps. Pending slots count once against capacity but remain hidden
from lookup, protection, parent admission, and reclamation. Installation fills
those slots in place, preserving the parent dependency acquired at reservation.
Reservation Drop removes both pending slots and releases its charge. Ordinary
publication performs promotion inside the branch's final comparison lock;
a stale comparison leaves the reserved storage uninstalled.

Exact branch reuse installs the source observation's existing commit while its
source reference still matches, without a new commit or owner effect. Explicit
fork plans create the requested component branches and a single-parent composite
commit; performed forks remain recoverable if the destination cannot install.

Retirement accepts an existing `ProductBranchObservation` as proof of the
installed occurrence. It compares owner, name, and incarnation, allowing later
head movement within that occurrence. An older occurrence cannot retire a
recreated name. The live registry needs no historical retired-name set.
