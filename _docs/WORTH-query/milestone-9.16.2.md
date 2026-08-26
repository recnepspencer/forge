# Milestone 9.16.2: Portable Packages And PostgreSQL Runtime Durability Foundation

> **Status:** Proposed — begins after Milestone 9.16.1.1 and is required before
> the Milestone 9.17 sequence
>
> **Product posture:** This milestone gives Workflow Editor a production-usable,
> restart-safe deployment now and establishes the runtime-level PostgreSQL
> composition boundary that Milestones 9.17.1 through 9.17.3 extend without
> moving the facade or weakening durability. It does not wait for Worth Store
> and is not Worth Store integration.

## Goal

Let a host publish an exact Query workflow release, start the current ordinary
runtime against PostgreSQL, durably acknowledge mutations, survive process or
machine loss, and resume Query's existing co-committed external-effect outbox.
Establish the stable physical composition and owner-defined durability seams
that the committed 9.17 component and composite owners extend additively.

The milestone establishes:

- stable semantic identities for every Rust type axis in portable package meaning;
- declaration-minted operation and effect provenance;
- a complete Query-owned typed decomposition and bounded reconstruction path;
- fresh Query validation and expected-identity comparison after reconstruction;
- one deterministic neutral release archive for GitHub and cross-store transfer;
- a Relational-owned pluggable canonical durability boundary;
- a production `worth-runtime-postgres` physical-adapter crate with distinct
  Query-package, Relational, dispatch, connection, and operations boundaries;
- four narrow provider-neutral persistence contracts for canonical Relational
  durability, immutable release storage, runtime-stream lifecycle, and dispatch
  coordination;
- a Query-execution-owned persistent opening and restart barrier that invokes
  owner recovery and rebinds fresh host authority before traffic is admitted,
  reexported by the leaf `worth-query-host` audience facade;
- restart-stable dispatch of existing Query outbox rows using their existing
  correlation and idempotency meaning; and
- committed successor contracts through which 9.17.1 adds Signal and
  branch-qualified component durability, 9.17.2 adds durable Runtime Bridge
  composite history/currentness, and 9.17.3 cuts Query recovery and dispatch
  eligibility to the performed composite publication.

The result is immediately useful without Worth Store. A Workflow Editor release
can be compiled, signed, installed, executed, killed after any acknowledged
commit or dispatch boundary, and recovered by a fresh process against
PostgreSQL. A future Worth Store adapter can reuse these logical contracts
without requiring a semantic rewrite.

## Roadmap Placement And Append-Only Rule

Milestone 9.16.2 is the durability corrective interstitial after Milestone
9.16.1.1.
It consumes the portable installation grammar established by Milestone 9.13,
the authority decomposition established by Milestone 9.13.2, the installed
workflow meaning established through Milestones 9.14 and 9.16, the
application-aftermath and co-committed outbox completed by 9.16 Runtime Phase
8, the closed Milestone 9.16.1 authority-convergence substrate, and the typed
installed graph-contract integrity repair specified by Milestone 9.16.1.1.

Milestone 9.16 retains its recorded statuses and remains open for Bank Phase 6
and Closure Phase 1. Milestone 9.16.2 begins only after 9.16.1.1 closes; it may
then proceed beside those 9.16 phases where boundaries do not overlap. The
roadmap does not advance to 9.17.1 until both 9.16 and 9.16.2 close.

This milestone does not reopen prior closure or move PostgreSQL into Query,
Relational, Signal, or Runtime Bridge semantics. It adds the immediate adapter
and stable parent topology. The 9.17 specifications must extend that topology
as their new owners appear; they may not defer durable component or composite
restart and thereby regress this milestone's ordinary durability.

## Central Claim

After Query acknowledges a PostgreSQL-backed application mutation, a fresh
process can recover the exact current product publication, its owner state, and
the existing Query outbox row, freshly install the expected Query package, and
resume eligible dispatch through a fenced claim without old process authority.
For the 9.16.2 runtime, the performed product publication is the Relational
publication. After 9.17.3, it is exclusively the Runtime Bridge performed
composite publication; the dispatch contract and runtime-level facade do not
move when that source becomes stronger.

The claim is false if:

- PostgreSQL receives a best-effort mirror after an in-memory commit;
- a mutation can be acknowledged before its canonical Relational commit is
  durable in PostgreSQL;
- the existing performed-but-durability-deferred settlement or its runtime-id-
  bound repair handle survives as an ordinary or recovery path;
- an adapter, archive, digest, or signature can mint Query authority;
- the adapter creates a second authoritative outbox payload instead of
  discovering and claiming Query's co-committed outbox row;
- an owner-local Relational commit remains sufficient to dispatch after
  Runtime Bridge composite publication becomes product currentness;
- an old commit receipt or runtime-local recovery handle is needed after restart;
- a stale, expired, or unfenced claimant can dispatch or acknowledge an effect;
- a release name selects "latest" instead of the expected semantic identity;
- Query, Relational, Signal, or Runtime Bridge owns SQL, migrations, pools,
  tables, indexes, leases, or retry policy;
- any upstream owner or `worth-query-host` depends on `worth-runtime-postgres`;
- one generic database trait erases the distinct authority, atomicity, and
  lifecycle contracts of owner durability, release storage, and dispatch;
- a persistent Query operation can pass resource admission without reserving
  the exact commit-provider capacity subject that bounds its durability lane;
- pool, queue, or transport saturation after a PostgreSQL transaction may have
  begun is reported as ordinary `Backpressured` or as permission to retry under
  a new mutation identity;
- PostgreSQL owns workflow semantics, validation, closure, or package digest;
- operators cannot index exact releases or pending dispatch work; or
- archive, recovery-scan, or global pending-work cost appears on warm execution.

## Current Boundary

The architecture already supplies the semantic core:

- `worth-query-declaration` owns callback-free schema, operation, effect, and
  workflow declaration meaning.
- `worth-query-installation` owns portable packages, validation, canonical
  identity, and private proof-bearing validated packages.
- `worth-query-host::facade::domain` is the legal package-install audience.
- `worth-query-execution` already creates the committed-dispatch outbox intent
  in the same Relational worker-intent batch as the application mutation.
- Relational publication already creates one canonical commit envelope, but a
  current independently borrowable publication path can finalize in-memory
  publication after durable append failure and return
  `performed_but_durability_deferred` with a `runtime_instance_id`-bound repair
  handle. That is present reality, not the destination durability law.
- Relational already owns checkpoint and replay recovery law for in-memory and
  persisted local-filesystem modes, and its canonical commit already carries a
  branch identity, but its append authority and deferred repair remain process-
  affine.
- Query already exposes `WorthQueryCommittedDispatchOutboxObservation` through
  `observe_committed_dispatch_outbox` and admits external dispatch only through
  current runtime authority.

The missing contracts are narrower than a new workflow engine and broader than
a package codec:

- no complete public typed package decomposition/recomposition contract;
- package-canonical Rust `type_name` values are not stable semantic identities;
- operation/effect references lack declaration-minted membership provenance;
- Relational durability has no narrow public backend implementation port;
- Relational has no compiler-visible prepare-persist-publish progression that
  prevents in-memory publication before admitted physical durability, and its
  deferred-settlement lane cannot survive a fresh process honestly;
- no PostgreSQL implementation of canonical append, checkpoints, and recovery;
- no store-neutral immutable release repository contract separated from host
  release trust and activation choice;
- no Query-execution-owned dispatch coordination contract for discovery,
  claiming, fencing, attempts, outcomes, and reconciliation;
- no public Query install path over a freshly recovered Relational runtime;
- existing outbox observation is bound to the live
  `relational_runtime_instance_id` and commit receipt, so a new process cannot
  discover and claim pending work; and
- no runtime-level composition root coordinates release verification, owner
  recovery, provider rebinding, outbox reconciliation, and readiness without
  making Query the physical-runtime owner.

The existing Query outbox is preserved. New work makes it durable and freshly
claimable; it does not replace it.

## Ownership Lock

| Responsibility | Owner |
|---|---|
| Workflow, schema, operation, effect, policy, artifact, and contribution meaning | Query declaration and installation |
| Stable portable identities, package validation, ordering, closure, work bounds, and semantic digest | Query |
| Typed package records and untrusted reconstruction candidates | Query installation |
| Application execution, existing outbox meaning, and fresh dispatch admission after performed product publication | Query execution |
| Relational component commit, co-committed outbox fact, durable-before-success law, checkpoint/replay order, and recovery validation | Relational |
| Pluggable canonical durability port | Relational |
| Signal component basis, publication, durable artifact, and recovery law beginning in 9.17.1 | Signal |
| Composite commits, product branch heads, product currentness, coordinated publication, durable artifact, and recovery law beginning in 9.17.2 | Runtime Bridge |
| SQL, migrations, transactions, rows, indexes, pools, retention operations, and database diagnostics | `worth-runtime-postgres` owner-specific adapters |
| Recovery dependency contract | Each semantic owner; Runtime Bridge owns component-before-composite ordering after 9.17.2 |
| Persistent opening, recovery invocation, fresh host binding, readiness, and lifecycle | Query execution; reexported without implementation by `worth-query-host` |
| Stable caller audience for persistent Query runtime opening | Leaf `worth-query-host` facade; reexports only and owns no runtime module |
| Release signatures, signer trust, Git provenance, registry revision, and activation policy | Host release system |
| Store-neutral release archive bytes and compatibility | `worth-query-package-archive` |
| Immutable exact-identity archive repository contract | `worth-query-package-archive`; descriptive storage only |
| Physical snapshot completeness | PostgreSQL adapter |
| Fresh acceptance of reconstructed package meaning | Query |
| Dispatch candidate, admission, claim/fence progression, retry/outcome meaning, and coordination port | Query execution |
| Execution-resource support, exact capacity-subject identity, atomic multi-provider reservation, and `Backpressured` admission | Query admission and execution |
| Physical durability/dispatch capacity subjects, bounded waits, pools, and release mechanics | Selected physical adapter; non-authoritative resource lifecycle only |
| SQL realization of release repository and dispatch coordination | PostgreSQL adapter beneath the owner contracts |
| External outcome and idempotency behavior | External-effect owner under Query's effect contract |
| Future graph topology, replication, and distributed capabilities | Worth Store adapter and Store |

