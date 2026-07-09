# Declaration Legality

## What This Feature Is

Declaration legality is the Query-owned structural review that happens after a
family is admitted and after the canonical declaration artifact exists.

The important split is:

- family admission decides whether the admitted operating world can admit the
  family at all
- legality review decides whether that already admitted family declaration is
  structurally legal inside that admitted operating world

This keeps support/config denial separate from legality denial.

## Why You Use It

- review a canonical declaration for legality without reopening family
  classification or support gating
- carry one legality evidence artifact into later progression and routing work
- carry one legality evidence artifact into declaration progression without
  redoing support or classification
- distinguish admission denial from legality denial in a typed way
- reuse foundational legality vocabulary through Query-owned artifacts

## Stable Entry Points

- `WorthQueryDeclarationLegalityContract`
- `WorthQueryAdmittedConfiguredDomainHandle::review_legality(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::review_legality_checked(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::declare_and_review(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::declare_review_and_progress(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::describe_foundational(...)`
- `WorthQueryDeclarationLegalityChecked`
- `WorthQueryDeclarationLegalityDenial`
- `WorthQueryDeclarationLegalityEvidence`
- `WorthQueryDeclarationAdmissionOrLegalityError`

Good to know:

- legality review stays anchored on the admitted handle
- canonical declarations do not expose direct legality-review methods
- the family marker now declares one explicit `legality_contract()`
- legality does not rerun family support gating
- concrete temporal declarations can now fail legality before progression when
  their retained time-aware meaning would require unsupported temporal runtime
  support or would collapse temporal execution into preview or historical truth
  basis
- concrete async/resource-backed declarations can now fail legality before
  progression when their retained source family, loading/failure posture, or
  preview/historical basis posture would require unsupported async runtime
  support or unsupported lifecycle/source admission
- route planning does not consume legality evidence directly on the ordinary
  success lane; it consumes admitted progression plus matching foundational
  evidence

## API Reference

The main public legality surfaces are:

Family contract:
- `legality_contract() -> WorthQueryDeclarationLegalityContract`

Legality contract class:
- `WorthQueryDeclarationLegalityClass`

Admitted-handle legality entry points:
- `review_legality(declaration) -> Result<WorthQueryDeclarationLegalityEvidence<D, I>, WorthQueryDeclarationLegalityDenial<D, I>>`
- `review_legality_checked(declaration) -> WorthQueryDeclarationLegalityChecked<D, I>`
- `declare_and_review(input) -> Result<WorthQueryDeclarationLegalityEvidence<D, I>, WorthQueryDeclarationAdmissionOrLegalityError<D, I>>`

Checked legality outcomes:
- `WorthQueryDeclarationLegalityChecked::Legal(WorthQueryDeclarationLegalityEvidence<D, I>)`
- `WorthQueryDeclarationLegalityChecked::Illegal(WorthQueryDeclarationLegalityDenial<D, I>)`

Combined declaration outcome:
- `WorthQueryDeclarationAdmissionOrLegalityError::Admission(WorthQueryDeclarationAdmissionError<D, I>)`
- `WorthQueryDeclarationAdmissionOrLegalityError::Legality(WorthQueryDeclarationLegalityDenial<D, I>)`

Progression convenience that consumes legality:
- `declare_review_and_progress(input) -> Result<WorthQueryAdmittedDeclarationProgression<D, I>, WorthQueryDeclarationEntryProgressionError<D, I>>`

Foundational-evidence entry that can consume legality truth:
- `describe_foundational(subject) -> Result<WorthQueryDeclarationFoundationalEvidence<D, I>, WorthQueryDeclarationFoundationalEvidenceDenial<D, I>>`
- `describe_foundational_checked(subject) -> WorthQueryDeclarationFoundationalEvidenceChecked<D, I>`
- `describe_foundational_with_profile(subject, profile) -> Result<WorthQueryDeclarationFoundationalEvidence<D, I>, WorthQueryDeclarationFoundationalEvidenceDenial<D, I>>`

Legality evidence inspection:
- `canonical_declaration() -> &WorthQueryCanonicalDeclarationArtifact<D, I>`
- `support_report() -> &WorthQueryDeclarationFamilySupportReport<D, I::Family>`
- `legality_contract() -> WorthQueryDeclarationLegalityContract`
- `aspect_contract() -> &WorthQueryDeclarationAspectContract`
- `reviewed_aspect_coverage() -> &WorthQueryDeclarationAspectCoverage`
- `declaration_family_key() -> &'static str`
- `operating_context_identity_digest() -> &str`
- `role_claim_category() -> FoundationalBoundaryArtifactCategory`
- `role_claim_role() -> FoundationalBoundaryArtifactRole`
- `surface_disposition() -> FoundationalBoundarySurfaceDispositionLegality`
- `legality_digest() -> &str`
- `is_structurally_legal() -> bool`

