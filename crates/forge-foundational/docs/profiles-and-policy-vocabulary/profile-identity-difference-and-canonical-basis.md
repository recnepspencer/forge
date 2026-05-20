# Profile Identity, Difference, And Canonical Basis

## What This Feature Is

This feature gives admitted profile meaning a stable identity and a structured
way to compare profiles. It also provides the canonical basis entries used when
profile meaning participates in canonicalization.

## Why You Use It

- Use this when two runtimes need to prove they are talking about the same
  admitted profile meaning.
- Use this when you want structured difference and compatibility classification
  instead of ad hoc field-by-field comparison.
- Use this when admitted profile meaning needs to flow into a canonical basis
  or digest lane.

## Stable Entry Points

Identity and difference:

- `derive_foundational_profile_identity(...)`
- `compare_foundational_profile_identities(...)`
- `compare_foundational_profiles(...)`
- `classify_foundational_profile_compatibility(...)`

Canonical basis:

- `prepare_admitted_foundational_profile_for_canonical_basis(...)`
- `foundational_profile_canonical_basis_entries(...)`

Supporting types:

- `FoundationalProfileIdentity`
- `FoundationalProfileDifferenceReport`
- `FoundationalProfileCompatibilityClass`

Good to know:

- `profiles_api::lower_lane::identity` is the inspectable lower lane.

## Core Mental Model

Identity is based on admitted profile meaning, not on how a caller happened to
construct a profile.

Difference and compatibility are also structured, not guessed:

- some changes only reduce richness
- some changes only narrow retention
- some changes shift support or certification posture
- some changes are fully incompatible

The canonical basis is the ordered, machine-stable representation of admitted
profile meaning that later lanes can digest or compare.

## How It Executes

The normal flow is:

1. admit a profile
2. derive a stable identity from the admitted artifact
3. compare profiles or identities when you need parity checks
4. prepare the admitted profile for canonical basis when it must participate in
   canonicalization

## Small Example

```rust
use forge_foundational::derive_foundational_profile_identity;

let identity = derive_foundational_profile_identity(&admitted_profile)?;
```

This is the smallest honest example because identity is only stable once the
profile has been admitted.

## Real Example

```rust
use forge_foundational::{
    classify_foundational_profile_compatibility, compare_foundational_profiles,
    derive_foundational_profile_identity,
};

let left_identity = derive_foundational_profile_identity(&left_admitted)?;
let right_identity = derive_foundational_profile_identity(&right_admitted)?;

if left_identity != right_identity {
    let report = compare_foundational_profiles(
        left_admitted.payload().admitted(),
        right_admitted.payload().admitted(),
    );
    let class = classify_foundational_profile_compatibility(&report);
    println!("compatibility class: {:?}", class);
}
```

What is authoritative here is admitted profile meaning. The identity and
difference layers derive from that stable source instead of inventing their
own truth.

## How It Relates To Other Features

- [Requested, Admitted, And Materialized Profile Progression](./requested-admitted-and-materialized-profile-progression.md)
  explains why admitted meaning is the stable source.
- [Proof-Bearing Profile Certification And Readmission](./proof-bearing-profile-certification-and-readmission.md)
  strengthens proof-bearing artifacts after profile meaning is already stable.
- [Profile Production Readiness](./profile-production-readiness.md)
  freezes identity and difference as certified Milestone 3 surfaces.

## Inspection And Debugging

Inspect these first:

- `profiles_api::lower_lane::identity` when you need the lower-lane identity,
  basis, difference, or compatibility vocabulary
- `FoundationalProfileDifferenceReport` when two profiles differ
- `FoundationalProfileCompatibilityClass` when you need a high-level summary
- canonical basis entries when two independently built profiles should compare
  equal but do not

If identity differs unexpectedly, check admitted family values first. The
problem is usually semantic drift, not ordering or construction history.

## Anti-Patterns

- Do not derive identity from requested or materialized meaning when admitted
  meaning is the real stable source.
- Do not treat digest equality as a replacement for explicit compatibility
  classification.
- Do not compare raw fields ad hoc when a structured difference report already
  exists.

## Current Limits

- This layer explains stable admitted meaning. It does not plan descriptive
  surfaces.
- Compatibility classes are intentionally coarse; use the full difference
  report when you need exact drift details.

## Related Docs

- [Requested, Admitted, And Materialized Profile Progression](./requested-admitted-and-materialized-profile-progression.md)
- [Profile Production Readiness](./profile-production-readiness.md)
