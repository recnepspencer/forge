# C.7: WAL, Checkpoint, Root Publication, And Physical Acknowledgment Join

## Goal

Install one canonical, Store-owned physical durability progression that binds
each admitted mutation to its WAL range, required backend barriers, page or
extent effects, pageLSNs, checkpoint or current-root publication, namespace
durability, exact terminal fate, and physical acknowledgment.

C.7 is complete only when the wrong order is unavailable to ordinary code and
when every surviving acknowledgment proves the exact effects required by the
admitted backend profile for the exact physical mutation identity.

## Why This Milestone Exists

C.4 through C.6 established the real media boundary, file-backed artifacts,
physical work topology, scheduler/executor separation, bounded residency, dirty
typestate, and exact writeback settlement. WAL, durability-ordering,
checkpoint, pageLSN, and root-publication mechanisms also exist in lower
crates. They are not yet one database durability law.

The most dangerous implementation is not an obviously broken WAL. It is a
well-written coordinator that calls individually valid mechanisms in a
plausible order, compares their receipts afterward, and returns success. Such a
system may pass every local test while:

- combining proofs from different operations, generations, scopes, or files;
- treating a WAL-only receipt as complete physical acknowledgment;
- letting a `WalCommit` label substitute for a WAL-before-data dependency;
- collapsing several mutations into one group-commit identity;
- retrying an effect whose physical fate is unknown;
- marking a C.6 frame clean before its exact data effect settles;
- treating rename completion as durable namespace publication; or
- leaving an isolated WAL, checkpoint, or root writer beside the canonical
  Store path.

C.7 is therefore an authority and causal-identity join, not a new mechanism
demo and not an orchestration wrapper around the current islands.

## Roadmap Placement And Inherited Truth

C.7 consumes:

- C.4's admitted filesystem owner, stable Store namespace, exact media
  operation identities, backend capability evidence, file synchronization,
  directory synchronization, atomic replacement, and indeterminate media
  outcomes;
- C.5's canonical page, segment, extent, manifest, catalog, stable record
  identity, current-root generation, copy-on-write candidate, and bounded
  physical format;
- C.5.1's Store-owned physical operation identity, derived Signal readiness,
  scheduler admission, executor-only media access, exact effect settlement,
  cancellation law, runtime generation, and terminal fate; and
- C.6's admitted dirty frame, exclusive writeback claim,
  `PhysicalWritebackSettlement`, bounded allocation scopes, frame generation,
  pressure evidence, and rule that dirty order is not durability order.

C.7 may consume those truths. It may not reconstruct or replace them.

C.7 precedes C.8 because recovery requires one real durability law. C.7 must
leave C.8 enough persisted truth to determine recovery source precedence and
resolve indeterminate operations, but C.7 does not itself claim fresh-process
recovery.

## Governing Boundary

C.7 owns physical durability ordering and physical acknowledgment.

It does not own:

- semantic MVCC, transaction visibility, or conflict resolution;
- branch identity, branch-head advancement, or branch-writer release;
- Query acknowledgment or semantic commit;
- buffer-pool construction, eviction, frame cleaning, or residency policy;
- backend capability invention;
- recovery source precedence or redo;
- stable-reader retention and reclaim policy; or
- integrity admission and corruption classification.

A physical acknowledgment means only:

> The exact mutation identity reached every physical durability edge required
> by its admitted request and backend profile, and every constituent proof is
> causally bound to that identity.

Operations that share a durability barrier do not thereby acquire semantic
order, shared mutation identity, or shared acknowledgment.

## Adversarial Constraint

The canonical durability path must survive this world:

> A real Store at least 32 times larger than its admitted resident-memory
> budget accepts independent physical mutations while three mutations share a
> group-commit opportunity, dirty eviction and exact writeback continue, a
> fuzzy checkpoint captures an exact source range, the WAL rotates at least
> twice, the bounded WAL-tail limit is reached, one caller cancels, one
> completion is delayed, and one process is killed at a named persistence
> seam. Workload and lawful scheduling are independently seeded. A fresh
> offline process receives only the Store root, stable format declarations,
> admitted backend profile, and expected scenario identity. It must prove that
> no persisted page outruns durable WAL, no root is promoted without its exact
> namespace barriers, no member identity is collapsed into its group, no
> ambiguous effect is labeled success or no-effect, and no bounded-memory or
> queue claim escapes into unbounded background work.

The system fails this milestone if the clean run is correct but any
individually valid receipt can be substituted, any effect can bypass the
C.5.1 topology, or any crash result can be explained only from writer-owned
memory, counters, logs, or serialized runtime state.

## Decisive Durability Courtroom

### Production subject

The production subject is the ordinary `ServingPhysicalRuntime` record
submission and checkpoint facades after C.7 cutover.

The call path must include:

```text
worth-store public physical facade
  -> Store-owned durability admission and identity
  -> worth-store-wal frame/range declaration
  -> Store-owned physical Signal readiness
  -> worth-store-io-scheduler resource admission
  -> Store executor
  -> C.4 filesystem media owner
  -> worth-store-physical-backend effect receipt
  -> C.6 dirty/writeback settlement where data frames participate
  -> Store-owned root/checkpoint publication settlement
  -> physical terminal outcome
```

`worth-store-wal`, `worth-store-recovery-physics`,
`worth-store-physical-backend`, `worth-store-buffer-pool`, Signal, and the
scheduler remain specialized participants. None may expose a second ordinary
end-to-end execution path.

The harness may:

- choose a declared production yieldpoint;
- perturb simultaneously lawful harness decisions;
- terminate a child process without cleanup;
- wrap the C.4 media observation boundary;
- invoke the read-only offline inspector; and
- materialize terminal evidence after the run.

It may not mint production receipts, edit private runtime state, mutate files
after the declared crash, supply recovered state, or treat a simulated error as
process death.

### Process roles

The courtroom retains the C.6 fresh-process accounting pattern:

1. **Producer** initializes the Store and writes the deterministic baseline.
2. **Serving writer** opens the Store through the ordinary facade, executes the
   durability workload, and is the only process that may be killed.
3. **Offline durability inspector** opens no Store runtime and parses WAL,
   page, checkpoint, manifest, catalog, and root artifacts through stable
   physical declarations only.
4. **Fresh physical reopener** attempts the ordinary physical open posture
   allowed before C.8 and reports discovered publication/residue facts without
   performing C.8 recovery or receiving writer heap state.

Each role records distinct process, binary, source, protocol, Store, and
runtime identities. The runner rejects role reuse.

### Initial world

The initial world fixes:

- a real, named filesystem and backend durability profile;
- one Store root initialized by the producer process;
- a Store data set at least 32 times the resident-byte budget;
- a page size and WAL segment size that force multiple pages and at least two
  WAL rotations during the workload;
- a bounded dirty-frame budget and a bounded checkpoint WAL-tail budget;
- three distinct physical mutations eligible for one shared barrier;
- stable, caller-supplied physical idempotency keys with no branch meaning;
- one current and one retainable previous physical root generation;
- separate workload and schedule seeds;
- exact media, WAL, page, checkpoint, root, allocation, queue, and
  acknowledgment expectations derived before the serving process starts; and
- absence of every staged WAL, checkpoint, root, or temporary artifact that the
  scenario has not yet lawfully created.

The expected operation identities and bytes are generated independently of the
runtime's classifiers and output projection.

### Crash-seam matrix

The serving process is killed in a fresh Store copy at each exact seam:

1. before WAL append;
2. during WAL append after a strict nonzero prefix but before the complete
   frame;
3. after complete WAL bytes are written but before the required WAL barrier;
4. after the required WAL barrier but before data dispatch;
5. during a data write after a strict nonzero prefix;
6. after data settlement but before current-root publication;
7. after atomic current-root replacement but before required parent-namespace
   durability; and
8. after complete physical durability but before acknowledgment is observed by
   the caller.

Each scenario changes only the crash seam. A distinct fixture must not
preconstruct its expected answer.

### Group-commit identity blender

Three mutations share one admitted barrier opportunity:

- one is cancelled while still proven pre-effect;
- one completes and returns its physical acknowledgment; and
- one completes physically but loses caller observation at the final seam.

The schedule may reorder member admission, WAL-range allocation, scheduler
dispatch, data settlement, and completion delivery only where prerequisites
permit.

The courtroom requires:

- one exact shared barrier execution;
- one exact shared root publication when the sealed group plan declares it;
- three permanently distinct operation and idempotency identities;
- one exact WAL subrange and data-effect set per member;
- no group-level terminal fate;
- no cancellation or acknowledgment propagation between members;
- no semantic order inferred from group position;
- one completed acknowledged member;
- one proven-no-effect cancelled member; and
- one completed-but-unobserved physical fact left for C.8 reconciliation.

### Checkpoint pressure siege

While foreground mutations continue:

- begin one fuzzy checkpoint from an exact admitted source range;
- keep dirtying pages while capture advances;
- force at least one dirty eviction and exact C.6 writeback;
- rotate the WAL at least twice;
- reach the declared retained-tail limit;
- submit one additional mutation that cannot be admitted without exceeding a
  queue, dirty, or retained-tail bound;
- publish the checkpoint candidate;
- attempt premature WAL deletion or recycling; and
- retain the current and immediately previous root bases required by the
  declared C.8 handoff.

The runtime must reject or backpressure before violating a bound. It may reduce
throughput. It may not stop mutation authority for the entire capture, allocate
the complete Store, widen a budget, discard required WAL, or acknowledge a
mutation from checkpoint progress.

### Independent observation

The offline inspector independently records:

- complete and torn WAL frames;
- segment and WAL generations;
- exact written and durable-prefix ranges under the named profile;
- page and extent identities, bytes, generations, and stored pageLSNs;
- checkpoint identity, source range, covered LSN range, and publication
  posture;
- current, previous, candidate, staged, and forbidden root artifacts;
- atomic replacement and parent-namespace durability observations available at
  the admitted media boundary;
- retained and prematurely missing WAL segments;
- per-operation idempotency identity and physical effect correlation;
- exact file paths, lengths, offsets, and digests; and
- absence of runtime, Signal, scheduler, pool, counter, or writer-memory
  authority in its process.

The independent oracle may share canonical byte grammar and stable identity
types. It may not call the runtime's durability classifier, settlement join,
recovery-source selector, or evidence projector.

### Required assertions

The courtroom must prove:

- every acknowledged mutation has one exact complete WAL frame and admitted
  durable WAL basis;
- every persisted pageLSN equals the greatest exact redo-record LSN reflected
  in that page image, its payload digest matches that ordered redo basis, and
  the complete basis is covered by independently observed durable WAL;
- no data effect dispatches before its matching WAL barrier proof;
- every WAL, barrier, checkpoint, root, and reused ExactWriteback effect joins
  its exact installed Signal basis, scheduler admission, executor effect, and
  Store settlement;
- no frame becomes clean without its matching C.6 effect and writeback
  settlement;
- every root or checkpoint publication consumes exact data and WAL bases;
- rename completion alone never satisfies a profile requiring directory or
  parent-namespace durability;
- group sharing reduces barrier executions without reducing member
  cardinality;
- physically disjoint mutation preparation and data work progress concurrently,
  coordinating only at exact WAL allocation/barrier and current-root cutover
  scopes;
- no receipt from another member, Store, runtime, generation, artifact, range,
  profile, or effect can settle the target mutation;
- a duplicate idempotency key with the same fingerprint performs no second
  effect, while the same key with a different fingerprint is denied before
  effect;
- cancellation before effect yields proven no effect;
- a complete authoritative WAL member prevents aggregate proven-no-effect even
  when data dispatch has not begun;
