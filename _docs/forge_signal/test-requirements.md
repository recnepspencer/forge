1. The hostile replay equivalence test
Purpose

Prove that the runtime is truly:

deterministic

transactional

replayable

provenance-stable

lineage-correct

planning-independent in semantics

serial/parallel equivalent

This is the single hardest “same world, same truth, same answer” test.

Scenario

Build a medium-large graph with all of the following present in one topology:

aspect-scoped dependencies

dynamic dependency discovery

conditional gates

comparator suppression

partition-aware outputs

structural memoization

keyed query families

multiple branches of dependent subgraphs

at least 3 execution stages in the planner

same-stage parallel precompute opportunities

Then run this exact script:

Start from a canonical baseline snapshot S0

Apply a deterministic patch sequence P1..P20

After selected patches, do:

explicit evaluation of target set A

lazy pull of target set B

snapshot capture

rollback-triggering fault injection during one transaction

retry after rollback

At patch 10, fork branch B1

On B1, apply P11b..P15b

On main branch, apply P11a..P15a

Restore an earlier snapshot on each branch

Resume forward execution from both restores

Record lineage across:

replacement

refresh

memoized reuse

invalidation without replacement

snapshot restore

branch switch / fork / merge markers

Execute the full scenario in four modes:

serial planner + serial execution

staged planner + parallel precompute

fresh process replay from captured snapshots/logs

event-by-event deterministic replay from recorded execution history

What this probes

It simultaneously probes:

deterministic evaluation order

deterministic artifact evolution

rollback hard-rewind correctness

snapshot capture fidelity

restore correctness

branch semantics

planner/executor parity

memoization correctness under replay

explain/provenance stability

lineage event stability

separation of truth from compute, because replay must work from host snapshots rather than hidden signal-owned truth

Required verification output

The output should be a verification package with these exact categories.

A. Final state equivalence report

For every tested mode, emit:

final node states

final artifact identities

final output values

final output change classifications

final cleanliness state (Clean / MaybeStale / Dirty)

final dependency graph snapshot

final per-aspect versions

Pass condition

All four modes must produce bit-for-bit identical canonicalized final-state artifacts, except for fields explicitly declared non-semantic, like wall-clock timestamps.

B. Snapshot equivalence matrix

For every captured snapshot:

snapshot ID

canonical node-state digest

canonical artifact-state digest

canonical dependency-state digest

lineage digest through that point

Pass condition

Restoring snapshot Sn and re-running the exact suffix workload must produce the same canonical digest as the original historical execution suffix.

That proves snapshot correctness plus replay correctness.

C. Rollback integrity report

For each injected failure:

transaction ID

fault point

pre-transaction digest

failure-time partial internal digest

post-rollback digest

retry digest after successful retry

Pass condition

post-rollback digest must equal pre-transaction digest exactly

retry digest must equal the digest from the equivalent no-failure control run

That proves hard-rewind, not soft best-effort cleanup.

D. Provenance equivalence report

For a selected set of nodes, emit canonicalized explain(node) outputs including:

invalidation provenance

dependency provenance

condition provenance

comparator provenance

recomputation provenance

host causality metadata if present

Pass condition

Across all execution modes and replay modes, the explanation must be semantically identical after canonicalization.

Not merely same final value — same causal explanation.

E. Lineage equivalence report

Emit the full lineage stream with canonical ordering, including:

ArtifactTransition

Invalidation

SnapshotRestore

BranchFork

BranchSwitch

BranchMerge

Pass condition

For equivalent historical executions, lineage records must be semantically identical and sequence-consistent. If replay produces different artifact ancestry or different restore/memoized-reuse history, the runtime has failed the Phase 5 promise.

Why this is ultimate

Because it proves the deepest claim in the vision:

the runtime is not just usually correct; it is historically reproducible, causally inspectable, and planner-independent in semantics.

If this fails, the “deterministic transactional auditable derived runtime” claim is not yet real.

S9.16.5 alignment note

Any certification bundle that exercises replay, lineage, explanation, provenance,
or cold artifact access must also prove the diagnostics-tier contract:

- DiagnosticsTier changes retained richness, not canonical runtime/replay/lineage truth
- RetentionBudget bounds retained history/detail/replay envelopes
- ReconstructionBudget gates explicit cold materialization only
- DiagnosticsAvailability distinguishes retained, reconstructed, omitted, denied, and unavailable outcomes explicitly
- ordinary summary/history/replay reads must perform zero cold reconstruction
- retained/reconstructed/denied cold-work counters must be attributable by access lane and API family
- retained-envelope shaping must follow the active runtime policy budget rather than tier defaults alone
- long-session branch/snapshot churn must remain bounded by retained history/detail/replay envelopes