Completeness is split but composed:

```text
signed release -> adapter verifies transport and emits Query records
    -> Query reconstructs candidate -> Query validates and identifies package

PostgreSQL artifacts -> adapter proves ordered recovery input
    -> each semantic owner verifies and reconstructs its current authority
    -> runtime composition orders owner recovery
    -> Query installs package and rebinds current providers
    -> adapter reconciles pending-work index
    -> Query joins the existing outbox row to performed product publication
    -> Query freshly admits a fenced claim
```

No proof in one layer substitutes for another.

## Public Contract And Dependency Lock

Database portability is a contract property, not a promise to generalize the
PostgreSQL schema later. This milestone freezes four separate ports because
they have different authorities, truth classes, atomicity needs, and replacement
fates:

1. Relational defines a canonical durability port for append, checkpoint, and
   ordered recovery input. Only a Relational-issued append request can enter
   it, and successful physical storage still cannot mint publication authority.
2. `worth-query-package-archive` defines an immutable exact-identity archive
   repository port. It stores and returns descriptive signed envelopes; Query
   reconstruction and current host trust still decide whether bytes may run.
3. Query execution defines a runtime-stream lifecycle port for
   generation-qualified activation, exact package/owner-format/schema/provider
   binding, and conditional compatibility transitions. It consumes concrete
   host release authority but never chooses host policy or validates a package.
4. Query execution defines a persistent dispatch coordination port for bounded
   candidate discovery, conditional claim/fence progression, attempts,
   outcomes, and reconciliation. It consumes Query-issued admission and never
   accepts an adapter-authored payload or adapter-decided eligibility.

There is no `DatabaseBackend`, generic key-value bag, shared transaction object,
SQL-shaped upstream request, or adapter callback into private authority. The
composition root supplies four implementations explicitly. A future database
adapter implements only the contracts it can satisfy and may use transactions,
compare-and-swap, conditional writes, or an equivalent primitive internally.
It must report capabilities and fail before readiness when durable append,
atomic claim-and-fence, conditional current-fence outcome recording, ordered
recovery, or namespace isolation cannot be honored. Portability never means
weakening the contract to the least capable database.

The Rust-facing contracts have this semantic shape:

```rust
trait RelationalDurabilityBackend {
    fn persist_prepared(
        &self,
        request: RelationalPreparedDurableAppendRequest,
    ) -> PhysicalRelationalAppendOutcome;

    fn store_checkpoint(
        &self,
        request: RelationalCheckpointStoreRequest,
    ) -> RelationalCheckpointStoreOutcome;

    fn read_recovery_artifacts(
        &self,
        request: RelationalRecoveryReadRequest,
    ) -> RelationalRecoveryReadOutcome;
}

trait QueryPackageArchiveRepository {
    fn store_exact(
        &self,
        record: SignedQueryPackageArchiveRecord,
    ) -> QueryPackageArchiveStoreOutcome;

    fn load_exact(
        &self,
        request: ExactQueryPackageArchiveRequest,
    ) -> QueryPackageArchiveLoadOutcome;
}

trait PersistentRuntimeStreamCatalog {
    fn record_activation(
        &self,
        request: AuthorizedExactReleaseActivation,
    ) -> QueryPackageActivationOutcome;

    fn bind_stream(
        &self,
        request: AdmittedPersistentRuntimeStreamBinding,
    ) -> PersistentRuntimeStreamBindingOutcome;

    fn load_binding(
        &self,
        request: ExactPersistentRuntimeStreamRequest,
    ) -> PersistentRuntimeStreamBindingLoadOutcome;
}

trait PersistentDispatchCoordinator {
    fn discover(
        &self,
        request: BoundedPendingDispatchDiscovery,
    ) -> PendingDispatchDiscoveryOutcome;

    fn claim(
        &self,
        request: CapacityReservedQueryDispatchClaim,
    ) -> DispatchClaimOutcome;

    fn record_attempt(
        &self,
        request: CurrentFencedDispatchAttempt,
    ) -> DispatchAttemptOutcome;

    fn record_outcome(
        &self,
        request: CurrentFencedExternalOutcome,
    ) -> DispatchOutcomeRecording;
}
```

Requests are owner-constructed constrained values, not public field bags.
Recovery reads return descriptive owner artifacts for owner readmission.
Archive reads return untrusted bytes. Runtime-stream activation consumes the
concrete `HostReleaseActivationAuthority` without deciding host policy or Query
validity. Dispatch discovery returns descriptive candidates; only `claim`
consumes a Query-admitted claim carrying a move-only capacity reservation, and only current-fenced
attempt/outcome requests can advance operational lifecycle state. Lease renewal
and reconciliation follow the same fence-qualified contract and may be split
into responsibility-named traits if implementation evidence shows distinct
lifecycle owners; they may not collapse into a generic persistence method.

Relational durability is a compiler-visible owner progression, not one method
called after publication:

```text
RelationalPreparedPublication
    -> RelationalPreparedDurableAppendRequest
    -> PhysicalAppendNotStarted
    -> PhysicalAppendStarted
    -> PhysicalRelationalAppendOutcome
    -> RelationalDurabilityAdmittedPublication
    -> RelationalCommitOutcome
```

Only Relational constructs the prepared request from its validated canonical
commit, exact branch-qualified owner scope, request/idempotency identity, and
current owner writer fence. Only the adapter crosses
`PhysicalAppendNotStarted -> PhysicalAppendStarted`; that transition is the
typed point after which Query `Backpressured` is unavailable. A persisted or
already-persisted physical outcome returns descriptive confirmation bound to
the exact request. Relational verifies it and alone constructs the terminal
owner outcome. A performed conclusion consumes the fence, installs the visible
root, and mints publication; proven no-effect, conflict, and rejection release the exact
writer generation for fresh admission; only `Indeterminate` retains it.

Phase 5 deletes `settle_performed_publication`,
`performed_but_durability_deferred`, runtime-instance-bound deferred settlement,
and every repair entry that first performs semantic publication and later tries
to append durability. No wrapper or compatibility alias may preserve that
ordering. Local filesystem, PostgreSQL, and future physical implementations all
sit beneath the same prepared-persisted-published progression.

The physical boundary exposes distinct start and settlement outcomes:

```rust
enum PhysicalAppendStartOutcome {
    Started(PhysicalAppendStarted),
    NoEffectRejected(PhysicalAppendPreEffectDenial),
}

enum PhysicalRelationalAppendOutcome {
    Persisted(PhysicalAppendConfirmation),
    AlreadyPersisted(PhysicalAppendConfirmation),
    AbortedNoEffect(PhysicalAppendAbortedNoEffect),
    Conflict(PhysicalAppendConflict),
    Indeterminate(DurableCommitRecoveryLocator),
}
```

Exact names may reconcile, but `PhysicalAppendPreEffectDenial` is constructible
only while the adapter still owns `PhysicalAppendNotStarted` and independently
observes that no transaction effect began. After `PhysicalAppendStarted`, the
adapter must settle through `PhysicalRelationalAppendOutcome`; Query may never
translate that settlement back into execution-resource `Backpressured`. The PostgreSQL
certification oracle observes transaction start, commit visibility, and durable
rows independently of Query's returned variant.

### Durability Capacity And Backpressure Lock

The four persistence ports remain the complete semantic persistence boundary.
Capacity is not a fifth persistence authority and does not authorize append,
publication, activation, claim, or dispatch. Instead, a persistent composition
must bind each physical provider into Query's existing execution-resource
admission contracts before ordinary traffic can become ready.

Every installed Query operation whose admitted path can reach persistent
Relational publication carries an exact durability commit-provider entry in its
`WorthQueryExecutionResourceSupportSnapshot`. That entry is backed by the
existing `WorthQueryExecutionCapacityPort` contract or its reconciled public
name and by an adapter-owned bounded durability-admission capacity subject.
The composition root privately constructs one
`PersistentDurabilityProviderBinding` (or reconciled name) that owns the
Relational backend implementation and its exact capacity handle together. The
binding, not a string or support snapshot, proves that reservation and effect
execution reach the same physical provider instance. A descriptive support row,
a different pool, a representation-equal binding id, or a same-named capacity
object is not equivalent. `commit_providers` remains Query's semantic resource-
admission axis; it does not make the physical adapter a Query semantic provider.

Query's existing atomic capacity reservation acquires the executor, graph,
conditional, commit-provider, and parallel-admission subjects before a managed
run, provider session, Relational preparation, or physical effect begins. The
durability reservation bounds admitted or locally queued commit work. It is an
opaque resource reservation, not a database connection, transaction, commit
receipt, publication proof, or recovery authority. PostgreSQL connections are
acquired only at the named durability effect phase and are never held across
Query planning, authorization, invariant execution, live delivery, or external
transport.

For the initial synchronous PostgreSQL lane, one successful durability
reservation atomically acquires one durability-admission permit, one bounded
blocking-executor permit, and one permit from the durability-reserved partition
of the connection pool. It does not check out or hold a connection object. The
three admitted widths are equal in this milestone; archive, recovery, dispatch,
and operator work use separately budgeted partitions. Any future decoupling
must introduce an explicitly named bounded queue with numeric capacity,
deadline, cancellation, occupancy, and overflow outcomes before changing those
widths. No second wait or queue may exist between the Query reservation and
effect-phase provider entry.

The typed outcome boundary is phase-locked:

- failure to reserve the exact Query commit-provider capacity subject returns
  `WorthQueryExecutionResourceAdmissionDenialKind::Backpressured` before
  provider work, Relational preparation, connection checkout, or SQL;
- a standalone Relational caller that did not enter through Query still meets
  the same bounded adapter admission and may receive a typed capacity rejection
  only while the adapter proves that no transaction or durable effect began;