- cancellation, timeout, or caller loss after possible effect cannot erase
  settlement and cannot become automatic retry authority;
- no partially settled member or group becomes reachable from the current root;
- torn WAL and data writes remain distinguishable from complete effects;
- stale completions fail at their first Store consumer;
- checkpoint work remains within exact capture, memory, queue, and WAL-tail
  bounds;
- WAL retention consumes a covering checkpoint and contiguous-tail proof;
- the current and retained previous root facts are sufficient inputs for C.8
  without claiming that C.7 recovered them;
- ordinary close drains every durability, barrier, checkpoint, publication,
  and acknowledgment obligation;
- abrupt death leaves only classified persisted residue; and
- no ordinary direct WAL, durability-runtime, checkpoint, or root writer
  remains outside the Store progression.

### Schedule perturbation and exact replay

C.7 extends the one C.6 schedule-perturbation harness. It does not create a
second scheduler, seed type, trace grammar, or replay protocol.

The replay identity contains:

```text
source revision
binary identity
workload seed
schedule seed
crash seam
backend profile
checkpoint policy identity
scale tier
mutation identity, when present
```

The closed C.7 decision vocabulary may perturb only simultaneously lawful
choices:

- group-member admission;
- WAL-range reservation among admitted members;
- WAL append dispatch;
- barrier completion delivery;
- data-effect dispatch;
- dirty-writeback opportunity;
- checkpoint capture slice;
- root-publication opportunity;
- pre-effect cancellation delivery; and
- terminal completion delivery.

It may not reorder prerequisites, sleep to manufacture races, alter the
production scheduler, or select the crash seam implicitly. The seam is an
explicit scenario input so identical replay inputs select an identical kill
boundary.

CI uses exactly 16 distinct revision-derived schedule seeds. The eight crash
seams are rotated so every seam appears in at least two lanes. Release
certification runs the canonical schedule at every seam and runs the complete
current C.7 mutation corpus. Scheduled hardware qualification may run the full
seed-by-seam matrix at larger scale on each genuinely admitted profile.

Every failure emits one exact rerun command.

### Mutation sensitivity

The initial corpus contains a causal controlled defect for each disputed edge:

- acknowledge before the required WAL barrier;
- dispatch data before matching WAL durability;
- persist a pageLSN ahead of durable WAL;
- choose an unrelated but range-contained pageLSN or omit one applied redo
  record from the page basis;
- collapse group-member identities;
- substitute another member's otherwise valid receipt;
- execute a duplicate same-fingerprint idempotency request twice;
- accept one idempotency key with a different physical fingerprint;
- include allocated operation, group, WAL-member, or WAL-range identity in the
  request fingerprint so a lawful retry conflicts with itself;
- omit effect-relevant policy, scope, payload, operation-family, or security
  basis from request-fingerprint equivalence;
- omit the required file synchronization;
- omit the required directory or parent-namespace synchronization;
- treat atomic replacement as durable root publication;
- clean a frame before exact C.6 settlement;
- automatically retry an indeterminate effect;
- accept a stale runtime, Store, WAL, checkpoint, or root generation;
- recycle WAL without a covering checkpoint and contiguous retained tail;
- bypass C.5.1 with one direct media effect;
- misbind or collapse one installed C.7 Signal work family or aspect partition;
- substitute a Foundational policy or performance receipt for scheduler
  admission or completed physical effect outside its exact declared role;
- accept a `StoreExecutedBoundaryReceiptEvidence` projection back into the
  authoritative mutation progression;
- ignore checkpoint-tail or queue admission;
- stop the schedule seed from affecting one declared decision;
- preserve a competing ordinary WAL, checkpoint, or root execution lane; and
- serialize the complete mutation lifecycle under one whole-Store lock.

Each mutant must exercise real production source, die at its nearest named
predicate, restore byte-identical source, and leave no injected residue.

After corpus adoption, every real executable C.7 production or certification
bug correction appends its causal mutation in the same change. Mutation
identities are append-only and never reused. A finding cannot close until its
mutation dies in the complete catalog-derived campaign.

### Mechanical anti-substitution gates

The milestone must mechanically reject:

- a WAL-only acknowledgment used as final C.7 acknowledgment;
- a durability request or counter used as completed-effect proof;
- a group barrier used as a member identity or member terminal outcome;
- a fixed-arity join, tuple, or optional slots used for a runtime-width group;
- a raw LSN comparison used to authorize data dispatch;
- request equivalence derived from allocated attempt or WAL identity;
- a Foundational profile, policy receipt, performance receipt, or executed
  evidence projection used as backend admission or physical authority;
- a direct `StoreDurabilityRuntime`, WAL executor, checkpoint writer, root log,
  or filesystem call reachable from the ordinary facade;
- Signal evaluation performing or authorizing durability effects;
- scheduler completion settling Store truth;
- C.7 importing Query, Relational, Runtime Bridge, branch, MVCC, or semantic
  writer authority;
- C.7 reaching into buffer-pool construction, frame-table, eviction, or clean
  transition internals;
- certification, replay, model, JSON, logs, or evidence bundles entering the
  ordinary progression;
- same-process crash inspection;
- graceful close standing in for abrupt death;
- an in-memory WAL or mock filesystem satisfying a physical claim; and
- a feature combination that reintroduces any removed ordinary execution
  route.

## Product Decision Lock

1. `worth-store` owns the one end-to-end physical durability progression.
   Lower crates own meaning or mechanics, not cross-owner orchestration.
2. `worth-store-wal` owns WAL frame grammar, LSN topology, append planning,
   bounded scan, and stable WAL identities. It performs no ordinary OS effect.
3. The C.4 filesystem owner remains the only ordinary media-effect boundary.
   Backend durability types declare and report admitted physical mechanics;
   they do not own mutation acknowledgment.
4. `worth-store-recovery-physics` retains recovery-facing WAL, checkpoint, tail,
   and crash-basis meaning. Its direct ordinary WAL execution path is removed.
5. C.6 dirty state proves a frame requires settlement. It never proves WAL
   order, durability, or acknowledgment.
6. The Store assigns each admitted mutation one physical operation identity and
   binds it to one caller idempotency key and canonical request fingerprint.
   Later operation, group, member, and WAL allocation live only in the
   persisted attempt binding. None of those identities may replace another or
   feed back into request equivalence.
7. Group commit may share the exact WAL barrier and one exact root publication
   for a sealed member set. Shared effects derive matching member proofs for
   every included mutation. Grouping never merges operation identity, WAL
   subrange, data effect, terminal fate, cancellation, recovery posture, or
   acknowledgment.
8. WAL range reservation is not WAL durability. WAL bytes written are not WAL
   durability. File synchronization is not namespace durability. Rename is not
   parent-directory durability. Each is a distinct proof state.
9. Data dispatch consumes the exact matching WAL-durable member proof. A
   `WalCommit` enum, queue class, boolean, counter, or raw LSN cannot substitute.
10. PageLSN meaning is split lawfully: WAL owns LSN identity and order,
    physical format owns encoded pageLSN bytes, Store owns their exact mutation
    join, and C.8 recovery will own comparison and redo policy.
11. Physical acknowledgment is constructed only from the strongest completed
    Store progression. Existing locally honest acknowledgment types are narrowed
    to their actual boundary or removed; aliases are forbidden.
12. A complete authoritative WAL member is already a physical effect and a
    durable continuation obligation. It prevents aggregate `ProvenNoEffect`
    even when no data page has yet changed.
13. Once an effect may have begun, cancellation and timeout cannot mean
    rollback. Settlement continues to completed or indeterminate physical fate.
14. Automatic retry requires proven no effect and the original exact
    idempotency identity. Indeterminate work opens no retry door before C.8
    reconciliation.
15. Checkpoint capture is fuzzy or non-blocking, carries an exact source range,
    and has hard memory, queue, and retained-WAL bounds. Whole-Store capture and
    unbounded tail growth are forbidden.
16. Root publication retains one canonical current-root authority. Any
    publication history needed by C.8 or C.10 derives from that progression; a
    second root authority or `root-publications.log` writer is forbidden.
17. The immediately previous root and required WAL/checkpoint bases remain
    physically retained until a later owner proves reclamation eligibility.
    C.7 does not infer semantic liveness.
18. Worth Signal derives dependency readiness and generic async lifecycle only.
    The scheduler admits resources only. The Store decides order and settlement;
    the executor alone performs effects.
19. `worth-proof` supplies outcome, progression, basis, and structural
    collection mechanics. C.4 owns platform durability semantics and exact
    capability claims; Store owns their sealed admission join and physical
    progression. A copied proof, digest, generic marker, or declaration cannot
    become admission or completed-effect evidence.
20. Worth Foundational and Store aspect-native contracts retain semantic aspect
    identity, derived routing bases, scheduler policy admission, and
    descriptive evidence projection. They do not own backend profile,
    physical fate, current root, or acknowledgment. No generic `Durable`,
    `Committed`, or `Published` Foundational flag becomes physical authority.
21. JSON exists only at the terminal evidence projection or an explicitly named
    external compatibility edge. It is forbidden from mutation admission,
    progression, Signal binding, scheduling, settlement, or acknowledgment.
22. C.7 adds no branch registry, branch queue, semantic transaction id, semantic
    commit receipt, MVCC generation, or global semantic lock.
23. C.7 may serialize the exact WAL-allocation, barrier, and current-root
    cutover scopes. It may not hold a whole-Store mutation lock across framing,
    data work, checkpoint capture, or caller observation; physically disjoint
    work remains concurrently admissible.
24. The product is unreleased. Cutover deletes obsolete paths, aliases,
    compatibility adapters, fallback executors, and legacy features in the same
    phase that replaces their last consumer.

## Semantic Vocabulary Lock

The following distinctions are normative:

- **physical mutation**: one Store-owned operation with exact scope and
  idempotency identity; never a semantic transaction;
- **request fingerprint**: versioned canonical equivalence of effect-relevant
  request inputs; never an attempt, operation, group, member, LSN, or WAL-range
  identity;
- **attempt binding**: the persisted relationship between one idempotency
  key/fingerprint and its allocated physical operation, group/member, and WAL
  facts; never request equivalence or completed fate;
- **WAL range reserved**: stable LSN identity exists; no durability claim;
- **WAL appended**: complete bytes were observed written; required barriers may
  still be absent;
- **WAL durable**: the exact member range is covered by the admitted backend
  barrier proof;
- **data settled**: exact page/extent effects reached completed, proven-no-
  effect, or indeterminate Store fate;
- **root replaced**: atomic replacement was observed; namespace durability may
  still be absent;
- **root namespace durable**: the admitted profile's exact root-publication
  barriers completed;
- **physical acknowledgment**: the completed mutation crossed every declared
  edge and may be reported to its caller;
- **completed but unobserved**: physical completion exists but the caller did
  not receive acknowledgment;
- **indeterminate**: an effect may have occurred and neither success nor
  no-effect is lawfully proven; and
- **semantic commit**: a future Part II decision that C.7 neither represents
  nor performs.

Production names must include the physical subject where omission would invite
semantic interpretation. Generic `CommitReceipt`, `Transaction`, `Durable`,
`Published`, `Success`, `Manager`, `Coordinator`, and `Handler` names are
forbidden when the stronger physical meaning is required.

## Normative Public API

### Durability configuration admission

The serving runtime consumes one admitted physical durability configuration:

```rust
let durability_basis: PhysicalDurabilityAdmissionBasis =
    media.physical_durability_admission_basis()?;

let durability = match PhysicalDurabilityDeclaration::builder()
    .group_commit(GroupCommitLimit::new(32)?, GroupCommitDelay::new(...)?)
    .checkpoint(PhysicalCheckpointPolicy::fuzzy(
        CheckpointMemoryLimit::new(...)?,
        RetainedWalTailLimit::new(...)?,
    ))
    .admit(durability_basis)
    .into_raw()
{
    TransitionOutcome::Success(policy) => policy,
    TransitionOutcome::Denied(denial) => return handle_policy_denial(denial),
    TransitionOutcome::Deferred(deferred) => return handle_policy_deferral(deferred),
    TransitionOutcome::Stale(stale) => return handle_stale_policy_basis(stale),
    TransitionOutcome::RebindRequired(rebind) => return rebind_policy(rebind),
    TransitionOutcome::Failed(failure) => return inspect_policy_failure(failure),
};

let serving = match media
    .initialize_record_store(PhysicalRecordInitialization::new(
        format,
        placement,
        access,
        durability,
    ))
    .into_raw()
{
    TransitionOutcome::Success(serving) => serving,
    TransitionOutcome::Denied(denial) => return handle_store_denial(denial),
    TransitionOutcome::Deferred(deferred) => return handle_store_deferral(deferred),
    TransitionOutcome::Stale(stale) => return handle_stale_store(stale),
    TransitionOutcome::RebindRequired(rebind) => return rebind_store(rebind),
    TransitionOutcome::Failed(failure) => return inspect_store_failure(failure),
};
```

The exact constructor shape may follow the final C.6 initialization facade, but
these laws are fixed:

- the declaration is configuration, not authority;
- `PhysicalDurabilityAdmissionBasis` is a sealed C.4-derived Store admission
  basis constructible only from `QualifiedFilesystemMedia`;
- the basis binds the exact `RootProfileQualificationBasis` and
  `AdmittedBackendCapabilityWitness` claims required by the declaration,
  including `Fsync`, `DirectorySync`, and `DurableRename` where the selected
  policy needs them;
- policy admission consumes that concrete basis and returns
  `ProofOutcome<AdmittedPhysicalDurabilityPolicy, ...>` with typed admitted,
  denied, deferred, stale, rebind-required, and failed branches;
- `worth-proof` supplies the outcome and progression mechanics; it does not
  define filesystem capability, durability-profile, or barrier semantics;
- unsupported, stale, mismatched, or insufficient profiles return typed
  denial before runtime construction;
- required limits are nonzero, finite, and explicit;
- no runtime opens with an optional durability owner;
- no `bool` selects weakened durability; and
- adding the C.7 owner breaks every incomplete initialize, open, close, abort,
  and observation construction site.

The admitted policy carries the fingerprint of its consumed C.4 admission
basis. Runtime initialization verifies that fingerprint against the same
`QualifiedFilesystemMedia` generation before creating any owner. Copying
capability names, profile labels, basis digests, or individual claims cannot
construct the sealed basis or satisfy this join.

### Ordinary physical mutation

The common path requests the strongest admitted ordinary physical boundary:

```rust
let request = PhysicalMutationRequest::platform_durable(
    PhysicalMutationIdempotencyKey::new(caller_key)?,
    PhysicalMutationDeadline::at(deadline),
);

let prepared = match serving
    .record_submission()
    .prepare_durable_append(
        RecordAppendBatch::try_from_iter(records)?,
        placement,
        request,
    )
    .into_raw()
{
    TransitionOutcome::Success(prepared) => prepared,
    TransitionOutcome::Denied(denial) => return handle_mutation_denial(denial),
    TransitionOutcome::Deferred(deferred) => return handle_mutation_deferral(deferred),
    TransitionOutcome::Stale(stale) => return handle_stale_mutation(stale),
    TransitionOutcome::RebindRequired(rebind) => return rebind_mutation(rebind),
    TransitionOutcome::Failed(failure) => return inspect_mutation_failure(failure),
};

match prepared.execute() {
    PhysicalMutationOutcome::Completed(completed) => {
        let acknowledgment: PhysicalMutationAcknowledgment =
            completed.into_acknowledgment();
        use_physical_fact(acknowledgment);
    }
    PhysicalMutationOutcome::ProvenNoEffect(no_effect) => {
        handle_retryable_physical_fate(no_effect);
    }
    PhysicalMutationOutcome::Indeterminate(indeterminate) => {
        preserve_for_recovery(indeterminate);
    }
}
```

This example fixes the public semantics, not every private method name.

- Preparation validates scope, idempotency, limits, health, Store generation,
  profile, and resource shape before effects.
- Preparation returns
  `ProofOutcome<PreparedPhysicalMutation, ...>` and preserves distinct denied,
  deferred, stale, rebind-required, and failed branches. It does not compress
  them into one error or convert admission failure into physical effect fate.
- `PreparedPhysicalMutation` is consuming and non-`Clone`.
- Deadlines are evaluated through the admitted C.5.1/Signal monotonic clock
  basis; wall clock, sleep duration, and ambient process time are not
  correctness inputs.
- Execution returns a typed physical fate rather than `Result<Success, Error>`
  for effectful uncertainty.
- Only `CompletedPhysicalMutation` exposes
  `into_acknowledgment`.
- `ProvenNoEffectPhysicalMutation` carries the exact denial, cancellation,
  timeout, or safe-retry basis and original idempotency identity.
- `IndeterminatePhysicalMutation` carries the exact persisted-effect and
  inspection basis required by C.8 and exposes no acknowledgment or automatic
  retry method.
- Opaque upstream correlation may be retained as metadata but cannot affect
  physical scope, grouping, scheduling, ordering, or fate.

The existing `append_batch(...)->PublishedRecordBatch` convenience is replaced
or narrowed so it cannot hide durability request, idempotency, cancellation,
indeterminate fate, or contractual cost. No compatibility wrapper retains the
old stronger-looking behavior.

### Idempotency

The first admitted use of a `PhysicalMutationIdempotencyKey` binds it to one
`PhysicalMutationRequestFingerprint`. That fingerprint is canonical input
equivalence and contains:

- Store identity and admitted durability-policy basis identity;
- exact physical scope;
- immutable payload digest;
- durability request;
- operation-family identity; and
- every security or authority basis that can change the lawful physical effect.

It excludes attempt-local facts: runtime generation, operation identity,
group identity, WAL member or range, queue placement, schedule choice,
deadline, cancellation state, completion-delivery correlation, and observation
metadata. Those facts can differ across a lawful retry without changing the
requested physical mutation.

The fingerprint uses the Store aspect-native
`StoreDigestEquivalenceBasis` and a new admitted
`StoreCanonicalBasisFamily::PhysicalMutationRequestFingerprint`. Version 1
freezes field order, canonical encoding, domain separator
`store.physical.mutation.request-fingerprint.v1`, and SHA-256. A future
algorithm or encoding change creates a new version; it never reinterprets an
existing fingerprint. Terminal JSON, debug formatting, WAL bytes, an allocated
LSN, or a caller-supplied digest is not a lawful fingerprint source.

The closed canonical registry is extended honestly rather than reusing
`WalRecord` or an evidence family:

- source kind: `StoreCanonicalBasisSourceKind::StorePhysicalMutationRequest`;
- field role:
  `StoreCanonicalBasisFieldRole::NativePhysicalMutationRequest`;
- lane: `StoreCanonicalBasisLane::PhysicalMutation`; and
- family:
  `StoreCanonicalBasisFamily::PhysicalMutationRequestFingerprint`.

The source is a validated native Store request-basis record, not a terminal
digest string, compatibility text, debug object, or raw JSON payload.

Allocation creates a distinct `PhysicalMutationAttemptBinding` containing:

- the idempotency key and request fingerprint;
- the Store/runtime and physical operation identities;
- any sealed group-member identity; and
- the later exact WAL member identity and range.

The binding, not the request fingerprint, gains allocation facts. Its canonical
representation is encoded in the WAL member so fresh-process inspection can
recover the same retry relationship. An in-memory map may accelerate lookup
during one runtime generation, but it is derived, bounded, and disposable. It
is never the only idempotency authority.

A duplicate request with the same key and fingerprint:

- joins or observes the existing in-flight attempt without a second effect;
- returns the already completed physical fact when lawfully available; or
- returns an exact inspection/reconciliation posture when fate is unresolved.

A duplicate key with a different fingerprint is denied before effects as an
idempotency conflict. It cannot silently select the first payload, replace it,
or create another WAL member.

A request fingerprint that includes its allocated attempt, operation, group,
or WAL identity is invalid by construction: it would make the same retry
unequal to itself. Conversely, a fingerprint that omits effect-relevant scope,
durability policy, payload, operation family, or security basis is incomplete
and cannot enter admission.

After a complete WAL member exists, cancellation or failure cannot produce the
aggregate `ProvenNoEffect` variant. The Store must finish the same in-process
obligation where possible or preserve one `IndeterminatePhysicalMutation` for
C.8. Resubmission must never append a second member merely because data or
acknowledgment was incomplete.

### Checkpoint lifecycle

Checkpoint work is a managed resource:

```rust
let checkpoint = match serving
    .checkpoints()
    .start(PhysicalCheckpointRequest::fuzzy(
        PhysicalCheckpointIdempotencyKey::new(key)?,
        PhysicalCheckpointDeadline::at(deadline),
    ))
    .into_raw()
{
    TransitionOutcome::Success(handle) => handle,
    TransitionOutcome::Denied(denial) => return handle_checkpoint_denial(denial),
    TransitionOutcome::Deferred(deferred) => return handle_checkpoint_deferral(deferred),
    TransitionOutcome::Stale(stale) => return handle_stale_checkpoint(stale),
    TransitionOutcome::RebindRequired(rebind) => return rebind_checkpoint(rebind),
    TransitionOutcome::Failed(failure) => return inspect_checkpoint_failure(failure),
};

observe(checkpoint.progress());
let outcome = checkpoint.wait();
```

The handle exposes:

- exact checkpoint identity and admitted source range;
- capture and retained-WAL progress;
- current bounded resource use;
- cancellation safe only before publication effects begin;
- completed, proven-no-effect, and indeterminate terminal outcomes;
- finalization and disposal; and
- no raw frame, pool, WAL writer, root writer, Signal, scheduler, or media
  authority.

Checkpoint start returns `ProofOutcome<PhysicalCheckpointHandle, ...>` with the
same admitted, denied, deferred, stale, rebind-required, and failed admission
distinctions. The handle's later terminal physical fate remains Store-owned
`Completed`, `ProvenNoEffect`, or `Indeterminate`; admission outcomes and
post-effect fate are not one enum.

The framework can enumerate and terminate every live checkpoint handle during
close. Fire-and-forget checkpoint work is forbidden.

### Observation

```rust
let observation = serving.durability_observation();

assert_eq!(observation.store_identity(), serving.store_identity());
assert_eq!(observation.runtime_identity(), serving.runtime_identity());
```

Observation exposes bounded, read-only facts:

- admitted profile and policy identity;
- active and peak mutation/group/checkpoint/acknowledgment work;
- WAL frames, bytes, ranges, rotations, and barriers;
- page/extent writes and pageLSN bindings;
- checkpoint captures, covered ranges, retained tails, and denials;
- root replacements, file syncs, directory syncs, and retained generations;
- completed, proven-no-effect, indeterminate, cancelled, timed-out, stale, and
  completed-but-unobserved mutation counts;
- exact physical amplification counters; and
- classified shutdown residue.

Observation cannot mint a phase, settle an effect, acknowledge a mutation,
admit a retry, delete WAL, or publish a root.

## Compiler-Visible Durability Progression

The Store progression is semantically equivalent to:

```text
AdmittedPhysicalMutation
  -> WalRangeReservedPhysicalMutation
  -> WalAppendedPhysicalMutation
  -> WalDurablePhysicalMutation
  -> DataDispatchedPhysicalMutation
  -> DataSettledPhysicalMutation
  -> RootPublicationPreparedPhysicalMutation
  -> RootReplacedPhysicalMutation
  -> RootNamespaceDurablePhysicalMutation
  -> CompletedPhysicalMutation
  -> PhysicalMutationAcknowledgment
```

Operation families that do not require every node use a distinct, exhaustively
declared progression. They do not skip phases through `Option`, flags, or
runtime branching on a generic state object.

Each phase:

- consumes the exact preceding type;
- preserves Store, runtime, operation, idempotency, profile, and scope identity;
- adds one stronger proof-bearing fact;
- exposes only the next legal actions;
- carries explicit cancellation and partial-effect posture; and
- cannot be constructed by callers, lower crates, Signal, scheduler,
  certification, or raw backend receipts.

The executor consumes an already lowered effect plan. It may observe facts
unavailable before execution, but it cannot rediscover grouping, WAL
dependencies, durability policy, root policy, retry policy, or acknowledgment
eligibility.

## Group Commit Contract

Group commit has a sealed group authority and separate member authority:

```text
AdmittedPhysicalDurabilityGroup
  owns: group identity, exact sealed members, grouping scope,
        shared WAL barrier requirement, optional shared root plan

WalBarrierMember<M>
  owns: exact mutation identity, idempotency identity, WAL subrange, data scope

RootPublicationMember<M>
  owns: exact mutation identity, exact included data settlement,
        exact shared successor-root membership
```

`SealedPhysicalDurabilityGroupMembers` is a Store-specific wrapper constructed
from Worth Proof `NonEmpty<WalBarrierMember<_>>`. Its consuming constructor
validates the ordered members into `UniqueVec` projections for mutation,
member, and idempotency identities while preserving the original order. The
wrapper therefore carries nonemptiness and uniqueness without the invalid
shape `NonEmpty<UniqueVec<_>>` or duplicated member storage. It additionally
proves homogeneous policy, scope, barrier, byte-limit, and
range-disjointness laws. Runtime-width groups are never represented by a
fixed-arity proof join, tuple, or optional member slots.

One sealed group barrier proof derives one `WalDurablePhysicalMutation` per
matching member by consuming exact membership and range inclusion. When the
sealed group also shares one successor root, its namespace-durable publication
proof derives one root-completed member proof for each and only each included
data settlement. A member cannot be acknowledged until every shared effect in
its declared progression is complete.

It is mechanically impossible to:

- create a member from a group id alone;
- acknowledge a group;
- substitute one member for another;
- remove a failed or cancelled member after its WAL range may exist;
- infer semantic order from group order;
- change member durability request through grouping;
- widen security, tenant, key, artifact, or physical scope; or
- use one member's WAL, data, or root membership to complete another.

Group width, aggregate bytes, queue age, and barrier delay are admitted before
group formation. Saturation rejects or dispatches a smaller lawful group; it
never grows an unbounded queue.

## WAL-Before-Data And PageLSN Contract

WAL range reservation allocates identity only. The WAL owner emits immutable
framed bytes and an append declaration. Each member frame carries:

- Store, runtime-generation, operation, idempotency, and group/member identity;
- exact physical scope and immutable mutation fingerprint;
- one exact `PhysicalMutationAttemptBinding`;
- `CanonicalRedoRecords`, a Store wrapper constructed from Worth Proof
  `NonEmpty<RedoRecord>` and admitted only after its owner-defined order
  validates into `CanonicalVec<RedoRecord>`;
- one exact LSN per redo record;
- the target page/extent identity and payload or canonical redo basis;
- the expected resulting payload digest;
- admitted durability-profile identity; and
- framing, version, and integrity fields required for bounded C.8 inspection.

Store lowers that declaration through the C.5.1 work topology.

The matching data-effect plan is constructed with its page/extent WAL basis but
cannot be dispatched until Store consumes:

- complete WAL append evidence;
- the exact required backend barrier proof;
- matching Store/runtime/group/member identities;
- exact range inclusion; and
- current lifecycle and health authority.

Every candidate page image carries an ordered `PageWalBasis` naming each redo
record reflected in its bytes. Its encoded pageLSN equals the greatest exact
redo-record LSN applied to that image, not an arbitrary member-range start,
end, frontier, or counter. Every lower included redo record must belong to the
same admitted image basis and the resulting payload digest must match.

Physical format encodes the pageLSN supplied by the admitted Store plan. It
cannot choose, advance, or validate WAL durability. A data dispatch is lawful
only when the durable WAL frontier covers the complete `PageWalBasis`. C.8
later compares stored pageLSN and redo identity to recovered WAL; C.7 proves
the written image never outran or misidentified its exact durable basis.

A raw `LogSequenceNumber`, range comparison, queue class, or barrier counter is
not dispatch authority.

## Root And Namespace Publication Contract

One Store-owned current-root progression consumes:

- the exact old current root;
- the successor candidate;
- every member data settlement required by that root;
- the exact WAL basis;
- the admitted backend publication requirement; and
- the current Store/runtime generation.

Publication progresses through candidate file durability, atomic replacement,
and required parent-namespace durability as distinct types.

The current-root authority changes only after the strongest required
publication proof exists. The old root and required supporting artifacts remain
retained. C.7 emits retention facts; C.8 and C.10 later consume them for
recovery and stable-reader policy.

The disconnected `PhysicalRootPublicationStore` and
`root-publications.log` do not survive as a second authority. Any reusable
old/new root validation or epoch meaning is moved behind the canonical Store
progression; the direct writer, file, runtime, and parallel current-root state
are deleted.

## Checkpoint And WAL-Retention Contract

A fuzzy checkpoint captures:

- one exact checkpoint identity;
- one admitted begin boundary;
- one exact covered WAL range;
- the physical root and dirty-generation bases it observed;
- bounded capture memory and I/O;
- exact concurrent-mutation posture;
- one published checkpoint artifact; and
- one contiguous retained WAL tail beginning at the checkpoint boundary.

Checkpoint capture does not freeze all mutation authority. A short,
responsibility-named cutover fence may protect the final publication boundary;
it cannot span full capture or whole-Store traversal.

WAL deletion or recycling requires:

- a namespace-durable checkpoint publication;
- exact checkpoint identity;
- a covering LSN range;
- a contiguous retained tail;
- absence of unresolved physical obligations requiring the candidate segment;
  and
- later retention/recovery constraints already admitted to C.7.

WAL age, file count, disk pressure, checkpoint existence, or a copied tail
range is not eligibility.

`ContiguousRetainedWalTail` is constructed from Worth Proof
`NonEmpty<RetainedWalSegment>` and admitted only after the owner-defined order
validates into `CanonicalVec<RetainedWalSegment>`. The Store wrapper adds
adjacency, range-coverage, checkpoint-boundary, and no-required-gap proof.
`CanonicalVec` supplies stable ordering only; it never proves nonemptiness,
contiguity, or retention eligibility by itself.

## Failure, Cancellation, And Acknowledgment Contract

Before any effect:

- denial, timeout, supersession, or cancellation produces proven no effect;
- all reserved resources and group membership are released or terminally
  accounted; and
- no WAL, data, checkpoint, or publication media operation occurs.

After an effect may have begun:

- cancellation stops caller waiting where lawful but not settlement;
- timeout cannot rewrite effect fate;
- short/torn/partial effects carry exact completed breadth;
- scheduler rejection after effect cannot become no-effect;
- completion delivery loss becomes completed-but-unobserved evidence where
  physical completion is proven;
- ambiguity becomes `IndeterminatePhysicalMutation`; and
- retry remains unavailable until exact no-effect proof or C.8
  reconciliation.

Physical acknowledgment is a consuming projection of
`CompletedPhysicalMutation`, not a mutable flag or independent canonical
artifact. Counters, traces, logs, reports, and Foundational projections derive
from the completed settlement and cannot manufacture it.

At an explicit support, certification, or cross-crate observation boundary,
Store may derive a one-way `StoreExecutedBoundaryReceiptEvidence` projection
from `PhysicalMutationAcknowledgment`. The projection names the physical
operation, request fingerprint, attempt binding, durability-policy basis, and
completed breadth. It cannot be accepted back into mutation admission,
progression, settlement, retry, acknowledgment construction, or current-root
authority. `ProvenNoEffect` and `Indeterminate` may produce separately named
diagnostic projections, never executed-boundary receipts.

## Signal, Proof, And Foundational Law

### Worth Signal

Signal represents derived readiness for:

- WAL append;
- required durability barrier work;
- checkpoint capture slices;
- root publication; and
- completion delivery.

C.7 installs these exact Store-owned physical work families into the existing
frozen binding and lifecycle:

- `WalAppend` for immutable member-frame append;
- `DurabilityBarrier` for the exact admitted WAL, file, or namespace barrier;
- `CheckpointCapture` for bounded capture slices; and
- `RootPublication` for candidate durability, replacement, and namespace
  effects.

Data work reuses C.6 `ExactWriteback` rather than creating a C.7 data queue.
The Store admits it only after consuming the exact
`WalDurablePhysicalMutation`. Completion delivery uses the existing generic
Signal resource-completion lifecycle; it is not a second C.7 effect family.

Store installs admitted physical aspect bindings and immutable observations.
Signal evaluators perform no media, WAL mutation, page mutation, checkpoint
publication, root replacement, settlement, or acknowledgment.

Destroying the Signal graph must not destroy any durability truth. After C.8,
it will be reconstructible from persisted physical authority and live runtime
admission.

### `worth-proof`

Worth Proof owns the reusable proof machinery C.7 consumes:

- `ProofOutcome` and `TransitionOutcome` category preservation;
- sealed progression and consumption mechanics;
- `NonEmpty`, `UniqueVec`, and `CanonicalVec` structural proof collections;
- exact basis and witness composition; and
- stale, rebind-required, assumption, and inspection posture.

C.7 uses that machinery without relocating domain meaning:

- durability-policy admission is a `ProofOutcome`;
- mutation preparation and checkpoint start preserve admitted, denied,
  deferred, stale, rebind-required, and failed categories;
- initialize, open, close, and abort retain the inherited Store
  `ProofOutcome` algebra;
- post-effect `Completed`, `ProvenNoEffect`, and `Indeterminate` remain
  Store-specific physical fate; and
- runtime-width groups and retained collections use structural collection
  proofs rather than fixed-arity joins.

Worth Proof does not define or own backend durability profiles, filesystem
capability claims, WAL bytes, barrier execution, pageLSN meaning, physical
effect receipts, current-root authority, acknowledgment, or retry eligibility.
Those remain with C.4, WAL, physical format, and Store according to their
existing boundaries.

Only `PhysicalDurabilityAdmissionBasis`, derived from
`QualifiedFilesystemMedia` and its exact C.4 qualification claims, admits the
durability policy. Only sealed receipts from the C.4 media owner, joined by
Store to exact identities and progression phases, establish that an effect
occurred.

Generic `AuthorityMarker` bounds are forbidden on every governed C.7 public or
cross-crate surface. Any underlying marker used to implement a sealed concrete
authority remains private and unnameable outside its owner. Copied digests,
profile labels, evidence projections, or public constructors open no C.7 door.

### Worth Foundational and Store aspect-native

Foundational owns stable aspect meaning, contract admission, validated values,
authoritative state, patches, and distinct projection/mutation/diagnostic mask
laws.

C.7 installs these exact derived work-basis contracts:

| Store aspect-native contract key | Basis posture and role | Masks | Signal family | Exact partition |
| --- | --- | --- | --- | --- |
| `store.physical.durability.policy-binding-basis` | projection; `Dependency` | projection only | `WalAppend`, `DurabilityBarrier`, `CheckpointCapture`, and `RootPublication` | admitted durability-policy basis identity |
| `store.physical.durability.wal-append-basis` | mutation; `DependencyAndOutput` | projection and mutation | `WalAppend` only | writable WAL segment plus Store/runtime generation |
| `store.physical.durability.barrier-basis` | mutation; `DependencyAndOutput` | projection and mutation | `DurabilityBarrier` only | exact WAL, file, or parent-namespace barrier scope and artifact identity |
| `store.physical.durability.checkpoint-capture-basis` | mutation; `DependencyAndOutput` | projection and mutation | `CheckpointCapture` only | checkpoint identity and admitted capture range |
| `store.physical.durability.root-publication-basis` | mutation; `DependencyAndOutput` | projection and mutation | `RootPublication` only | candidate-root publication identity and Store/runtime generation |

These are derived routing and invalidation bases. They are not authoritative
“WAL generation,” “durability-profile generation,” “current-root generation,”
or “checkpoint generation” state. In particular,
`root-publication-basis` does not name or advance the current root.

The C.7 bases join the inherited lifecycle-health and C.6
`store.physical.record.frame-writeback-basis` contracts. Only the
responsibility-named Store `work_semantics/` owner constructs them as
`PhysicalWorkSemanticBasis`; lower physical owners remain Foundational- and
Signal-agnostic. Store maps each admitted aspect to bounded Signal routing
slots through the existing frozen binding law. A native patch invalidates
exactly its declared physical work family and partition; raw Signal bits cannot
create or widen that dependency.

Every C.7 scheduler demand for group/barrier, WAL append/tail, checkpoint
capture, or root publication consumes a matching
`FoundationalPolicyAdmissionReceipt` from the existing policy-admission path.
The receipt proves that the named demand was admitted under its exact work
class and finite budget; it does not prove execution or physical durability.

Counter-backed completion can be projected at an explicit observation or
certification boundary as
`StorePerformanceReceiptEvidence<FoundationalAuthoritativePerformanceClaim>`.
That evidence
names the admitted policy receipt, work class, scale, completed breadth, and
exact counters. It is derived evidence, never scheduler capacity, effect
settlement, or mutation authority. The ordinary hot path updates bounded
counters and does not materialize evidence objects or JSON.

`PhysicalMutationAcknowledgment` may project one-way to
`StoreExecutedBoundaryReceiptEvidence` as defined above. No Foundational or
aspect-native receipt is accepted back as C.7 authority.

Foundational does not own:

- WAL bytes or durable prefixes;
- pageLSNs;
- current-root authority;
- effect completion;
- physical mutation fate; or
- acknowledgment.

The general Foundational `FoundationalProfileSet` is not the C.4 backend
durability profile and cannot admit one.

A generic `durable=true`, `published=true`, `committed=true`, or JSON object is
forbidden as internal authority.

## Authority Type Ledger

| Type or responsibility | Constructed by | Proves | Authorizes | Cannot authorize | Consumed by |
| --- | --- | --- | --- | --- | --- |
| `PhysicalDurabilityAdmissionBasis` | C.4 physical-backend owner from `QualifiedFilesystemMedia`, `RootProfileQualificationBasis`, and exact `AdmittedBackendCapabilityWitness` claims | the qualified media generation satisfies the requested filesystem capability vocabulary | Store durability-policy admission only | an effect, barrier completion, current root, or acknowledgment | `PhysicalDurabilityDeclaration::admit` |
| `AdmittedPhysicalDurabilityPolicy` | Store from declaration plus consumed `PhysicalDurabilityAdmissionBasis` through `ProofOutcome` | exact supported C.4 profile, barrier posture, and finite C.7 limits | construction of the matching C.7 runtime owner | effect completion | Store instance construction |
| `PhysicalMutationIdempotencyKey` | caller through validated public constructor | stable physical retry identity | admission lookup and duplicate correlation | operation scope, success, semantic transaction identity | Store mutation admission |
| `PhysicalMutationRequestFingerprint` | Store from the admitted canonical request basis | exact effect-relevant request equivalence independent of any attempt | same-key duplicate comparison | operation/group/WAL identity, execution, or success | idempotency admission and WAL attempt binding |
| `PhysicalMutationAttemptBinding` | Store admission and WAL reservation | exact key/fingerprint relationship to one operation and later group/member/range | continuation and fresh-process reconciliation of that one attempt | equivalence of a different request or completed fate | WAL frame, progression, and C.8 |
| `AdmittedPhysicalMutation` | Store | current identity, scope, policy, generation, and no effect yet | WAL planning | media access or acknowledgment | WAL reservation |
| `SealedPhysicalDurabilityGroupMembers` | Store from `NonEmpty` members plus preserving-order `UniqueVec` identity validation | nonempty exact members with unique mutation, member, and idempotency identities | group-policy admission | effect, group barrier, or member fate | group admission |
| `AdmittedPhysicalDurabilityGroup` | Store from `SealedPhysicalDurabilityGroupMembers` | immutable group membership and allowed shared effects | one group barrier and declared shared root plan | member identity, fate, or acknowledgment | group execution |
| `WalRangeReservedPhysicalMutation` | Store plus WAL owner | exact immutable WAL range and bytes for one member | WAL append declaration | WAL durability or data dispatch | physical work lowering |
| `WalDurablePhysicalMutation` | Store from matching append and barrier receipts | exact member WAL basis is durable under profile | matching data dispatch | another member, root publication, or acknowledgment | executor lowering |
| `PhysicalWritebackSettlement` | existing C.6 Store progression | exact frame effect fate and Signal settlement | C.6 clean/retry/inspection transition | WAL or root durability | C.7 data-settlement join |
| `DataSettledPhysicalMutation` | Store | every required member data effect has exact terminal fate | root publication when completed | semantic visibility | root progression |
| `RootNamespaceDurablePhysicalMutation` | Store from exact root and namespace receipts | required current-root publication barriers completed | final physical completion | branch-head or semantic commit | completion |
| `CompletedPhysicalMutation` | Store | every declared physical edge completed | creation of one physical acknowledgment | retry or semantic publication | caller projection |
| `PhysicalMutationAcknowledgment` | consuming completed outcome | caller-visible exact physical completion fact | correlation by a future adapter | branch progression, writer release, Query acknowledgment | Part II adapter later |
| `ProvenNoEffectPhysicalMutation` | Store settlement | no physical effect occurred for exact attempt | safe same-idempotency retry subject to fresh admission | success | caller or retry admission |
| `IndeterminatePhysicalMutation` | Store settlement | possible effect with exact unresolved basis | preservation and C.8 inspection | acknowledgment or automatic retry | C.8 |
| `PhysicalCheckpointHandle` | Store checkpoint owner | one managed bounded checkpoint lifecycle | progress, safe cancellation, finalization | raw WAL/root/frame mutation | caller and lifecycle drain |
| `ContiguousRetainedWalTail` | Store from checkpoint and WAL inventory | exact retained tail follows exact checkpoint | retention eligibility evaluation | recovery success | C.8 and WAL retention |
| `FoundationalPolicyAdmissionReceipt` | existing Foundational policy-admission path | one named scheduler demand fits its admitted work class and finite budget | matching scheduler demand admission | backend capability, execution, settlement, or durability | C.7 scheduler |
| `StorePerformanceReceiptEvidence<FoundationalAuthoritativePerformanceClaim>` | Store observation/certification projection from policy receipt and exact counters | described completed cost within the named boundary | nothing | capacity, execution, settlement, or acknowledgment | operators and certification |
| `StoreExecutedBoundaryReceiptEvidence` | one-way Store projection from `PhysicalMutationAcknowledgment` | descriptive evidence of the named completed physical boundary | nothing | admission, progression, retry, root authority, or acknowledgment reconstruction | support, certification, and cross-crate observation |
| observations and reports | read-only Store/certification projections | executed facts within their named boundary | nothing | any phase transition | callers, operators, certification |

## Required Destination Directory And Module Plan

Status legend:

- **retain**: existing responsibility remains in place;
- **create**: C.7 establishes the populated boundary;
- **move**: responsibility relocates without compatibility re-export;
- **replace**: successor becomes the only ordinary owner;
- **remove**: path and authority disappear; and
- **committed successor**: named future insertion point, not an empty file to
  create now.

### Store-owned cross-domain orchestration

```text
workspaces/worth-store/crates/worth-store/src/physical_runtime/
├── durability/                                      # create; stable C.7 owner
│   ├── mod.rs                                       # create; facade/export only
│   ├── admission/
│   │   ├── platform_basis_join.rs                    # create; Store/C.4 identity join
│   │   ├── policy.rs                                # create
│   │   ├── mutation_preparation.rs                  # create
│   │   ├── checkpoint_start.rs                      # create
│   │   └── denial.rs                                # create
│   ├── mutation/
│   │   ├── identity.rs                              # create
│   │   ├── request_fingerprint.rs                   # create
│   │   ├── attempt_binding.rs                       # create
│   │   ├── idempotency_registry.rs                  # create; bounded derived index
│   │   ├── outcome.rs                               # create
│   │   └── progression/
│   │       ├── admitted.rs                          # create
│   │       ├── wal_reserved.rs                      # create
│   │       ├── wal_appended.rs                      # create
│   │       ├── wal_durable.rs                       # create
│   │       ├── data_settled.rs                      # create
│   │       ├── root_replaced.rs                     # create
│   │       ├── namespace_durable.rs                 # create
│   │       └── completed.rs                         # create
│   ├── grouping/
│   │   ├── admission.rs                             # create
│   │   ├── unique_membership.rs                     # create
│   │   ├── wal_barrier.rs                           # create
│   │   ├── root_publication.rs                      # create
│   │   └── member_settlement.rs                     # create
│   ├── wal/
│   │   ├── append_declaration.rs                    # create
│   │   ├── member_basis.rs                          # create
│   │   ├── canonical_redo.rs                        # create
│   │   └── barrier_join.rs                          # create
│   ├── data/
│   │   ├── page_wal_basis.rs                        # create
│   │   └── writeback_join.rs                        # create
│   ├── checkpoint/
│   │   ├── handle.rs                                # create
│   │   ├── capture.rs                               # create
│   │   ├── progress.rs                              # create
│   │   ├── publication.rs                           # create
│   │   └── retained_wal_tail.rs                     # create
│   ├── publication/
│   │   ├── root_transition.rs                       # create
│   │   ├── namespace_durability.rs                  # create
│   │   └── retained_root.rs                         # create
│   ├── settlement/
│   │   ├── acknowledgment.rs                        # create
│   │   ├── proven_no_effect.rs                      # create
│   │   ├── indeterminate.rs                         # create
│   │   └── completed_unobserved.rs                  # create
│   ├── lifecycle/
│   │   ├── managed_work.rs                          # create
│   │   └── drain.rs                                 # create
│   ├── evidence_projection/
│   │   ├── executed_boundary.rs                     # create; one-way only
│   │   ├── performance.rs                           # create; counter-backed
│   │   └── diagnostic_fate.rs                       # create; non-authoritative
│   └── observation/
│       ├── counters.rs                              # create
│       └── snapshot.rs                              # create
├── record_serving/
│   ├── publication/                                 # retain and narrow
│   │   ├── candidate_data.rs                        # retain candidate meaning
│   │   ├── plan.rs                                  # retain record plan
│   │   └── ...                                      # move durability orchestration out
│   ├── work_semantics/                              # retain sole semantic-basis owner
│   │   └── durability/                              # create; expected to grow by family
│   │       ├── policy_binding_basis.rs              # create
│   │       ├── wal_append_basis.rs                  # create
│   │       ├── barrier_basis.rs                     # create
│   │       ├── checkpoint_capture_basis.rs          # create
│   │       └── root_publication_basis.rs            # create
│   └── residency/dirty/                             # retain C.6 owner
│       └── outcome.rs                               # retain writeback settlement
└── work/                                            # retain C.5.1 topology
    ├── signal_binding/                              # retain/extend admitted bindings
    ├── scheduling/                                  # retain/extend resource lowering
    ├── execution/                                   # retain sole media dispatch
    └── settlement/                                  # retain exact effect truth
```

