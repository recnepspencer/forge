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

- `WorthQueryAdmittedConfiguredDomainHandle::declare(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::declare_checked(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::declare_with_version(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::review_legality(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::review_legality_checked(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::declare_and_review(...)`
- `WorthQueryDeclarationInput`
- `WorthQueryDeclarationFamilyMarker`
- `WorthQueryTemporalDeclarationClause`
- `WorthQueryTemporalDeclarationSupport`
- `WorthQueryTemporalDuration`
- `WorthQueryTemporalWindowKind`
- `WorthQueryCanonicalDeclarationArtifact`
- `WorthQueryCanonicalDeclarationArtifact::compare_under(...)`
- `WorthQueryDeclarationCanonicalizationVersion`

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
- `WorthQueryDeclarationInput::canonical_declaration_entries() -> Vec<WorthQueryDeclarationCanonicalEntry>`
- `WorthQueryDeclarationInput::async_resource_declaration_clauses() -> Vec<WorthQueryAsyncDeclarationClause>`
- `WorthQueryDeclarationInput::temporal_declaration_clauses() -> Vec<WorthQueryTemporalDeclarationClause>`
- `WorthQueryDeclarationCanonicalEntry::new(locus, kind, value) -> WorthQueryDeclarationCanonicalEntry`
- `WorthQueryDeclarationCanonicalEntry::text(locus, value) -> WorthQueryDeclarationCanonicalEntry`
- `WorthQueryDeclarationCanonicalEntry::locus() -> &str`
- `WorthQueryDeclarationCanonicalEntry::kind() -> WorthQueryDeclarationCanonicalEntryKind`
- `WorthQueryDeclarationCanonicalEntry::value() -> &WorthQueryDeclarationCanonicalValue`
- `WorthQueryDeclarationCanonicalEntryKind::{Header, Shape, Value, Field, Identity}`
- `WorthQueryDeclarationCanonicalValue::{Null, Bool, SignedInteger, UnsignedInteger, ExactText, DecimalText}`

Temporal declaration vocabulary:
- `WorthQueryTemporalDeclarationClause::stale_after(duration) -> WorthQueryTemporalDeclarationClause`
- `WorthQueryTemporalDeclarationClause::interval(every) -> WorthQueryTemporalDeclarationClause`
- `WorthQueryTemporalDeclarationClause::deadline(within) -> WorthQueryTemporalDeclarationClause`
- `WorthQueryTemporalDeclarationClause::rolling_window(width) -> WorthQueryTemporalDeclarationClause`
- `WorthQueryTemporalDeclarationClause::sliding_window(width, step) -> WorthQueryTemporalDeclarationClause`
- `WorthQueryTemporalDuration::milliseconds(value) -> WorthQueryTemporalDuration`
- `WorthQueryTemporalDuration::seconds(value) -> WorthQueryTemporalDuration`
- `WorthQueryTemporalDuration::minutes(value) -> WorthQueryTemporalDuration`
- `WorthQueryDeclarationFamilyMarker::temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport`

Async declaration vocabulary:
- `WorthQueryAsyncDeclarationClause::resource_request(source_family, loading_posture, failure_posture, request_identity) -> WorthQueryAsyncDeclarationClause`
- `WorthQueryAsyncDeclarationClause::completion_request(source_family, failure_posture, request_identity) -> WorthQueryAsyncDeclarationClause`
- `WorthQueryAsyncRequestIdentityPart::text(key, value) -> WorthQueryAsyncRequestIdentityPart`
- `WorthQueryAsyncSourceFamily::{BridgeResource, ExternalResource, HostResource}`
- `WorthQueryAsyncLoadingPosture::{Blocking, BackgroundRefresh}`
- `WorthQueryAsyncFailurePosture::{FailClosed, RetainStaleValue}`
- `WorthQueryDeclarationFamilyMarker::async_declaration_support() -> WorthQueryAsyncDeclarationSupport`

Authoring:
- `declare(input) -> Result<WorthQueryCanonicalDeclarationArtifact<D, I>, WorthQueryDeclarationAdmissionError<D, I>>`
- `declare_checked(input) -> WorthQueryDeclaredFamilyChecked<D, I>`
- `declare_with_version(input, version) -> Result<WorthQueryCanonicalDeclarationArtifact<D, I>, WorthQueryDeclarationAdmissionError<D, I>>`

Artifact inspection:
- `handle_identity_digest() -> &str`
- `declaration_family_key() -> &'static str`
- `declaration_taxonomy() -> WorthQueryDeclarationFamilyTaxonomy`
- `async_resource_clauses() -> &[WorthQueryAsyncDeclarationClause]`
- `temporal_clauses() -> &[WorthQueryTemporalDeclarationClause]`
- `declaration_primary_authority_family() -> WorthQueryDeclarationPrimaryAuthorityFamily`
- `declaration_signal_compatibility() -> WorthQuerySignalCompatibilityPosture`
- `declaration_grouped_posture() -> WorthQueryGroupedDeclarationPosture`
- `canonical_basis_bundle() -> &CanonicalBundleReadyArtifact`
- `declaration_digest() -> &CanonicalDerivedDigest`
- `version() -> &WorthQueryDeclarationCanonicalizationVersion`
- `canonicalization_version() -> &WorthQueryDeclarationCanonicalizationVersion`

