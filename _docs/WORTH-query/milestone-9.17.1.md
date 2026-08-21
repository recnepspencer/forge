# Milestone 9.17.1: Owner Component Bases And Relational Branch-Local MVCC

## Goal

Establish the owner-local substrate required by composite product branches:

- exact, private-minted Relational and Signal component basis artifacts that
  cannot be substituted by raw ids, equal ordinals, copied descriptors, or
  derived state; and
- genuine Relational branch-local MVCC in which transactions, snapshots,
  conflicts, versions, history, retention, and publication are qualified by
  one Relational branch and unrelated branch writers make concurrent progress;
- PostgreSQL durability and fresh-process recovery qualified by the same exact
  Relational branch axes; and
- a Signal-owned versioned durable artifact/recovery port with its
  `worth-runtime-postgres` implementation, so exact Signal bases required by a
  future composite commit survive restart.

This milestone ends at owner-issued component publication candidates and exact
component basis observation. It does not create a composite world commit,
product branch, Bridge publication coordinator, or public Query branch
workflow.

## Roadmap Placement

[Milestone 9.17](./milestone-9.17.md) is the governing umbrella. Milestone
9.16.1 already carries one typed branch affinity through Query's provider-
session progression, but it intentionally does not supply multi-head mechanics
or exact Relational-plus-Signal component composition. Milestone 9.16.2 is also
closed before this work begins; its package records and PostgreSQL ordinary
durability foundation is inherited. This milestone extends its populated owner
topology rather than moving the facade: Relational persistence becomes branch-
qualified and Signal gains its owner-defined adapter. Package records, SQL rows,
recovered snapshots, and dispatch leases carry no component authority.

9.17.1 solves the hard owner-local problem first:

```text
Relational owner                            Signal owner
    |                                           |
    v                                           v
exact branch basis                         exact branch basis
    |                                           |
    v                                           |
branch-qualified snapshot                       |
    -> transaction                               |
    -> conflict/read-set validation              |
    -> prepared publication candidate            |
    -> owner-local commit                        |
    -> next exact branch basis                    |
    |                                           |
    +------------------- carried only -----------+
                         no composition yet
```

[Milestone 9.17.2](./milestone-9.17.2.md) consumes these owner-issued artifacts
to build Runtime Bridge composition authority. Building Bridge composition
before this milestone closes would merely wrap the old global Relational
coordinator and force the composition layer to guess at owner currentness.

## Current Boundary

### Relational

Relational already has branch identity, branch heads, history, snapshots,
retention, transaction preparation, conflict classification, and commit
publication concepts. The blocking topology is that ordinary transaction and
commit entry still rely on mutable access to one broad runtime and globally
shared allocation/coordinator state. A blocked writer can therefore serialize
work whose semantic authority is confined to a different branch.

The current owner must be decomposed by authority, not merely hidden behind a
finer lock:

- immutable branch-qualified observation;
- branch-local transaction state and write/read sets;
- branch-local head/currentness and conflict comparison;
- allocation whose identity remains globally safe without making unrelated
  branch publication mutually exclusive;
- history and retention capable of preserving exact bases used outside the
  current head; and
- owner-local prepared candidates that have no composite product authority.

The existing 9.16.2 PostgreSQL backend must be branch-qualified with the same
owner axes. A global durable stream selected outside the Relational owner would
reintroduce the exact global-currentness assumption this milestone removes.

### Signal

Signal already has a strong branch basis artifact, snapshot and restore,
definition-bound state, branch fork/lifecycle, and graph-instance currentness.
This milestone should extend or adapt that existing authority only where the
component-basis contract requires it. It must not introduce a second Signal
branch model or reduce the basis to a branch id plus version number.

Signal must additionally own a versioned durable component artifact and bounded
fresh-process recovery/readmission contract. PostgreSQL stores that artifact
through the committed `worth-runtime-postgres::owner::signal` sibling; neither
the adapter nor Runtime Bridge may reinterpret Signal state.

### Shared substrates

