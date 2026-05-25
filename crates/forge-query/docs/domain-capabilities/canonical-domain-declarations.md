# Canonical Domain Declarations

## What This Feature Is

Canonical domain declarations are the point where an admitted configured domain
handle is allowed to express declaration-local meaning through Query.

The important boundary is:

- the admitted configured handle carries the stable operating world
- the declaration input carries declaration-local meaning inside that world
- the declaration family's marker carries retained family identity and family
  posture plus later declaration-side contracts
- Query canonicalizes the combined meaning into one authoritative declaration
  artifact

That artifact is basis-first. Its digest is derived from the canonical basis
bundle; the digest is not the source of truth.

## Why You Use It

- declare domain-local intent from an admitted operating world
- freeze one canonical declaration identity before legality, routing, or
  continuation begin
- retain family identity and Query-owned family posture on the canonical
  declaration artifact
- get checked family admission before paying canonicalization work
- compare declarations under an explicit equivalence basis

## Stable Entry Points

- `ForgeQueryAdmittedConfiguredDomainHandle::declare(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_with_version(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::review_legality(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::review_legality_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_and_review(...)`
- `ForgeQueryDeclarationInput`
- `ForgeQueryDeclarationFamilyMarker`
- `ForgeQueryCanonicalDeclarationArtifact`
- `ForgeQueryCanonicalDeclarationArtifact::compare_under(...)`
- `ForgeQueryDeclarationCanonicalizationVersion`

Good to know:

- Query does not own your concrete family nouns
- the ordinary lane stays generic: `handle.declare(input)`
- family admission now happens before canonicalization
- the retained family marker meaning now includes later legality,
  progression, and route-contract posture
- raw declaration normalization stays behind Query-owned front doors

## API Reference

The main public declaration artifact surfaces are:

Declaration input authoring:
- `ForgeQueryDeclarationInput::canonical_declaration_entries() -> Vec<ForgeQueryDeclarationCanonicalEntry>`
- `ForgeQueryDeclarationCanonicalEntry::new(locus, kind, value) -> ForgeQueryDeclarationCanonicalEntry`
- `ForgeQueryDeclarationCanonicalEntry::text(locus, value) -> ForgeQueryDeclarationCanonicalEntry`
- `ForgeQueryDeclarationCanonicalEntry::locus() -> &str`
- `ForgeQueryDeclarationCanonicalEntry::kind() -> ForgeQueryDeclarationCanonicalEntryKind`
- `ForgeQueryDeclarationCanonicalEntry::value() -> &ForgeQueryDeclarationCanonicalValue`
- `ForgeQueryDeclarationCanonicalEntryKind::{Header, Shape, Value, Field, Identity}`
- `ForgeQueryDeclarationCanonicalValue::{Null, Bool, SignedInteger, UnsignedInteger, ExactText, DecimalText}`

Authoring:
- `declare(input) -> Result<ForgeQueryCanonicalDeclarationArtifact<D, I>, ForgeQueryDeclarationAdmissionError<D, I>>`
- `declare_checked(input) -> ForgeQueryDeclaredFamilyChecked<D, I>`
- `declare_with_version(input, version) -> Result<ForgeQueryCanonicalDeclarationArtifact<D, I>, ForgeQueryDeclarationAdmissionError<D, I>>`

Artifact inspection:
- `handle_identity_digest() -> &str`
- `declaration_family_key() -> &'static str`
- `declaration_taxonomy() -> ForgeQueryDeclarationFamilyTaxonomy`
- `declaration_primary_authority_family() -> ForgeQueryDeclarationPrimaryAuthorityFamily`
- `declaration_signal_compatibility() -> ForgeQuerySignalCompatibilityPosture`
- `declaration_grouped_posture() -> ForgeQueryGroupedDeclarationPosture`
- `canonical_basis_bundle() -> &CanonicalBundleReadyArtifact`
- `declaration_digest() -> &CanonicalDerivedDigest`
- `version() -> &ForgeQueryDeclarationCanonicalizationVersion`
- `canonicalization_version() -> &ForgeQueryDeclarationCanonicalizationVersion`

