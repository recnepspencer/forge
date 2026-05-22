# Certified And Readmitted Performance Bundles

## What This Feature Is

This is the stronger performance lane. It upgrades specific lower-lane
performance artifacts into proof-bearing certified bundles and supports
trust-boundary bridging plus readmission when a stronger claim is real.

## Why You Use It

- to strengthen a current-basis hot-path executed receipt into a proof-bearing
  operational claim
- to strengthen a support-expansion report into a proof-bearing compatibility
  claim
- to carry stronger certified performance evidence across a trust boundary
- to readmit that evidence with explicit authority and basis

## Stable Entry Points

- `forge_foundational::performance_api::stronger_lane::certified`
- `certify_hot_path_counter_backed_performance_receipt(...)`
- `certify_support_expansion_performance_report(...)`
- `bridge_certified_performance_bundle_trust_boundary(...)`
- `readmit_certified_performance_bundle_after_boundary(...)`
- `foundational_performance_certified_attachment_authority()`
- `foundational_performance_certified_readmission_authority()`

## Core Mental Model

This lane does not invent new descriptive meaning. It strengthens existing
lower-lane meaning through the shared `forge-proof` substrate.

- the source artifact must already be honest in the lower lane
- certification adds stronger proof-bearing attachment
- bridging marks a trust-boundary crossing
- readmission re-establishes current-basis reuse with explicit authority

## How It Executes

1. Build honest lower-lane source evidence.
2. Certify it with the appropriate stronger-lane entrypoint.
3. If the artifact crosses a boundary, bridge it.
4. Readmit it with an explicit canonical-basis-ready source and readmission
   authority.

## Small Example

```rust
use forge_foundational::performance_api::stronger_lane::certified;

let certified = certified::certify_hot_path_counter_backed_performance_receipt(
    receipt,
    certified::foundational_performance_certified_attachment_authority(),
)?;
```

This is the smallest honest example because it starts from a qualifying executed
receipt rather than trying to mint proof from a plain claim or bundle.

## Real Example

```rust
use forge_foundational::performance_api::{
    lower_lane::basis,
    stronger_lane::certified,
};

let certified = certified::certify_hot_path_counter_backed_performance_receipt(
    receipt,
    certified::foundational_performance_certified_attachment_authority(),
)?;

let basis_ready =
    basis::prepare_counter_backed_performance_receipt_for_canonical_basis(
        basis::performance_basis_rule_version(),
        certified.source(),
    )?;

let bridged = certified::bridge_certified_performance_bundle_trust_boundary(certified);
let readmitted = certified::readmit_certified_performance_bundle_after_boundary(
    bridged,
    basis_ready,
    certified::foundational_performance_certified_readmission_authority(),
);
```

What is authoritative:

- the certified source artifact plus attached proof

What is derived:

- trust-boundary bridge and readmission outcome

What gets retained:

- certified class, source kind, proofs, and readmission basis

What gets inspected:

- `certified.certified_class()`
- `certified.source_kind()`
- `certified.source_digest()`
- `readmitted.readmission_basis()`

## How It Relates To Other Features

- Start from
  [Counter-Backed Performance Receipts](./counter-backed-performance-receipts.md)
  for hot-path operational certification.
- Start from
  [Performance Report Planning And Materialization](./performance-report-planning-and-materialization.md)
  for support-expansion certification.
- Keep
  [Performance Production Readiness](./performance-production-readiness.md)
  separate; readiness is its own stronger seam, not part of every certified
  bundle.

## Inspection And Debugging

Check these first:

- `certified.source_kind()`
- `certified.certified_class()`
- `certified.source_digest()`
- `readmitted.readmission_basis()`

Common denial cases:

- hot-path certification without the required operational exclusions
- support certification without a `SupportExpansion` report boundary
- attempts to certify plain claims, plain bundles, or plain reports

## Anti-Patterns

- trying to certify plain descriptive or plain lower-lane artifacts
- using certification as a shortcut around lower-lane honesty
- treating support-expansion certification as if it were current-basis hot-path
  truth
- collapsing readiness certification into ordinary certified-bundle workflows

## Current Limits

- shipped certification source kinds are intentionally narrow:
  current-basis counter-backed receipts and support-expansion reports
- stronger proof does not replace descriptive or lower-lane APIs
- trust-boundary readmission still depends on explicit canonical-basis-ready
  source evidence

## Related Docs

- [Counter-Backed Performance Receipts](./counter-backed-performance-receipts.md)
- [Performance Report Planning And Materialization](./performance-report-planning-and-materialization.md)
- [Performance Production Readiness](./performance-production-readiness.md)