- Use `worth-foundational` for portable branch/commit identity vocabulary,
  canonical basis encoding, locators, parentage descriptions, mismatch
  descriptions, provenance, and performance units that must retain the same
  meaning across owners.
- Use `worth-proof` for unresolved-to-admitted component-basis progression,
  assumption basis, freshness, trust-boundary weakening/readmission,
  structural facts, checked outcomes, and performed owner publication beneath
  stronger Relational- or Signal-owned types.
- Do not put runtime counters, locks, head maps, transaction tables, retention
  registries, or owner-specific authority in either substrate crate.

## Adversarial Courtroom

Create two Relational branches from one retained basis. On branch A, begin a
transaction that blocks at the deepest lawful point before publication. While
it remains blocked, execute and commit a transaction on branch B. Then race two
writers against branch A's same expected head.

In the same world:

- branches A and B reach equal local version ordinals with different history;
- one transaction carries a branch-A snapshot with a branch-B identity;
- a stale transaction uses a current-looking numeric version;
- a copied or serialized basis crosses a trust boundary without owner
  readmission;
- a branch is archived while snapshots, transactions, and external component
  retention pins remain;
- branch-local commit succeeds while unrelated allocation and retention
  maintenance runs;
- cancellation occurs before reservation, after reservation, after validation,
  after owner-local immutable candidate creation, and before head movement;
- the exact Signal basis is reused across multiple future product branches;
- a foreign-runtime, stale-generation, restored, or definition-incompatible
  Signal basis is substituted; and
- a Signal derived cache or diagnostic digest is offered as basis authority.
- the process is killed after either owner reports durable publication, all
  process-local bases are destroyed, and a fresh runtime reopens both exact
  component branches from PostgreSQL; and
- a Relational branch is recovered from another branch's checkpoint/tail or a
  Signal artifact is substituted across definition/runtime generation.

The independent owner oracle must observe:

- branch B commits while branch A is blocked;
- exactly one same-head branch-A writer wins and the loser receives the exact
  typed stale/conflict posture;
- branch-qualified snapshots never observe or validate against another branch;
- equal ordinals do not compare as equal authority;
- cancelled or rejected work does not move a branch head;
- retained bases survive while any exact live obligation requires them;
- reclamation occurs only after every owner-issued obligation releases;
- Signal basis reuse performs no clone, evaluation, or head lookup;
- hostile component-basis substitutions deny before owner effects; and
- every emitted component candidate and committed basis can be traced to one
  actual owner-performed publication;
- acknowledged branch publications survive fresh-process recovery without
  collapsing branch identities; and
- recovered Signal/Relational descriptors require fresh owner readmission
  before operational use.

A per-branch mutex placed behind the existing global mutable runtime is not
enough if entering the transaction, allocating identity, publishing history,
or retaining snapshots still serializes unrelated branches.

## Product Decision Lock

1. Each runtime mints and validates its own exact component basis.
2. A component basis is runtime-affine and branch-qualified. It retains exact
   version/generation, lifecycle, definition/schema, snapshot, and retention
   posture needed by its owner.
3. Portable Foundational branch and commit values may be retained inside the
   owner artifact but never substitute for it.
4. Generic Proof carriers supply progression law but cannot enter owner
   mutation or publication APIs in place of owner-specific artifacts.
5. Trust-boundary serialization, checkpoint restoration, or process transfer
   weakens basis confidence. The owner must readmit against current authority
   before operational use.
6. Relational transaction identity, snapshot, read set, write set, expected
   head, conflict comparison, candidate, commit, and receipt all name one exact
   branch.
7. Branch versions need only be ordered within their declared authority scope.
   Equal numbers on different branches or runtimes are never equivalent.
8. Unrelated Relational branches have independently borrowable and
   coordinatable mutation state. No ordinary global commit mutex or global
   `&mut RelationalRuntime` entry may serialize them.
9. Shared allocators may use bounded atomic, partitioned, or reservation
   mechanics, but allocation coordination cannot become a disguised global
   commit coordinator.
10. Conflict meaning remains Relational-owned and compares the exact branch,
    expected basis, authoritative read set, and changed truth required by the
    transaction contract.
