# Relational Owner Component Port

This document freezes the Relational component boundary that Query Milestone
9.17.2 may consume. It is a port over owner-issued artifacts and typed outcomes,
not access to Relational internals and not a composite commit implementation.

The complete owner mental model is in
[`BRANCH_LOCAL_MVCC.md`](./BRANCH_LOCAL_MVCC.md). The executable publication
flow is [`examples/branch_local_mvcc.rs`](./examples/branch_local_mvcc.rs).

## Ownership rule

Relational alone owns:

- runtime and branch identity;
- the branch-reference catalog, lifecycle, generation, and current root;
- schema admission, transaction validation, and immutable root construction;
- canonical Relational commits, history, patches, snapshots, and replay input;
- branch-local compare-and-publish and publication settlement; and
- component retention and reclamation accounting.

A later composite owner may hold, compare, retain, and coordinate artifacts
issued by this port. It cannot mint them, rewrite their fields, derive identity
from a digest or version, choose a Relational head from a proxy, or mutate a
component branch directly.

## Artifacts available to 9.17.2

| Artifact | Meaning | Authority posture |
| --- | --- | --- |
| `RelationalBranchIdentity` | One runtime-owned mutable reference identity | Selects only after owner validation |
| `RelationalBranchBasisDescriptor` | Serializable exact target and generation | Descriptive; must be readmitted |
| `AdmittedRelationalBranchBasis` | Exact owner admission with retained immutable root | Accepted by governed component operations |
| `RelationalBranchObservation` | Exact read selection derived from an admitted basis | Opens owner snapshot reads, not writes |
| `RelationalBranchRetentionLease` | External obligation preserving one exact target | Runtime-affine and single-release |
| `PreparedRelationalCommitCandidate` | Validated, branch-bound, single-use proposed successor | May enter only its owner's publication port |
| `RelationalPublicationOutcome` | Typed result of owner compare-and-publish | Distinguishes movement from no movement |
| `PerformedRelationalCommit` | Non-cloneable proof of in-process component movement | Must return to the owner for settlement |
| `RelationalForkOutcome` | Exact source/target/provenance observation after owner fork | Evidence only; does not mint a basis |
| lifecycle outcomes | Owner result for archive or delete | Preserve pending versus completed deletion |

Canonical encodings and digests aid transport and comparison. They do not
replace owner identity, readmission, retention, or currentness.

## Observation and readmission

The composite owner starts from an explicit `RelationalBranchIdentity` and
asks Relational to `observe_branch`. If a descriptor crosses a storage,
process, message, or trust boundary, it returns through
`readmit_branch_basis`. When an external lease already preserves the exact
target, `readmit_retained_branch_basis` checks both the descriptor and lease
against the same owner.

Readmission can deny foreign runtime, identity mismatch, stale generation,
wrong immutable target, unavailable or archived branch, unavailable retained
target, and bounded retention failures. Callers must preserve these classes;
flattening them into a generic "head changed" result destroys retry and safety
semantics.

## Retention contract

Before a composite artifact promises that a component basis remains usable,
it acquires `retain_component_basis`. Ownership of the returned lease is part
of the composite artifact's lifecycle. Exactly one of these terminal actions
must occur:

- transfer the lease into the performed successor artifact;
- release it with `release_component_basis`;
- return it in a typed denial when owner mismatch prevents release; or
- let its explicit terminal accounting report owner loss.

A lease preserves residency, not freshness. The branch may advance while an
older exact target remains retained. A composite owner must compare or readmit
before treating any basis as current.

## Preparation and publication

The component publication progression is:

```text
admitted predecessor basis
  -> begin_branch_transaction
  -> prepare_branch_transaction
  -> PreparedRelationalCommitCandidate
  -> publication_port.compare_and_publish
  -> Performed | Stale | Denied | Interrupted | Deferred | Failed
  -> settle_performed_publication, only for Performed
```

Preparation performs validation and immutable-root construction without
moving the branch. The candidate is opaque and can be consumed only once.
`compare_and_publish` is the Relational linearization point; the composite
owner must not infer movement from candidate creation, a reserved commit ID,
patch bytes, or diagnostics.

Outcome handling is exact:

- `Performed` means the Relational reference moved. It carries the canonical
  component commit and successor admitted basis. Cancellation reported after
  linearization is attached to this performed evidence.
- `Stale` means expected and observed reference observations differ and no
  component movement was performed by this attempt.
- `Denied`, `Interrupted`, `Deferred`, and `Failed` are no-movement results for
  this candidate. They retain their typed owner reason.

After `Performed`, the component owner must run
`settle_performed_publication`. Settlement acknowledges durability posture,
history/patch availability, and optional projection repair through the one
canonical commit route. A composite layer cannot relabel an unsettled
component movement as rollback or erase it because a sibling component failed.

## Cancellation contract

Before Relational linearization, cancellation or timeout returns a typed
`Interrupted` outcome and preserves the predecessor reference. During or after
the critical section, movement wins: the owner returns `Performed` with
`late_interruption` evidence, and settlement remains mandatory.

This distinction is the handoff needed by coordinated publication. It prevents
a composite owner from claiming "nothing happened" after one component has
already moved. Milestone 9.17.2 must use its own typed prepare/compatibility/
publication protocol to ensure no half-current product world; it may not
simulate component rollback through raw Relational mutation.

## Fork and lifecycle posture

Fork consumes an owner-issued `AdmittedRelationalForkSourceBasis`, creates one
new reference, and shares the immutable source root. A later composite owner
may request or record this outcome but cannot manufacture the target cell.

Archive denies new ordinary work after its metadata movement. Delete forbids
the main branch and may return `WaitingForActiveOperations` while retained
transactions, candidates, snapshots, performed settlements, or external leases
remain. Composite cleanup must retain this pending state and retry through the
owner; it cannot delete owner catalog entries itself.

## Allowed 9.17.2 integration

Milestone 9.17.2 may:

- carry exact Relational and Signal component descriptors in one Bridge-owned
  correspondence;
- readmit and retain each component basis through its owner;
- prepare component work without claiming currentness changed;
- compare component predecessor observations with the correspondence;
- consume typed component publication outcomes;
- publish a Bridge-owned product reference only through its specified
  coordinated protocol; and
- transfer or release every component retention obligation explicitly.

Milestone 9.17.2 may not:

- construct or deserialize an admitted Relational basis;
- select a branch through `None`, a raw name, a version, a commit ID, a digest,
  a snapshot, or an ambient `"main"` fallback;
- accept a generic `AuthorityMarker` as component authority;
- call private root, branch-cell, history-catalog, or raw publication mutation;
- create a second Relational currentness table inside Bridge or Query;
- expose a combined component authority as though Relational issued it;
- treat prepared, stale, denied, or interrupted work as performed; or
- promise physical persistence or restart recovery for the in-memory owner.

## Memory and durability boundary

Relational branch cells, roots, owner admission, and retention accounting are
memory-resident. Canonical history and durability settlement remain
Relational-owned, but restart durability for this branch-owner model is
deferred to Worth Store integration. The composite owner must preserve this
posture rather than inventing a serialized owner token or hidden persistence
abstraction.

Signal's corresponding port is documented in
[`../worth-signal/BRANCH_BASES.md`](../worth-signal/BRANCH_BASES.md#owner-component-port).
