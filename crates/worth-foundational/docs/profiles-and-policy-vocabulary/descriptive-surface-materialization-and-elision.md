# Descriptive Surface Materialization And Elision

## What This Feature Is

This feature plans which descriptive surfaces a profiled target can actually
show. It covers surface inventories, target applicability, cost, absence
causes, and named elision profiles such as operational summary.

## Why You Use It

- Use this when you need to know which descriptive surfaces are available for a
  target before you materialize them.
- Use this when reduced richness, weak retention, or weak certification should
  hide some surfaces without changing the underlying truth.
- Use this when you need a clear reason for why a surface is missing.

## Stable Entry Points

Common path:

- `profiles().materialization().for_boundary_artifact(...)`
- `profiles().materialization().for_support_artifact(...)`
- `profiles().materialization().for_proof_bearing_artifact(...)`
- `.full_fidelity()`
- `.operational_summary()`
- `.selected(...)`

Lower lane:

- `plan_foundational_profile_materialization(...)`
- `plan_foundational_profile_materialization_with_elision(...)`
- `plan_selected_foundational_profile_materialization(...)`
- `boundary_artifact_surface_inventory()`
- `support_artifact_surface_inventory()`
- `proof_bearing_artifact_surface_inventory()`
- `foundational_profile_applicability::<...>()`

Good to know:

- `profiles_api::common_path` is the recommended grouped public lane.
- `profiles_api::lower_lane::materialization` is the inspectable lower lane.

## Core Mental Model

Materialization planning is a policy decision surface, not a rendering step.

It answers:

- which surfaces are legal for this target kind
- which of those surfaces were requested
- which requested surfaces are available
- which requested surfaces are unavailable, and why

A "surface" here means a descriptive view such as history, replay, lineage,
provenance, or forensic diagnostics.

## How It Executes

Planning starts from a materialized profile and a target kind.

The planner then:

1. loads the legal surface inventory for that target
2. applies any named elision profile or explicit selected list
3. computes availability per surface
4. records typed absence causes where a surface is unavailable
5. records cost counters for inventory size, requested size, and visible size

## Small Example

```rust
use worth_foundational::{profiles, FoundationalDescriptiveSurface};

let plan = profiles()
    .materialization()
    .for_proof_bearing_artifact(&proof_bearing)
    .selected(&[FoundationalDescriptiveSurface::Provenance])?;
```

This is the smallest honest example because it shows the target-aware planning
entry and an explicit selected-surface request.

## Real Example

```rust
use worth_foundational::{profiles, FoundationalDescriptiveSurface};

let support_plan = profiles()
    .materialization()
    .for_support_artifact(&support_artifact)
    .operational_summary();

let forensic = support_plan
    .decision_for(FoundationalDescriptiveSurface::ForensicDiagnostics)
    .expect("forensic decision");

if !forensic.is_available() {
    println!("forensic diagnostics unavailable: {:?}", forensic.absence_cause());
}
```

What is authoritative here is the plan, not a guessed surface list. The plan
tells you what the runtime can honestly show for that target and why anything
is missing.

## How It Relates To Other Features

- [Target-Aware Profile Attachment](./target-aware-profile-attachment.md)
  gives you the profiled artifact this planner consumes.
- [Profile Families And Composed Profile Sets](./profile-families-and-composed-profile-sets.md)
  explains where richness, retention, support, and certification posture come
  from.
- [Profile Identity, Difference, And Canonical Basis](./profile-identity-difference-and-canonical-basis.md)
  covers the canonical identity of admitted profile meaning, which is separate
  from descriptive surface planning.

## Inspection And Debugging

Inspect these first:

- `profiles_api::lower_lane::materialization` when you need inventory,
  applicability, or planning vocabulary directly
- `plan.decisions()` for the full availability table
- `plan.cost()` for inventory and requested counts
- `decision.absence_cause()` for typed missing-surface reasons
- `foundational_profile_applicability::<Target>()` when you need to know which
  families and decisions govern a surface

If a surface is missing, look at richness, retention, support posture, and
certification posture before looking for a bug.

## Anti-Patterns

- Do not treat selected surfaces as legal for every target kind.
- Do not collapse all missing surfaces into one generic "not available" reason.
- Do not assume operational summary means the same surface set for every target.

## Current Limits

- This layer plans and explains descriptive availability. It does not certify
  stronger proof-bearing claims.
- Illegal or duplicate selected surfaces fail closed instead of being cleaned
  up automatically.

## Related Docs

- [Target-Aware Profile Attachment](./target-aware-profile-attachment.md)
- [Proof-Bearing Profile Certification And Readmission](./proof-bearing-profile-certification-and-readmission.md)