11. A prepared Relational publication candidate is immutable owner output with
    no authority to move a future product branch. It can only become
    Relational-current through Relational publication and product-current only
    through 9.17.2 Bridge orchestration.
12. Signal exact-basis reuse is immutable sharing. Mutation requires Signal-
    issued fork or advancement and returns a new exact basis.
13. Derived indexes, caches, snapshots without retained owner currentness,
    diagnostics, canonical digests, and Query projections are non-authoritative.
14. Retention is current-obligation-bound. Head movement alone cannot reclaim a
    basis retained by another branch, snapshot, transaction, publication
    candidate, correction, or recovery obligation.
15. Owner-local ordinary execution and reconstructive checkpoint/recovery work
    remain different cost lanes.
16. Relational durable streams, checkpoints, replay tails, and recovery cursors
    are qualified by exact runtime and branch. The PostgreSQL adapter cannot
    choose or infer a branch head.
17. Signal owns its durable artifact format, compatibility, bounded decode, and
    recovery validation. PostgreSQL stores exact artifacts and physical indexes
    only.
18. Acknowledged owner publication means that owner's canonical artifact is
    durable before success. Recovery creates fresh owner authority; stored
    descriptors and snapshots remain non-authoritative.

## Compiler-Enforced Progression

The runtime types must expose phase order, not a conventionally ordered method
bag. The exact names may follow the owning crate vocabulary, but the legal
shape is:

```text
portable component basis descriptor
    -> owner-resolved component basis
    -> owner-admitted current component basis
    -> owner-scoped observation or transaction basis

Relational transaction intent
    -> branch-bound transaction
    -> validated branch-local proposal
    -> prepared owner publication candidate
    -> performed Relational publication
    -> exact committed Relational branch basis
```

Each arrow consumes a private-minted predecessor and the specific owner
witness or capability required for the transition. Checked runtime transitions
preserve success, denial, deferred, stale, rebind-required, cancellation, and
failure posture. A public caller must be unable to:

- construct an admitted basis from raw Foundational values;
- use a descriptor after boundary crossing without owner readmission;
- prepare without a branch-bound validated proposal;
- publish a raw or merely prepared candidate;
- reuse a consumed publication witness; or
- pair a transaction from one branch with another branch's expected head.

Use `worth-proof` contracts such as explicit binding axes, freshness source,
readmission, structural facts, and `Performed` under private owner wrappers.
Do not create a parallel local proof framework, and do not expose generic proof
parameters as caller-selectable authority markers.

## Destination Topology

```text
worth-relational/
    branch/
        identity.rs
        basis.rs
        creation.rs
        lifecycle.rs
    history/
        commit.rs
        parentage.rs
        retention.rs
    mvcc/
        branch_version.rs
        snapshot.rs
        transaction.rs
        read_set.rs
        conflict.rs
        coordinator.rs
        preparation.rs
        publication.rs
    facade/
        branch_observation.rs
        branch_transaction.rs
    durability/
        branch_stream.rs
        branch_checkpoint.rs
        branch_recovery.rs
        facade.rs

worth-signal/
    branch/
        basis.rs
        fork.rs
        snapshot.rs
        restore.rs
        lifecycle.rs
    durability/
        artifact.rs
        compatibility.rs
        recovery.rs
        facade.rs

worth-runtime-postgres/
    owner/
        relational/
            branch_stream.rs
            branch_checkpoint.rs
            branch_recovery.rs
        signal/
            component_artifact.rs
            branch_recovery.rs
            catalog.rs

worth-relational-certification/
    branch_local_mvcc/
        independent_progress.rs
        same_head_race.rs
        equal_ordinal_substitution.rs
        retention.rs

worth-signal certification owner/
    branch_basis/
        exact_reuse.rs
        substitution.rs
        readmission.rs
```

Existing repository topology may require responsibility-preserving names rather
than literal directories, but the final structure must make branch identity,
history, MVCC, and facade ownership spatially obvious. Forbidden placement
includes `helpers.rs`, a new `branch_manager.rs`, Bridge-owned component
currentness, Query-owned basis minting, SQL inside either component owner,
adapter-selected branch heads, and a test-only coordinator.

