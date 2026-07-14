# Declaration Foundational Evidence

## What This Feature Is

Declaration foundational evidence is the Query-owned boundary that lowers
retained declaration truth into shared `worth-foundational` evidence artifacts.

The important split is:

- legality review proves structural declaration truth
- declaration progression proves admitted, deferred, denied, stale, rebind, or
  failed progression truth
- foundational evidence describes that retained truth through provenance,
  support, receipts, and attachment bundles without re-deriving declaration
  meaning

This is the first declaration-side feature whose job is descriptive publication
rather than stronger admission or stronger legality/progression proof.

## Why You Use It

- describe declaration truth through shared foundational evidence artifacts
- preserve admitted-world identity, declaration identity, legality truth, and
  progression truth in one inspectable wrapper
- keep denied, deferred, stale, rebind-required, and failed declaration paths
  first-class instead of success-only
- hand route planning, receipts, and envelopes one stable evidence
  language instead of local report dialects

## Stable Entry Points

- `WorthQueryDeclarationFoundationalEvidenceInput`
- `WorthQueryDeclarationFoundationalEvidenceClass`
- `WorthQueryDeclarationFoundationalEvidenceChecked`
- `WorthQueryDeclarationFoundationalEvidence`
- `WorthQueryDeclarationFoundationalEvidenceDenial`
- `WorthQueryAdmittedConfiguredDomainHandle::describe_foundational(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::describe_foundational_checked(...)`
- `WorthQueryAdmittedConfiguredDomainHandle::describe_foundational_with_profile(...)`

Good to know:

- the admitted handle stays the entry surface because foundational description
  still depends on retained admitted-world proof
- later retained declaration-side artifacts compose the same
  `WorthQueryAdmittedWorldBasis` witness rather than re-retaining raw admitted
  handle and operating-context digests independently
- foundational evidence starts from legality or progression truth, not from raw
  declaration input or canonical declarations alone
- Query owns the declaration-side lowering boundary; `worth-foundational` owns
  the provenance, support, receipt, and bundle primitives

## API Reference

Evidence-input constructors:

- `WorthQueryDeclarationFoundationalEvidenceInput::legality_evidence(evidence)`
- `WorthQueryDeclarationFoundationalEvidenceInput::legality_denial(denial)`
- `WorthQueryDeclarationFoundationalEvidenceInput::legality_checked(checked)`
- `WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed)`
- `WorthQueryDeclarationFoundationalEvidenceInput::progression_checked(checked)`

Admitted-handle evidence entry points:

- `describe_foundational(subject) -> Result<WorthQueryDeclarationFoundationalEvidence<D, I>, WorthQueryDeclarationFoundationalEvidenceDenial<D, I>>`
- `describe_foundational_checked(subject) -> WorthQueryDeclarationFoundationalEvidenceChecked<D, I>`
- `describe_foundational_with_profile(subject, profile) -> Result<WorthQueryDeclarationFoundationalEvidence<D, I>, WorthQueryDeclarationFoundationalEvidenceDenial<D, I>>`

Evidence classes:

- `WorthQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted`
- `WorthQueryDeclarationFoundationalEvidenceClass::LegalityDenied`
- `WorthQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted`
- `WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred`
- `WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDenied`
- `WorthQueryDeclarationFoundationalEvidenceClass::ProgressionStale`
- `WorthQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired`
- `WorthQueryDeclarationFoundationalEvidenceClass::ProgressionFailed`

Checked evidence outcomes:

- `WorthQueryDeclarationFoundationalEvidenceChecked::Described(WorthQueryDeclarationFoundationalEvidence<D, I>)`
- `WorthQueryDeclarationFoundationalEvidenceChecked::ConstructionDenied(WorthQueryDeclarationFoundationalEvidenceDenial<D, I>)`

Foundational evidence inspection:

- `class() -> WorthQueryDeclarationFoundationalEvidenceClass`
- `handle_identity_digest() -> &str`
- `operating_context_identity_digest() -> &str`
- `declaration_family_key() -> &'static str`
- `declaration_digest() -> &str`
- `support_digest() -> &str`
- `legality_digest() -> Option<&str>`
- `progression_digest() -> Option<&str>`
- `provenance() -> &FoundationalBoundaryEvidenceProvenanceArtifact`
- `planning_receipt() -> Option<&FoundationalBoundaryEvidencePlanningReceiptArtifact>`
- `receipt() -> Option<&FoundationalBoundaryEvidenceCompletedReceiptArtifact>`
- `support_attachment() -> Option<&FoundationalBoundaryEvidenceSupportAttachment>`
- `legality_contract() -> WorthQueryDeclarationLegalityContract`
- `progression_contract() -> Option<WorthQueryDeclarationProgressionContract>`
- `attachment_bundle() -> &FoundationalMaterializedBoundaryEvidenceAttachmentBundle`
- `attachment_bundle_digest() -> &CanonicalDerivedDigest`
- `materialization_profile() -> FoundationalBoundaryEvidenceMaterializationProfile`
- `aspect_contract() -> &WorthQueryDeclarationAspectContract`
- `aspect_coverage() -> &WorthQueryDeclarationAspectCoverage`
- `aspect_coverage_basis() -> WorthQueryDeclarationAspectCoverageBasis`
- `aspect_publication() -> &WorthQueryDeclarationAspectPublication`
- `subject() -> &WorthQueryDeclarationFoundationalEvidenceInput<D, I>`