The dominant axis of `physical_runtime/durability/` is Store-owned physical
durability authority. It contains cross-owner ordering and settlement only. WAL
grammar, media mechanics, residency mechanics, recovery decisions, integrity,
stable-reader policy, semantic commit, and certification are excluded.

`mutation/`, `grouping/`, `checkpoint/`, `publication/`, `settlement/`, and
`observation/` are distinct because they have different identity,
cardinality, lifecycle, failure, and replacement fate. A single
`durability.rs`, `transaction.rs`, `manager.rs`, `coordinator.rs`, or
`pipeline.rs` file is forbidden.

`admission/platform_basis_join.rs` consumes and verifies the sealed C.4
admission basis against Store policy/runtime identity; it does not restate
capability semantics.
`mutation/request_fingerprint.rs` owns canonical input equivalence, while
`mutation/attempt_binding.rs` owns later operation/group/WAL allocation facts.
They may not be collapsed. The inherited
`record_serving/work_semantics/durability/` boundary is the only C.7 production
directory that constructs Foundational/Store aspect-native
`PhysicalWorkSemanticBasis` values. The directory is justified even if an
early phase temporarily populates only one contract because four more named
families are committed in this milestone. `evidence_projection/` is derived
truth and has no dependency path back into admission, progression, or
settlement.

### Store aspect-native canonical registry

```text
workspaces/worth-store/crates/worth-store-aspect-native/src/
├── canonical_basis.rs                               # retain/extend closed enums
└── canonical_basis/
    ├── canonical_basis_sources.rs                   # retain/extend admission map
    ├── canonical_basis_domains.rs                   # retain/extend exact domain
    └── canonical_basis_construction.rs              # retain/extend native source
```

This boundary adds only the named request-fingerprint family, source kind,
field role, lane, domain, and native construction route. It does not import
`worth-store`, own idempotency, allocate operations, read WAL, or decide retry.
Store supplies validated native fields; the registry supplies canonical-basis
admission and digest equivalence.

### WAL meaning

```text
workspaces/worth-store/crates/worth-store-wal/src/
├── wal_topology/                                    # retain
├── artifact_store/                                  # retain encoding/scan; remove execution
├── append/
│   ├── mod.rs                                       # retain facade
│   ├── frame_plan.rs                                # move/split current planning
│   └── durability_admission.rs                      # narrow to declaration
├── publication_declaration/                         # replace durable_publication/
│   ├── mod.rs
│   ├── wal_scope.rs
│   └── checkpoint_scope.rs
└── queue_declaration/                               # replace generic durability/
    ├── mod.rs
    └── grouping_scope.rs
```

`worth-store-wal` owns immutable WAL meaning and declarations. Direct calls to
`StoreDurabilityRuntime`, filesystem paths as execution authority, final
acknowledgment, root authority, checkpoint execution, and Store progression are
excluded.

The renamed declaration directories prevent a plan or scope from presenting
itself as completed durability.

### Recovery-facing artifacts

```text
workspaces/worth-store/crates/worth-store-recovery-physics/src/
├── wal_recovery_basis/                              # replace ordinary wal_durability/
│   ├── mod.rs
│   ├── append_receipt.rs                            # narrow to observed WAL fact
│   ├── crash_basis.rs                               # move retained crash meaning
│   └── durability_observation.rs                    # retain read-only fact
├── checkpoint_cutover/                              # retain and narrow
│   ├── checkpoint_id.rs
│   ├── checkpoint_lsn.rs
│   ├── checkpoint_manifest.rs
│   ├── checkpoint_validation.rs
│   └── wal_retention.rs
├── source_precedence/                               # committed C.8 consumer
├── redo_replay/                                     # committed C.8 consumer
└── page_redo/                                       # committed C.8 consumer

remove:
  wal_durability/executed_append.rs
  direct ordinary acknowledgment construction
  any StoreDurabilityRuntime composition used by ordinary C.7
```

Recovery-facing types describe persisted facts and future recovery decisions.
They do not execute ordinary C.7 effects or mint ordinary acknowledgment.

### Backend and media

```text
workspaces/worth-store/crates/worth-store-physical-backend/src/
├── durability_profile/                              # retain
│   └── physical_admission_basis.rs                  # create from qualified media
├── artifact_tree/ or admitted media operation home  # retain C.4 effect mechanics
└── durability_ordering/                             # narrow to declarations/receipts
    ├── admission.rs
    ├── requirement.rs
    ├── receipt/
    └── counters.rs

remove:
  durability_ordering/execution/file_runtime.rs
  StoreDurabilityRuntime as a direct filesystem owner
  public execution construction outside the C.4 media owner
```

The backend may report exact barrier mechanics and sealed physical receipts. It
does not know mutation grouping, pageLSN policy, current-root authority,
checkpoint policy, or acknowledgment.

`physical_admission_basis.rs` binds the already admitted
`RootProfileQualificationBasis` and exact
`AdmittedBackendCapabilityWitness` claims to one
`QualifiedFilesystemMedia` generation. It exposes the concrete sealed
`PhysicalDurabilityAdmissionBasis` required by Store policy admission, not raw
claims, a generic marker, or an independently constructible profile.

### Root publication

```text
workspaces/worth-store/crates/worth-store-physical-isolation/src/publication/
├── epochs.rs                                        # move reusable validation if admitted
├── intent.rs                                        # narrow or move pure root-transition meaning
└── receipt.rs                                       # narrow to stable-reader facts for C.10

remove:
  store.rs
  runtime.rs
  root-publications.log writer and parser
  parallel current-root state
```

Reusable pure validation may remain only if it consumes canonical C.5/C.7 root
identity without importing Store internals or retaining a second artifact.
C.10 later adds stable-reader and reclaim authority as siblings under its own
owner; it does not resurrect this writer.

### Certification harness

```text
workspaces/worth-store/tools/store-test-runner/src/
├── courtroom_campaign/
│   ├── schedule_perturbation/                       # move shared C.6 mechanism
│   │   ├── seed.rs
│   │   ├── decision.rs
│   │   └── trace.rs
│   └── durable_publication_siege/                   # create
│       ├── execution.rs
│       ├── world.rs
│       ├── crash_seam.rs
│       ├── protocol/
│       ├── oracle/
│       │   ├── wal_prefix.rs
│       │   ├── page_lsn.rs
│       │   ├── group_identity.rs
│       │   ├── checkpoint_bound.rs
│       │   ├── root_publication.rs
│       │   └── terminal_fate.rs
│       ├── evidence_projection/
│       └── reporting.rs
└── mutation_campaign/
    └── catalog/
        └── physical_reconstruction_c7.rs            # create

workspaces/worth-store/crates/worth-store-physical-certification/src/
└── durability_inspection/                           # create
    ├── wal.rs
    ├── pages.rs
    ├── checkpoint.rs
    ├── roots.rs
    └── manifest.rs
```

The shared schedule directory is justified now because C.6 and C.7 are two
committed courtrooms with the same seed, trace, cleanup, and replay authority.
It contains no C.6 or C.7 semantic decisions; each courtroom owns its closed
decision vocabulary.

The inspector is read-only certification infrastructure. It imports stable
lower artifact declarations, not the `worth-store` runtime or its durability
classifier.

### Structural enforcement

Dependency, visibility, feature, and source gates must prove:

- only `worth-store` composes WAL, Signal, scheduler, C.6 residency settlement,
  and backend durability effects;
- WAL, recovery physics, backend, buffer pool, physical format, and
  certification remain Signal-agnostic;
- certification and replay remain outside ordinary features;
- no lower crate imports `worth-store`;
- no direct filesystem durability executor survives outside C.4 media;
- no semantic or branch crate enters Part I;
- no public facade exports internal progression constructors; and
- no generic `AuthorityMarker`, Foundational profile, or evidence projection
  appears in a C.7 admission or progression signature;
- only `record_serving/work_semantics/durability/` constructs C.7
  Foundational/Store aspect-native work bases;
- `evidence_projection/` has no reverse dependency into admission,
  progression, execution, or settlement;
- request-fingerprint construction has no dependency on operation, group, WAL
  allocation, scheduler, or observation modules; and
- all ordinary durability APIs are reachable from semantic physical roots
  without internal module archaeology.

## Performance And Resource Contract

C.7 names and bounds:

- mutation batch record count and bytes;
- group width, aggregate bytes, maximum queue age, and maximum barrier delay;
- WAL frame bytes, segment bytes, active segments, rotations, and retained tail;
- concurrent admitted WAL, data, checkpoint, publication, and acknowledgment
  work;
- checkpoint capture memory, capture I/O breadth, dirty-page interaction, and
  retained-tail ceiling;
- pending completed-but-unobserved and indeterminate operations;
- current and retained root generations; and
- close/drain work.

Admission rejects before expensive framing, copying, allocation, queueing, or
effects where the disqualifying fact is already available.

Every scheduler demand carries its matching
`FoundationalPolicyAdmissionReceipt`; there is no global receipt that admits
all C.7 work. The exact receipt work class and finite budget must agree with the
group/barrier, WAL append/tail, checkpoint capture, root publication, or
completion-delivery demand. A mismatched, stale, copied, or broader receipt is
denied before queue admission.

The ordinary mutation path may scale with:

```text
declared mutation bytes
+ WAL framing for that mutation
+ exact touched physical frames/extents
+ admitted group-share coordination
+ required durability barriers
+ exact root/checkpoint publication granule
```

It may not scale with total Store size, total historical WAL, consumer count,
diagnostic richness, mutation-corpus size, or offline-verifier work.

Group commit must expose logical-to-physical amplification:

- mutations admitted;
- groups formed;
- members per group;
- WAL frames and bytes;
- barrier executions;
- data writes and bytes;
- root publications;
- acknowledgments; and
- per-member fate.

Checkpoint work has a distinct background/maintenance lane. Deferral does not
erase cost. If retained-WAL or queue pressure reaches its bound, the runtime
backpressures or denies before unbounded growth.

Elapsed-time gates are secondary to structural counters. Performance evidence
must name hardware, filesystem, profile, scale, cold/warm posture, arrival and
burst model, queue utilization, repetitions, and percentiles.

Release and courtroom performance reports use
`StorePerformanceReceiptEvidence<FoundationalAuthoritativePerformanceClaim>`
projected from the matching policy receipt and counter-backed completion facts.
Support or cross-crate completed-operation reports use the one-way
`StoreExecutedBoundaryReceiptEvidence` projection. Neither projection is
created merely to run the ordinary path, and neither is accepted as an input
to it.

## Cleanup And Cutover Contract

Every phase maintains a C.7 removal ledger with:

- present surface;
- exact current callers;
- semantic responsibility worth preserving;
- destination owner;
- disposition of preserve, narrow, move, replace, or delete;
- last consumer;
- deletion phase; and
- mechanical absence proof.

The following decisions are fixed:

| Present surface | C.7 disposition |
| --- | --- |
| `worth-store-wal` frame grammar, LSN topology, append planning, and bounded scan | Preserve and narrow to meaning/declarations; remove ordinary execution |
| `worth-store-wal::DurablePublicationDeclaration` | Rename to publication declaration; it is not completed durability |
| WAL queue declaration and grouping scope | Preserve as scheduler input; group identity and fate remain Store-owned |
| `StoreDurabilityRuntime` and `durability_ordering/execution/file_runtime.rs` | Replace with C.4 media-owner execution; delete direct runtime |
| backend durability requirements and sealed barrier receipts | Preserve and narrow to mechanism evidence |
| `worth-store-recovery-physics::execute_wal_durability*` | Remove from ordinary execution; courtroom control moves to C.4 yieldpoints |
| current `DurableAckReceipt` / `AcknowledgmentPrecondition` | Narrow to WAL-boundary fact or replace; must not remain final C.7 acknowledgment |
| checkpoint id/range/manifest/validation/retention semantics | Preserve and connect through Store |
| recovery source precedence, redo, and quarantine | Preserve for C.8; do not pull into C.7 ordinary execution |
| C.5 `RecordPublicationDirector` candidate planning and physical record publication | Preserve candidate meaning; move cross-owner durability order into C.7 |
| existing `append_batch(...)->PublishedRecordBatch` convenience | Replace or narrow so outcome exposes durability request, idempotency, and indeterminate fate |
| C.6 `PhysicalWritebackSettlement` | Preserve exactly; consume through the C.7 data join |
| any request fingerprint containing allocated operation, group, member, LSN, or WAL-range identity | Delete; replace with canonical request equivalence plus separate persisted attempt binding |
| generic or authoritative-sounding durability/WAL/checkpoint/root “generation” aspect | Delete; replace only with the exact derived C.7 work-basis contracts |
| ad hoc scheduler budget token or C.7-local policy receipt | Delete; consume the existing `FoundationalPolicyAdmissionReceipt` path |
| receipt/report object accepted back into admission or progression | Delete the reverse lane; retain only explicit one-way Foundational evidence projections |
| `PhysicalRootPublicationStore`, runtime, and `root-publications.log` | Delete as parallel root authority; move only pure admitted validation |
| isolated WAL/checkpoint/root certification demonstrations | Rebind to canonical courtroom or delete when redundant |
| direct source paths, feature flags, and test constructors for removed lanes | Delete in the same phase as last consumer |
| compatibility aliases and deprecated re-exports | Forbidden |
| stale roadmap, audit, README, and examples | Revise or remove before closeout |

A replacement is incomplete while its predecessor compiles in an ordinary
feature graph.

At closeout, checks must prove:

- one ordinary WAL effect route;
- one ordinary data effect route;
- one canonical current-root authority;
- one physical acknowledgment type;
- zero direct `StoreDurabilityRuntime` product callsites;
- zero ordinary direct `execute_wal_durability` callsites;
- zero `root-publications.log` writer or parallel root runtime;
- zero public progression constructors;
- zero allocated attempt or WAL facts in request-fingerprint construction;
- zero generic durability/profile aspects or generic authority-marker bounds;
- zero evidence-projection imports in admission, progression, execution, or
  settlement;
- zero semantic transaction/branch/Query vocabulary below the future adapter;
- zero certification feature leakage;
- zero C.7-named production identifiers; and
- zero temporary courtroom hooks outside sealed C.4 yieldpoints.

## Documentation Deliverables

### Physical durability guide

Create `_docs/worth-store/physical-durability-and-checkpoints.md` for Store
callers and operators.

It must explain:

- physical durability versus semantic commit;
- the public mutation request and typed outcome topology;
- request-fingerprint equivalence versus attempt/WAL binding;
- idempotency, cancellation, timeout, completed-but-unobserved, and
  indeterminate behavior;
- backend-profile-relative durability;
- C.4 capability admission versus Worth Proof progression mechanics;
- group commit without identity merging;
- checkpoint lifecycle, progress, bounds, and WAL retention;
- operator handling of inspection-required outcomes;
- exact cost and amplification surfaces; and
- Foundational policy-admission and one-way evidence projections, including
  their explicit non-authoritative status; and
- what C.8 recovery will and will not later add.

All examples compile against the ordinary facade.

### Owner documentation

Revise:

- `worth-store-wal/README.md` to state that it owns WAL meaning and
  declarations, not ordinary execution or final acknowledgment;
- the C.6 bounded access guide to link dirty/writeback settlement to the C.7
  durability guide without claiming dirty implies durable;
- `storage-foundation-aspect-native-gate.md` to name the five exact C.7 derived
  work-basis contracts, roles, masks, families, and partitions, and to state
  that none owns physical truth;
- the physical reality audit rows for WAL execution, backend durability,
  checkpoint, root publication, acknowledgment, and removed paths; and
- the reconstruction roadmap's C.7 current-contract and C.8 handoff links.

Add responsibility-named README material only where a continuing crate audience
needs it. Do not create phase summaries, closeout narratives, or duplicate
architecture explanations.

Remove or correct every document that teaches:

- `DurableAckReceipt` as complete C.7 acknowledgment;
- direct `StoreDurabilityRuntime` product use;
- direct recovery-physics WAL execution as the ordinary lane;
- `root-publications.log` as canonical current-root authority;
- rename without namespace durability;
- checkpoint existence as WAL-deletion eligibility; or
- physical acknowledgment as semantic commit.

Documentation source paths, examples, and public names are checked against the
final implementation in CI.

## Phase Plan

### Phase 1: Freeze Authority, Vocabulary, And Removal Truth

**Becomes true:** every existing WAL, barrier, checkpoint, pageLSN,
root-publication, acknowledgment, and ordinary caller is traced in both
directions and receives one final disposition.

**Consumes:** C.2 audit method, C.4-C.6 contracts, current Cargo graph, current
production call paths.

**Establishes:** the C.7 removal ledger, semantic vocabulary lock, authority
type ledger, final API decision, and destination topology.

**Mechanically forbids:** new C.7 production methods, crates, aliases, or
compatibility paths before their owner and proof boundary are named.

**Evidence:** manual source traces, Cargo dependency inventory, ordinary
feature graph, public API inventory, and focused mechanism probes.

**Next may trust:** no current mechanism is being promoted by name alone and no
duplicate path is accidentally omitted from cutover.

**Cleanup:** delete already-unreachable demonstrations and stale feature edges
whose disposition needs no replacement.

### Phase 2: Admit Physical Durability Policy And Mutation Identity

**Becomes true:** one Store runtime consumes a sealed
`PhysicalDurabilityAdmissionBasis` from its `QualifiedFilesystemMedia`, and
every mutation has exact canonical request equivalence, operation,
idempotency, scope, profile, lifecycle, and bounded-resource identity before
effects.

**Consumes:** C.4 `QualifiedFilesystemMedia`,
`RootProfileQualificationBasis`, exact `AdmittedBackendCapabilityWitness`
claims, C.5.1 physical work identity and `ProofOutcome` topology, C.6 policy
construction pattern, and the Store aspect-native canonical basis registry.

**Establishes:** sealed physical durability admission basis, admitted
durability policy, `PhysicalMutationRequestFingerprint` canonical
family/source/role/lane, mutation request, the attempt-binding contract that
Phase 3 must complete after allocation, internal admission types, exact
`ProofOutcome` denial topology, durability-policy binding work basis,
group/checkpoint limits, and exhaustive runtime lifecycle propagation.

**Mechanically forbids:** optional durability owners, booleans, generic
authority markers, Foundational profiles as backend authority, raw profile
labels, ambient idempotency, allocation facts in request fingerprints, omitted
effect-relevant fingerprint fields, and incomplete construction.

**Evidence:** builder/admission tests, profile-denial tests, construction-total
compiler failures, fingerprint version/golden vectors, retry-equivalence and
conflict mutants, basis-substitution attacks, and zero-effect rejection proofs.

**Next may trust:** every later phase starts from one exact admitted physical
mutation under one real profile.

**Cleanup:** remove obsolete configuration and public constructors that can
express unadmitted or unlimited durability.

### Phase 3: Bind WAL Meaning To The Canonical Work Topology

**Becomes true:** ordinary mutations receive immutable WAL frames and member
ranges from `worth-store-wal`, then append only through Store Signal,
scheduler, executor, and C.4 media.

**Consumes:** admitted mutation, WAL grammar/topology, C.5.1 work progression,
C.4 media.

**Establishes:** reserved and appended phase types, append declarations,
completion of `PhysicalMutationAttemptBinding` with exact member/range
identity, canonical ordered nonempty redo, WAL-append work basis, matching
Foundational policy admission, exact append settlement, and WAL observation.

**Mechanically forbids:** direct WAL filesystem execution, final
acknowledgment, data dispatch, recovery-physics ordinary execution, and any WAL
allocation field feeding back into request equivalence.

**Evidence:** real WAL append journey, bounded scan by independent inspector,
torn-frame seam, persisted attempt-binding inspection, direct-media source
gate, and receipt-substitution attack.

**Next may trust:** WAL bytes and ranges are real, exact, and on the one
production path, but not yet necessarily durable.

**Cleanup:** delete StoreDurabilityRuntime-based WAL execution and migrate or
delete its last ordinary callers.

### Phase 4: Make WAL-Before-Data And PageLSN Unskippable

**Becomes true:** only the matching admitted WAL barrier proof can construct a
data-dispatchable mutation, and every persisted pageLSN is bound to that proof.

**Consumes:** appended WAL mutation, backend profile/barrier receipts, C.6
writeback claim and settlement, physical-format pageLSN encoding.

**Establishes:** WAL-durable, data-dispatched, and data-settled types, exact
page/extent WAL bases, and the exact durability-barrier work basis and policy
receipt join.

**Mechanically forbids:** queue labels, counters, raw LSN comparisons, foreign
receipts, early frame cleaning, and scheduler-derived settlement.

**Evidence:** compile-fail progression cases, WAL-before-data inversion mutant,
pageLSN-ahead mutant, stale/foreign receipt matrix, and real C.6 writeback join.

**Next may trust:** completed data never outran its exact durable WAL basis.

**Cleanup:** remove legacy pageLSN or writeback paths that perform the join by
convention or duplicate Store settlement.

### Phase 5: Share Physical Cost Without Sharing Mutation Identity

**Becomes true:** lawful group commit amortizes WAL barriers and one explicitly
planned group root publication while preserving exact member identity, range,
effect, cancellation, fate, and acknowledgment.

**Consumes:** admitted members, WAL ranges, grouping policy, scheduler
admission, barrier proofs.

**Establishes:** group admission through
`SealedPhysicalDurabilityGroupMembers`, constructed from nonempty members and
validated unique identity projections, shared barrier, optional shared root
publication, and exact member-derivation types with hard queue and delay
bounds.

**Mechanically forbids:** group acknowledgment, identity collapse, cross-member
receipt substitution, fixed-arity runtime groups, semantic ordering, and
unbounded batching.

**Evidence:** three-member identity blender, cancellation/delay/reorder
schedule, barrier/root amplification counters, duplicate/conflicting
idempotency cases, and member-collapse/substitution mutants.

**Next may trust:** shared physical effects reduce cost without changing
individual truth or admitting partial group visibility.

**Cleanup:** delete grouping surfaces that copy member fields into one aggregate
outcome or expose group membership as transaction identity.

### Phase 6: Install Bounded Fuzzy Checkpoint And WAL Retention

**Becomes true:** checkpoint capture proceeds under concurrent mutation with an
exact source range, bounded memory, bounded retained WAL, managed lifecycle,
and proof-gated retention.

**Consumes:** WAL/data progression, C.6 scoped allocations and pressure,
checkpoint artifact semantics, scheduler resources.

**Establishes:** checkpoint `ProofOutcome`, handle, exact checkpoint-capture
work basis and policy receipt, capture progress, publication candidate,
covered range, canonical nonempty `ContiguousRetainedWalTail`, and retention
eligibility.

