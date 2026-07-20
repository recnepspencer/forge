# Domain Capabilities

Domain work enters Query through a package installed into one runtime. The
runtime returns an installed handle, and that handle is the root for domain
reads, workflows, declarations, contributions, live work, recovery, and
inspection.

Start with [Runtime-Installed Domains](./runtime-installed-domains.md). It
defines the setup grammar, runtime affinity, operating-context identity,
extension pattern, diagnostics, and current limits.

## Choose The Next Guide

- [Runtime-Installed Domains](./runtime-installed-domains.md) for package
  declaration, installation, handle lookup, contexts, and extension traits.
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

## Certification

- [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
- [Goldens, Boundaries, And Hostile Certification](./certification/goldens-boundaries-and-hostile-certification.md)
- [Public Doc Coverage](./public-doc-coverage.md)

Certification exposes evidence and sabotage coverage. It does not mint domain
handles or promote diagnostics into operational authority.

## Related Docs

- [Worth Query Docs Home](../README.md)
- [Declarative Query Experience](../capabilities/declarative-query-experience.md)
- [Consumer Kit](../foundations/consumer-kit.md)
- [Inspection](../capabilities/inspection.md)
