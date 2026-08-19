# Milestone 9.16.2: Portable Packages And PostgreSQL Runtime Durability Foundation

> **Status:** Proposed — required before the Milestone 9.17 sequence
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
- a `worth-query-host` restart barrier above Query and Relational that invokes
  owner recovery and rebinds fresh host authority before traffic is admitted;
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

Milestone 9.16.2 is the second corrective interstitial after Milestone 9.16.
It consumes the portable installation grammar established by Milestone 9.13,
the authority decomposition established by Milestone 9.13.2, the installed
workflow meaning established through Milestones 9.14 and 9.16, the
application-aftermath and co-committed outbox completed by 9.16 Runtime Phase
8, and the closed Milestone 9.16.1 authority-convergence substrate.

Milestone 9.16 retains its recorded statuses and remains open for Bank Phase 6
and Closure Phase 1. Work on 9.16.2 may proceed beside those phases where
boundaries do not overlap. The roadmap does not advance to 9.17.1 until both
9.16 and 9.16.2 close.

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
- Relational publication already creates one canonical commit envelope and
  refuses publication success when durable append fails.
- Relational already owns checkpoint and replay recovery law for in-memory and
  persisted local-filesystem modes.
- Query already exposes `WorthQueryCommittedDispatchOutboxObservation` through
  `observe_committed_dispatch_outbox` and admits external dispatch only through
  current runtime authority.

The missing contracts are narrower than a new workflow engine and broader than
a package codec:

- no complete public typed package decomposition/recomposition contract;
- package-canonical Rust `type_name` values are not stable semantic identities;
- operation/effect references lack declaration-minted membership provenance;
- Relational durability has no narrow public backend implementation port;
- no PostgreSQL implementation of canonical append, checkpoints, and recovery;
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
| Recovery invocation, fresh host binding, readiness, and lifecycle | `worth-query-host` as audience composition facade consuming owner recovery facades |
| Release signatures, signer trust, Git provenance, registry revision, and activation policy | Host release system |
| Store-neutral release archive bytes and compatibility | `worth-query-package-archive` |
| Physical snapshot completeness | PostgreSQL adapter |
| Fresh acceptance of reconstructed package meaning | Query |
| Dispatch lease, fencing epoch, attempt ledger, retry schedule, and work index | PostgreSQL dispatch adapter under Query-issued admission |
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

## Decisive Failure Court

Before any facade is accepted, one real PostgreSQL test starts an NCR mutation,
kills the process at controlled points around canonical commit and notification
dispatch, deletes every process-local receipt and handle, and starts a competing
fresh runtime. The milestone fails if the new runtime cannot reconstruct the
exact expected package and acknowledged NCR state, if mutation and outbox tear,
if pending work requires the old runtime id, if two claimants can hold a current
fence, or if a stale claimant can send or acknowledge.

This court forces the implementation to put PostgreSQL in Relational's commit
path, reuse Query's existing outbox fact, and create fresh runtime authority.
A package codec, host-side dual write, shadow queue, serialized receipt, or
happy-path restart demo cannot pass it.

The committed successor mutation repeats the same court after 9.17.3 while
forcing a Relational owner commit followed by failed Signal preparation or a
failed Runtime Bridge product-head CAS. The outbox fact must survive as exact
owner state but remain ineligible and unsent because no performed composite
publication exists. Deleting the Bridge-publication gate must turn that evidence
red.

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

After restart:

1. the adapter discovers a pending outbox identity and source commit;
2. Query observes the exact recovered outbox fact in the current runtime;
3. Query verifies package, operation/effect contract, correlation identity,
   exact product publication, and current dispatch eligibility;
4. the adapter acquires a lease and monotonically increasing fencing epoch in
   a PostgreSQL transaction;
5. Query issues dispatch admission bound to the current runtime, performed
   product publication, outbox fact, and fence;
6. transport sends the existing payload with the existing idempotency key;
7. the adapter records outcome only if the fence is current;
8. the adapter records operational attempt/outcome under the current fence;
   any semantic aftermath that changes workflow state re-enters the ordinary
   Query publication path; and
