# Digest Preparation And Canonical Basis

## What This Feature Is

This feature turns Milestone 1 aspect artifacts into a proof-bearing canonical
basis that later digest and equality lanes can trust. Use it when you need
stable ordering and representation-independent comparison without reinterpreting
contracts, masks, patches, or admitted state yourself.

This is not the final digest algorithm. It is the preparation layer that says
"these inputs are ready to be hashed or compared in one canonical way."

## Why You Use It

- Use this when later code needs canonical ordering for contracts, masks,
  patches, admitted state, identities, or locators.
- Use this when you want equality or digest inputs to come from one shared
  meaning-preserving basis instead of ad hoc field ordering.
- Use this when a downstream crate should depend on Milestone 1 semantics
  without re-learning value widths, shape families, or patch distinctions.

## Stable Entry Points

Common path:

- `canonicalization().basis().at(...)`
- `.from_contract(...)`
- `.from_mask(...)`
- `.from_patch(...)`
- `.from_state(...)`
- `.from_identity(...)`
- `.from_aspect_locator(...)`
- `.from_aspect_field_locator(...)`
- `.from_aspect_contract_locator(...)`
- `.from_value_locator(...)`
- `.from_source_locator(...)`
- `.from_mismatch_locator(...)`
- `.from_transition_locator(...)`
- `.from_boundary_artifact_locator(...)`
- `.bundle(...)`

Lower lane:

- `prepare_aspect_contract_for_canonical_basis(...)`
- `prepare_aspect_mask_for_canonical_basis(...)`
- `prepare_aspect_patch_for_canonical_basis(...)`
- `prepare_aspect_state_for_canonical_basis(...)`
- `prepare_identity_for_canonical_basis(...)`
- `prepare_locator_for_canonical_basis(...)`
- `prepare_canonical_basis_bundle(...)`
- `CanonicalBasisReadyArtifact`
- `CanonicalBundleReadyArtifact`
- `CanonicalBasisEntry`
- `CanonicalBasisValue`
- `CanonicalBasisDomain`
- `CanonicalBasisConstructionDenial`
- `CanonicalizationRuleVersion`
- locator preparation includes value locators, source locators, mismatch
  locators, boundary-artifact locators, and transition locators

Good to know:

- the `digest_preparation` public surface is broader than hashing alone
- this page is about building canonical basis artifacts, not about final digest
  derivation APIs
- readiness proof matters here because basis preparation is supposed to freeze
  Milestone 1 meaning before Milestone 2 digest work builds on it

## Core Mental Model

Digest preparation answers one question:

- how do we turn aspect-native meaning into a canonical sequence of typed basis
  entries that downstream comparison and digest lanes can reuse?

That means this layer owns:

- stable ordering
- typed basis values
- explicit domain tags like contract, mask, patch, or state
- proof that the basis came from ready Milestone 1 surfaces

It does **not** own:

- contract authoring
- validation
- state admission
- final digest algorithm choice

Think of it as the "freeze meaning into canonical entries" step that comes
after aspect semantics are already decided.

## How It Executes

The normal flow is:

1. obtain a Milestone 1 artifact that already crossed the right authority
   boundary
2. choose the canonicalization rule version
3. prepare one canonical basis artifact from that contract, mask, patch, state,
   identity, or locator
4. optionally bundle multiple ready basis artifacts together
5. hand the ready basis into comparison, export, or digest lanes

The important rule is that you do not feed loose raw data into this lane. You
feed meaning-bearing Milestone 1 artifacts into it.

## Small Example

```rust
use forge_foundational::{canonicalization, CanonicalizationRuleVersion};

let version = CanonicalizationRuleVersion::new("m2.surface-basis")?;
let ready = canonicalization().basis().at(version).from_contract(contract);
```

This is the smallest honest example because it shows the public front door for
turning one authoritative semantic object into a canonical basis artifact.

## Real Example