Legality denial inspection:
- `canonical_declaration() -> &WorthQueryCanonicalDeclarationArtifact<D, I>`
- `declaration_family_key() -> &'static str`
- `handle_identity_digest() -> &str`
- `operating_context_identity_digest() -> &str`
- `declaration_digest() -> String`
- `support_report() -> &WorthQueryDeclarationFamilySupportReport<D, I::Family>`
- `legality_contract() -> WorthQueryDeclarationLegalityContract`
- `capability_status() -> WorthQueryDeclarationCapabilityStatus`

Legality denial variants:
- `WrongAdmittedWorld`
- `IllegalRoleClaim`
- `IllegalSurfaceDisposition`
- `DeferredByLegalityBoundary`
- `UnsupportedLegalityClass`
- `TemporalProjectionUnsupported`
- `AsyncProjectionUnsupported`

Temporal legality denial inspection:
- `temporal_denial_kind() -> Option<WorthQueryTemporalLegalityDenialKind>`

Async legality denial inspection:
- `async_denial_kind() -> Option<WorthQueryAsyncLegalityDenialKind>`

Temporal legality denial kinds:
- `RuntimeFacadeDeferred`
- `RuntimeFacadeUnsupported`
- `HistoricalTruthBasisUnsupported`
- `PreviewTruthBasisUnsupported`
- `HistoricalSignalBasisUnsupported`
- `PreviewSignalBasisUnsupported`

Async legality denial kinds:
- `RuntimeFacadeDeferred`
- `RuntimeFacadeUnsupported`
- `UnsupportedSourceFamily`
- `UnsupportedLoadingPosture`
- `UnsupportedFailurePosture`
- `CompletionLifecycleUnsupported`
- `HistoricalTruthBasisUnsupported`
- `PreviewTruthBasisUnsupported`
- `HistoricalSignalBasisUnsupported`
- `PreviewSignalBasisUnsupported`

Legality contract presets:
- `authoritative_hot_artifact() -> WorthQueryDeclarationLegalityContract`
- `descriptive_deferred_support() -> WorthQueryDeclarationLegalityContract`
- `planned_unavailable_support() -> WorthQueryDeclarationLegalityContract`
- `receipt_hot_boundary() -> WorthQueryDeclarationLegalityContract`
- `deferred_boundary() -> WorthQueryDeclarationLegalityContract`
- `unsupported_boundary() -> WorthQueryDeclarationLegalityContract`

Legality contract inspection:
- `legality_class() -> WorthQueryDeclarationLegalityClass`
- `category() -> FoundationalBoundaryArtifactCategory`
- `role() -> FoundationalBoundaryArtifactRole`
- `delivery_class() -> FoundationalBoundaryDeliveryClass`
- `availability() -> FoundationalBoundaryAvailability`

## Core Mental Model

Think of legality as the first declaration-side proof after canonicalization:

1. the admitted handle proves the operating world
2. the canonical declaration proves declaration identity and family posture
3. the family marker contributes one Query-owned legality contract
4. Query reviews that declaration against foundational role and surface
   legality rules
5. if the declaration carries temporal clauses, Query also verifies that the
   retained time-aware meaning does not depend on unsupported temporal runtime
   support and does not collapse clock-driven execution into preview-derived or
   historical truth basis
6. if the declaration carries async/resource clauses, Query also verifies that
   the retained source family, lifecycle posture, and basis posture do not
   depend on unsupported async runtime support or unsupported preview or
   historical basis projection

If legality passes, you get one evidence artifact for other declaration-side
features. If it fails, you get one typed legality denial instead of a generic
rejection.

## How It Executes

1. define `legality_contract()` on the family marker
2. admit the declaration family through `declare(...)` or `declare_checked(...)`
3. call `review_legality(...)` or `review_legality_checked(...)`
4. Query combines:
   - admitted handle identity
   - canonical declaration artifact
   - admitted family support report
   - the family legality contract
   - the retained aspect coverage carried by that family support report
5. Query evaluates:
   - boundary role claim legality
   - boundary surface disposition legality
   - temporal legality guards for retained time-aware declarations
   - async legality guards for retained source/lifecycle-aware declarations
6. Query returns legality evidence or a typed legality denial

The convenience lane `declare_and_review(...)` preserves the same split. It
still performs admission first and legality second.

Important:

