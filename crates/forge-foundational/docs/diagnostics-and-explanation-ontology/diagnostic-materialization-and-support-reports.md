# Diagnostic Materialization And Support Reports

## What This Feature Is

This feature turns diagnostic rows into two real surfaces:

- `FoundationalDiagnosticSupportReport`
- `FoundationalDiagnosticExplanationBundle`

It also gives you an explicit `plan(...)` seam before you materialize those
surfaces, so cost, profile richness, availability, partiality, and fallback
debt stay visible.

## Why You Use It

Use this surface when you need to:

- build a support report or explanation bundle from typed rows
- narrow diagnostic breadth with a profile without changing outcome truth
- surface retained, deferred, reconstructable, redacted, or unavailable
  evidence honestly
- carry named gaps when support is partial instead of bluffing completeness
- keep fallback debt and repeated rediscovery visible

This is the main descriptive diagnostics lane. Most runtime-facing diagnostic
use should start here.

## Stable Entry Points

Planning and materialization:

- `plan_diagnostic_support_report(...)`
- `materialize_diagnostic_support_report(...)`
- `plan_diagnostic_explanation_bundle(...)`
- `materialize_diagnostic_explanation_bundle(...)`

Inputs and result surfaces:

- `FoundationalDiagnosticSupportInput`
- `FoundationalDiagnosticExplanationInput`
- `FoundationalDiagnosticMaterializationPlan<_>`
- `FoundationalDiagnosticSupportReport`
- `FoundationalDiagnosticExplanationBundle`

Important supporting types:

- `FoundationalDiagnosticSurfaceAvailability`
- `FoundationalDiagnosticPartiality`
- `FoundationalDiagnosticNamedGap`
- `FoundationalDiagnosticGapClass`
- `FoundationalDiagnosticGapTarget`
- `FoundationalDiagnosticGapClosurePosture`
- `FoundationalDiagnosticSupportClaimStrength`
- `FoundationalDiagnosticCounterSnapshot`
- `FoundationalDiagnosticAssemblyDebt`

## Core Mental Model

There are two jobs here:

- plan the bundle honestly
- materialize the bundle honestly

Planning decides:

- which rows are required, standard, or forensic
- what rows are visible at the chosen profile richness
- whether the delivery class and availability posture are legal
- whether partiality is complete or partial-with-named-gaps
- whether fallback debt or repeated rediscovery must be surfaced

Materialization then freezes that plan into a concrete support report or
explanation bundle.

## How It Executes

You supply:

- a subject
- an outcome kind
- row inventories split into required, standard, and forensic rows
- an availability posture
- a profile
- a delivery class
- counters and any assembly debt

The planner then:

- validates availability and delivery legality
- validates durable or certified support claims
- selects visible rows based on profile richness
- sorts rows canonically
- rejects empty named-gap partiality
- rejects fake debt like zero-count fallback rows

## Small Example

```rust
use forge_foundational::{
    materialize_diagnostic_explanation_bundle, FoundationalDiagnosticCounterSnapshot,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticExplanationInput,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticPartiality,
    FoundationalDiagnosticSurfaceAvailability,
};

let input = FoundationalDiagnosticExplanationInput::new(
    subject,
    FoundationalDiagnosticOutcomeKind::Denied,
    vec![required_row],
    vec![standard_row],
    vec![],
    FoundationalDiagnosticSurfaceAvailability::retained_hot(),
    FoundationalDiagnosticPartiality::Complete,
    FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
    vec![],
);

let bundle = materialize_diagnostic_explanation_bundle(
    input,
    profile,
    FoundationalDiagnosticDeliveryClass::CanDefer,
)?;
```

## Real Example

Use the plan seam when you care about visible rows, fallback debt, or profile
effects before committing to a bundle:

```rust
use forge_foundational::{
    plan_diagnostic_support_report, FoundationalDiagnosticAssemblyDebt,
    FoundationalDiagnosticAssemblyDebtClass, FoundationalDiagnosticCounterSnapshot,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticNamedGap,
    FoundationalDiagnosticPartiality, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSupportInput, FoundationalDiagnosticSurfaceAvailability,
};

let input = FoundationalDiagnosticSupportInput::new(
    subject,
    outcome_kind,
    vec![required_support_row],
    vec![standard_support_row],
    vec![forensic_support_row],
    FoundationalDiagnosticSurfaceAvailability::deferred_cold(),
    FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly,
    FoundationalDiagnosticPartiality::PartialWithNamedGaps(vec![named_gap]),
    FoundationalDiagnosticCounterSnapshot::new(2, 1, 0, 1, 0, 1),
    vec![
        FoundationalDiagnosticAssemblyDebt::new(
            FoundationalDiagnosticAssemblyDebtClass::RowScanFallback,
            1,
        ),
    ],
);

let plan = plan_diagnostic_support_report(
    input,
    profile,
    FoundationalDiagnosticDeliveryClass::CanDefer,
)?;

let visible_rows = plan.selected_rows();
let support = plan.materialize();
```

## How It Relates To Other Features

- [Diagnostic Outcomes, Subjects, And Rows](./diagnostic-outcomes-subjects-and-rows.md)
  defines the row inventories you feed into this layer.
- [Diagnostic Canonical Basis And Comparison](./diagnostic-canonical-basis-and-comparison.md)
  canonicalizes the bundles this layer produces.
- [Certified Diagnostic Bundles And Attachments](./certified-diagnostic-bundles-and-attachments.md)
  is the stronger lane for certified claims after plain descriptive bundles are
  already honest.

## Inspection And Debugging

Inspect these first:

- `bundle.rows()` or `plan.selected_rows()` to see exactly what survived the
  active profile richness
- `bundle.availability()` to see whether evidence is retained, deferred,
  reconstructable, redacted, or unavailable
- `bundle.named_gaps()` to see partiality explicitly
- `bundle.assembly_debts()` and `bundle.counter_snapshot()` to see whether the
  runtime had to widen scope or rediscover information

If a report feels too optimistic, the support-claim strength and active profile
are the first things to check.

## Anti-Patterns

- Do not hide missing coverage in prose when the bundle is partial.
- Do not claim durable or certified support while surfacing no visible rows at
  the chosen richness.
- Do not bury fallback or rediscovery work in comments or counters no one can
  reach.
- Do not rescan broad runtime state after canonical diagnostic artifacts
  already exist just to make a report look richer.

## Current Limits

- This lane is descriptive only. It does not certify hostile coverage.
- If you need stronger proof-bearing claims about coverage or attachment, move
  to the certified bundle lane instead of overloading support reports.

## Related Docs

- [Diagnostic Canonical Basis And Comparison](./diagnostic-canonical-basis-and-comparison.md)
- [Certified Diagnostic Bundles And Attachments](./certified-diagnostic-bundles-and-attachments.md)
