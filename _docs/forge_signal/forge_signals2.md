# Forge Signal Vision 2

## Thesis

Forge uses two distinct runtimes for two distinct jobs:

- `forge-relational` is the source-of-truth runtime. It owns identity,
  mutation, history, diffs, and traversal over the model graph.
- `forge-signal` is the derived-computation runtime. It owns invalidation,
  recomputation, scheduling, convergence, conditional execution, and runtime
  self-inspection for computed state.
- The bridge layer connects them. Relational changes become signal
  invalidations, and signals evaluate against stable host snapshots without
  becoming the owner of truth.

This split is not optional. `forge-signal` is execution and derived-state
infrastructure, never truth. It exists to make derived computation
deterministic, transactional, aspect-aware, traceable, and scalable while
remaining fully decoupled from domain-specific storage and semantics.

`forge-signal` should be designed as a standalone generic library. Forge is a
major target, but not the definition of the runtime. The same core should make
sense for geometry kernels, chip-design and simulation systems, AI world-model
engines, interactive editors, workflow platforms, financial systems, and other
domains where dependency-aware incremental computation matters.

## What This Runtime Is For

`forge-signal` exists for product surfaces where derived computation must be
incremental, explainable, and safe under change.

It is meant to support:

- AI systems that need speculative branch evaluation, causal explanation, and
  controlled recomputation over changing world state
- geometry kernels that need partial recomputation, tolerance-aware invalidation,
  and replayable derived-state transitions instead of fragile rebuild cascades
- chip-design and simulation systems that need branchable analysis, snapshot-safe
  concurrent evaluation, convergence control, and deterministically replayable
  derived outputs
- interactive products that need priority-aware propagation, responsive refresh,
  and progressive refinement without surrendering correctness
- workflow, node-editor, and visual-editor systems that need conditional graph
  execution, transactional refresh, and graph introspection at scale
- compiler and IR systems that need query-style incremental execution,
  structural memoization, and trustworthy explanation of why a result changed

The technical thesis is the same across all of them:

- dependency tracking must be explicit
- invalidation granularity must be precise
- recomputation must be explainable
- execution must be transactional
- derived state must be replayable
- scheduling must remain deterministic when required

## Why This Runtime Is Different

These are not optional add-ons. They are the capabilities that make
`forge-signal` strategically different from ordinary reactive graphs:

- aspect-aware invalidation and n-granularity dependency slices
- conditional nodes and policy-aware evaluation gates
- maybe-stale state rather than crude dirty-only invalidation
- transactional invalidation and hard rewind
- lazy pull-based recomputation
- reactive diff propagation and output identity suppression
- partial recomputation boundaries
- structural memoization
- execution provenance and causal explanation
- graph introspection and dependency inspection
- explicit execution planning and prepared execution
- parallel precompute and future parallel evaluation
- speculative evaluation and branchable execution state
- deterministic execution as a product contract
- explicit deterministic-mode versus optimized-mode scheduling
- cost-aware and priority-aware scheduling
- convergence and fixed-point execution policies
- temporal and previous-value signal support
- adaptive tolerance propagation
- snapshot, replay, and time-travel-ready execution state

If these are treated as “nice to have later,” the runtime collapses back into a
basic invalidation engine and loses the leverage needed for industrial-grade
derived computation.

## Mission

`forge-signal` is the universal derived-computation substrate for Forge. More
generally, it is a generic incremental computation runtime for systems that
need dependency-aware recomputation over host-managed state.

Every expensive or dependency-sensitive derived value can be modeled as a
signal node: derived artifacts, validation summaries, topology-derived caches,
simulation results, pricing outputs, analysis pipelines, incremental queries,
or progressive solver stages.

The runtime exists to answer one question reliably:

> Given a set of upstream changes, what must recompute, in what order, under
> what conditions, under what tolerance and scheduling policy, and with what
> causal trace?

The runtime must answer that question with these non-negotiable properties:

1. Deterministic execution when deterministic mode is requested
2. Transactional rollback semantics
3. Aspect-aware invalidation granularity
4. Explicit separation from truth-state storage
5. First-class runtime self-inspection into why recomputation happened
6. Policy-aware execution for tolerance, priority, convergence, and cost
7. Branchable and replayable execution state

Diagnostics are not optional polish. `forge-signal` must assume there will be
runtime bugs, host bugs, policy mistakes, convergence pathologies, and
hard-to-reproduce invalidation failures. Provenance, inspection, and metrics
are therefore part of the product contract, not just developer support tooling.