- pool closure, deadline expiry, or dependency loss before a transaction begins
  is a typed no-effect physical rejection or dependency failure, not an
  ambiguous commit;
- after a PostgreSQL transaction may have begun, saturation is never reported
  as ordinary Query backpressure and never grants a new-identity retry. The
  operation resolves only through performed, already-performed, rejected,
  conflict, or indeterminate commit posture according to what the adapter can
  prove; and
- an indeterminate result retains the exact recovery locator and owner fence
  needed to resolve commit fate, but it does not retain a database connection
  or leak a capacity reservation indefinitely. Releasing local capacity cannot
  release or reconstruct the unresolved owner writer fence.

Dispatch pressure is a separate operational lane. A worker reserves bounded
send/worker capacity before acquiring a dispatch claim; a slow or saturated
transport stops new claims or advances typed bounded retry posture while the
canonical outbox occurrence remains durable. No SQL transaction remains open
during a network send. Live-view and subscription backpressure remains a
derived-delivery policy and may retain within a window, report a gap, or
terminate a consumer; it cannot delay, undo, or weaken an otherwise admitted
durable publication.

The public outcome of an append whose client loses the response around commit
is typed. It distinguishes:

```rust
enum RelationalCommitOutcome {
    Performed(PerformedRelationalPublication),
    AlreadyPerformed(PerformedRelationalPublication),
    Rejected(RelationalCommitDenial),
    Conflict(RelationalCommitConflict),
    Indeterminate(RelationalIndeterminateCommit),
}
```

The physical backend returns only a private append confirmation, duplicate,
conflict, rejection, or indeterminate locator. Relational verifies that result
against its issued request and is the only layer that constructs the public
`RelationalCommitOutcome` and performed publication.

`AlreadyPerformed` is available only for the same stable request identity and
same canonical artifact. `RelationalIndeterminateCommit` carries a restart-
stable descriptive recovery locator plus a sealed, non-cloneable, exact owner-
scope writer fence. The live fence is authority and is never serialized. Its
persisted unresolved request/branch record lets a fresh Relational owner
reconstruct the blocked posture and issue fresh recovery authority before any
later writer can enter that scope. It never invites a retry under a new
identity. The exact public names may align with existing owner vocabulary, but
no ordinary failure variant may falsely mean "safe to try again as new work."

Dependency direction is mechanical:

```text
Relational facade -----------------------> owner durability port
Query package archive facade ------------> immutable archive repository port
Query execution facade ------------------> stream lifecycle + dispatch + opening
worth-query-host ------------------------> reexports Query audience surfaces
worth-runtime-postgres ------------------> implements the four owner ports
application composition root ------------> selects PostgreSQL implementations
```

None of Relational, Query installation, Query execution, package archive, or
`worth-query-host` may import `worth-runtime-postgres`. `worth-query-host` stays
facade-only under Road 1; adding `src/runtime`, a private recovery engine, SQL,
or adapter lifecycle there is a boundary failure.

Every Relational durability, checkpoint, recovery, unresolved-commit, and
retention request is qualified by adapter namespace, durable runtime stream,
and exact owner-issued Relational branch identity. In 9.16.2 the ordinary lane
uses the exact primary branch already carried by the canonical commit; this is
not permission for the catalog or adapter to choose a branch head. The runtime
stream groups package compatibility and owner lifecycle only. It never means
"current Relational world." Milestone 9.17.1 strengthens branch references and
concurrency behind this already branch-qualified owner scope without adding a
branch axis to the port or rekeying persisted families.

Important authority and non-authority artifacts are frozen as follows:

| Artifact | Constructed by / proves | Authorizes | Cannot authorize |
|---|---|---|---|
| Declaration-minted operation/effect reference | Owning Query declaration; exact membership and provenance | Entry into package validation for that declaration | Installation, execution, persistence, or dispatch |
| Untrusted package record set/candidate | Archive decoder and Query reconstruction progression; bounded descriptive closure | Fresh Query validation only | Installed meaning or runtime opening |
| Validated package and semantic identity | Query installation after fresh validation | Query installation under current runtime construction | Host trust, database access, or dispatch |
| Signed release envelope | Host release build/signing system; byte provenance under a named signer | Host trust evaluation only | Query validation or activation by itself |
| `HostReleaseActivationAuthority` | Concrete platform authority from `worth-proof`, minted under host release policy | Recording one exact generation-qualified activation | Query validation, owner recovery, or cross-namespace activation |
| `RelationalPreparedDurableAppendRequest` | Relational publication progression; exact canonical artifact, branch, writer fence, and request identity | One backend append attempt through the selected bound provider | Publication authority merely because storage succeeds |
| `RelationalIndeterminateCommit` | Relational after an effect-started append whose fate is unresolved; restart-stable locator plus exact owner-scope writer fence | Fate resolution through fresh-owner recovery while later writers remain excluded | New-identity retry, publication, or capacity ownership |
| Performed Relational publication | Relational after admitted durable append outcome | Current Relational owner use and the 9.16.2 publication carrier | Future Bridge composite currentness |
| Performed product publication carrier | Query execution, privately retaining current owner-performed evidence | Fresh dispatch-admission evaluation | Caller-selected publication or payload creation |
| Dispatch admission and current fence | Query execution after exact outbox/publication/binding checks and conditional claim | One current external-send/outcome progression | Another occurrence, namespace, runtime, or expired fence |
| Persistent runtime readiness | Query execution after the complete opening progression | Ordinary traffic admission | Any owner authority omitted from that progression |

Exact type names may be reconciled with existing vocabulary, but the concrete
platform authority placement and who can construct each state may not drift.
The activation surface cannot fall back to a generic `AuthorityMarker` bound.

## Persisted Truth And Lifecycle Ledger

Every persisted family has exactly one truth class. Implementation planning
must maintain this ledger down to tables and record families; an unclassified
row cannot ship.

| Persisted family | Truth class | Authority and rebuild posture |
|---|---|---|
| Signed package envelope and exact archive bytes | Canonical descriptive artifact | Package archive owns byte compatibility; host trust and fresh Query validation are still required |
| Host-selected active package identity and activation generation | Authoritative operational lifecycle state | Host release system decides; a row records but cannot choose or validate a release |
| Query package record projections and lookup catalogs | Rebuildable derived projection | Rebuilt from exact archive bytes; deletion cannot alter executable meaning |
| Relational commit envelopes and checkpoint artifacts | Canonical semantic-owner artifact | Relational defines format, ordering, validation, and recovery |
| Unresolved Relational append request, exact branch scope, writer generation, and observed physical fate | Authoritative operational lifecycle state | Relational defines the blocked-writer and recovery progression; the adapter conditionally persists its descriptive record, but only a fresh Relational owner may issue recovery authority or release the fence |
| Co-committed Query outbox occurrence, payload, correlation, and idempotency identity | Canonical owner artifact within Relational publication | Query creates the outbox meaning; adapter may only locate the exact occurrence |
| Pending-work locator and projection high-water marks | Rebuildable derived projection | Reconstructed from retained canonical outbox occurrences and owner publications |
| Current claim, lease, fence, attempt, next-attempt, and terminal/unresolved outcome | Authoritative operational lifecycle state | Query execution defines transitions; physical adapter performs conditional persistence |
| Readiness, health snapshots, counters, metrics, and diagnostics | Diagnostic sidecar | Open no authority and may be discarded without changing runtime truth |
| Migration ledger and physical compatibility version | Authoritative physical lifecycle state | Adapter-owned; cannot reinterpret an owner artifact or package archive |

Only rows classified as rebuildable derived projections may be destructively
rebuilt. Tests and operator tools must name the class they delete. Unresolved
outbox occurrence, exact source publication, payload, correlation, and
idempotency identity must remain reconstructible through checkpointing,
compaction, retention, backup, and restore. The implementation may carry that
closure in a checkpoint or retain its canonical source history, but it may not
prune the last authoritative source while work remains unresolved.

## Decisive Failure Court

Before any facade is accepted, the production persistent Bank world starts a
transfer that produces both double-entry state and an external payment-notice
outbox occurrence. The court checkpoints and compacts while that notice remains
pending, kills the application or PostgreSQL at controlled commit and dispatch
boundaries, deletes every process-local receipt, handle, and rebuildable
pending-work projection, and starts competing fresh runtimes against the same
durable stream. The milestone fails if the new runtime cannot reconstruct the
exact expected package and acknowledged balances, if ledger mutation and outbox
tear, if pending work requires an old runtime id, if the unresolved notice was
pruned, if two claimants can hold a current fence, or if a stale claimant can
send or acknowledge.

This court forces the implementation to put PostgreSQL in Relational's commit
path, reuse Query's existing outbox fact, and create fresh runtime authority.
A package codec, host-side dual write, shadow queue, serialized receipt, or
happy-path restart demo cannot pass it.

NCR remains a separate final reference-consumer cutover proof. It must install
the same public release and persistence surfaces and prove its state plus
notification journey, but it does not substitute for the Bank court's stronger
ledger/outbox oracle, compaction, contention, and external-rail adversity.

The successor mutation repeats this court after 9.17.3 with a Relational commit
followed by failed Signal preparation or Runtime Bridge product-head CAS. The
outbox remains exact owner state but ineligible and unsent. Deleting the Bridge-
publication gate or deriving its carrier from row existence must turn the
evidence red.

## Governing Runtime Design

### PostgreSQL is in the commit path

The Relational adapter implements Relational's canonical durable-artifact port.
A PostgreSQL transaction appends the canonical component commit and required
catalog metadata before Relational reports publication success. Query does not
commit in memory first and enqueue a later database write.

This is the first owner implementation under `worth-runtime-postgres`, not the
definition of product currentness for all time. Milestone 9.17.1 branch-
qualifies this path and adds the Signal owner adapter. Milestone 9.17.2 adds the
Runtime Bridge product-head store adapter, where the composite commit and head
CAS become durable before Bridge can mint performed composite publication.