2. The adversarial granularity suppression test
Purpose

Prove that aspect-aware invalidation, comparator suppression, partition-scoped propagation, structural memoization, and conditional gates all work together without semantic leaks.

This is the test that proves Forge Signal is not just a reactive engine, but a precision recomputation engine.

Scenario

Construct a graph with deliberate granularity traps:

upstream nodes exposing many independent aspects

downstream nodes subscribing to:

single aspects

aspect groups

partition subsets

dynamic subsets discovered at evaluation time

comparator policies including:

exact equality

epsilon/tolerance

structural identity

partition-local diff suppression

conditions including:

debounce

on-demand

delta threshold

custom host gate

query families keyed by host-supplied structural keys

memoization caches shared across families where legal

Then generate a long randomized-but-seeded workload of host changes with these properties:

some changes alter irrelevant aspects

some alter relevant aspects but produce comparator-suppressed outputs

some alter one partition while leaving others identical

some flip conditions from blocked to allowed

some alter dynamic dependency shape

some cause same-value recomputes that should not propagate

some force true downstream recompute

some invalidate huge transitive regions where only a tiny frontier should actually run

Run thousands of mutations.

Also run a reference oracle beside it:

either a brute-force full recompute engine

or a “recompute everything every time” correctness harness

What this probes

It simultaneously probes:

aspect invalidation correctness

absence of over-invalidation

absence of under-invalidation

comparator correctness

result diff correctness

partition-aware suppression

dynamic dependency capture correctness

condition gating correctness

memoization validity boundaries

query family key correctness

runtime trust metrics honesty

Required verification output
A. Precision matrix

For every mutation step, emit:

changed host aspects

nodes marked dirty / maybe-stale

nodes actually evaluated

nodes skipped due to condition

nodes suppressed due to comparator / unchanged output

nodes reused via memoization

partitions reported changed

downstream nodes propagated to

Pass condition

This matrix must exactly match the oracle’s semantic necessity set:

every node that must recompute eventually does

no node that need not recompute is actually executed unless explicitly allowed by policy

no partition outside the true changed set is reported changed

This is the most important precision proof.

B. Waste ratio report

For the full workload, emit:

total dirty marks

total evaluations

total suppressed propagations

total memoized reuses

total partition-local suppressions

total condition deferrals

total no-op invalidations

full-recompute reference evaluation count

Pass condition

Not just “smaller than brute force.”
The runtime must show a stable and explainable suppression profile under adversarial conditions, and every suppression class must be attributable by provenance.

In other words: optimization must stay explainable, not become mysterious magic.

C. Correctness parity report

At each step, compare runtime outputs to full oracle outputs:

final values

per-partition values

per-query-family values

exposed downstream artifacts

Pass condition

Exact parity on all semantically visible outputs for all steps.

If suppression ever hides a necessary recompute, this catches it.

D. Granularity attribution report

For selected nodes, emit structured explanations showing:

why they were dirtied

which aspect triggered dirtiness

which dependency path mattered

whether comparator suppressed downstream effect

whether partition diff narrowed propagation

whether memoized reuse occurred

whether conditions deferred execution

Pass condition

Explanation must align with the precision matrix and with actual runtime behavior. The runtime cannot merely do the right thing; it must explain the right reason.

Why this is ultimate

Because most incremental runtimes can be correct by being noisy.
The hard problem is being correct and surgically precise at the same time.

This test proves the claim:

Forge Signal answers not only what must recompute, but what must not.

That is one of the core promises of the whole system.

3. The dual-history branch and restoration torture test
Purpose

Prove that Phase 5 is real: snapshots, lineage, replay, branchable evaluation, restore semantics, memoized reuse history, and future bridge-grade causality all behave coherently through time.

This is the “time and history are first-class” test.

Scenario

Build a scenario with:

a stable baseline graph

several expensive derived artifacts

keyed families

partial outputs / partitions

branchable evaluation state

snapshot capture before and after meaningful transitions

Then execute a deliberately twisted history:

Baseline evaluation on branch main

Capture snapshot S1

Apply upstream change A

Evaluate a subset of targets only

Capture snapshot S2