## Phase Plan

### Phase 1: Component Basis Vocabulary And Owner Ports

Freeze the portable Foundational vocabulary reused by both component owners and
the stronger owner-specific basis artifacts admitted into future composition.
Inventory and delete raw id/version pairing, equal-ordinal comparison, ambient
head selection, and basis creation outside owner facades. Install exact binding
axes and compile-visible owner readmission.

This phase closes only when raw Foundational values and generic Proof carriers
cannot mint or substitute owner component authority.

### Phase 2: Branch-Qualified Relational Observation And Transaction State

Separate immutable branch observation from branch-local mutation state.
Qualify snapshots, transactions, read sets, write sets, expected heads,
conflicts, versions, and receipts by one branch. Make cross-branch assembly
unrepresentable or typed-denied before effects.

This phase must not claim concurrency merely because branch ids appear in
types. The ordinary entry path must stop requiring broad mutable runtime access
whose borrow or lock serializes unrelated branches.

### Phase 3: Branch-Local Coordination And Owner Publication

Install independently coordinatable branch state, safe allocation, owner-local
preparation, atomic Relational head publication, history recording, and exact
committed basis issuance. Preserve one canonical Relational commit artifact
from which history, receipts, and downstream candidate views derive.

The selected branch may coordinate with its own head and touched storage.
Unrelated branches must make measured progress while another writer blocks.

### Phase 4: Component Durability, Retention, And Fresh-Process Readmission

Bind snapshot, transaction, branch, and future external composition pins to
exact component bases. Define archive, deletion, candidate release,
branch-qualified Relational checkpoint/tail recovery, Signal artifact recovery,
trust-boundary weakening, owner readmission, and reclamation. Populate the
Signal PostgreSQL owner sibling and extend the Relational sibling without
changing the 9.16.2 facade. Keep ordinary commit cost separate from retention
scans and reconstruction.

### Phase 5: Owner-Local Hostile Certification And 9.17.2 Handoff

Run the complete owner courtroom with real transaction/fork/restore facades,
independent branch and history oracles, exact counters, default and admitted
parallel configurations, compile-fail public misuse cases, cancellation at
every effect boundary, and mutation-sensitive controls. Freeze the owner ports
that 9.17.2 consumes without exposing internal storage or mutation authority.
Include real-PostgreSQL fresh-process recovery and cross-branch artifact
substitution mutants.

## Performance Contract

- Opening an observation or transaction is O(1) plus declared branch-local
  snapshot pinning; it does not scan branches or history.
- Validation and conflict detection scale with the transaction's declared read
  and write sets plus actual branch-local overlap, not total graph size or
  other branches.
- Publication coordinates only the selected branch, touched partitions, and
  globally necessary bounded allocation mechanics.
- A blocked branch A writer contributes exact zero branch-B wait count.
- Component basis validation is O(1) in the fixed identity/binding axes.
- Signal exact-basis reuse performs zero graph copy, evaluation, or cache
  duplication.
- Retention reclamation and checkpoint reconstruction are explicit cold or
  maintenance operations with their own counters and bounds.
- Owner publication performs only the selected branch's required durability
  barriers; unrelated branch streams contribute zero synchronous writes.
- Counters distinguish branch-state acquisition, snapshot pins, read/write
  validation, conflicts, allocations, owner preparation, publication, history
  append, retention pins, waits, retries, cancellation cleanup, and
  reconstruction, durable bytes, barriers, and recovery artifacts.

## Proof Portfolio

The proof portfolio must include:

- public compile-pass examples for the intended owner facades;
- compile-fail cases for raw basis minting, phase skipping, cross-branch
  transaction/head pairing, generic authority substitution, and prepared-
  candidate publication;
- branch A blocked / branch B progress with a deterministic scheduler or
  equivalent controlled concurrency court;
- same-head winner/loser and equal-ordinal foreign-branch twins;
- checkpoint/serialization freshness downgrade and owner readmission;
- exact Signal basis reuse and incompatible/foreign/stale basis denial;
- retention survival and exact reclamation after every obligation releases;
- cancellation at every named effect boundary;
- slope evidence as unrelated branch population, history, and writers grow;
  and