Comparison:
- `compare_under(other, basis) -> Result<WorthQueryCanonicalDeclarationComparison, WorthQueryDeclarationCanonicalizationError>`

Comparison result inspection:
- `outcome() -> &CanonicalComparisonOutcome`
- `equivalent_basis() -> Option<&CanonicalEquivalentBasis>`
- `mismatch_basis() -> Option<&CanonicalMismatchBasis>`
- `unsupported_basis() -> Option<&CanonicalMismatchBasis>`

Checked admission outcomes:
- `WorthQueryDeclaredFamilyChecked::Admitted(WorthQueryCanonicalDeclarationArtifact<D, I>)`
- `WorthQueryDeclaredFamilyChecked::Deferred(WorthQueryDeclarationCapabilityDenial<D, I::Family>)`
- `WorthQueryDeclaredFamilyChecked::Unsupported(WorthQueryDeclarationCapabilityDenial<D, I::Family>)`
- `WorthQueryDeclaredFamilyChecked::InvalidContext(WorthQueryDeclarationCapabilityDenial<D, I::Family>)`
- `WorthQueryDeclaredFamilyChecked::AsyncDeferred(WorthQueryAsyncDeclarationDenial<D, I::Family>)`
- `WorthQueryDeclaredFamilyChecked::AsyncUnsupported(WorthQueryAsyncDeclarationDenial<D, I::Family>)`
- `WorthQueryDeclaredFamilyChecked::Canonicalization(WorthQueryDeclarationCanonicalizationError)`

Admission error family:
- `WorthQueryDeclarationAdmissionError::Deferred(WorthQueryDeclarationCapabilityDenial<D, I::Family>)`
- `WorthQueryDeclarationAdmissionError::Unsupported(WorthQueryDeclarationCapabilityDenial<D, I::Family>)`
- `WorthQueryDeclarationAdmissionError::InvalidContext(WorthQueryDeclarationCapabilityDenial<D, I::Family>)`
- `WorthQueryDeclarationAdmissionError::AsyncDeferred(WorthQueryAsyncDeclarationDenial<D, I::Family>)`
- `WorthQueryDeclarationAdmissionError::AsyncUnsupported(WorthQueryAsyncDeclarationDenial<D, I::Family>)`
- `WorthQueryDeclarationAdmissionError::Canonicalization(WorthQueryDeclarationCanonicalizationError)`

Canonicalization version helpers:
- `WorthQueryDeclarationCanonicalizationVersion::pinned_v1() -> WorthQueryDeclarationCanonicalizationVersion`
- `WorthQueryDeclarationCanonicalizationVersion::explicit(foundational) -> WorthQueryDeclarationCanonicalizationVersion`
- `foundational() -> &CanonicalizationRuleVersion`

Witness access:
- `relational_truth() -> WorthQueryRelationalTruthDeclaration<'_, D, I>`
- `bridge_continuation() -> WorthQueryBridgeContinuationDeclaration<'_, D, I>`
- `signal_compatible() -> WorthQuerySignalCompatibleDeclaration<'_, D, I>`
- `neighborhood_capable() -> WorthQueryNeighborhoodCapableDeclaration<'_, D, I>`
- `batch_capable() -> WorthQueryBatchCapableDeclaration<'_, D, I>`

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

Temporal meaning now lives in layer 2. If a declaration has a freshness limit,
an interval, a deadline, or a rolling or sliding window, that meaning belongs
to the declaration input itself. It is not observer metadata and not later
runtime setup.

Async/resource-backed request meaning now lives in layer 2 as well. If a
declaration depends on a canonical async source family, request identity,
loading posture, or failure posture, that meaning belongs to the declaration
input itself. It is not transport-adapter metadata and not a later view-model
status enum.

Families must opt into temporal clauses explicitly. A declaration that adds
temporal meaning to a family that never declared temporal support fails closed
at declaration admission instead of silently inheriting new behavior.

Families must opt into async/resource clauses explicitly too. A declaration
that adds async request meaning to a family that never declared async support
fails closed at declaration admission instead of silently inheriting fetch-like
behavior from an adapter.

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
   `WorthQueryDeclarationInput<YourDomain>`
2. define its associated family through `WorthQueryDeclarationFamilyMarker`
3. start from an admitted configured handle
4. call `declare_checked(...)`, `declare(...)`, or `declare_with_version(...)`
5. Query evaluates family admission from:
   - admitted handle support snapshot
   - required capability families
   - required config sections
   - family posture carried by the family marker
6. Query gathers declaration-local canonical entries, canonical async/resource
   clauses, and canonical temporal
   clauses from the same input
7. if admitted, Query combines:
   - admitted handle identity
   - domain-scoped family identity
   - retained family taxonomy
   - declaration-local canonical components
   - normalized temporal declaration clauses