Apply upstream change B

Force a transaction failure during recompute of one expensive node

Roll back

Re-run successfully

Fork branch explore

On explore, apply C, D, E

Restore S1 on explore

Re-evaluate under a different target request pattern

Memoized-reuse some artifacts

On main, continue with F, G

Restore S2 on main

Continue forward again

Merge selected branch results logically at the host layer or through future branch semantics

Replay both histories from snapshots and from recorded events

Compare all artifact ancestry and explanations

The key is that the same artifact families must experience:

replacement

refresh with meaningful diff

refresh with no downstream change

invalidation without immediate recompute

restore

memoized reuse after restore

branch divergence

branch convergence markers

What this probes

It simultaneously probes:

snapshot fidelity

restore semantics

branch-local state isolation

lineage continuity

artifact identity evolution

replayability of historical paths

divergence and reconvergence semantics

current-state explanation vs historical explanation

readiness for bridge-carried causality

distinction between signal lineage and truth lineage

Required verification output
A. Branch-state divergence map

For each branch and checkpoint, emit:

active snapshot base

node-state digest

artifact-state digest

dirty/maybe-stale/clean map

dependency graph digest

planned targets

realized evaluations

Pass condition

Branches must remain isolated after fork. Restoring or recomputing on one branch must not mutate the other. Equivalent historical suffixes on equivalent bases must converge to identical digests.

B. Artifact ancestry graph

For selected artifacts across time, emit a graph of:

artifact ID

node

parent artifact ID

transition kind

execution record ID

semantic segment ID

branch ID

snapshot restore associations

invalidation events

memoized reuse references

Pass condition

The ancestry graph must be acyclic where derivational parentage is claimed, branch-aware, and historically coherent.

Specific required checks:

restore must not masquerade as derivational parentage

memoized reuse must not masquerade as fresh recomputation

invalidation must not create fake artifact replacement

branch-local artifacts must remain branch-local unless merge semantics explicitly say otherwise

C. Historical explainability report

For selected checkpoints, emit two views:

current-state explanation
Why is this node in its current state right now?

historical evolution explanation
How did this artifact or node state evolve from S1 through the current checkpoint?

Pass condition

The current-state explanation and historical explanation must agree, but not collapse into each other.

That proves the runtime can answer both:

“why is it this way now?”

“how did it get here?”

Without Phase 5, this usually breaks.

D. Replay branch parity report

Replay each branch history by:

snapshot restore + suffix events

raw event log replay from origin

fresh recomputation from equivalent truth snapshots

Pass condition

For each branch checkpoint:

final node-state digest identical

final artifact-state digest identical

lineage digest identical

explanation digest identical

That proves branchable replay is not just visually plausible but semantically closed.

E. Causality threading report

Attach mock host causality metadata at change boundaries, such as:

truth transaction ID

patch ID

changed entity/aspect references

Then verify it appears consistently through:

invalidation provenance

explain outputs

lineage records

replay artifacts

Pass condition

The same host causality token that originated the change must remain traceable through all affected signal-side historical surfaces.

This is not full Phase 6 bridge completion, but it proves Phase 5 is bridge-ready instead of being a dead-end local history model.

Why this is ultimate

Because this is the test that proves Forge Signal is not only an incremental runtime, but a time-aware execution substrate with honest historical semantics.

If this fails, snapshots and lineage are just decorative features.
If it passes, you have something much rarer.

What these three tests collectively prove

Together they prove the whole promise stack:

Test 1 proves

deterministic semantics

transactional semantics

replay semantics

planner/executor parity

provenance stability

Test 2 proves

precision invalidation

suppression correctness

granular recomputation boundaries

memoization validity

explainable efficiency

Test 3 proves

snapshot/restore truthfulness

branch-local historical correctness

lineage honesty

replay across time

bridge-ready causality threading

The most important meta-rule

For all three tests, the verification output must be based on canonicalized machine-checkable artifacts, not “developer looked at logs and it seemed right.”

That means every test should emit canonical digests for:

node states

artifacts

dependency shape

explanations

lineage streams

branch histories

replay outcomes

If you do not make the outputs canonical and comparable, you will accidentally turn a truth-runtime claim into a vibes-runtime claim.

1. The topology churn test

This targets graphs whose dependency shape changes constantly during evaluation.

Why it matters

Geometry kernels and game systems both have workloads where the set of dependencies is not stable:

a boolean operation touches different faces/edges after topology changes