Comparison:
- `compare_under(other, basis) -> Result<ForgeQueryCanonicalDeclarationComparison, ForgeQueryDeclarationCanonicalizationError>`

Comparison result inspection:
- `outcome() -> &CanonicalComparisonOutcome`
- `equivalent_basis() -> Option<&CanonicalEquivalentBasis>`
- `mismatch_basis() -> Option<&CanonicalMismatchBasis>`
- `unsupported_basis() -> Option<&CanonicalMismatchBasis>`

Checked admission outcomes:
- `ForgeQueryDeclaredFamilyChecked::Admitted(ForgeQueryCanonicalDeclarationArtifact<D, I>)`
- `ForgeQueryDeclaredFamilyChecked::Deferred(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclaredFamilyChecked::Unsupported(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclaredFamilyChecked::InvalidContext(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclaredFamilyChecked::Canonicalization(ForgeQueryDeclarationCanonicalizationError)`

Admission error family:
- `ForgeQueryDeclarationAdmissionError::Deferred(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclarationAdmissionError::Unsupported(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclarationAdmissionError::InvalidContext(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclarationAdmissionError::Canonicalization(ForgeQueryDeclarationCanonicalizationError)`

Canonicalization version helpers:
- `ForgeQueryDeclarationCanonicalizationVersion::pinned_v1() -> ForgeQueryDeclarationCanonicalizationVersion`
- `ForgeQueryDeclarationCanonicalizationVersion::explicit(foundational) -> ForgeQueryDeclarationCanonicalizationVersion`
- `foundational() -> &CanonicalizationRuleVersion`

Witness access:
- `relational_truth() -> ForgeQueryRelationalTruthDeclaration<'_, D, I>`
- `bridge_continuation() -> ForgeQueryBridgeContinuationDeclaration<'_, D, I>`
- `signal_compatible() -> ForgeQuerySignalCompatibleDeclaration<'_, D, I>`
- `neighborhood_capable() -> ForgeQueryNeighborhoodCapableDeclaration<'_, D, I>`
- `batch_capable() -> ForgeQueryBatchCapableDeclaration<'_, D, I>`

The witness methods are only present when the declaration family's structural
posture admits them.

## Core Mental Model

Think of declaration work as four layers:

1. the handle proves the admitted operating world
2. the declaration input says what you mean inside that world
3. the family marker says what family this declaration belongs to and what kind
   of family it is
4. Query canonicalizes the combined meaning into one retained declaration
   artifact

That split matters because two declarations with identical local syntax are not
necessarily the same declaration if:

- the admitted operating world changed
- the family identity changed
- the family posture changed

Canonical declaration formation is also no longer unconditional. Query first
checks whether the admitted handle can admit the declaration family in the
current support/config posture. Only admitted families reach canonicalization.

## How It Executes

1. define a downstream declaration input type that implements
   `ForgeQueryDeclarationInput<YourDomain>`
2. define its associated family through `ForgeQueryDeclarationFamilyMarker`
3. start from an admitted configured handle
4. call `declare_checked(...)`, `declare(...)`, or `declare_with_version(...)`
5. Query evaluates family admission from:
   - admitted handle support snapshot
   - required capability families
   - required config sections
   - family posture carried by the family marker
6. if admitted, Query combines:
   - admitted handle identity
   - domain-scoped family identity
   - retained family taxonomy
   - declaration-local canonical components
7. Query lowers that meaning into one foundational canonical basis bundle
8. Query derives the declaration digest from the retained bundle

The ordinary lane uses the default canonicalization version for you. Use the
explicit version surface when you are doing proof work, certification, or
advanced tooling.

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "Geometry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CollaborativeWorld;

impl ForgeQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Relational]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdge;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdge {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeAtMidpoint {
    edge_ref: &'static str,
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeAtMidpoint {
    type Family = SplitEdge;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(CollaborativeWorld)
    .validate()?
    .admit()?;

let declaration = handle.declare(SplitEdgeAtMidpoint {
    edge_ref: "edge:42",
})?;

let digest = declaration.declaration_digest();
let truth = declaration.relational_truth();
```

## Real Example

```rust
use forge_foundational::facade::CanonicalEquivalenceBasis;
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCapabilityStatus, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclaredFamilyChecked,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.geometry"
    }

