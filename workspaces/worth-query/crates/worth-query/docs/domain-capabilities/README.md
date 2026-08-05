# Domain Capabilities

Domain work enters Query through a package installed into one runtime. The
runtime returns an installed handle, and that handle is the root for domain
reads, workflows, declarations, contributions, live work, recovery, and
inspection.

Start with
[Runtime-Installed Domains And Operations](./runtime-installed-domains.md).
It defines portable operation meaning, volatile provider registration, the
single operating-world root, bound execution, publication, consumption,
diagnostics, and current limits.

## Choose The Next Guide

- [Canonical Graph Obligation Progression](./canonical-graph-obligation-progression.md)
  for installed graph-work meaning, graph-read planning, managed sessions,
  actual owner execution, terminals, publication, and downstream inspection.
- [Runtime-Installed Domains And Operations](./runtime-installed-domains.md)
  for package declaration, operation semantics, provider installation,
  binding, execution, publication, and settlement.
- [Conditional Installed Operations](./conditional-installed-operations.md)
  for semantic truth dependencies, conditions, triggers, thresholds,
  correspondence, Signal decisions, and Query re-entry.
- [Installed Operation Re-Execution And Replay](./installed-operation-reexecution-and-replay.md)
  for fresh workflow execution, cert-only equivalence, localized divergence,
  and retained-snapshot historical replay.
- [Installed Operation Aftermath](./installed-operation-aftermath.md) for exact
  inverse, compensation, postcondition verification, and partial-effect
  recovery evidence.
- [Installed Operation Lineage And Promotion](./installed-operation-lineage-and-promotion.md)
  for effect-bound identity evolution, persistent naming, and sparse durable
  graph identity.
- [Bound Projection Lifecycle, Sharing, And Consumer Invalidation](./bound-projection-sharing-and-invalidation.md)
  for declaration-indexed native access, relationship-specific compatibility,
  managed transitions, dependency impact, one shared live owner, move-only
  consumer leases, and exact invalidation deltas.
- [Consumption Cost Evidence](./consumption-cost-evidence.md) for exact
  boundary-local counters, settled operation snapshots, denial budgets, and
  explicit Foundational receipt export.
- [Typed Binding Pipeline](./typed-binding-pipeline.md) when an installed
  declaration context must select the next proof-bearing input.
- [Ordinary Outcomes](./ordinary-outcomes.md) for the common completed/stopped
  result vocabulary.
- [Recovery Boundary](./recovery-boundary.md) for typed next actions after a
  stale, unsupported, or mismatched operation.
- [Continuation Pipeline](./continuation-pipeline.md) for prepared work that
  must be readmitted before execution.
- [Lower-Runtime Capability Routing](./lower-runtime-capability-routing.md) when
  domain meaning crosses into a lower runtime with explicit boundary evidence.
- [Inspection Vs Readiness Vs Recovery](./choosing/inspection-vs-readiness-vs-recovery.md)
  when you need to choose between support posture, retained truth, and repair
  guidance.

## Domain-Owned Vocabulary

A downstream domain crate may define extension traits over
`WorthQueryInstalledDomainHandle<D>`. Extensions can give generic Query
operations domain-native names and payloads. They must delegate to the installed
handle; Query continues to own canonical identity, admission, planning,
execution, receipts, and diagnostics.

Family helpers and grouped authoring follow the same rule:

- [Family Helpers](./family-helpers.md)
- [Grouped Authoring](./grouped-authoring.md)
- [Grouped Products](./grouped-products.md)
- [Grouped Contributions](./grouped-contributions.md)
- [Grouped Support And Readiness](./grouped-support-readiness.md)

## Lifecycle And Diagnostics

- [Declaration Entry Inspection](./declaration-entry-inspection.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Recovery Boundary](./recovery-boundary.md)

These surfaces consume installed authority or artifacts derived from it. They
are not alternative setup roots.

## Certification And Closeout

- [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
- [Goldens, Boundaries, And Hostile Certification](./certification/goldens-boundaries-and-hostile-certification.md)
- [Installed Operation Certification Kit](./certification/installed-operation-certification-kit.md)
- [Installed Domain Closeout Evidence](./platform-entry-closeout.md)
- [Domain Capability Documentation Certification](./public-doc-coverage.md)

These are certification-audience evidence and maintenance guidance. They are
not ordinary setup, execution, or authority-minting APIs.

## Related Docs

- [Worth Query Docs Home](../README.md)
- [Declarative Query Experience](../capabilities/declarative-query-experience.md)
- [Consumer Kit](../foundations/consumer-kit.md)
- [Inspection](../capabilities/inspection.md)