Relational remains synchronous in this milestone. The adapter uses a bounded
pool of synchronous PostgreSQL connections for append and recovery. Async hosts
run Query on an explicit bounded blocking executor at the host edge. This does
not make Query or Relational accidentally async and does not allow one unbounded
blocking task per request.

The bounded blocking executor, durability-admission subject, and durability-
reserved pool partition are distinct resources bound by one declared operating
envelope. Query reserves their opaque composite permit before expensive
semantic work; the adapter checks out a connection object only when the issued
Relational append request reaches `PhysicalAppendStarted`. Their admitted
widths are equal in this milestone, and no hidden queue or second capacity
check sits between reservation and checkout.

The PostgreSQL adapter declares an admitted durability profile before readiness.
For the profile allowed to acknowledge durable publication, startup verifies at
least `fsync`, `full_page_writes`, and `synchronous_commit`, or an explicitly
documented equivalent or stronger posture for the supported deployment. An
incompatible posture is rejected or exposed as a typed downgraded profile that
cannot produce the durable acknowledgement claimed by this milestone.

Failure claims are separated rather than hidden behind "machine loss":

- application-process death with PostgreSQL still running;
- PostgreSQL-process death and restart;
- operating-system restart with the same durable storage;
- machine replacement from a verified backup/restore artifact; and
- permanent storage loss, whose admitted recovery-point and recovery-time
  limits must be stated rather than called lossless recovery.

The first four have explicit courts and operator procedures. The last has an
honest documented RPO/RTO posture; this milestone does not claim recovery from
destroyed canonical storage without a usable backup.

### Canonical artifacts and queryable projections are distinct

Each owner retains its canonical durable artifact and recovery authority.
PostgreSQL stores those artifacts in owner-qualified families and also stores
adapter-owned catalogs and projections so the runtime is operable and indexable:

- namespace and durable runtime-stream catalog;
- exact package identity and activation metadata;
- normalized package record families keyed by package, semantic key, and
  declared ordinal where order is meaningful;
- canonical commit sequence, envelope, checksum, and publication metadata;
- checkpoint payload, covered commit, checksum, and catalog;
- committed-successor Signal component artifacts and recovery catalogs;
- committed-successor Runtime Bridge composite commits, product-head CAS
  records, retention obligations, and recovery catalogs;
- derived committed-outbox discovery keyed to the canonical Query outbox and
  source commit;
- dispatch claim, fence, lease deadline, attempt, next-attempt, and outcome; and
- migration and projection-version state.

Relational emits and accepts its own versioned durable artifact representation;
the adapter never serializes private runtime objects. Relational owns artifact
format compatibility, checksums, canonical ordering, and bounded decode. The
adapter stores those artifacts exactly and may migrate only its surrounding SQL
schema and rebuildable projections. A Relational artifact-format migration must
be an explicit Relational compatibility path, not an adapter reinterpretation.

Package and outbox projections are derived or operational indexes. They do not
replace Query validation or any component/composition owner's recovery
authority and must be rebuildable from canonical owner artifacts. This makes
PostgreSQL queryable without making its topology platform meaning.

Checkpoint, compaction, and retention may remove only owner history already
covered by the checkpoint and not needed to reconstruct unresolved outbox work.
Before pruning, the operation proves that every unresolved outbox occurrence
and exact source-publication/idempotency binding remains in retained canonical
history or is carried canonically by the checkpoint. Backup and restore preserve
the same closure.

### Existing outbox, new restart claimant

The authoritative payload remains the outbox fact co-committed by Query in the
Relational transaction. The adapter may keep a derived pending-work index but
cannot accept a host-supplied shadow payload.

Dispatch requires a private Query `PerformedProductPublication` carrier that
retains the current product-publication owner's performed artifact. Query may
construct that carrier only by consuming owner evidence through the current
owner facade. In 9.16.2 it retains the performed Relational publication. In
9.17.3 it retains the performed Runtime Bridge composite publication and exact
Relational component basis containing the outbox. The carrier's meaning remains
stable while its lower-owner evidence becomes stronger; it is not a generic
caller-selected authority marker.

Consequently, a Relational outbox fact prepared for a composite operation is
durable but not dispatchable until Runtime Bridge product publication succeeds.
Failed Signal preparation, stale product head, cancellation, or failed composite
CAS leaves no eligible dispatch. Owner cleanup may later reclaim the orphan only
under the exact retention contract.

Query adds a bounded current-runtime discovery surface that observes committed
outbox facts by durable runtime stream and exact commit range, returning only
descriptive candidates until fresh claim admission. The adapter persists a
projection high-water mark and exact outbox/commit locators, not copied effect
payload authority. A background reconciler advances that cursor in order; the
startup barrier repairs any gap left by a crash before readiness. Ordinary claim
polling uses the indexed pending rows and never scans the complete commit log.

Discovery does not reserve external-send capacity. Before a descriptive
candidate may enter conditional claim, Query execution must reserve the exact
dispatch worker and transport capacity required to begin the attempt within the
lease posture. Saturation leaves the candidate pending and unclaimed. The
reservation is sealed as a move-only
`DispatchCapacityReservedClaimPermit`. Query execution combines that permit
with semantic claim admission into `CapacityReservedQueryDispatchClaim`, the
only request accepted by the coordination port. Successful claim settlement
consumes it into the current fenced attempt lifecycle; denial or failure
releases it explicitly. A database claim may never become a queue slot for work
that cannot yet begin.

After restart:

1. the adapter discovers a pending outbox identity and source commit;
2. Query observes the exact recovered outbox fact in the current runtime;
3. Query verifies package, operation/effect contract, correlation identity,
   exact product publication, and current dispatch eligibility, then issues
   semantic claim admission;
4. Query execution reserves exact worker/transport capacity and seals
   `DispatchCapacityReservedClaimPermit`;
5. `CapacityReservedQueryDispatchClaim` consumes the semantic admission and
   permit, and the adapter acquires a lease plus monotonically increasing
   fencing epoch in one bounded PostgreSQL transaction that closes on return;
6. Query verifies the claim result and consumes the retained permit into a
   current fenced dispatch attempt;
7. the adapter records the attempt in its own bounded transaction only if the
   fence is current;
8. transport sends the existing payload with the existing idempotency key;
9. the adapter records the outcome only if the fence is current;
10. any semantic aftermath that changes workflow state re-enters the ordinary
    Query publication path; and
11. the work index advances or is removed.

Claiming is at-least-once across crash ambiguity. Exactly-once external effect
is claimed only when the external owner honors the stable idempotency key. A
crash after send but before acknowledgement retries with the same key. A stale
claimant cannot acknowledge or begin a new send after its fence expires.

### Execution-owned recovery barrier

`WorthQueryHost::open_persistent` admits no traffic until it:

1. connects and verifies database/migration compatibility;
2. selects the release by independently expected package identity;
3. verifies signature/provenance under host policy;
4. decodes the neutral archive under limits;
5. reconstructs an untrusted candidate and freshly validates it;
6. rejects any expected/recomputed identity mismatch;
7. invokes every configured owner's recovery facade in dependency order;
8. in 9.16.2, asks Relational to readmit checkpoint, tail, and unresolved append fences;
9. after 9.17.2, additionally recovers Signal owner state and Runtime Bridge
   composite history/product heads only after referenced component bases exist;
10. installs the package through Query's recovered-runtime constructor;
11. verifies that the selected package's declared provider capabilities and
    binding contracts are supported by the supplied host bindings;
12. rebinds current providers, clock, authorizer, transport, and secrets;
13. rebuilds or verifies package and pending-outbox projections;
14. joins pending outboxes to performed product publications and enables fresh
   fenced claim admission; and
15. publishes readiness.

Any failure leaves the runtime unready. Providers and secrets are never
deserialized from storage. Query execution owns this progression and its typed
opening states. The host facade only reexports the audience surface; it cannot
validate component bases, mint composite currentness, or bypass Query dispatch
admission. `worth-runtime-postgres` supplies physical implementations and does
not own the recovery progression.

### Multi-tenant and release coexistence posture

Every physical key is qualified by adapter namespace and durable runtime stream;
owner artifacts and indexes are additionally qualified by their exact owner
scope, including Relational branch identity.
Package identity, not a mutable name, selects executable meaning. Multiple
releases coexist. Activation is a host-governed pointer updated transactionally
after the release is stored and freshly validated. One namespace cannot scan,
claim, replay, checkpoint, or activate another through the public API.

A durable runtime stream is bound to an exact package semantic identity,
owner-artifact format versions, required schema contracts, and descriptive host
provider-binding requirements. Activation may not reinterpret an existing
stream under different package meaning or incompatible provider contracts.
The binding selects no Relational branch head, Signal basis, or Runtime Bridge
product currentness; those remain owner-issued references beneath the stream.
Opening an incompatible release yields a typed migration-required or
unsupported-compatibility outcome before readiness; a pointer flip cannot
silently migrate owner state.

One database/schema per environment and shared-table namespace isolation are
both deployable. Database roles and row-level security are options; API-level
qualification and hostile cross-namespace tests are mandatory in both modes.

## Portable Package Contract

### Stable semantic identities and provenance

Every package-relevant marker type declares stable semantic identity. Module
paths and `type_name` are diagnostics only. Blank identities, collisions, and
incompatible redeclarations fail. Operation and effect references are minted by
their exact schema declaration with private membership provenance; same-spelled
forgeries cannot enter validated package meaning.

### Complete records and bounded reconstruction

Query exposes a versioned manifest and typed record families covering every
semantic input to package validation. Export is descriptive and exposes no
proof, installed handle, callback, provider, secret, runtime id, receipt, claim,
or dispatch authority.

Reconstruction consumes untrusted records with declared count, byte, nesting,
and canonical-work bounds. It rejects omissions, duplicates, illegal ordering,
cross-package references, unsupported versions, and trailing required meaning.
Closing yields only an unvalidated candidate. Ordinary Query validation derives
a fresh proof/digest and the caller separately supplies expected identity.