For geometry kernels specifically, this runtime should make it possible to stop
treating recomputation as a black box. Partial rebuilds, tolerance-aware
refresh, solver convergence, and causal explanation of geometric change should
become native runtime strengths rather than scattered domain workarounds.

## Architectural Model

### Runtime stack

| Layer | Responsibility | Owns |
| --- | --- | --- |
| `forge-relational` | Truth-state graph runtime | identity, transactions, history, diffs, traversal, integrity |
| `forge-signal` | Derived-computation runtime | dependency DAG, invalidation, recomputation, scheduling, conditions, convergence, runtime self-inspection |
| Bridge / integration | Decoupled coordination | patch-to-invalidation, aspect mapping, snapshot evaluation, node-key mapping |

### What `forge-signal` owns

- evaluation dependency graph scheduling and invalidation
- deterministic ordering and deterministic-mode execution semantics
- optimized-mode execution policy where allowed
- conditional and policy-aware evaluation gates
- transactional invalidation and hard rewind on failure
- node-scoped execution metadata such as aspects, conditions, comparator policy,
  priority, cost hints, and telemetry
- query-style incremental execution semantics
- partial recomputation boundaries and changed-region reporting
- future execution planning, staged execution, and parallel dispatch
- convergence policy and fixed-point style execution support
- snapshot, replay, and branchable execution-state surfaces

### What `forge-signal` does not own

- truth-state graphs, domain entities, or structural storage
- domain numerics, geometry kernels, topology mutation, or schema rules
- semantic meaning of aspects beyond host-defined slots
- host identity models, diffs, or lineage semantics
- permanent fusion with relational storage
- mandatory use of the bridge for standalone signal use

### Structural rule

Signals consume host snapshots and emit derived-state refresh. They do not
become a second source of truth, and they do not own the structural graph they
observe.

## Provenance Model

`forge-signal` must make it possible to explain why a node is in its current
state, not merely that it was evaluated.

The provenance ladder should stay explicit:

- invalidation provenance: which upstream changes dirtied or maybe-staled the
  node
- dependency provenance: which dependencies were considered and how their
  versions compared
- condition provenance: which evaluation conditions deferred or allowed work
- comparator provenance: which changes were suppressed as not meaningful
- policy provenance: which tolerance, cost, priority, or convergence policy
  altered execution
- recomputation provenance: whether work actually ran and what trace summary it
  produced
- host causality metadata: optional upstream provenance attached by host
  runtimes or bridge integration

This is the baseline for trust, debugging, compliance, optimization, and
hard-software diagnosis. The explanation surface, metrics surface, and
inspection APIs should all reinforce this same causal model rather than invent
separate diagnostic stories.

## API Strategy

`forge-signal` should expose two public faces built on one runtime.

The product requirement is not only “easy mode for simple cases.” The
full-power surface must also be beautiful for expert users who need explicit
control. In Forge, “beautiful” means explicit without boilerplate, powerful
without generic noise, and predictable without ambient magic.

Both surfaces must remain directly usable without `forge-relational` or the
bridge. The bridge is an integration layer, not the required entrypoint to the
signal runtime.

### `forge-signal-core`

This is the low-level runtime surface. It keeps all control explicit:

- `SignalGraph`
- `SignalRuntime`
- transactions
- aspects
- evaluation conditions
- comparator policies
- explicit invalidation and evaluation entrypoints
- planners, schedulers, and execution modes
- snapshot, replay, and inspection surfaces

This surface exists for kernel internals, integration layers,
performance-sensitive paths, solver-style workloads, and any host that needs
precise runtime control.

The design goal for this surface is not raw capability alone. The core API
should read as a clean execution pipeline:

- configure runtime and policy once
- open a transaction or execution scope explicitly
- mark dirty inputs and emit staged changes
- plan evaluation under named policies
- evaluate targets under deterministic or optimized execution modes
- commit or rollback deterministically

### `forge-signal-easy`

This is the ergonomic signal surface. It is a core product pillar because the
runtime should feel simple for common use, not only powerful for specialists.

The easy surface should provide:

- input signals
- computed signals
- effects/watchers
- batched updates
- automatic dependency capture
- implicit lazy pull on read for common workflows

The easy surface must compile down to the same runtime primitives as the core
layer. It is a UX layer, not a separate execution engine.

## Capability Pillars

### Dependency and Invalidation Architecture

#### Dependency DAG

Technical role:
The runtime must own an explicit computation graph with stable dependency
semantics and cycle policy.

What this enables:

- industrial incremental execution instead of opaque callback chains
- debuggable dependency topology
- reproducible evaluation plans

#### Aspect-aware invalidation

Technical role:
Nodes subscribe to precise change slices rather than whole-value invalidation.

What this enables:

- narrow recomputation in large graphs
- n-granularity refresh for geometry, simulation, and application state
- better proportionality under heavy churn

#### Maybe-stale state

Technical role:
The runtime must distinguish “must recompute now” from “may need recompute if
asked.”

What this enables:

- lower recomputation pressure
- better lazy evaluation behavior
- more accurate explanation of execution state

#### Conditional and policy-aware nodes

Technical role:
Execution gates such as on-demand, debounce, aspect filters, delta thresholds,
and custom policies are runtime responsibilities.

What this enables:

- responsive products with controlled recomputation
- solver and simulation flows with deferred work
- host-visible execution semantics instead of ad hoc wrapper logic

### Incremental Computation Architecture

#### Lazy recomputation

Technical role:
Work only runs when values are required under current policy.

What this enables:

- lower cost under churn
- stable pull-based query semantics
- better host control over expensive computations

#### Reactive diff propagation

Technical role:
Nodes can emit result diffs and suppress downstream work when outputs did not
meaningfully change.

What this enables:

- large graph scalability
- more precise incremental refresh
- runtime-level downstream suppression instead of domain-layer guesswork

#### Partial recomputation boundaries

Technical role:
Nodes may expose internal partitions or changed-region metadata so recomputation
is narrower than whole-node refresh.

What this enables:

- geometry and simulation workloads that only rebuild touched regions
- large dataflow graphs with proportional work
- better bridge behavior over large truth deltas

#### Structural memoization

Technical role:
The runtime must support reuse by structural input identity, not only node
identity.

What this enables:

- shared computation across repeated shapes
- incremental compiler and AI workloads with recurring structure
- fewer redundant recomputations in large search spaces

#### Query-style incremental execution

Technical role:
Keyed computation families must be native runtime surfaces, not bolted-on
patterns.

What this enables:

- compiler-style incremental queries
- keyed simulation or analysis caches
- scalable host APIs over large parameter spaces

### Scheduling and Execution Architecture

#### Deterministic and optimized scheduling modes

Technical role:
The runtime must explicitly support a deterministic mode for replay/debugging
and an optimized mode for performance-oriented execution where allowed.

What this enables:

- exact replay and certification surfaces
- faster production execution without muddying debug semantics
- honest host choice between certainty and throughput

#### Cost-aware scheduling

Technical role:
Nodes may expose cost hints so planning and execution can prefer better
dispatch strategies.

What this enables:

- better large-graph planning
- smarter parallelism thresholds
- more honest work shaping under industrial workloads

#### Priority-aware propagation

Technical role:
The runtime may prefer high-priority derived work before low-priority work.

What this enables:

- interactive responsiveness
- progressive refinement
- foreground/background computation splits

#### Parallel execution

Technical role:
Prepared evaluation and independent execution lanes should scale out when
policy, legality, and profitability permit.

What this enables:

- large analysis workloads
- multi-core evaluation for heavy derived state
- better use of immutable snapshot-based execution

### Branching, Replay, and Convergence Architecture

#### Speculative evaluation and branchable execution state

Technical role:
The runtime must support evaluating alternate computational branches and
discarding or committing them explicitly.

What this enables:

- AI search and branch-and-choose systems
- solver experimentation
- design-space exploration without truth mutation

#### Snapshot, replay, and time-travel-ready execution state

Technical role:
Execution state should be capturable, restorable, replayable, and comparable.

What this enables:

- replay-oriented debugging
- branch comparison
- runtime certification and diagnosis

#### Fixed-point and convergence policies

Technical role:
The runtime should support iterative convergence and fixed-point execution
where some workloads are not pure DAG-style one-pass refresh.

What this enables:

- constraint solving
- simulation equilibria
- solver-style kernels and iterative analyses

#### Temporal and previous-value signals

Technical role:
Some derived values depend on prior states, not only current dependencies.

What this enables:

- simulation and animation-style workloads
- temporal analytics
- previous-state-sensitive derived computation

### Tolerance and Numerical Policy Architecture

#### Comparator and tolerance policies

Technical role:
The runtime must allow exact and tolerance-aware suppression policies.

What this enables:

- numerically stable incremental systems
- geometry and simulation refresh without pointless churn
- host-controlled semantic change thresholds

#### Adaptive tolerance propagation

Technical role:
Tolerance and uncertainty may need to propagate through downstream execution
instead of remaining local to one comparator.

