# Export Bundles And Producer Shape

## What This Feature Is

This feature turns a ready canonical basis bundle into a named export bundle
that another producer or consumer can carry across a boundary. It also keeps
producer shape, export manifest structure, trust-boundary bridging, and
readmission explicit.

## Why You Use It

- Use this when canonical fixtures or manifests need to travel outside the
  immediate producer.
- Use this when producer shape is part of the published contract.
- Use this when export comparison should distinguish semantic mismatch from
  manifest mismatch.

## Stable Entry Points

Common path:

- `canonicalization().export().from_bundle(...)`
- `.named(...)`
- `.for_producer_shape(...)`
- `.under(...)`
- `canonicalization().export().compare(...)`
- `canonicalization().export().mismatch_basis(...)`
- `canonicalization().export().manifest_mismatch(...)`
- `canonicalization().export().bridge(...)`
- `canonicalization().export().readmit(...)`

Lower lane:

- `prepare_canonical_export_bundle(...)`
- `compare_canonical_exports(...)`
- `bridge_canonical_export_trust_boundary(...)`
- `readmit_canonical_export_after_boundary(...)`
- `CanonicalExportReadyArtifact`
- `CanonicalExportBundle`
- `CanonicalExportManifest`
- `CanonicalExportManifestMismatch`
- `CanonicalProducerShape`

Good to know:

- `canonicalization_api::common_path` is the recommended grouped public lane.
- `canonicalization_api::lower_lane::export` is the inspectable lower lane.
- export is downstream of a ready basis bundle; it is not an alternate authoring
  path.

## Core Mental Model

Export is a publication lane, not a source-of-truth lane.

You start from a ready basis bundle that already represents stable canonical
meaning. Export then adds:

- a fixture or bundle name
- a producer shape
- an export manifest
- boundary-crossing and readmission vocabulary

That is why export stays downstream of basis readiness. It should not become a
back door for constructing canonical meaning from scratch.

## How It Executes

The normal flow is:

1. prepare a ready canonical basis bundle
2. name the export bundle
3. choose the producer shape explicitly
4. admit the export bundle under an equivalence basis
5. compare exports or bridge them across a boundary when needed
6. readmit after boundary crossing before treating the export as current again

## Small Example

```rust
use worth_foundational::{canonicalization, CanonicalEquivalenceBasis, CanonicalProducerShape};
use worth_proof::TransitionOutcome;

let export = match canonicalization()
    .export()
    .from_bundle(bundle_ready)
    .named("support-fixture")
    .for_producer_shape(CanonicalProducerShape::SingleProducer)
    .under(CanonicalEquivalenceBasis::Strict)
{
    TransitionOutcome::Success(export) => export,
    other => return Err(format!("export admission failed: {other:?}").into()),
};
```

This is the smallest honest example because export only begins after bundle
readiness already exists.

## Real Example

```rust
use worth_foundational::{canonicalization, CanonicalEquivalenceBasis, CanonicalProducerShape};
use worth_proof::AuthorityWitness;

let left = canonicalization()
    .export()
    .from_bundle(left_bundle)
    .named("fixture-a")
    .for_producer_shape(CanonicalProducerShape::SingleProducer)
    .under(CanonicalEquivalenceBasis::Strict)?;

let right = canonicalization()
    .export()
    .from_bundle(right_bundle)
    .named("fixture-a")
    .for_producer_shape(CanonicalProducerShape::SingleProducer)
    .under(CanonicalEquivalenceBasis::Strict)?;

let comparison = canonicalization().export().compare(&left, &right);

if let Some(mismatch) = canonicalization().export().manifest_mismatch(&comparison) {
    println!("manifest mismatch: {:?}", mismatch.kind());
}

let bridged = canonicalization().export().bridge(left);
let readmitted = canonicalization().export().readmit(
    bridged,
    rule_version,
    AuthorityWitness::from_authority_marker(authority),
);
```

What is authoritative here is still the ready export artifact after admission
or readmission. Boundary bridging preserves shape, not current authority.

## How It Relates To Other Features

- [Canonical Basis Sequences And Entry Grammar](./canonical-basis-sequences-and-entry-grammar.md)
  explains where the ready bundle comes from.
- [Equivalence And Mismatch Classification](./equivalence-and-mismatch-classification.md)
  explains the basis-comparison model export builds on.
- [Digest Derivation And Slot Semantics](./digest-derivation-and-slot-semantics.md)
  can derive digest output from ready export artifacts after export admission.

## Inspection And Debugging

Inspect these first:

- `canonicalization_api::lower_lane::export` when you need exact export or
  readmission vocabulary
- `CanonicalExportManifestMismatch` when export comparison fails by manifest
  rather than basis
- `CanonicalMismatchBasis` when export comparison fails by semantic mismatch
- producer shape and fixture name when two exports should align but do not

If export comparison surprises you, decide first whether the problem is a
manifest mismatch or a semantic mismatch. The fix is often different.

## Anti-Patterns

- Do not use export bundles as a substitute for ready basis construction.
- Do not assume boundary bridging preserves current authority.
- Do not treat manifest mismatch and semantic mismatch as the same failure.

## Current Limits

- Export standardizes publication shape for Milestone 2. It does not replace
  later artifact-taxonomy work.
- Readmission is explicit on purpose; there is no silent "bridge and keep
  authority" shortcut.

## Related Docs

- [Equivalence And Mismatch Classification](./equivalence-and-mismatch-classification.md)
- [Digest Derivation And Slot Semantics](./digest-derivation-and-slot-semantics.md)