### Neutral release archive and versioning

`worth-query-package-archive` deterministically encodes the manifest and typed
record stream, is bounded and self-versioned, and contains no proof-bearing
type. The host signs an envelope containing archive bytes, expected Query
identity, compiler/toolchain metadata, release metadata, provenance, and the
descriptive provider-capability/binding requirements needed to install the
package. It never contains provider instances, credentials, or secrets.

The contract freezes independent envelope, manifest, and record protocol
versions; canonical field and record ordering; integer and text encodings;
duplicate/unknown-field posture; maximum bytes, records, nesting, and declared
work; checksum/signature coverage; and downgrade behavior. Golden byte vectors
and cross-version fixtures are authoritative compatibility evidence. An
unsupported required record or version fails closed before candidate creation.
Unknown optional data is accepted only when its versioned compatibility rule
proves that ignoring it cannot change package meaning. No decoder may silently
upgrade, reinterpret, or partially accept an archive.

Git tags and GitHub releases label human releases. Query's semantic digest
labels executable meaning. Neither is manually incremented in workflow source,
and neither substitutes for the other.

## Certified Persistent Bank World

The existing Bank world is extended through a new persistence-specific layer;
tests do not reuse its imperative seed values or hard-coded numeric identities
as authority. The fixture progression is explicit:

```text
BankPersistenceWorldDefinition
    -> CompiledBankPersistenceRelease
    -> ProductionSeededPersistentBankWorld
    -> CertifiedPersistentBankBaseline
```

Each arrow crosses the production declaration, package, installation, execution,
and persistence facades for that stage. The certified baseline exposes semantic
handles such as `world.alice`, `world.accounts.alice_checking`,
`world.operations.transfer`, `world.effects.payment_notice`,
`world.pending_notice`, and `world.runtime_stream`; scenario code never repeats
raw ids, digests, ordinals, database keys, or package identities.

The world supplies six named valid baselines:

- an empty persistent installation with one exact installed release;
- an ordinary operating bank with balanced double-entry accounts;
- a bank with one committed transfer and pending external payment notice;
- that pending-notice bank after checkpoint and lawful compaction;
- coexisting compatible and incompatible releases bound to distinct streams;
- a bank restored into a separate PostgreSQL database from a verified backup.

Corruption, cross-spliced releases, stale fences, and partial restores are named
invalid fixtures, never alternate valid baselines. Tests derive small causal
deltas such as submit transfer, commit notification-bearing transition, expire
claim, supersede fence, acknowledge external effect, checkpoint, compact, and
restore. Environment start, release installation, domain seeding, baseline
audit, scenario action, independent observation, and teardown are separately
diagnosable stages.

The oracle reads the external payment rail and an independently decoded
Relational durable artifact; it does not use a Query read result, pending-work
projection, adapter status row, or the code
path that produced the claimed outcome as its only evidence. It verifies exact
double-entry conservation, occurrence count, payload identity, idempotency key,
current fence, and absence of cross-namespace effects.

Cost is layered. Immutable release compilation and a supported PostgreSQL image
may be suite-scoped; each ordinary test receives an isolated database or schema
namespace and durable stream. Application-process crash cases may share the
server. PostgreSQL-process, storage, migration, and backup/restore cases receive
a dedicated cold container or runner lane. Authentik and full HTTP nodes are
used only by the final Bank/NCR product courts that make authentication or
cross-process consumer claims; phase-local durability proofs do not pay for
irrelevant services.

## Adversarial Courtroom

Certification uses public facades, a real PostgreSQL server, production adapter,
production archive, and separate processes. It kills writer or dispatcher at
each meaningful boundary and proves:

1. same-named distinct releases coexist and exact identity selects each;
2. module moves preserve identity while semantic changes alter it;
3. dropped, duplicated, corrupt, cross-spliced, oversized, unsupported, forged,
   and illegally ordered records fail closed;
4. archive, digest, signature, old proof, receipt, or physical snapshot cannot
   mint Query authority;
5. incompatible PostgreSQL durability settings or missing conditional-write
   capability cannot enter durable-ready posture;
6. a kill before PostgreSQL commit yields no acknowledged mutation;
7. a kill after commit but before response recovers complete mutation and outbox
   or neither, never torn state, and reports commit ambiguity honestly;
8. checkpoint plus tail equals uninterrupted Relational state;
9. pending outbox work survives checkpoint, compaction, projection destruction,
   backup, and separate-database restore with the same payload and idempotency;
10. corrupt, missing, duplicated, forked, or out-of-order artifacts fail before
   readiness;
11. missing/wrong package, owner-artifact version, schema contract, or provider
    binding fails instead of installing latest by name or reinterpreting a stream;
12. pending existing outbox work is found after all old handles are gone;
13. racing workers yield at most one current fence;
14. an expired worker cannot send or acknowledge after a higher fence exists;
15. crashes before send, after send, and before acknowledgement converge under
    the same idempotency identity without a second outbox payload;
16. poison work reaches existing unresolved/terminal posture without blocking
    unrelated work;
17. namespaces cannot access one another's release, history, or work;
18. derived indexes can be dropped and rebuilt without changing meaning;
19. unsupported migrations refuse startup without partial canonical mutation;
20. 4,096 unrelated packages and long history do not make exact lookup or claim
    globally linear;
21. warm execution performs no archive/decomposition/recovery scan;
22. application, PostgreSQL, operating-system-storage, and verified-restore
    crash models close under their stated durability/RPO posture;
23. the persistent Bank transfer and payment notice survive the complete hostile
    sequence under an independent ledger/external-rail oracle; and
24. a fresh process runs an NCR workflow, commits state and notification outbox,
    dies, recovers, resumes notification, and serves exact resulting state;
25. simultaneous persistent operations at the declared durability-capacity
    boundary admit exactly the reserved commit-provider width, reject excess
    arrivals as `Backpressured` before Relational preparation or SQL, and admit
    fresh work after reservations release; and
26. pool loss or saturation is no-effect only with independent proof; once a
    transaction may have begun, stalls or response loss never yield backpressure,
    and every terminal outcome consumes, releases, or retains its fence exactly;
27. the performed-but-durability-deferred lane and runtime-id-bound repair
    surface are absent, and mutating the progression back to publish-before-
    persistence fails the commit crash court;
28. equal commit ordinals and same-named branches from another namespace,
    stream, or Relational owner scope cannot append, checkpoint, recover, or
    satisfy an indeterminate fence; and
29. a fully stalled live subscriber makes zero progress while PostgreSQL commit
    completes, its transaction closes, and Query acknowledges; only post-
    publication retain/gap/terminate delivery posture may change.

Each numbered claim has a named sabotage mutation. At minimum, acknowledgement
before durable append, trust of claimed package identity, latest-by-name
selection, shadow outbox payload, database-status dispatch eligibility, stale
fence send/outcome, readiness before provider rebinding, pruning the final
pending-outbox source, descriptive capacity in place of atomic reservation,
post-transaction backpressure, publish-before-persist, runtime-id-bound
indeterminate repair, missing branch qualification, claim-before-send-capacity,
live fanout inside append, partial incompatible migration, and warm global scans
must each be made to fail their owning proof.

Testcontainers or the repository Docker harness must use a real supported
PostgreSQL version. An in-memory fake cannot close the milestone.

## Destination Directory And Crate Topology

Exact leaf names may align with existing laws, but ownership may not drift:

```text
crates/worth-proof/src/release/
  activation_authority.rs          # created; concrete host release authority

crates/worth-relational/src/durability/
  backend/
    contract.rs              # implementable ports; owner-issued append authority
    artifact.rs
    denial.rs
    work.rs
  progression/
    prepared.rs              # owner-issued append request before physical effect
    physical_effect.rs       # not-started/started and settled physical outcomes
    admitted.rs              # verified durability evidence consumed by publication
  indeterminate/
    fence.rs                 # exact owner-scope writer exclusion
    recovery.rs              # fresh-owner fate resolution
  authority.rs               # durable-before-publication law
  checkpoints/               # existing checkpoint meaning
  recovery/                  # existing reconstruction law
  log/                       # local filesystem behind the same port

workspaces/worth-query/crates/worth-query-declaration/src/
  portable_identity/
  application_schema/        # minted operation/effect references

workspaces/worth-query/crates/worth-query-installation/src/package/
  portable_records/
    manifest.rs
    record.rs
    record_view.rs
    record_set.rs
    limits.rs
  reconstruction/
    candidate.rs
    progression.rs
    denial.rs
    expected_identity.rs

workspaces/worth-query/crates/worth-query-execution/src/
  domain_computation/
    primary_graph/persistent_runtime.rs
    application_aftermath/committed_dispatch_claim/
      candidate.rs
      admission.rs
      fence.rs
      denial.rs
  persistent_runtime/                    # created; execution-owned lifecycle
    opening.rs
    owner_progression.rs
    provider_rebinding.rs
    capacity/
      durability_commit.rs               # existing Query commit-provider reservation binding
      dispatch_send.rs                   # worker/transport capacity before claim
    stream_catalog/
      contract.rs
      binding.rs
      activation.rs
      compatibility.rs
      denial.rs
    readiness.rs
    dispatch_coordination/               # owner port, not SQL mechanics
      contract.rs
      discovery.rs
      claim.rs
      attempt.rs
      outcome.rs
      reconciliation.rs
      denial.rs

workspaces/worth-query/crates/worth-query-host/src/
  facade.rs                              # existing; reexports only

workspaces/worth-query/crates/worth-query-package-archive/src/
  facade.rs
  envelope.rs
  encoding.rs
  decoding.rs
  limits.rs
  denial.rs
  repository/                            # created; descriptive archive port
    contract.rs
    record.rs
    denial.rs

crates/worth-runtime-postgres/
  migrations/
  src/
    facade.rs
    configuration.rs
    connection/
      opening.rs
      pool.rs
      shutdown.rs
    capacity/
      durability_commit.rs         # bounded admission subject, not connection authority
      dispatch_worker.rs           # bounded physical worker posture beneath Query ownership
    owner/
      relational/                  # created and populated in 9.16.2
        append.rs
        checkpoints.rs
        recovery.rs
        catalog.rs
      query_package/               # created and populated in 9.16.2
        storage.rs
        reconstruction.rs
        activation.rs
        projection.rs
      signal/                      # committed successor: 9.17.1
        component_artifact.rs
        branch_recovery.rs
        catalog.rs
      runtime_world/               # committed successor: 9.17.2
        composite_commit.rs
        product_head.rs
        retention.rs
        recovery.rs
    dispatch/
      discovery.rs
      publication_locator_index.rs
      claim.rs
      lease.rs
      attempt.rs
      outcome.rs
      reconciliation.rs
    schema/
      migration.rs
      compatibility.rs
    observability/
      health.rs
      metrics.rs
      diagnostics.rs
    denial.rs

crates/worth-runtime-persistence-certification/src/
  facade.rs
  relational_durability.rs
  package_archive_repository.rs
  runtime_stream_catalog.rs
  dispatch_coordination.rs
  capability_profile.rs

crates/worth-runtime-postgres-certification/tests/
  postgres_runtime_certification.rs      # one intentional integration target
  postgres_runtime_certification/
    postgres_environment.rs
    physical_artifact_oracle.rs
    fresh_process_recovery.rs
    commit_crash_matrix.rs
    dispatch_crash_matrix.rs
    namespace_isolation.rs
    migration_compatibility.rs
    projection_rebuild.rs
    ncr_restart_journey.rs
    composite_publication_gate.rs        # committed successor: 9.17.3

workspaces/worth-query-bank-world/crates/bank-courtroom/tests/
  persistent_runtime_courtroom.rs        # one intentional integration target
  persistent_runtime_courtroom/
    definition.rs
    compilation.rs
    production_seed.rs
    baseline.rs
    handles.rs
    oracle.rs
    environment.rs
    commit_crashes.rs
    pending_notice_restart.rs
    dispatch_fencing.rs
    compaction_restore.rs
    release_coexistence.rs
```