Construction-denial inspection:

- `class() -> WorthQueryDeclarationFoundationalEvidenceClass`
- `reason() -> &str`

Construction-denial variants:

- `WrongAdmittedWorld`
- `Provenance`
- `Support`
- `AttachmentCanonicalBasis`
- `AttachmentDigest`

Materialization profiles:

- `FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness`
- `FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics`
- `FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics`

Aspect-publication inspection:

- `aspect_publication().present()`
- `aspect_publication().widened()`
- `aspect_publication().elided()`
- `aspect_publication().masked()`

## Core Mental Model

Think of foundational evidence as a publication boundary over retained truth:

1. configured handles retain admitted-world identity
2. canonical declarations retain declaration identity and family posture
3. legality and progression retain stronger declaration truth
4. foundational evidence lowers that retained truth into one shared evidence
   language without reopening support, taxonomy, or progression meaning

If two declarations really mean the same thing at the retained-proof level, the
foundational evidence should converge. If their retained truth differs, the
foundational evidence should stay observably different.

## How It Executes

1. start from one legality or progression artifact
2. wrap it in `WorthQueryDeclarationFoundationalEvidenceInput`
3. call one of the admitted-handle description entry points
4. Query injects the retained:
   - handle identity digest
   - operating-context identity digest
   - declaration digest
   - support digest
   - legality digest when present
   - progression digest when present
5. Query lowers the retained truth into:
   - foundational provenance
   - foundational support attachment
   - foundational planning or completed receipt
   - one materialized attachment bundle and bundle digest
6. Query returns one evidence wrapper or one typed construction denial

`describe_foundational(...)` uses the default full descriptive bundle for this
direct feature. Use `describe_foundational_with_profile(...)` when you want the
same retained truth with a leaner materialized bundle or when another surface,
such as declaration-entry orchestration, has already chosen the lean
foundational default for you.

Those profiles now carry explicit semantic
publication meaning:

- `ElideSupportAndDiagnostics` publishes only the required semantic slices that
  were visibly covered
- `ElideDiagnostics` may widen publication to preserved support-relevant slices
- `FullDescriptiveRichness` may widen publication again to intentionally
  published descriptive slices
- masked or conflicting slices stay masked rather than silently counting as
  present

`aspect_coverage_basis()` keeps that publication honest:

- `ReviewedRetainedCoverage` means the evidence is publishing slices backed by
  legality/progression review
- `SupportReportedCoverage` means the evidence is publishing support-reported
  coverage on a denied legality path rather than pretending review succeeded

That distinction matters in practice:

- admitted progression and legality-admitted evidence can publish reviewed
  retained coverage
- legality-denied evidence may still describe support-reported slices, but it
  must not claim the same review basis as retained admitted proof
- conflicting slices remain masked even on full descriptive publication rather
  than being upgraded into visible truth

## Small Example

```rust
use worth_query::facade::foundation::{
    WorthQueryDeclarationFoundationalEvidenceChecked,
    WorthQueryDeclarationFoundationalEvidenceInput,
};

match handle.describe_foundational_checked(
    WorthQueryDeclarationFoundationalEvidenceInput::progression_checked(
        handle.progress_declaration_checked(
            handle.declare_and_review(
                geometry_session.attach_face_material_for_active_selection()?,
            )?,
        ),
    ),
) {
    WorthQueryDeclarationFoundationalEvidenceChecked::Described(evidence) => {
        assert_eq!(evidence.declaration_family_key(), "attach-face-material");
    }
    WorthQueryDeclarationFoundationalEvidenceChecked::ConstructionDenied(denial) => {
        panic!("unexpected evidence denial: {}", denial.reason());
    }
}
```

## Real Example

This is a low-level retained-shape example. The app-facing DX should still be
dynamic context such as "attach material for the active face selection." The
explicit refs below are just the canonical declaration content Query retains
after your session already decided what that intent means.