a solver watches different constraints after a branch of logic flips

an AI or gameplay system changes which entities it observes based on world state

a query family suddenly fans out across a different subset

A runtime can look fine on stable DAGs and still fail once dependency capture changes every frame or every operation.

What to stress

dynamic dependency discovery that changes shape every evaluation

repeated add/remove of dependencies on hot nodes

oscillation between dependency sets A and B

conditional nodes whose conditions themselves depend on dynamic reads

partition subscriptions that grow and shrink aggressively

repeated invalidation during topology shape churn

What to verify

no stale edges remain after dependency reshaping

no phantom dependencies accumulate

no under-invalidation after dropping and re-adding dependencies

dependency inspection always reflects the current graph truthfully

replay reproduces the same dependency shape history

planner cost does not explode under churn

This flushes out “dependency leak” bugs fast.

2. The fanout shockwave test

This targets huge invalidation blast radii.

Why it matters

In geometry and game workloads, some upstream nodes are effectively “structural hubs”:

transform hierarchies

world-space caches

canonical mesh state

material/visibility sets

simulation clocks

global constraint summaries

A single small change can cause giant maybe-stale cones. The runtime has to stay correct and bounded.

What to stress

one hot node with enormous downstream fanout

mixed aspect subscriptions so only part of fanout should truly propagate

chained high-fanout layers

selective target evaluation after broad invalidation

repeated updates to the same hub with small semantic deltas

What to verify

invalidation breadth matches actual semantic reach

downstream suppression prevents recompute floods when outputs are unchanged

telemetry and explanation remain bounded and usable

planner/runtime overhead grows acceptably with fanout

no queue duplication, repeated re-dirtying, or pathological re-enqueue behavior

This finds “works in principle, collapses under breadth” problems.

3. The deep chain numerical stability test

This targets long dependency chains where small differences cascade.

Why it matters

Geometry kernels and simulation/game systems often have:

long transform chains

derived bounds from derived bounds

repeated tolerance-based suppression

incremental updates where tiny numeric differences matter sometimes and not others

A runtime can become semantically inconsistent when comparator policies interact with deep chains.

What to stress

long linear chains and layered diamonds

exact comparators mixed with tolerance comparators

repeated tiny upstream deltas near epsilon thresholds

alternating significant/insignificant updates

restore/replay after threshold-boundary behavior

What to verify

same seeded run always crosses thresholds at the same points

comparator suppression never hides semantically required downstream work

replay reproduces the same suppression history

explanations show why a threshold suppressed or allowed propagation

branch/replay does not diverge due to threshold instability

This is especially important before geometry kernels, because tolerance semantics are where trust dies quietly.

4. The hot/cold locality skew test

This targets real workload skew instead of balanced synthetic graphs.

Why it matters

Real systems are not uniform.

You usually get:

a tiny set of super-hot nodes

a huge cold tail

occasional cold-to-hot promotions

repeated reads of the same keyed families

sparse mutations against a giant graph

A runtime can benchmark well on smooth graphs and still perform terribly under skew.

What to stress

a giant graph with 1–5% very hot nodes

repeated updates concentrated on hot regions

occasional structural changes in cold regions

heavy snapshotting and diagnostic collection while hot set churns

repeated same-target pulls

What to verify

planner cost is not dominated by cold graph scan

snapshot and diagnostics do not scale with untouched cold regions

memoization and query caches remain effective under skew

cold regions do not experience accidental dirtiness

locality-sensitive storage assumptions hold

This is one of the best pre-production realism tests.

5. The orphaned-history test

This targets lineage and snapshot correctness under deletion, replacement, and branch discard.

Why it matters

Geometry and game systems both produce lots of “this thing used to matter, now it does not” history:

geometry entities disappear after a boolean

a procedural generation branch is abandoned

cached artifacts get invalidated and replaced repeatedly

branch-local experiments are discarded

History systems often get confused when the current world no longer contains the old objects.

What to stress

artifact replacement chains

invalidation without replacement

snapshot restore of artifacts no longer current

branch discard after many lineage events

repeated remove/recreate of logically similar artifacts

branch merge where one side references dead history

What to verify

lineage remains queryable for non-current artifacts

deletion/discard does not corrupt ancestry

restore does not fabricate derivation where none exists

history for dead branches remains coherent if retained

GC/compaction policies do not break historical explanation

This is very important for “truthful past, changing present” systems.