Unmarked Relational, declaration, installation, execution-domain, archive, and
PostgreSQL paths are existing or populated according to the phase that owns
them; comments identify newly created and committed-successor destinations.
No committed-successor file is created empty. The `owner` axis separates
physical implementations by semantic authority. `connection` owns only
PostgreSQL resource lifecycle. `capacity` exposes bounded, non-authoritative
resource-admission subjects; it owns neither semantic eligibility nor SQL
transactions. `dispatch` implements operational indexing,
fencing, and attempts beneath the Query-execution coordination port. The
adapter facade and the `worth-query-host` audience facade remain stable through
9.17; successors add populated owner siblings and stronger owner recovery
without moving either. The certification trees use one integration crate per
intentional court rather than compiling every scenario module as a separate
integration target. `worth-runtime-persistence-certification` is cert-band only:
it owns reusable provider-conformance cases for the four public ports and may
never become an ordinary runtime dependency. PostgreSQL certification
instantiates every case; a future database adapter closes the same kit plus its
own physical crash and operations courts.

Committed-successor paths document destination topology and are not created as
empty production placeholders. Query, Relational, Signal, and Runtime Bridge do
not depend on `worth-runtime-postgres`; each defines its owner port and the
adapter depends through that facade. `worth-query-host` cannot gain an owned
runtime subtree. Forbidden placements include a Query-owned physical runtime,
generic artifact bags, generic database-provider bags, SQL inside owner crates,
Bridge currentness inside the Relational adapter, physical recovery ordering
presented as semantic authority, or dispatch eligibility decided by a database
row.

## Ordered Phase Plan

These are implementation and review gates, not capability-group headings. Each
phase must close its own authority and proof boundary before the next phase may
consume it. A later end-to-end court cannot retroactively make an earlier
package, durability, recovery, or dispatch boundary honest.

### Phase 1: Stable Identity And Declared Provenance

Starting from the existing validated portable package and the declaration-
owned application aspect identity/revision and external-effect correlation
identity delivered by 9.16.1.1, ship stable identities for every remaining
package-relevant Rust axis, remove package-canonical `type_name`, and make
operation/effect references declaration-minted. Phase 1 carries the 9.16.1.1
identities unchanged and may not replace them with package-local equivalents.
Only the owning declarations may construct provenance-bearing references;
copied names and caller-selected digests open no package or dispatch path.
Compile-fail, collision, module-move, semantic-mutation, and warm-cost evidence
lets Phase 2 trust the identity vocabulary without trusting representation
accidents.

### Phase 2: Complete Typed Package Export

Consume one freshly validated package and export a versioned manifest plus the
complete typed logical record families in canonical order under explicit size
and count budgets. Those records include the retained 9.16.1.1 native schema
contracts, typed operation read and touch scopes, external-effect correlation
family, and installed reconciliation procedure; export may not reconstruct
them from strings or declaration summaries. The export is descriptive meaning,
never live runtime authority, and no adapter-specific node, edge, row,
document, or callback may enter it. Closure-inventory and mutation-sensitive
omission/duplication tests let Phase 3 trust that every package-relevant
semantic family has one stable portable projection.

### Phase 3: Bounded Reconstruction And Fresh Query Validation

Consume only untrusted typed records, enforce manifest closure and decode
budgets, reconstruct a candidate, recompute semantic identity, and ask Query to
validate it freshly. Neither stored identity nor successful decoding may mint
a validated package. Round-trip, cross-splice, dropped/duplicated record,
unsupported-version, forged-identity, and budget-exhaustion courts let Phase 4
trust exact semantic re-entry rather than a serialization shortcut.

### Phase 4: Neutral Release Archive And Trust Envelope

Encode the complete typed package as one deterministic, versioned,
store-neutral archive and define the host-owned signature/provenance envelope,
expected-identity comparison, compatibility window, and Git/GitHub release
workflow. Archive bytes, signatures, tags, filenames, and release names remain
non-authoritative until Phase 3 reconstruction succeeds under current host
policy. Determinism, tamper, downgrade, coexistence, and wrong-expected-release
evidence lets later physical stores retain and transport one independently
verifiable release artifact without dictating their schema.

### Phase 5: Relational Durability Authority And Backend Contract

Move local filesystem durability behind a Relational-owned, publicly
implementable backend contract while preserving owner-issued append authority,
versioned canonical artifact encoding, append-before-success, checkpoints,
replay, and recovery. Replace the current performed-then-deferred settlement
with the compiler-visible prepared-persisted-published progression, restart-
stable unresolved request descriptors, and owner writer fences. Delete
`settle_performed_publication`, `performed_but_durability_deferred`, runtime-id-
bound repair, and every compatibility entry to them. Migrate existing local
durability tests onto the new order, then run backend conformance and artifact-
compatibility tests; tests that assert the deleted order are replaced rather
than preserved. No Query/PostgreSQL type enters Relational, and no successful
backend write may mint Relational publication authority. Phase 8 may trust one
owner-defined durability contract whose local implementation already proves
acknowledgement, indeterminate fencing, and recovery semantics.

### Phase 6: PostgreSQL Adapter, Connection Lifecycle, And Migrations

Create the stable `worth-runtime-postgres` facade, configuration, connection
lifecycle, bounded blocking/pool posture, namespace boundary, migration ledger,
compatibility refusal, durability-profile admission, provider capability report,
adapter-owned durability capacity subject, and transaction
mechanics. Bind the durability subject into Query's existing exact
commit-provider support shape without creating a PostgreSQL-specific Query
contract or holding a connection during semantic work. This phase exposes honest
database readiness only; it does not claim package installation, Relational
recovery, or dispatch readiness. Real-PostgreSQL migration, rollback-refusal,
durability-setting, pool-exhaustion, namespace-isolation, and
atomic-reservation versus descriptive-capacity, hidden-queue, connection-hold,
and dependency-enforcement evidence lets
owner adapters enter beneath the facade without moving it or leaking SQL
upstream.

### Phase 7: Exact Package Registry, Coexistence, And Activation

Populate the Query-package PostgreSQL owner boundary with archive storage,
exact semantic-identity lookup, normalized derived projections, provenance,
generation-qualified activation records, and exact runtime-stream compatibility
bindings. Store multiple same-named and
incompatible releases without latest-name selection; activation records report
host policy choice only when consuming the concrete
`HostReleaseActivationAuthority`; they never bypass Query reconstruction or
validation.
Destroy/rebuild projections, racing activation, wrong-identity, wrong provider
contract, incompatible stream binding, namespace, and
4,096-package lookup courts let Phase 9 select one exact release without a
catalog scan or physical row becoming semantic authority.

### Phase 8: PostgreSQL Relational Commit, Checkpoint, And Replay

Implement the Phase 5 backend contract in the Relational PostgreSQL owner
boundary. Atomically persist each canonical Relational commit and its existing
co-committed Query outbox facts before acknowledgement; add versioned
checkpoints, bounded replay tails, recovery cursors, retention posture, and
rebuildable indexes, all qualified by namespace, stream, and exact owner-issued
Relational branch identity. Preserve every unresolved outbox occurrence and source
publication through checkpoint, compaction, backup, and restore. Return the
typed performed/already-performed/rejected/conflict/indeterminate outcome around
ambiguous commit responses. Query-originated appends consume the exact Phase 6
commit-provider reservation before Relational preparation; standalone owner
appends use the same bounded physical admission and may report capacity denial
only from `PhysicalAppendNotStarted`. Kill-before-start, kill-after-start,
kill-before-commit,
kill-after-commit-before-response, torn-write, corruption, ordering,
checkpoint-plus-tail, pending-after-compaction, same-named capacity
substitution, indeterminate-writer-fence, and backend-conformance
evidence lets Phase 9 recover owner truth without SQL rows acquiring
publication authority.

### Phase 9: Execution-Owned Owner-First Recovery, Rebinding, And Readiness

