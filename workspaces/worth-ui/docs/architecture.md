# Worth UI Architecture

Worth UI turns authored application meaning into a host-neutral mounted frame.
Application code chooses declarations, capabilities, Query views, and a host.
The platform keeps parsing, runtime state, publication, and native mechanics in
their owning layers.

## Source To Mounted Flow

```text
file source or typed Rust authoring
-> worth-ui-dsl compile and canonical semantic package
-> file candidate submission or typed Rust preparation input
-> application preparation
-> graph admission and planning
-> active application session
-> mounted frame assembly
-> host-contract presentation
-> publication or a typed non-success outcome
```

File transport and language meaning are intentionally separate. The runtime
reads and watches files, then carries the settled snapshot as one candidate
submission. Typed Rust input enters the application builder directly.
`worth-ui-dsl` compiles both forms, reports language errors, and produces their
shared sealed semantic package. Runtime preparation consumes that package; it
never reparses source on a steady frame.

## Authority Owners

- `worth-ui-dsl` owns authored syntax, source structure, diagnostics,
  normalization, and the sealed semantic package.
- `worth-ui` owns the named product facades developers import.
- `worth-ui-runtime` owns the active application, graph, planning, mounted
  publication, host exchange, and runtime inspection transitions.
- `worth-ui-query-binding` translates installed Worth Query authority into
  Worth UI registrations. The UI runtime does not recreate Query state.
- `worth-ui-host-contract` defines inert host capabilities, mounted input, and
  mechanical outcomes.
- Host adapters such as `worth-ui-host-egui` perform native mechanics. They do
  not receive source, Query, graph, or publication authority.
- `worth-ui-certification` proves the boundaries; it is not an application API.

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

The arrows are not permission for every lower layer to import every peer.
Authored meaning flows from DSL to runtime. Query crosses only through the
binding crate. Runtime sends sealed mounted mechanics to the host contract.
Adapters depend on the host contract and never on runtime internals.

Within the runtime, the allowed owner graph is documented in
[Runtime subsystems](./runtime-subsystems.md). Graph does not depend on
mounting; observation cannot publish frames; inspection cannot mutate or
reconstruct operational state.

## Failure And Publication

Preparation, planning, mounting, host presentation, and publication have
different failure owners. A denial before effects preserves the previous
published frame. If native effects may have started, the runtime retains the
previous semantic publication but marks the affected binding uncertain until
typed reconciliation succeeds.

Application replacement follows the same rule. Candidate work remains off to
the side until application, plan, allocation, mounted identity, and frame can
publish as one coherent successor.

## Cost Posture

Source acquisition and lowering are reconstructive work. Steady frames operate
from the active sealed generation. Changed-frame work follows affected
instances and indexes; unchanged publication uses an exact reuse witness.
Inspection and rich cost reports are materialized outside the measured
executor interval when requested.

See [Authored composition](./authored-composition.md),
[Application lifecycle](./application-lifecycle.md), and
[Application inspection](./inspection.md) for developer-facing use.