```rust
use forge_foundational::{
    admit_authoritative_record_aspect_state, canonicalization, validate_aspect_value,
    AspectContract, AspectLocator, AspectValue, BoundarySourceLocator, CanonicalBasisDomain,
    CanonicalizationRuleVersion, LocatorAuthority, ScalarAspectType,
};
use forge_proof::TransitionOutcome;

let version = CanonicalizationRuleVersion::new("m2.surface-basis")?;

let contract = AspectContract::scalar(
    forge_foundational::AspectKey::new("count").expect("valid key"),
    forge_foundational::AspectIdentity(1),
    forge_foundational::AspectContractRevision(1),
    ScalarAspectType::Int64,
);

let TransitionOutcome::Success(validated) =
    validate_aspect_value(&contract, AspectValue::Int64(1).into())
else {
    panic!("expected validated value");
};

let TransitionOutcome::Success(state) = admit_authoritative_record_aspect_state([validated]) else {
    panic!("expected admitted state");
};

let TransitionOutcome::Success(contract_basis) =
    canonicalization().basis().at(version.clone()).from_contract(contract)
else {
    panic!("expected ready contract basis");
};

let TransitionOutcome::Success(state_basis) =
    canonicalization().basis().at(version).from_state(state)
else {
    panic!("expected ready state basis");
};

let TransitionOutcome::Success(locator_basis) = canonicalization()
    .basis()
    .at(CanonicalizationRuleVersion::new("m2.surface-basis")?)
    .from_source_locator(BoundarySourceLocator::aspect(AspectLocator::new(
        LocatorAuthority::SupportOnly,
        forge_foundational::AspectKey::new("count").expect("valid key"),
    )))
else {
    panic!("expected ready locator basis");
};

assert_ne!(contract_basis.payload().domain(), state_basis.payload().domain());
assert_eq!(locator_basis.payload().domain(), CanonicalBasisDomain::Locator);
```

What is authoritative here is the canonical basis artifact, not the caller's
local serialization idea. The contract and admitted state already own meaning;
digest preparation just freezes that meaning into one comparison-ready basis.

## How It Relates To Other Features

- [Validation And Authoritative State Admission](./validation-and-authoritative-state-admission.md)
  is the authority boundary that state preparation depends on.
- [Authoritative Patches And Apply Flow](./authoritative-patches-and-apply-flow.md)
  matters because patch preparation preserves set-versus-clear semantics.
- [Identities, Locators, And Blind-Consumer Addressing](./identities-locators-and-blind-consumer-addressing.md)
  matters because identities and locators can also be canonicalized as typed
  basis artifacts.
- [Milestone 1 Production Readiness](./milestone-1-production-readiness.md)
  freezes this surface as the public `digest_preparation` inventory item.

## Inspection And Debugging

Inspect these first:

- `CanonicalBasisDomain` when a basis artifact looks like it came from the
  wrong semantic lane
- `CanonicalBasisEntry` rows when ordering or equivalence feels suspicious
- `CanonicalBasisValue` when numeric widths, content identity, or field loci
  should differ but do not
- `CanonicalBasisConstructionDenial` when a surface is not ready to enter this
  lane

If canonical comparison or digest output looks wrong, confirm the basis entries
first. Many problems start before hashing.

## Anti-Patterns

- Do not build your own contract-or-state serialization for equality or digest
  input when this basis lane already exists.
- Do not feed raw values into digest preparation as if validation and admission
  were optional.
- Do not confuse canonical basis preparation with the final digest algorithm.
- Do not treat grouped `aspects()` or `compatibility()` front doors as if they
  already cover canonical basis work.

## Current Limits

- This layer prepares canonical basis artifacts. It does not itself define the
  final digest algorithm or receipt format.
- Basis preparation depends on ready Milestone 1 surfaces and intentionally
  rejects unsupported shortcuts.
- Canonical basis is the semantic handoff to later digest work, not a
  substitute for later milestone boundary artifacts.

## Related Docs

- [Validation And Authoritative State Admission](./validation-and-authoritative-state-admission.md)
- [Authoritative Patches And Apply Flow](./authoritative-patches-and-apply-flow.md)
- [Identities, Locators, And Blind-Consumer Addressing](./identities-locators-and-blind-consumer-addressing.md)
- [Milestone 1 Production Readiness](./milestone-1-production-readiness.md)