Ship Query's recovered-runtime installer and Query execution's persistent
recovery barrier, reexported through `worth-query-host`. Consume the exact Phase
7 release, reconstruct and freshly
validate it, ask Relational to readmit Phase 8 owner truth, create a new runtime
generation, rebind current providers/secrets/clock/authorizer, reconcile
derived indexes, bind the exact persistent commit-provider capacity subject into
installed operation support, and expose readiness only after closure. Startup failure must
leave execution and dispatch closed. Fresh-process equality, missing provider,
wrong package, foreign or same-named capacity authority, corrupt owner history,
and destroyed-index evidence freezes the
ordered owner-recovery protocol that 9.17 extends and lets Phase 10 trust one
live recovered runtime.

### Phase 10: Existing-Outbox Observation And Fresh Query Admission

Ship Query's private performed-product-publication carrier and fresh-claim
admission over the exact recovered outbox fact, package/effect contract,
correlation identity, provider binding, runtime generation, and current
performed publication. Never create or copy a second payload. In the present
runtime the carrier retains Relational performed publication; freeze the 9.17
handoff that replaces its source with Bridge performed composite publication.
Forgery, stale-runtime, wrong-release, wrong-binding, non-current-publication,
and old-handle-loss evidence mechanically forbids a PostgreSQL status row from
authorizing dispatch and lets Phase 11 claim only Query-admitted work.

### Phase 11: PostgreSQL Discovery, Claiming, Leasing, And Fencing

Define Query execution's persistent dispatch coordination contract, then build
its PostgreSQL derived pending-work index, bounded discovery, transactional
claim, lease renewal, monotonically increasing fencing epoch, and current-fence
verification beneath Query admission. Reserve exact worker/transport capacity
before conditional claim so a database lease never becomes an unbounded local
queue. SQL transactions must end before any network send, and deleting the
index must permit exact rebuild from canonical
Relational history. Contention, lease expiry, stale worker, index destruction,
namespace isolation, send-capacity saturation, and bounded-polling evidence lets Phase 12 hold one
current operational claim without confusing that claim with effect authority
or exactly-once delivery.

### Phase 12: Dispatch, Idempotency, Retry, And Crash Reconciliation

Cross the external transport boundary only after consuming Phase 10 admission
and the current Phase 11 fence. Send the existing payload under its stable
idempotency key; durably record attempts and outcomes, bounded backoff, poison
posture, cancellation, indeterminate response recovery, and reconciliation.
Operational delivery rows do not become workflow truth, and no direct
post-dispatch mutation bypasses Query. Crashes before send, after send, before
acknowledgement, during retry, and after lease replacement prove at-least-once
runtime behavior while leaving exactly-once consequence to the idempotent
external owner.

### Phase 13: Operational Reconstruction, Migration, And Disaster Recovery

Close the reconstructive and administrative lanes independently of ordinary
execution: projection destruction/rebuild, checkpoint and retention operations,
supported schema upgrades, incompatible-version refusal, backup, restore into
a separate database, and recovery from the restored artifacts. Every operation
is bounded, reports progress and failure posture, and cannot silently activate
another release, prune unresolved outbox source truth, or weaken durability.
Migration fault injection, corrupt
backup, partial restore, derived-state deletion, and separately restored
representative workflow history evidence lets operators trust recovery without
using diagnostic state as authority.

### Phase 14: Production Observability, Capacity, And Isolation

Ship readiness/liveness semantics, owner- and lane-specific health, metrics,
work/amplification counters, queue and pool saturation posture, alertable poison
work, namespace isolation, and documented capacity limits. Prove exact indexed
lookup and bounded claim/recovery work at 4,096 unrelated packages and long
history, exact commit-provider reservation under concurrent saturation, zero
Relational preparation and SQL for backpressured excess arrivals, zero
archive/recovery scans during warm execution, and fail-closed overload before
unbounded queues or pool starvation. Prove separately that post-transaction
ambiguity never takes the pre-effect backpressure outcome. These observations
remain sidecars; deleting them cannot alter package, Relational, outbox, or
dispatch truth. Phase 15 may trust an operable production composition rather
than a correct but unmanageable library.

### Phase 15: Persistent Bank And NCR Certification, Then Workflow-Editor Cutover

Exercise the real composition root from one signed GitHub NCR release through
exact installation, submit and transition execution, atomic state plus existing
notification outbox commit, process death, fresh recovery, provider rebinding,
fenced notification dispatch, observable outcome, backup/restore, and release
coexistence. Compile/run caller and operator documentation against the public
facades, including 9.16.1.1 typed read/touch and aftermath inspection; run
dependency, facade, mutation, scale, crash, and residue courts; then cut
Workflow Editor to the signed archive plus PostgreSQL runtime. NCR may not add
a local operation-contract mirror, structured touch grammar, or aftermath
summary. This phase integrates already-proved boundaries and may not absorb
unfinished work from Phases 1 through 14 or substitute an in-memory reenactment
for the real PostgreSQL fresh-process court.

Each phase closes through its final implementation, direct tests, required
repository checks, and review. Later work cannot hide an unmet authority or
crash-consistency guarantee.

## Caller DX Target

The composition is workflow-forward and small:

```rust
let release = WorkflowRelease::from_signed_archive(
    archive_bytes,
    expected_package_identity,
    host_release_verifier,
)?;

let persistence = WorthRuntimePostgres::connect(
    PostgresRuntimeConfiguration::builder()
        .database_url(database_url)
        .namespace("acme.medical")?
        .runtime_stream("quality.workflows")?
        .durability_admission_limit(16)?
        .blocking_executor_limit(16)?
        .durability_connection_limit(16)?
        .connection_limit(32)?
        .dispatch_worker_limit(32)?
        .build()?,
)?;

let providers = PersistentQueryRuntimeProviders::new(
    persistence.relational_durability(),
    persistence.query_package_archives(),
    persistence.runtime_stream_catalog(),
    persistence.dispatch_coordination(),
)?;

let runtime = WorthQueryHost::open_persistent(
    providers,
    release,
    HostRuntimeBindings::builder()
        .principal_provider(principals)
        .authorization_provider(authorization)
        .clock(clock)
        .effect_transport(notifications)
        .build()?,
)?;

runtime.wait_until_ready()?;
runtime.execute(ncr_submit_request)?;
runtime.dispatch_worker(worker_configuration)?.run_until_shutdown(shutdown)?;
```

Names may align with existing facades, but callers never manually wire a
Relational runtime, decode envelopes, scan SQL, construct outbox payloads, or
retain receipts for recovery. The provider bundle binds the adapter's exact
durability and dispatch capacity subjects into Query admission; callers may size
those subjects but cannot substitute descriptive availability or construct a
reservation.

## Workflow Editor And NCR Requirement Closure

This milestone gives Workflow Editor:

- content-derived Query identity rather than hand-maintained versions;
- Git/GitHub provenance outside Query semantics;
- signed deterministic coexisting releases and exact rollback selection;
- PostgreSQL durability for workflow facts from the first production release;
- recovery under each admitted application, database, operating-system, and
  backup/restore crash model;
- durable NCR transitions plus co-committed notification intent;
- restart-safe notification dispatch using the existing outbox;
- queryable release, commit, and pending-work indexes; and
- runtime-level PostgreSQL adapter and Query-execution persistent surface,
  reexported by the leaf Query-host facade, that 9.17 extends without moving NCR
  host integration; and
- a provider construction progression and owner contracts that a later Worth
  Store adapter implements without changing authored workflow definitions,
  public host entry, commit outcomes, or owner recovery law.

It does not define the Workflow Editor DSL/UI, NCR rules, notification templates,
or host authorization policy. Those consumers use this durable runtime.

## Work And Performance Contract

- Export, archive decode, and reconstruction are cold release/startup work.
- Append cost is proportional to the current commit and catalog updates, not
  total history.
- Recovery is proportional to one checkpoint and uncovered tail under limits.
- Package lookup is indexed by namespace and semantic identity.
- Dispatch polling is indexed by namespace, status, and next-attempt.
- Claim is one bounded transaction; SQL transactions never remain open during
  external dispatch.
- Persistent Query execution atomically reserves the exact durability
  commit-provider capacity subject with its other execution resources before
  Relational preparation or physical effects.
- The durability reservation bounds admitted or queued work but is never a
  database connection or semantic authority; connection checkout begins only
  at the append effect phase.
- Pre-effect saturation returns typed backpressure with zero owner preparation
  and zero SQL. Once a transaction may have begun, only the typed commit-outcome
  topology applies, including indeterminate recovery.
- Dispatch worker/transport capacity is reserved before claim. Saturation leaves
  canonical outbox work durable and unclaimed rather than queued behind a lease.
- Live/subscription delivery backpressure never holds a transaction open or
  enters the authoritative commit cost.
- Checkpoint/projection rebuild work is explicit and bounded.
- Each physical adapter reports required durability, ordered-recovery,
  conditional-write/fencing, and namespace-isolation capabilities before
  readiness; absence is a typed refusal, not a slower fallback.
- Warm execution has zero archive, decomposition, recovery-catalog, or global
  work-scan cost.
- Pools, blocking capacity, retries, leases, recovery, and retention limits are
  explicit configuration.

## Documentation Deliverables

- Revise
  `workspaces/worth-query/crates/worth-query/docs/AI_README.md` for ordinary
  Query callers. It must route portable export, exact release selection,
  persistent opening, readiness, and dispatch through the public audience
  facades and show typed compatibility and indeterminate-commit outcomes.
- Create
  `workspaces/worth-query/crates/worth-query/docs/portable-packages.md` as the
  developer authority for semantic identity, complete records, reconstruction,
  archive versions, expected identity, provider requirements, Git/GitHub
  provenance, coexistence, and downgrade refusal.
- Create
  `workspaces/worth-query/crates/worth-query/docs/persistent-runtime.md` for host
  integrators. It must explain the four provider-neutral ports, execution-owned
  opening progression, runtime-stream binding, exact commit-provider capacity
  registration and reservation, readiness, fresh authority, pre-effect
  backpressure versus post-effect indeterminate outcomes, dispatch-capacity-
  before-claim, fencing/idempotency, and the 9.17 product-publication handoff.