What this enables:

- error-aware numerical systems
- more honest recomputation policy for scientific and geometric workloads
- better proportionality in approximate-but-controlled pipelines

### Introspection and Trust Architecture

#### Execution trace and decision provenance

Technical role:
The runtime must explain why work ran, why work did not run, and why the
current value is considered valid.

What this enables:

- debugging
- optimization
- compliance and auditability
- AI-assisted reasoning over compute behavior

#### Graph inspection and dependency inspection

Technical role:
The runtime must expose the graph itself as a queryable object.

What this enables:

- hot-path analysis
- dependency-chain inspection
- runtime visualization
- performance diagnosis at scale

#### Metrics and hot-path inspection

Technical role:
Execution counts, duration, cost, chain depth, and recomputation shape must be
product surfaces.

What this enables:

- scale tuning
- cost-aware scheduling feedback
- high-confidence optimization work instead of guesswork

#### Diagnostics and harness infrastructure

Technical role:
The runtime must defend itself with diagnostics-first execution and a real
certification harness.

What this enables:

- regression-resistant evolution
- scenario-driven validation
- trustworthy parity, replay, and failure certification

## Domain Fit

### AI Systems

`forge-signal` should support:

- speculative branch evaluation
- structural memoization over repeated problem shapes
- execution causality for “why did this result change?”
- replayable evaluation over retained truth snapshots

Revolutionary use:
an AI system can treat computation itself as a branchable, explainable search
substrate instead of relying on opaque chains of ad hoc cached functions.

### Geometry and CAD

`forge-signal` should support:

- partial recomputation over touched regions
- tolerance-aware invalidation and adaptive tolerance propagation
- convergence-aware execution for iterative operators and solvers
- replayable explanation of why geometric derived state changed

Revolutionary use:
geometry kernels can move from “rebuild and hope” toward a runtime that can
selectively refresh, explain, and certify geometric recomputation under
aggressive model change.

### Chip Design and Simulation

`forge-signal` should support:

- branchable analysis over immutable snapshots
- convergence-aware and temporal execution for simulation-style workloads
- cost-aware scheduling over very large dependency graphs
- replayable and diagnosable derived-state behavior

Revolutionary use:
chip systems can treat derived analysis as a certifiable execution graph rather
than a tangle of tool-local passes and hidden invalidation policies.

### Interactive Products and Editors

`forge-signal` should support:

- priority-aware propagation
- responsive refresh with maybe-stale behavior
- deterministic-mode debugging and optimized-mode operation
- graph introspection for runtime tooling

Revolutionary use:
large interactive systems can stop trading correctness for responsiveness and
instead get both from explicit runtime scheduling and invalidation policy.

### Compiler, IR, and Query Systems

`forge-signal` should support:

- query-style incremental execution
- structural memoization
- dependency and causality inspection
- replayable result evolution

Revolutionary use:
compiler and IR systems can get a real incremental execution runtime with
introspection and replay instead of stitching those guarantees together around
query caches.

## Roadmap Direction

This file is a vision document, not the execution roadmap. But the engineering
implications should be derivable from it.

If a capability is named here and not yet fully present in the runtime, it is a
roadmap item.

If a capability is present in code but not yet proven under hostile scenarios,
it is a certification item.

The highest-signal remaining product directions are:

- snapshot, replay, and branchable execution-state productization
- speculative evaluation and branch execution
- deterministic-versus-optimized scheduling mode completion
- cost-aware and priority-aware scheduler maturity
- fixed-point and convergence policy support
- temporal and previous-value signal support
- adaptive tolerance propagation
- bridge-grade dual-runtime integration over relational snapshots and patchsets

## Non-Goals

- turning the runtime into the owner of truth-state storage
- fusing signal execution permanently into relational storage
- hiding execution policy behind implicit magic
- reducing explanation and diagnostics to developer-only tooling
- treating deterministic replay, branching, or inspection as optional polish

## Companion Documents

- [_docs/forge_signal/forge_signal_vision.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge_signal/forge_signal_vision.md)
- [_docs/engineering/forge_signal_phase1_plan.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_phase1_plan.md)
- [_docs/engineering/forge_signal_scale_hardening_plan.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_scale_hardening_plan.md)
- [_docs/forge-relational/forge_relational_vision.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/forge_relational_vision.md)

The signal runtime becomes strategically important when it stops being “a
reactive graph” and becomes a certifiable execution substrate: branchable,
explainable, replayable, and precise enough to carry hard derived computation
for serious systems.