8. Query lowers that meaning into one foundational canonical basis bundle
9. Query derives the declaration digest from the retained bundle

The ordinary lane uses the default canonicalization version for you. Use the
explicit version surface when you are doing proof work, certification, or
advanced tooling.

## Small Example

```rust
use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "Geometry"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CollaborativeWorld;

impl WorthQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Relational]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttachFaceMaterial;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AttachFaceMaterial {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "attach-face-material"
    }

    fn required_capability_families() -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AttachFaceMaterialAssignment {
    face_ref: &'static str,
    material_profile_ref: &'static str,
}

impl WorthQueryDeclarationInput<GeometryDomain> for AttachFaceMaterialAssignment {
    type Family = AttachFaceMaterial;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            WorthQueryDeclarationCanonicalEntry::text("face_ref", self.face_ref),
            WorthQueryDeclarationCanonicalEntry::text(
                "material_profile_ref",
                self.material_profile_ref,
            ),
        ]
    }
}

let query = WorthQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(CollaborativeWorld)
    .validate()?
    .admit()?;

let declaration = handle.declare(AttachFaceMaterialAssignment {
    face_ref: "face:loading-bay-west",
    material_profile_ref: "material-profile:fire-rated-primer",
})?;

let digest = declaration.declaration_digest();
let truth = declaration.relational_truth();
```

Canonical declaration entries are the retained internal shape, not the ideal
app-facing geometry DX. Other declaration-entry docs may show
dynamic context such as active selection or active neighborhood binding on the
outside while still lowering into canonical entries internally.

Here is the smallest temporal example:

```rust
use worth_query::facade::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationInput,
    WorthQueryTemporalDeclarationClause, WorthQueryTemporalDuration,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReadEdgeTemperature {
    edge_ref: &'static str,
}

impl WorthQueryDeclarationInput<GeometryDomain> for ReadEdgeTemperature {
    type Family = AttachFaceMaterial;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        vec![
            WorthQueryTemporalDeclarationClause::stale_after(
                WorthQueryTemporalDuration::seconds(30),
            ),
            WorthQueryTemporalDeclarationClause::rolling_window(
                WorthQueryTemporalDuration::minutes(5),
            ),
        ]
    }
}
```

Those temporal clauses become part of canonical declaration identity. A
different freshness limit or window width produces a different declaration.

## Real Example

```rust
use worth_foundational::facade::CanonicalEquivalenceBasis;
use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationCapabilityStatus, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclaredFamilyChecked,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.geometry"
    }

    fn display_name(&self) -> &'static str {
        "Worth Geometry"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CollaborativeWorld;

impl WorthQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Relational]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttachFaceMaterial;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AttachFaceMaterial {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "attach-face-material"
    }

    fn required_capability_families() -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AttachFaceMaterialForActiveSelection;

impl WorthQueryDeclarationInput<GeometryDomain> for AttachFaceMaterialForActiveSelection {
    type Family = AttachFaceMaterial;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            WorthQueryDeclarationCanonicalEntry::text(
                "selection_scope",
                "active-face-selection",
            ),
            WorthQueryDeclarationCanonicalEntry::text(
                "material_edit_intent",
                "attach-material-from-current-selection",
            ),
        ]
    }
}

let query = WorthQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(CollaborativeWorld)
    .validate()?
    .admit()?;

assert_eq!(
    handle.family_support::<AttachFaceMaterial>().declare_status(),
    WorthQueryDeclarationCapabilityStatus::Admitted,
);

match handle.declare_checked(AttachFaceMaterialForActiveSelection) {
    WorthQueryDeclaredFamilyChecked::Admitted(left) => {
        let right = handle.declare(AttachFaceMaterialForActiveSelection)?;
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
- the user-facing geometry story can still be active selection while the
  canonical artifact lowers into stable retained facts internally
- temporal meaning can be authored through typed clauses before canonicalization
- equivalent temporal forms lower to the same retained identity

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
- whether a family explicitly supports temporal declaration clauses
- whether two declarations differ because the admitted operating world changed
- whether a family posture mismatch is structural or support-dependent
- whether two authoring paths produced the same canonical declaration
- whether a freshness or window change actually changed declaration identity

## Anti-Patterns

- treating declaration authoring as unconditional after handle admission
- rebuilding family meaning from ad hoc strings after Query has retained it
- canonicalizing first and checking family support later
- hashing downstream declaration structs directly instead of using the retained
  canonical declaration artifact
- treating the digest as the authority instead of the canonical basis bundle
- attaching freshness or interval behavior after canonicalization through a
  watcher or observer option instead of the declaration input itself

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
- lower-authority relational truth routing
- lower-authority bridge continuation routing
- lower-authority signal compatibility classification
- grouped execution semantics
- runtime continuation
- timer scheduling
- wake delivery
- mixed-cause execution
- async completion participation

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
- [Async Resources And Result State](../capabilities/async-resources-and-result-state.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Domain Capabilities](./README.md)
