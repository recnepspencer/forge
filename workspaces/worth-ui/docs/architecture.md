# Worth UI Architecture

Worth UI turns authored application meaning into a host-neutral mounted frame.
Application code chooses declarations, capabilities, Query views, and a host.
The platform keeps parsing, semantic observation, consequence planning,
runtime state, publication, and native mechanics in their owning layers.

## Source To Mounted Flow

```text
file source or typed Rust authoring
-> worth-ui-dsl compile and canonical semantic package
-> candidate submission
-> application preparation
-> graph admission and planning
-> active application session
-> mounted frame assembly
-> host-contract presentation
-> publication or a typed non-success outcome
```

File transport and language meaning are separate. Runtime reads and watches
files, then carries one settled snapshot into one DSL compile. Typed Rust input
enters the semantic compiler directly. Runtime preparation consumes the sealed
package; it never reparses source on a steady frame.

## Observation To Rebind Flow

```text
owner-specific source / host / measurement / Query / committed-runtime evidence
-> admitted observation turn
-> semantic classification
-> produced facts + declared consumed aspects
-> indexed affected scope
-> identity lifecycle
-> immutable rebind plan
-> final admission and reservation
-> canonical host effects
-> one atomic semantic + mounted publication
```

This is a state-change compiler, not an event callback. Each governed phase
requires the concrete value issued by the previous owner. A watcher, pixel,
digest, inspection receipt, or host result cannot construct a plan or publish.

## Authority Owners

- `worth-ui-dsl` owns syntax, source structure, diagnostics, normalization, and
  the sealed semantic package.
- `worth-ui` owns the named product facades developers import.
- `worth-ui-runtime` owns active application, observation, rebind planning,
  mounted publication, host exchange, recovery, and runtime inspection.
- `worth-ui-query-binding` translates installed Worth Query authority into
  Worth UI registrations and consequences. Runtime does not recreate Query.
- `worth-ui-host-contract` defines inert host capabilities, mounted input, and
  mechanical outcomes.
- Host adapters perform native mechanics. They never receive source, Query,
  graph, semantic-classification, or publication authority.
- `worth-ui-certification` proves boundaries; it is not an application API.

An identity or digest can explain what happened. It cannot launch an
application, execute a plan, publish a frame, or reconstruct authority.

## Dependency Direction

```text
application code
-> worth-ui facade
-> worth-ui-runtime
-> worth-ui-query-binding -> worth-query
-> worth-ui-host-contract <- host adapters
```

The arrows do not permit every lower layer to import every peer. Authored
meaning flows from DSL to runtime. Query crosses only through the binding
crate. Runtime sends sealed mounted mechanics to the host contract. Adapters
depend on the host contract and never on runtime internals.

Within runtime, graph does not depend on mounting; observation cannot publish;
rebind planning cannot mutate mounted truth; inspection cannot mutate or
reconstruct operational state. The active session coordinates complete
transitions across named owners.

## Failure And Publication

Preparation, observation admission, classification, planning, final admission,
mounting, host presentation, and publication have different denial owners. A
denial before effects preserves the prior published generation and frame.

Candidate work remains off to the side until application, graph, plan,
allocation, mounted identity, and frame can publish as one coherent successor.
If native effects may have started, runtime retains the previous semantic
publication and exact recovery authority; it never reports optimistic rollback.

`UiRebindRuntimeState` owns plan, receipt, completion, recovery, terminal
decision, and causal-evidence capacity. Canonical cutover still occurs through
the application and mounted publication owners—there is no watcher-owned or
renderer-owned alternate.

## Cost Posture

Source acquisition and DSL compilation are reconstructive source work. Steady
frames operate from the active sealed generation. Post-classification rebind
work is reported as `O + F + A + C + R + G + M + B`: observations, facts,
affected consumers, conflicts, resets, graph work, mounted work, and retained
bookkeeping. Physical presentation/reconciliation remains a separate host cost.

Changed-frame work follows declared consumers and derived indexes. Unchanged
publication uses exact reuse evidence and performs no new rebind work. Rich
inspection and causal reports are materialized outside measured execution when
requested.

## Extension Boundaries

New product data, intents, portal/focus/motion/appearance facts, expressions,
and authored composition extend their semantic owner and feed the existing
observation/planning/publication progression. They must not add a universal
event bag, a second executor, or a host-adapter semantic lane.

See [Runtime subsystems](./runtime-subsystems.md),
[Authored composition](./authored-composition.md),
[Application lifecycle](./application-lifecycle.md),
[Hot rebind](./hot-rebind.md), and
[Application inspection](./inspection.md).
