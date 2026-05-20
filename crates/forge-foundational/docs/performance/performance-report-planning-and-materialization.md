# Performance Report Planning And Materialization

## What This Feature Is

This seam widens lower-lane performance artifacts into explicit reports. It
does that honestly by splitting attachment, request shape, plan inspection, and
materialization into distinct steps.

## Why You Use It

- to inspect richer output than plain claim, bundle, or receipt access
- to request layout, contract, counter, budget, or support-bearing sections
- to keep expensive report widening visible before it happens
- to let profile policy elide optional sections centrally

## Stable Entry Points

- `forge_foundational::performance_api::lower_lane::reports`
- `attach_performance_bundle(...)`
- `attach_policy_admission_receipt(...)`
- `attach_counter_backed_performance_receipt(...)`
- `plan_performance_report(...)`
- `FoundationalPerformanceReportRequest`
- `FoundationalPerformanceReportPlan`
- `FoundationalMaterializedPerformanceReport`

## Core Mental Model

This seam treats broad report output as a planned expansion, not a free getter.

- an attachment says what source and target combination is legal
- the request is the stable object-spec shape
- the plan is the inspectable accountability surface
- materialization is the only public widening step

The key boundary is the materialization boundary:

- `ClaimInspectionOnly`
- `ReportAssembly`
- `SupportExpansion`

## How It Executes

1. Attach a legal source to a legal report target.
2. Construct a `FoundationalPerformanceReportRequest`.
3. Plan the report and inspect included/excluded sections plus the
   materialization boundary.
4. Materialize only if you want the widened report surface.

## Small Example

```rust
use forge_foundational::performance_api::lower_lane::reports;

let plan = reports::plan_performance_report(
    forge_foundational::FoundationalPerformanceReportRequest {
        source: attached,
        profile,
        include_layout_intent: true,
        include_contract_names: false,
        include_counter_specs: false,
        include_counter_rows: false,
        include_supporting_evidence_rows: false,
        include_budget_decisions: false,
        include_denied_work: false,
        include_widened_work: false,
    },
);
```

This is the smallest honest example because it stops at the plan and makes the
widening boundary visible before any extra assembly happens.

## Real Example

```rust
use forge_foundational::performance_api::lower_lane::reports;

let attached = reports::attach_counter_backed_performance_receipt(
    forge_foundational::FoundationalPerformanceAttachmentTargetKind::BoundaryReport,
    receipt,
)?;

let plan = reports::plan_performance_report(
    forge_foundational::FoundationalPerformanceReportRequest {
        source: attached,
        profile,
        include_layout_intent: true,
        include_contract_names: false,
        include_counter_specs: true,
        include_counter_rows: true,
        include_supporting_evidence_rows: true,
        include_budget_decisions: false,
        include_denied_work: false,
        include_widened_work: false,
    },
);

let report = plan.materialize();
```

What is authoritative:

- the lower-lane source artifact

What is derived:

- the report sections and support-bearing expansion

What gets retained:

- exact section decisions and materialization boundary

What gets inspected:

- `plan.materialization_boundary()`
- `plan.included_sections()`
- `plan.excluded_sections()`
- `report.counter_rows()`
- `report.supporting_evidence_rows()`

## How It Relates To Other Features

- Use
  [Counter-Backed Performance Receipts](./counter-backed-performance-receipts.md)
  when the source is executed evidence.
- Use
  [Policy Admission Receipts](./policy-admission-receipts.md)
  when the source is pre-execution policy.
- Use
  [Certified And Readmitted Performance Bundles](./certified-and-readmitted-performance-bundles.md)
  only after a materialized support-expansion report or a qualifying hot-path
  receipt exists.

## Inspection And Debugging

Inspect the plan before materializing:

- `plan.materialization_boundary()`
- `plan.included_sections()`
- `plan.excluded_sections()`

Expect explicit causes such as:

- `ProfileElided`
- `UnavailableFromSource`

If a source cannot attach to a target, the attachment denial is part of the
feature contract, not an incidental runtime error.

## Anti-Patterns

- treating report materialization as a cheap accessor
- using bundle support rows directly instead of explicit report widening
- assuming every requested section is available from every source family
- widening a hot operational path into support expansion by default

## Current Limits

- report widening is explicit and intentionally more expensive than direct lane
  inspection
- source availability still constrains which sections can appear
- this seam does not itself mint stronger proof-bearing artifacts

## Related Docs

- [Counter-Backed Performance Receipts](./counter-backed-performance-receipts.md)
- [Policy Admission Receipts](./policy-admission-receipts.md)
- [Certified And Readmitted Performance Bundles](./certified-and-readmitted-performance-bundles.md)