6. The determinism under hostile scheduling test

This is a stronger version of serial/parallel parity.

Why it matters

Game engines especially will pressure you toward aggressive scheduling. Geometry kernels may too once expensive derived work gets parallelized.

It is one thing to match serial and parallel once. It is another to remain deterministic under ugly executor behavior.

What to stress

randomized worker wake order

randomized task steal order

artificial stalls

artificial preemption points

repeated runs with same logical inputs but different execution timing

stage-local parallel work with different chunking strategies

What to verify

final state digest identical across all runs

lineage identical

explain output identical after canonicalization

no hidden race-dependent dependency capture

no ordering-sensitive comparator or memoization behavior

If this fails, the runtime is not really deterministic — it is just lucky.

7. The corruption resistance test

This targets bad host integrations and half-broken runtime state.

Why it matters

Before geometry or game use, you want to know how the runtime behaves when the host lies, forgets, or violates a contract.

Because real integrations will.

What to stress

Inject deliberate bad behavior:

invalid host snapshot version metadata

wrong aspect mapping

reused structural key for semantically different artifact

missing causality metadata

duplicate node registration attempts

stale snapshot restore request

branch switch to unknown lineage state

illegal cycle introduction attempt during dynamic rewire

What to verify

runtime rejects impossible states loudly

diagnostics identify contract violation class clearly

rollback restores consistency after failed operations

bad host metadata cannot silently poison causal story

corruption is contained rather than spreading

This is less about elegance and more about survivability.

8. The memory pressure and retention test

This targets long-lived systems.

Why it matters

Game editors, CAD tools, simulations, and runtime authoring tools may stay alive for hours. History, diagnostics, memoization, snapshots, and lineage can quietly become a memory disaster.

What to stress

long run with thousands of snapshots

bounded diagnostics retention

lineage retention with compaction policy

repeated branch creation/discard

query-family cache churn

repeated hot/cold region transitions

What to verify

memory growth follows policy instead of drifting upward

retained history stays queryable within declared bounds

compaction does not corrupt replay or explanation for retained windows

dropped history fails explicitly, not ambiguously

performance does not degrade sharply after long uptime

A lot of runtimes fail here before they fail anywhere else.

9. The oscillation test

This targets systems that flip back and forth between nearby states.

Why it matters

Games and simulations do this constantly. Geometry tools do it whenever a user drags a handle or scrubs a parameter.

What to stress

repeated A ↔ B ↔ A ↔ B updates

threshold-boundary oscillation

dependency-shape oscillation

branch restore to an old state followed by return to newer state

memoized reuse opportunities across oscillation

What to verify

no cumulative dependency garbage

no lineage corruption from repeated revisit of similar states

memoized reuse is correct, not stale reuse

output suppression stays stable and does not “learn the wrong thing”

replay reproduces oscillation exactly

This flushes out state contamination bugs.

10. The hostile domain adapter test

This is the most practical one before geometry kernels or game engines.

Why it matters

The runtime itself may be solid, but the integration seam is where domain bugs enter.

So build fake-but-brutal adapters that mimic domain patterns without needing the real kernel or engine yet.

Two adapters I would build
A. Fake geometry adapter

Model:

entities with topology-like IDs

face/edge/region-style partitions

tolerance-sensitive derived summaries

structural edits that remap dependency neighborhoods

boolean-like operations that replace many artifacts at once

Verify:

topology churn

partition-aware diffing

lineage through replacement and restore

tolerance gating behavior

replay after structural edits

B. Fake game-world adapter

Model:

entity/component graph

transforms

visibility/culling summaries

AI perception queries

frame-like repeated updates

hot moving subset inside large cold world

Verify:

hot/cold skew

fanout from transforms/global state

dynamic dependency capture

per-frame determinism

bounded diagnostics across long sessions

This gives you domain-shaped pressure before the actual domains add their own complexity.

If I had to pick the highest-value additional set

If you do not want ten more tests, I would prioritize these five:

Topology churn

Fanout shockwave

Deep chain numerical stability

Determinism under hostile scheduling

Memory pressure and retention

Those five, combined with the original three, would expose most of the serious hidden risks before geometry kernel or game engine adoption.

The meta-principle

Before using this on geometry kernels or game engines, you want proof across five axes:

semantic correctness

historical correctness

scheduler correctness

precision under hostile graph shape

boundedness over long-lived runs

The original three hit the first two hardest.
These extra tests hit the last three.