9. the work index advances or is removed.

Claiming is at-least-once across crash ambiguity. Exactly-once external effect
is claimed only when the external owner honors the stable idempotency key. A
crash after send but before acknowledgement retries with the same key. A stale
claimant cannot acknowledge or begin a new send after its fence expires.

### Recovery barrier

`WorthQueryHost::open_persistent` admits no traffic until it:

1. connects and verifies database/migration compatibility;
2. selects the release by independently expected package identity;
3. verifies signature/provenance under host policy;
4. decodes the neutral archive under limits;
5. reconstructs an untrusted candidate and freshly validates it;
6. rejects any expected/recomputed identity mismatch;
7. invokes every configured owner's recovery facade in dependency order;
8. in 9.16.2, asks Relational to reconstruct from checkpoint plus commit tail;
9. after 9.17.2, additionally recovers Signal owner state and Runtime Bridge
   composite history/product heads only after referenced component bases exist;
10. installs the package through Query's recovered-runtime constructor;
11. rebinds current providers, clock, authorizer, transport, and secrets;
12. rebuilds or verifies package and pending-outbox projections;
13. joins pending outboxes to performed product publications and enables fresh
   fenced claim admission; and
14. publishes readiness.

Any failure leaves the runtime unready. Providers and secrets are never
deserialized from storage. The host facade invokes owner calls but cannot
validate component bases, mint composite currentness, or bypass Query dispatch
admission. `worth-runtime-postgres` supplies physical implementations and does
not own the recovery progression.

### Multi-tenant and release coexistence posture

Every physical key is qualified by adapter namespace and durable runtime stream.
Package identity, not a mutable name, selects executable meaning. Multiple
releases coexist. Activation is a host-governed pointer updated transactionally
after the release is stored and freshly validated. One namespace cannot scan,
claim, replay, checkpoint, or activate another through the public API.

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
identity, compiler/toolchain metadata, release metadata, and provenance.

Git tags and GitHub releases label human releases. Query's semantic digest
labels executable meaning. Neither is manually incremented in workflow source,
and neither substitutes for the other.

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
5. a kill before PostgreSQL commit yields no acknowledged mutation;
6. a kill after commit but before response recovers complete mutation and outbox
   or neither, never torn state;
7. checkpoint plus tail equals uninterrupted Relational state;
8. corrupt, missing, duplicated, forked, or out-of-order artifacts fail before
   readiness;
9. missing/wrong package fails instead of installing latest by name;
10. pending existing outbox work is found after all old handles are gone;
11. racing workers yield at most one current fence;
12. an expired worker cannot send or acknowledge after a higher fence exists;
13. crashes before send, after send, and before acknowledgement converge under
    the same idempotency identity without a second outbox payload;
14. poison work reaches existing unresolved/terminal posture without blocking
    unrelated work;
15. namespaces cannot access one another's release, history, or work;
16. derived indexes can be dropped and rebuilt without changing meaning;
17. unsupported migrations refuse startup without partial canonical mutation;
18. 4,096 unrelated packages and long history do not make exact lookup or claim
    globally linear;
19. warm execution performs no archive/decomposition/recovery scan; and
20. a fresh process runs an NCR workflow, commits state and notification outbox,
    dies, recovers, resumes notification, and serves exact resulting state.

Testcontainers or the repository Docker harness must use a real supported
PostgreSQL version. An in-memory fake cannot close the milestone.

## Destination Directory And Crate Topology

Exact leaf names may align with existing laws, but ownership may not drift:

```text
crates/worth-relational/src/durability/
  backend/
    contract.rs              # implementable ports; owner-issued append authority
    artifact.rs
    denial.rs
    work.rs
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

workspaces/worth-query/crates/worth-query-execution/src/domain_computation/
  primary_graph/persistent_runtime.rs
  application_aftermath/committed_dispatch_claim/
    candidate.rs
    admission.rs
    fence.rs
    denial.rs

workspaces/worth-query/crates/worth-query-host/src/runtime/recovery/
  opening.rs
  owner_progression.rs
  provider_rebinding.rs
  dispatch_reconciliation.rs
  readiness.rs

workspaces/worth-query/crates/worth-query-package-archive/src/
  facade.rs
  envelope.rs
  encoding.rs
  decoding.rs
  limits.rs
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

crates/worth-runtime-postgres-certification/tests/
  fresh_process_recovery.rs
  commit_crash_matrix.rs
  dispatch_crash_matrix.rs
  namespace_isolation.rs
  migration_compatibility.rs
  projection_rebuild.rs
  ncr_restart_journey.rs
  composite_publication_gate.rs   # committed successor: 9.17.3
```

The `owner` axis separates physical implementations by semantic authority.
`connection` owns only PostgreSQL resource lifecycle. `dispatch` owns
operational indexing, fencing, and attempts beneath Query admission. The
adapter facade and the `worth-query-host` audience facade remain stable through
9.17; successors add populated owner siblings and stronger owner recovery
without moving either.

Committed-successor paths document destination topology and are not created as
empty production placeholders. Query, Relational, Signal, and Runtime Bridge do
not depend on `worth-runtime-postgres`; each defines its owner port and the
adapter depends through that facade. Forbidden placements include a Query-owned
physical runtime, generic artifact bags, SQL inside owner crates, Bridge
currentness inside the Relational adapter, physical recovery ordering presented
as semantic authority, or dispatch eligibility decided by a database row.

## Ordered Phase Plan

### Phase 1: Stable Identity And Declared Provenance

Ship stable identities for every package-relevant Rust axis, remove
package-canonical `type_name`, and make operation/effect references
declaration-minted. Prove stability, collision denial, forgery denial, and no
warm regression.

### Phase 2: Package Decomposition And Reconstruction

Ship versioned manifest, typed records, bounded export/reconstruction, fresh
validation, and expected-identity comparison. Prove completeness with
mutation-sensitive twins and hostile record tests.

### Phase 3: Relational Pluggable Durability Boundary

Move local filesystem durability behind a Relational-owned, publicly
implementable backend contract while preserving owner-issued append authority,
versioned canonical artifact encoding, append-before-success, checkpoints,
replay, and recovery. Run existing local durability tests unchanged plus backend
conformance and artifact-compatibility tests. No Query/PostgreSQL type enters
Relational.

### Phase 4: Runtime-Level PostgreSQL Adapter And Current Owner Implementations

Create the stable `worth-runtime-postgres` adapter facade and populate its Query-package
and Relational owner boundaries. Ship migrations, catalogs, archive storage,
normalized package projections, exact activation, canonical append,
checkpoints, and recovery loading with bounded synchronous pools. Prove that
Query/Relational own no SQL and that later Signal/Runtime Bridge adapters enter
as siblings without changing facade or dependency direction.

### Phase 5: Fresh-Process Recovery And Provider Rebinding

Ship Query's recovered-runtime installer and the `worth-query-host` persistent
recovery barrier. Reconstruct package and current Relational owner state, create
a new runtime generation, rebind host providers/secrets, reconcile indexes, and
expose readiness only after closure. Freeze the ordered owner-recovery protocol
that 9.17 extends. Prove kill/restart equality with uninterrupted execution.

### Phase 6: Existing Outbox Restart Claim And Dispatch

Ship Query's private performed-product-publication carrier and fresh-claim
admission plus PostgreSQL discovery, lease, fencing, attempt, retry, outcome,
and reconciliation. Never create a second payload. In the present runtime the
carrier retains Relational performed publication; freeze the 9.17 handoff that
replaces its source with Bridge performed composite publication. Prove crash
ambiguity, competing workers, stale fences, poison isolation, stable
idempotency, and recovery without old handles.

### Phase 7: Archive, NCR Journey, Operations, And Cutover

Ship neutral archive, GitHub release example, complete NCR restart journey,
operator docs, health/metrics, backup/restore and migration runbooks, facade
checks, scale/work counters, and residue searches. Cut Workflow Editor to the
signed archive plus PostgreSQL runtime. Close only with a fresh-process real
PostgreSQL court.