**Mechanically forbids:** stop-the-world capture, whole-Store materialization,
fire-and-forget checkpoint work, tail-budget escape, and checkpoint-existence
deletion.

**Evidence:** 32-times-resident checkpoint pressure siege, exact allocation and
I/O counters, continued foreground progress, tail-bound denial, and premature
retention mutant.

**Next may trust:** a published checkpoint can later bound recovery without
having claimed recovery.

**Cleanup:** remove sharp/demo checkpoint execution, duplicate retention
decisions, and unbounded capture helpers not used by the canonical path.

### Phase 7: Join Root Replacement To Namespace Durability

**Becomes true:** one canonical current-root transition consumes exact
WAL/data/checkpoint bases and distinguishes candidate durability, replacement,
namespace durability, and retained old-root truth.

**Consumes:** completed data/checkpoint facts, C.5 root candidate, C.4 atomic
replacement and directory sync, admitted profile.

**Establishes:** root-prepared, root-replaced, namespace-durable, and retained-
root types plus the exact root-publication work basis, partition, and matching
policy receipt.

**Mechanically forbids:** parallel root authority, rename-as-durable,
current-root advance before exact barriers, and early old-root deletion.

**Evidence:** root crash seams, exact artifact manifests, missing-directory-
sync mutant, foreign-root/generation tests, and offline root inspection.

**Next may trust:** current-root authority changes once and only from exact
profile-complete physical effects.

**Cleanup:** delete `PhysicalRootPublicationStore`, `root-publications.log`,
parallel root runtime, and any duplicate current-root cache with authority.

### Phase 8: Publish Typed Outcomes And Cut Over The Ordinary Facade

**Becomes true:** the ordinary facade exposes explicit durability request,
idempotency, deadline/cancellation, completed, proven-no-effect, and
indeterminate outcomes; only completion yields physical acknowledgment.

**Consumes:** full WAL/data/root progression and C.5.1 cancellation/settlement.

**Establishes:** final public mutation and checkpoint APIs, inherited
`ProofOutcome` admission topology, Store-owned physical fate, exact
observation, completed-but-unobserved evidence, one-way
`StoreExecutedBoundaryReceiptEvidence` and diagnostic projections, and
lifecycle drain.

**Mechanically forbids:** generic success/error acknowledgment, hidden weak
durability, automatic indeterminate retry, semantic commit interpretation, and
accepting any Foundational evidence projection back as authority.

**Evidence:** compiled caller examples, cancellation at every effect boundary,
acknowledgment-loss seam, stale-generation delivery, public UI compiler tests,
reverse-evidence-projection compile failures, and close with work in every
lifecycle phase.

**Next may trust:** every ordinary caller uses one honest physical outcome
surface.

**Cleanup:** remove or narrow `append_batch(...)->PublishedRecordBatch`, old
acknowledgment types, compatibility wrappers, and alternate ordinary callers.

### Phase 9: Complete Cutover, Delete Islands, And Publish Documentation

**Becomes true:** every retained mechanism participates through the canonical
Store path, every displaced executor or authority is gone, and callers/operators
have one current contract.

**Consumes:** removal ledger, final API, actual Cargo graph, current docs and
audit.

**Establishes:** final dependency direction, feature graph, owner READMEs,
physical durability/checkpoint guide, exact aspect-native gate documentation,
audit truth, and roadmap handoffs.

**Mechanically forbids:** legacy features, deprecated aliases, direct executor
callers, certification leakage, stale examples, and undocumented operational
outcomes.

**Evidence:** source/dependency/feature/deletion gates, compiled docs, link
checks, bidirectional audit traces, and zero unresolved removal rows.

**Next may trust:** C.7 has one production authority and one discoverable
caller/operator contract.

**Cleanup:** remove obsolete tests, fixtures, snapshots, closeout artifacts,
temporary adapters, and dependencies exposed by deletion.

### Phase 10: Hostile Courtroom, Mutation Closure, And C.8 Handoff

**Becomes true:** the joined system survives every named crash seam, 16 seeded
CI schedules, the canonical release schedule, the checkpoint pressure siege,
the complete current mutation corpus, and named hardware qualification.

**Consumes:** final source, complete progression, independent inspector,
schedule harness, mutation catalog, removal ledger, and documentation.

**Establishes:** requirement/evidence closure ledger and the one C.8 physical
durability handoff, plus counter-backed
`StorePerformanceReceiptEvidence<FoundationalAuthoritativePerformanceClaim>`
for each governed performance claim.

**Mechanically forbids:** stale evidence, same-process inspection, wrong-reason
green, missing mutant, source restoration residue, temporary courtroom hooks,
and successor access to live runtime state.

**Evidence:** owner, smoke, CI, release, and hardware products; exact replay;
complete mutation report; independent artifact report; final source identity;
constitution and line-cap gates; and reverse ledger attack.

**Next may trust:** C.8 receives real persisted durability facts rather than
mechanism vocabulary.

**Cleanup:** delete temporary courtroom-only production hooks. Retain only
sealed C.4 yieldpoints/interposers and shared schedule infrastructure with a
continuing responsibility.

## C.8 Successor Handoff

C.7 exposes one sealed, branch-agnostic recovery input containing:

- stable Store identity and final C.7 source/profile identity;
- current and immediately previous physical root bases;
- namespace-durable root-publication evidence;
- latest namespace-durable checkpoint identity and exact covered LSN range;
- contiguous retained WAL tail inventory;
- stable physical operation and idempotency identities, canonical request
  fingerprints, and persisted attempt/WAL bindings;
- completed, proven-no-effect, completed-but-unobserved, and indeterminate
  physical mutation facts;
- exact backend durability profile and barrier evidence;
- bounded recovery allocation admission from C.6; and
- classified staged or partial artifact residue.

C.8 may consume those facts to decide physical source precedence, redo, root
selection, and operation-fate reconciliation in a fresh process.

C.8 does not receive:

- live `ServingPhysicalRuntime`;
- buffer-pool contents, frames, leases, or clean authority;
- Signal graph, scheduler state, queues, callbacks, or counters;
- a writer-owned decoded WAL or page model;
- C.7 acknowledgment construction;
- Foundational policy, performance, or executed-boundary evidence as
  replacement authority for persisted physical facts;
- semantic mutation, branch, MVCC, Query, or writer authority; or
- a claim that indeterminate work has already been resolved.

The handoff is constructible only from the final C.7 closeout progression. An
evidence bundle, report, digest, copied field set, or public constructor cannot
mint it.

## Milestone Must Ship

C.7 is incomplete without:

- one admitted physical durability policy under a sealed C.4-derived
  `PhysicalDurabilityAdmissionBasis`;
- one versioned canonical `PhysicalMutationRequestFingerprint` separated from
  its persisted `PhysicalMutationAttemptBinding`;
- one identity-preserving Store durability progression;
- real WAL append through C.5.1 and C.4;
- compiler-visible WAL-before-data and pageLSN binding;
- group commit with distinct member truth;
- bounded fuzzy checkpoint capture and contiguous retained WAL tail;
- exact root replacement and namespace durability;
- completed, proven-no-effect, and indeterminate public outcomes;
- physical acknowledgment available only from exact completion;
- bounded queue, memory, WAL-tail, and amplification contracts;
- current read-only observation and operator checkpoint lifecycle;
- exact Store aspect-native work bases for policy binding, WAL append,
  barriers, checkpoint capture, and root publication;
- matching `FoundationalPolicyAdmissionReceipt` scheduler admission and
  one-way Store performance/executed-boundary evidence projections;
- removal of direct WAL, durability-runtime, checkpoint, and root execution
  islands;
- updated public/operator docs, owner READMEs, audit, and roadmap handoff;
- compile-time authority and progression denial;
- actual process death at every named seam;
- independent offline artifact observation;
- 16 revision-derived replayable CI schedules;
- canonical release execution at all crash seams;
- append-only mutation regression closure; and
- the sealed C.8 handoff.

## Must Preserve

- C.4 remains the one media authority and fault-observation boundary.
- C.5 physical format and stable Store/record identities remain canonical.
- C.5.1 remains the one work, Signal, scheduler, executor, settlement, and
  shutdown topology.
- C.6 retains pool, lease, dirty, clean, pressure, and writeback authority.
- WAL remains specialized artifact meaning, not Store or semantic authority.
- Backend profiles remain concrete physical capability facts.
- Signal remains derived and disposable.
- Certification remains read-only or fault-injecting courtroom authority.
- Part I remains branch-, MVCC-, Query-, and semantic-writer-agnostic.
- Stores larger than memory, bounded resources, real crashes, independent
  observation, exact replay, and mutation sensitivity remain non-negotiable.

## Explicit Non-Goals

C.7 does not:

- implement C.8 recovery source selection, redo, or fresh-process fate
  reconciliation;
- implement C.9 integrity admission or corruption repair;
- implement C.10 stable-reader leases, reclaim, fairness, or maintenance QoS;
- implement C.11 index/blob durability policy beyond the generic physical
  progression they will later consume;
- formalize all transitions for C.12;
- advance semantic branch heads or release semantic writers;
- define Query acknowledgment or semantic transaction commit;
- infer semantic liveness from root or WAL retention;
- expose raw WAL, page, checkpoint, root, backend, Signal, scheduler, or pool
  mutation authority; or
- retain compatibility with displaced unreleased APIs.

## Acceptance Evidence

Closeout evidence includes:

- source, binary, Store, runtime, format, profile, and harness identities;
- admitted durability and checkpoint policy;
- workload seed, schedule seed, crash seam, scale tier, and exact rerun command;
- complete operation/idempotency/group/member identity trace;
- request-fingerprint canonical version, input-basis identity, golden-vector
  digest, and persisted attempt/WAL binding;
- C.4 durability admission basis and exact backend capability-claim identities;
- C.7 Foundational work-basis contract revisions, roles, masks, families, and
  partitions;
- scheduler `FoundationalPolicyAdmissionReceipt` identities and derived
  `StorePerformanceReceiptEvidence<FoundationalAuthoritativePerformanceClaim>`
  for governed cost claims;
- one-way `StoreExecutedBoundaryReceiptEvidence` only where a support,
  certification, or cross-crate boundary consumes it;
- WAL frames, ranges, barriers, rotations, and durable-prefix evidence;
- page/extent effects and pageLSN bindings;
- C.6 dirty/writeback settlements;
- checkpoint capture, publication, covered range, and retained-tail evidence;
- root candidate, replacement, namespace durability, and retained-root facts;
- completed, proven-no-effect, indeterminate, cancellation, timeout, stale,
  and completed-but-unobserved outcomes;
- exact file, directory, sync, byte, frame, queue, memory, and amplification
  counters;
- independent inspector report;
- fresh reopener report;
- deletion/dependency/feature/source gates;
- documentation compilation and link evidence;
- all 16 CI schedule reports;
- canonical all-seam release report;
- complete catalog-derived mutation report; and
- sealed C.8 handoff construction proof.

No row closes from a previous revision, a different backend profile, a
different harness identity, or a report whose source restoration is not
byte-identical.

## Closeout Gate

C.7 closes only when one real production progression owns every physical
durability edge; exact mutation identity survives WAL grouping, barriers, data
settlement, checkpointing, root publication, cancellation, crash, and
acknowledgment; no page outruns durable WAL; no root is promoted without the
exact barriers required by its admitted profile; no ambiguous effect is called
success or no-effect; checkpoint and retained-WAL work remain bounded under
continued foreground mutation; every displaced execution or authority island
has been deleted; the hostile fresh-process courtroom detects every current
controlled defect under exact replay; documentation teaches the real API and
operator lifecycle; and C.8 receives persisted physical truth without live
runtime, pool, Signal, scheduler, or semantic authority.