- Create `crates/worth-runtime-postgres/docs/operator-runbook.md` for operators.
  It must cover supported PostgreSQL versions, required durability settings,
  migrations, roles and namespace isolation, capacity, poison work, checkpoint
  and compaction safety, durability-admission/blocking-executor/pool sizing,
  saturation and deadline posture, backup/restore, crash models, RPO/RTO,
  diagnostics, and incompatible-startup recovery.
- Create
  `workspaces/worth-query-bank-world/crates/bank-courtroom/docs/persistent-bank-world.md`
  for certification maintainers. It must explain definitions, semantic handles,
  baselines, scenario deltas, independent oracles, cost lanes, and how to add a
  causal persistence case without raw fixture identities.
- Revise the continuing Workflow Editor/NCR deployment guide chosen during
  Phase 15; if no authoritative guide exists, create
  `_docs/WORTH-query/workflow-editor-postgres-deployment.md`. It must contain one
  complete signed-release deployment, kill/restart, notification recovery,
  rollback/coexistence, and backup/restore walkthrough.

Across those named documents, the checked coverage is:

- carriage and fresh readmission of the Milestone 9.16.1.1 native schema,
  typed read/touch, correlation-family, and reconciliation contracts;
- package identity, records, reconstruction, archives, and Git/GitHub versioning;
- PostgreSQL setup, migrations, compatibility, backup/restore, and recovery;
- durable acknowledgement and readiness guarantees;
- existing outbox meaning, fencing, retry/idempotency, poison work, and operator
  intervention;
- activation, coexistence, rollback, namespace isolation, metrics, and capacity;
- Query/Relational/Signal/Runtime-Bridge/PostgreSQL-composition/host/external-
  owner/future-Store boundaries and their 9.17 succession; and
- a complete NCR deployment and kill/restart walkthrough.

Examples and compile-fail tests are support contract. AI README and coding docs
must route callers through public facades, never direct SQL/private workarounds.

## Must Ship

- exact package records and reconstruction for every retained Milestone
  9.16.1.1 installed graph and aftermath contract;
- stable identities and declaration-minted operation/effect provenance;
- complete package export, bounded reconstruction, fresh validation, and
  expected-identity comparison;
- deterministic neutral archive;
- Relational pluggable canonical durability port and versioned durable artifact
  representation;
- stable `worth-runtime-postgres` adapter facade, Query-execution persistent
  opening/recovery surface reexported by `worth-query-host`, PostgreSQL
  Relational commit/checkpoint backend, and exact-identity package registry;
- separate Relational durability, immutable archive repository, Query runtime-
  stream lifecycle, and Query dispatch coordination ports with physical-
  capability admission;
- exact binding of the selected physical durability provider into Query's
  existing commit-provider capacity subject, atomic pre-effect reservation,
  bounded adapter admission for standalone Relational use, and phase-correct
  backpressure/commit-ambiguity outcomes;
- dispatch worker and transport capacity reservation before claim, with live
  delivery pressure remaining outside authoritative commit cost;
- one cert-only reusable conformance kit covering all four ports, exact
  capacity binding, and durability progression, instantiated by PostgreSQL and
  reusable at the provider-neutral boundary for future adapters, including Store;
- Query recovered-runtime installation and provider rebinding;
- startup recovery/readiness barrier;
- restart discovery and fenced claiming of Query's existing outbox through a
  performed-product-publication carrier;
- durable attempts/outcomes with stable idempotency;
- migrations, configuration, health, metrics, backup/restore, and operator docs;
- real-PostgreSQL crash, scale, isolation, migration, projection, outbox,
  persistent Bank, and NCR certification; and
- facade, dependency, docs, and residue proof.

## Must Preserve

- Milestone 9.16.1.1 application aspect identity/revision, native schema
  catalog, typed operation read/touch scopes, effect separation, and complete
  typed aftermath inspection without package-local mirrors;
- Query ownership of workflow meaning, validation, digest, execution, and
  dispatch admission;
- Relational ownership of publication, canonical envelopes,
  append-before-success, checkpoints, replay, and recovery;
- the existing co-committed Query outbox payload and identities;
- private proofs, receipts, recovery handles, runtime ids, and live authority;
- host ownership of providers, secrets, release trust, Git, and activation;
- Signal ownership of Signal component truth and Runtime Bridge ownership of
  composite product currentness when those committed successors land;
- runtime-level adapter ownership of SQL, migrations, indexes, pools, and
  physical conditional-write mechanics without semantic authority;
- Query-execution ownership of claim/fence/attempt/outcome transitions and
  persistent opening, with the host facade remaining a leaf reexport;
- Query ownership of execution-resource admission and atomic capacity
  reservation, with physical adapters owning only their bounded capacity
  subjects, pools, waits, and release mechanics;
- external-owner responsibility for exactly-once behavior via idempotency;
- cold/warm work separation; and
- future Worth Store reuse without PostgreSQL topology leakage.

## Explicit Non-Goals

- Worth Store production integration or Store production-crate edits;
- PostgreSQL becoming Query meaning or Relational importing SQL types;
- MongoDB, SQLite, RocksDB, or custom graph adapters;
- distributed multi-primary publication or cross-region consensus;
- automatic database failover beyond configured PostgreSQL guarantees;
- exactly-once external effects without an idempotent external owner;
- serializing callbacks, credentials, providers, proofs, receipts, handles, or
  dispatch authority;
- a second outbox payload or host dual-write substitute;
- automatic migration between incompatible workflow releases;
- the Workflow Editor DSL/UI; and
- implementing Milestone 9.17 component/composite semantics inside 9.16.2;
  their durable adapter extensions remain required work in those milestones.

## Allowed Debt

- One documented PostgreSQL major-version window may ship first.
- Cross-region replication/failover may remain deployment concerns with honest
  failure documentation.
- Worth Store may later add graph topology, retention, replication, and
  distributed recovery.
- Activation may initially be explicit rather than a fleet rollout controller.

No debt permits best-effort durability, torn mutation/outbox state, unfenced
dispatch, unbounded recovery, or latest-name release selection.

## Acceptance Evidence

The closure ledger contains:

- the green `Portable Package And PostgreSQL Runtime Durability Certification`
  suite defined and owned by this specification;

- authority mapping for every persisted and derived artifact;
- public API/dependency-direction proof;
- stable identity/provenance compile-fail and mutation proof;
- exact 9.16.1.1 installed-contract export/reconstruction equality and mutants
  that drop, widen, cross-splice, summarize, or string-encode those contracts;
- record-family inventory and hostile mutants;
- archive determinism, corruption, compatibility, and budget evidence;
- local filesystem and PostgreSQL backend conformance;
- transactional append, checkpoint, and fresh-process recovery evidence;
- client acknowledgement and typed commit-ambiguity crash matrix;
- admitted PostgreSQL durability-profile and physical-capability evidence;
- concurrent commit-provider saturation evidence proving exact reservation
  width, zero Relational preparation/SQL for backpressured excess arrivals,
  release/readmission, no same-named or foreign capacity substitution, and no
  hidden queue between reservation, blocking executor, and pool;
- phase-boundary evidence proving pre-transaction capacity refusal remains
  no-effect while post-transaction stalls and response loss can produce only
  the typed commit outcomes and never ordinary backpressure;
- outbox crash, contention, fencing, retry, poison, and idempotency matrix;
- dispatch-capacity-before-claim and live-consumer-saturation evidence proving
  no lease-backed local queue, no SQL transaction across send, and no loss or
  weakening of canonical outbox work;
- pending-outbox survival after checkpoint, compaction, projection deletion,
  backup, and restore;
- proof no shadow payload or dual write exists;
- proof the current publication carrier cannot be caller-minted and that the
  9.17 Bridge-performed source enters without changing its stable Query meaning;
- destination-topology proof that Signal and Runtime Bridge PostgreSQL adapters
  enter as owner siblings without moving the facade;
- namespace, release coexistence, migration, and projection-rebuild proof;
- scale/work-counter evidence;
- complete persistent Bank transfer/notice/compaction/restart journey plus the
  independent NCR submit/transition/notification/restart consumer journey;
- backup/restore into a separately restored database;
- operational health/diagnostic examples;
- docs/facade examples; and
- residue searches for package-canonical `type_name`, private constructors,
  SQL outside adapter, serialized authority, duplicate outbox payloads, and
  unbounded scans, plus persistence paths that bypass exact commit-provider
  reservation or report post-effect backpressure.

## Handoff

Workflow Editor can ship signed Query releases backed immediately by PostgreSQL.
NCR workflows are the first end-to-end consumer and prove durable workflow state
plus restart-safe notifications.

Future adapters consume owner-defined contracts:

1. Query typed package export/reconstruction for workflow meaning;
2. Relational canonical component durability/recovery;
3. Signal canonical component durability/recovery after 9.17.1; and
4. Runtime Bridge composite history/currentness durability/recovery after
   9.17.2.

Worth Store may implement these contracts and add native graph indexes,
retention, replication, and distributed recovery. It does not inherit
PostgreSQL table topology, and Query does not change workflow meaning.
PostgreSQL export tooling may later stream the same typed package records and
canonical Relational artifacts into Worth Store under a separate Store import
protocol.

Milestone 9.17 must preserve durability continuously. 9.17.1 branch-qualifies
Relational persistence and adds Signal recovery; 9.17.2 adds durable Bridge
composite commits and product-head CAS; 9.17.3 recovers the complete product
world and makes Bridge performed publication the only source for Query dispatch
admission. PostgreSQL rows, leases, archives, and snapshots remain
non-authoritative throughout.

After 9.17, dispatch attempt/outcome rows remain operational delivery truth.
Any acknowledged, completed, unresolved, or recovery consequence that changes
product workflow state must be submitted as a new Query aftermath operation and
become current only through Runtime Bridge composite publication; no sideband
Relational mutation may create a hidden product-history lane.