    fn display_name(&self) -> &'static str {
        "Worth Geometry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CollaborativeWorld;

impl ForgeQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Relational]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdge;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdge {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeAtMidpoint(&'static str);

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeAtMidpoint {
    type Family = SplitEdge;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.0)]
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(CollaborativeWorld)
    .validate()?
    .admit()?;

assert_eq!(
    handle.family_support::<SplitEdge>().declare_status(),
    ForgeQueryDeclarationCapabilityStatus::Admitted,
);

match handle.declare_checked(SplitEdgeAtMidpoint("edge:42")) {
    ForgeQueryDeclaredFamilyChecked::Admitted(left) => {
        let right = handle.declare(SplitEdgeAtMidpoint("edge:42"))?;
        let comparison =
            left.compare_under(&right, CanonicalEquivalenceBasis::ExactCanonicalBasis)?;
        let _truth = left.relational_truth();
        let _signal = left.signal_compatible();
        let _grouped = left.neighborhood_capable();
        assert!(comparison.outcome().is_equivalent());
    }
    other => panic!("expected admitted declaration, got {other:?}"),
}
```

What this example is showing:

- support admission and canonicalization are separate steps
- the canonical artifact retains family posture for later typed witness access
- later comparison still works over the retained canonical artifact

## How It Relates To Other Features

- [Platform Entry](./platform-entry.md) gives you the typed domain front door
- [Configured Domain Handles](./configured-domain-handles.md) give you the
  admitted operating world declaration work depends on
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md) defines the
  family identity and taxonomy retained on the artifact
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
  defines family support reports, checked admission, and structural witness
  surfaces
- [Declaration Legality](./declaration-legality.md) reviews the admitted
  canonical declaration for structural legality
- [Declaration Progression](./declaration-progression.md) turns the legality-cleared
  canonical declaration into one proof-bearing progression artifact or one typed
  progression outcome
- [Declaration Route Plans](./declaration-route-plan.md) turns admitted
  progression proof plus matching foundational evidence into one explicit
  lower-authority route set

## Inspection And Debugging

The main inspection surfaces are:

- `handle.family_support::<F>()`
- `handle.family_support_checked::<F>()`
- `handle.review_legality(...)`
- `handle.review_legality_checked(...)`
- `declaration_family_key()`
- `declaration_taxonomy()`
- `declaration_digest()`
- `handle_identity_digest()`
- `canonical_basis_bundle()`
- `canonicalization_version()`
- `compare_under(...)`

Use them to answer:

- whether a family is admitted before canonicalization
- whether two declarations differ because the admitted operating world changed
- whether a family posture mismatch is structural or support-dependent
- whether two authoring paths produced the same canonical declaration

## Anti-Patterns

- treating declaration authoring as unconditional after handle admission
- rebuilding family meaning from ad hoc strings after Query has retained it
- canonicalizing first and checking family support later
- hashing downstream declaration structs directly instead of using the retained
  canonical declaration artifact
- treating the digest as the authority instead of the canonical basis bundle

## Current Limits

Canonical domain declarations decide family admission and canonical declaration
identity. Structural legality is handled separately by
[Declaration Legality](./declaration-legality.md), and proof-bearing
declaration progression is handled separately by
[Declaration Progression](./declaration-progression.md). This feature still
does not decide:

- explicit lower-authority route planning
- Query boundary receipts
- Query boundary envelopes
- lower-authority routing into relational, bridge, or signal surfaces
- grouped execution semantics
- runtime continuation

This feature gives other Query declaration features one retained canonical
artifact plus one retained family admission boundary to build on.

## Related Docs

- [Platform Entry](./platform-entry.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
- [Declaration Legality](./declaration-legality.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Domain Capabilities](./README.md)