```rust
use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;
use worth_query::facade::foundation::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationFoundationalEvidenceClass,
    WorthQueryDeclarationFoundationalEvidenceInput, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationProgressionContract,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "worth.geometry" }
    fn display_name(&self) -> &'static str { "Worth Geometry" }

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

    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> WorthQueryDeclarationProgressionContract {
        WorthQueryDeclarationProgressionContract::admitted_current()
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

let progressed = handle.declare_review_and_progress(AttachFaceMaterialAssignment {
    face_ref: "face:loading-bay-west",
    material_profile_ref: "material-profile:fire-rated-primer",
})?;

let evidence = handle.describe_foundational(
    WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed),
)?;

assert_eq!(
    evidence.class(),
    WorthQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted,
);
assert_eq!(evidence.declaration_family_key(), "attach-face-material");
assert_eq!(evidence.operating_context_identity_digest(), "geometry.collaborative");
assert!(evidence.progression_digest().is_some());
assert!(evidence.support_attachment().is_some());
assert!(evidence.receipt().is_some());

let lean = handle.describe_foundational_with_profile(
    WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
        handle.declare_review_and_progress(AttachFaceMaterialAssignment {
            face_ref: "face:loading-bay-west",
            material_profile_ref: "material-profile:fire-rated-primer",
        })?,
    ),
    FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
)?;

assert_eq!(
    lean.materialization_profile(),
    FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
);
```

What this example is showing:

- the foundational-evidence lane starts from retained progression truth
- the evidence wrapper preserves admitted-world identity and declaration truth
- materialization profile changes the bundle shape without changing the retained
  declaration truth being described
- `aspect_publication()` is where later route and orchestration surfaces learn
  which semantic slices were actually present, widened, elided, or masked

## Aspect Semantics

Foundational evidence is the first publication-oriented aspect
surface in the declaration-entry pipeline. Profiles such as lean,
support-ready, and richer publication are no longer allowed to mean only
"different descriptive richness." They must also describe which semantic
slices are present, widened, elided, or masked so later route/materialization/
orchestration surfaces can stay semantically honest without rediscovering that
publication breadth themselves.

## How It Relates To Other Features

- [Configured Domain Handles](./configured-domain-handles.md) retain the
  admitted-world identity evidence description must carry forward
- [Declaration Legality](./declaration-legality.md) supplies legality evidence
  and legality denials that can be described directly
- [Declaration Progression](./declaration-progression.md) supplies admitted,
  deferred, denied, stale, rebind, and failed progression truths
- [Declaration Route Plans](./declaration-route-plan.md) consumes admitted
  progression plus matching foundational evidence and turns that retained
  declaration truth into one explicit route set
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
  records the public crossing posture that follows from retained route truth
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
  publish the one self-describing public artifact that carries retained
  evidence, route truth, and receipt truth together

## Inspection And Debugging

Use these surfaces when reviewing foundational evidence:

- `describe_foundational_checked(...)`
- `evidence.class()`
- `evidence.handle_identity_digest()`
- `evidence.operating_context_identity_digest()`
- `evidence.declaration_digest()`
- `evidence.support_digest()`
- `evidence.legality_digest()`
- `evidence.progression_digest()`
- `evidence.provenance()`
- `evidence.planning_receipt()`
- `evidence.receipt()`
- `evidence.support_attachment()`
- `evidence.attachment_bundle_digest()`
- `evidence.aspect_contract()`
- `evidence.aspect_coverage()`
- `evidence.aspect_coverage_basis()`
- `evidence.aspect_publication()`

Use them to answer:

- whether the declaration truth being described is legality-admitted, denied,
  progression-admitted, deferred, stale, rebind-required, or failed
- whether two equivalent declaration paths converge to the same foundational
  evidence bundle digest
- whether a difference came from world identity, legality truth, or progression
  outcome rather than from formatting or local callback behavior
- whether lean, support-ready, and full-descriptive publication changed the
  semantic slices honestly without changing the retained declaration truth

## Anti-Patterns

- attempting foundational description from raw declaration input
- attempting foundational description from canonical declarations alone
- rebuilding foundational evidence meaning from family keys or payload shape
- treating foundational descriptive receipts as if they were later Query
  boundary-crossing receipts
- using stale evidence as if it were fresh admitted progression

## Current Limits

Declaration foundational evidence now publishes legality and progression truth
through shared foundational artifacts. It still does not decide:

- route planning
- lower-authority boundary crossing
- public Query boundary receipts
- public Query envelopes
- public Query relational truth routing
- public Query bridge continuation routing
- public Query signal compatibility classification
- continuity or diagnostic attachments

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Legality](./declaration-legality.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Domain Capabilities](./README.md)