- legality now carries semantic aspect coverage, not raw declaration field names
- if a declaration uses a canonical field like `face_ref`, that remains part of
  canonical declaration identity, but the legality proof handed to later
  consumers
  is the semantic slice coverage retained from family support
- reviewed aspect coverage is allowed to preserve masked slices explicitly; a
  legality success does not silently promote masked support into visible
  semantic presence
- temporal legality is about retained declaration meaning, not about clocks or
  wake execution; the lower runtime still owns actual scheduling and wake
  delivery later
- async legality is about retained declaration meaning, not about pending
  state, callback arrival, retries, or completion causality; those remain later
  runtime-owned surfaces

## Small Example

```rust
use worth_query::facade::{
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationLegalityChecked,
};

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AttachFaceMaterial {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "attach-face-material"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

match handle.review_legality_checked(handle.declare(AttachMaterialForActiveFaceSelection)?) {
    WorthQueryDeclarationLegalityChecked::Legal(legal) => {
        assert!(legal.is_structurally_legal());
    }
    WorthQueryDeclarationLegalityChecked::Illegal(denial) => {
        panic!("unexpected legality denial: {:?}", std::mem::discriminant(&denial));
    }
}
```

## Real Example

```rust
use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationAdmissionOrLegalityError,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
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

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AttachMaterialForActiveFaceSelection;

impl WorthQueryDeclarationInput<GeometryDomain> for AttachMaterialForActiveFaceSelection {
    type Family = AttachFaceMaterial;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            WorthQueryDeclarationCanonicalEntry::text(
                "selection_scope",
                "active-face-selection",
            ),
            WorthQueryDeclarationCanonicalEntry::text(
                "legality_focus",
                "current-selection-material-attachment",
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

match handle.declare_and_review(AttachMaterialForActiveFaceSelection) {
    Ok(legal) => {
        assert_eq!(legal.declaration_family_key(), "attach-face-material");
        assert!(legal.is_structurally_legal());
    }
    Err(WorthQueryDeclarationAdmissionOrLegalityError::Admission(admission)) => {
        let _ = admission;
    }
    Err(WorthQueryDeclarationAdmissionOrLegalityError::Legality(denial)) => {
        let _ = denial;
    }
}
```

What this example is showing:

- family admission and legality are distinct typed steps
- the family marker now carries the legality contract as part of the live
  declaration boundary
- the legality evidence artifact retains both the canonical declaration and the
  legality contract
- the primary geometry mental model is active context, while the canonicalized
  declaration entries are internal retained evidence

## Aspect Semantics

Legality records which semantic slices were actually
reviewed. A future legality success is no longer allowed to mean only "the
broad declaration artifact passed." It must also say which aspect-qualified
structural scope was cleared so progression and later route/binding surfaces do
not over-trust a coarse legality success.

## How It Relates To Other Features

- [Canonical Domain Declarations](./canonical-domain-declarations.md) produce
  the canonical declaration artifact legality consumes
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md) freezes the
  retained family posture legality must not rediscover from payloads
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
  decides family admission before legality begins
- [Declaration Progression](./declaration-progression.md) consumes legality
  evidence and preserves progression truth for later declaration-side features
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
  can describe either legality evidence or legality denial directly without
  forcing a progression step first
- [Declaration Route Plans](./declaration-route-plan.md) begins after admitted
  progression and matching foundational evidence rather than consuming legality
  evidence directly

## Inspection And Debugging

Use these surfaces when reviewing legality:

- `review_legality_checked(...)`
- `declare_and_review(...)`
- `legal.canonical_declaration()`
- `legal.support_report()`
- `legal.legality_contract()`
- `legal.legality_digest()`
- `denial.support_report()`
- `denial.legality_contract()`

Use them to answer:

- whether the declaration failed at admission or legality
- whether the denial came from role claim or surface disposition
- whether two equivalent declarations converged to the same legality digest

## Anti-Patterns

- treating legality review as if it were another family-support query
- rebuilding legality meaning from taxonomy or payload shape instead of the
  explicit legality contract
- attaching legality review directly to the canonical declaration artifact
- collapsing admission denial and legality denial into one generic failure lane

## Current Limits

Declaration legality now proves structural legality inside one admitted
operating world. Declaration progression is handled separately by
[Declaration Progression](./declaration-progression.md), and foundational
description is handled separately by
[Declaration Foundational Evidence](./declaration-foundational-evidence.md).
This feature still does not decide:

- proof progression
- foundational evidence materialization
- lower-authority route planning
- Query boundary receipts
- Query boundary envelopes
- Query relational truth routing
- Query bridge continuation routing
- grouped execution semantics
- continuation execution

## Related Docs

- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Configured Domain Handles](./configured-domain-handles.md)