- mutation probes that reintroduce the global coordinator, drop one binding
  axis, accept equal ordinals, bypass readmission, or reclaim a retained basis.

Tests must use production world constructors and real owner facades. A fixture
that directly writes heads, versions, snapshots, or basis generations cannot
certify this milestone.

## Documentation Deliverables

- Relational branch-local MVCC architecture and concurrency contract;
- Relational component basis and Signal component basis owner guides;
- trust-boundary downgrade/readmission examples;
- branch-qualified Relational PostgreSQL recovery and Signal component recovery
  operator guide;
- exact ordinary versus reconstruction cost-lane documentation;
- executable facade examples; and
- a 9.17.2 integration contract naming only the owner ports and artifacts the
  Bridge may consume.

## Must Preserve

- all 9.16.1 branch-affine provider-session semantics;
- all 9.16.2 package identity, fresh-validation, PostgreSQL durability foundation,
  runtime-level facade, recovery-barrier, and existing-outbox guarantees, with
  no branch/component authority added to records, SQL rows, snapshots, or
  dispatch leases;
- Relational authoritative graph and commit meaning;
- Signal definition-bound branch and derived-state meaning;
- existing snapshot, history, retention, and recovery guarantees;
- Foundational as portable vocabulary rather than authority;
- Proof as progression law rather than runtime owner; and
- Query and Bridge inability to mutate component internals directly.

## Explicit Non-Goals

- composite basis correspondence;
- composite commits or product branch references;
- coordinated Relational-plus-Signal publication;
- Query public branch workflow;
- semantic merge, rebase, correction, or distributed cross-runtime recovery; and
- replacing Signal's existing branch authority with a parallel model.

## Allowed Debt

- Store-native graph persistence, replication, and distributed recovery remain
  later Store/cross-runtime work. PostgreSQL component restart is not debt.
- No global ordinary Relational commit coordinator, broad mutable transaction
  entry, raw basis constructor, equal-ordinal authority comparison, missing
  owner readmission, or test-only branch path may remain debt.

## Parallelization And Store Dependency

Signal basis-contract adaptation and Relational branch-state decomposition may
progress in parallel after the complete shared binding vocabulary freezes.
Relational publication cannot close before branch-qualified observation,
transaction, and conflict law; retention/certification cannot close before the
publication artifact is canonical. This milestone is not blocked on
`worth-store`.

## Acceptance Evidence

Milestone 9.17.1 closes only when `Owner Component Basis And Relational
Branch-Local MVCC Certification` in
[test-requirements.md](./test-requirements.md) passes and:

- both component owners issue exact non-substitutable bases through their real
  facades;
- Relational branch-local transactions and publication no longer require an
  ordinary global mutable/coordinator boundary;
- unrelated branch writers make controlled concurrent progress;
- same-branch races, stale bases, foreign bases, and equal ordinals preserve
  exact typed outcomes;
- owner-local preparation and publication are compiler-ordered;
- checkpoint and trust-boundary artifacts cannot become operational without
  readmission;
- retention and cancellation leave exact bounded lifecycle posture;
- real PostgreSQL reopens exact Relational and Signal component branches after
  process loss, while cross-branch and cross-generation substitutions fail;
- exact counters and slopes match the declared cost boundary;
- facade, dependency, residue, line-cap, constitutional, and documentation
  checks agree; and
- no composite or Query product authority is claimed.

## Handoff

[Milestone 9.17.2](./milestone-9.17.2.md) receives only:

- owner-issued exact component bases;
- owner validation/readmission ports;
- owner-local prepared publication candidates and typed outcomes;
- exact committed component bases;
- versioned owner durable artifacts and bounded recovery/readmission ports;
- retention pin/release capabilities; and
- bounded cancellation and cleanup contracts.

It may compose these artifacts, but it may not inspect or mutate component
internals, compare raw ordinals as authority, fabricate owner currentness, or
reintroduce a global cross-owner lock.