Each phase ends with a closure ledger. Later work cannot hide an unmet
authority or crash-consistency guarantee.

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
        .connection_limit(16)?
        .build()?,
)?;

let runtime = WorthQueryHost::open_persistent(
    persistence,
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
retain receipts for recovery.

## Workflow Editor And NCR Requirement Closure

This milestone gives Workflow Editor:

- content-derived Query identity rather than hand-maintained versions;
- Git/GitHub provenance outside Query semantics;
- signed deterministic coexisting releases and exact rollback selection;
- PostgreSQL durability for workflow facts from the first production release;
- recovery after host or machine restart;
- durable NCR transitions plus co-committed notification intent;
- restart-safe notification dispatch using the existing outbox;
- queryable release, commit, and pending-work indexes; and
- runtime-level PostgreSQL adapter and persistent Query-host facades that 9.17
  extend without moving NCR host integration; and
- contracts a later Worth Store adapter can implement without changing
  authored workflow definitions.

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
- Checkpoint/projection rebuild work is explicit and bounded.
- Warm execution has zero archive, decomposition, recovery-catalog, or global
  work-scan cost.
- Pools, blocking capacity, retries, leases, recovery, and retention limits are
  explicit configuration.

## Documentation Deliverables

Document:

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

- stable identities and declaration-minted operation/effect provenance;
- complete package export, bounded reconstruction, fresh validation, and
  expected-identity comparison;
- deterministic neutral archive;
- Relational pluggable canonical durability port and versioned durable artifact
  representation;
- stable `worth-runtime-postgres` adapter facade, `worth-query-host` persistent
  recovery facade, PostgreSQL Relational commit/checkpoint backend, and exact-
  identity package registry;
- Query recovered-runtime installation and provider rebinding;
- startup recovery/readiness barrier;
- restart discovery and fenced claiming of Query's existing outbox through a
  performed-product-publication carrier;
- durable attempts/outcomes with stable idempotency;
- migrations, configuration, health, metrics, backup/restore, and operator docs;
- real-PostgreSQL crash, scale, isolation, migration, projection, outbox, and NCR
  certification; and
- facade, dependency, docs, and residue proof.

## Must Preserve

- Query ownership of workflow meaning, validation, digest, execution, and
  dispatch admission;
- Relational ownership of publication, canonical envelopes,
  append-before-success, checkpoints, replay, and recovery;
- the existing co-committed Query outbox payload and identities;
- private proofs, receipts, recovery handles, runtime ids, and live authority;
- host ownership of providers, secrets, release trust, Git, and activation;
- Signal ownership of Signal component truth and Runtime Bridge ownership of
  composite product currentness when those committed successors land;
- runtime-level adapter ownership of SQL, migrations, leases, indexes, and
  physical recovery mechanics without semantic authority;
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
  suite from [test-requirements.md](./test-requirements.md);

- authority mapping for every persisted and derived artifact;
- public API/dependency-direction proof;
- stable identity/provenance compile-fail and mutation proof;
- record-family inventory and hostile mutants;
- archive determinism, corruption, compatibility, and budget evidence;
- local filesystem and PostgreSQL backend conformance;
- transactional append, checkpoint, and fresh-process recovery evidence;
- client acknowledgement and commit crash matrix;
- outbox crash, contention, fencing, retry, poison, and idempotency matrix;
- proof no shadow payload or dual write exists;
- proof the current publication carrier cannot be caller-minted and that the
  9.17 Bridge-performed source enters without changing its stable Query meaning;
- destination-topology proof that Signal and Runtime Bridge PostgreSQL adapters
  enter as owner siblings without moving the facade;
- namespace, release coexistence, migration, and projection-rebuild proof;
- scale/work-counter evidence;
- complete NCR submit/transition/notification/restart journey;
- backup/restore into a separately restored database;
- operational health/diagnostic examples;
- docs/facade examples; and
- residue searches for package-canonical `type_name`, private constructors,
  SQL outside adapter, serialized authority, duplicate outbox payloads, and
  unbounded scans.

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
