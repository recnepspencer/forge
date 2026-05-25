use forge_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
    FoundationalBoundaryAvailability, FoundationalBoundaryDeliveryClass,
};

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationAdmissionOrLegalityError, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCapabilityStatus, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityChecked,
    ForgeQueryDeclarationLegalityClass, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.legality"
    }

    fn display_name(&self) -> &'static str {
        "GeometryLegalityDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CollaborativeWorld {
    regime: &'static str,
}

impl CollaborativeWorld {
    fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.{}", self.regime)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LegalFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for LegalFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct IllegalRoleFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for IllegalRoleFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::new(
            ForgeQueryDeclarationLegalityClass::AuthoritativeHotArtifact,
            FoundationalBoundaryArtifactCategory::Summary,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
            FoundationalBoundaryDeliveryClass::MustBeHot,
            FoundationalBoundaryAvailability::Present,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct IllegalDispositionFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for IllegalDispositionFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::new(
            ForgeQueryDeclarationLegalityClass::AuthoritativeHotArtifact,
            FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
            FoundationalBoundaryDeliveryClass::MustBeHot,
            FoundationalBoundaryAvailability::Deferred,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DeferredLegalityFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for DeferredLegalityFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::deferred_boundary()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DurableAdmissionFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for DurableAdmissionFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::DurableArtifacts]
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Declaration<F> {
    edge_ref: &'static str,
    _family: std::marker::PhantomData<F>,
}

impl<F> Declaration<F> {
    fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _family: std::marker::PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for Declaration<LegalFamily> {
    type Family = LegalFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for Declaration<IllegalRoleFamily> {
    type Family = IllegalRoleFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for Declaration<IllegalDispositionFamily> {
    type Family = IllegalDispositionFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for Declaration<DeferredLegalityFamily> {
    type Family = DeferredLegalityFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for Declaration<DurableAdmissionFamily> {
    type Family = DurableAdmissionFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}

fn admitted_handle(
    regime: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, CollaborativeWorld>
{
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(CollaborativeWorld::named(regime))
        .validate()
        .expect("context should validate")
        .admit()
        .expect("context should admit")
}

#[test]
fn legal_declaration_review_yields_legality_evidence() {
    let handle = admitted_handle("collaborative");
    let declaration = handle
        .declare(Declaration::<LegalFamily>::new("edge:42"))
        .expect("declaration should admit");

    let legal = handle
        .review_legality(declaration)
        .expect("legality review should pass");

    assert_eq!(legal.declaration_family_key(), "split-edge");
    assert!(legal.is_structurally_legal());
    assert_eq!(
        legal.legality_contract(),
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    );
    assert_eq!(
        legal.support_report().declare_status(),
        ForgeQueryDeclarationCapabilityStatus::Admitted
    );
    assert_eq!(
        legal.operating_context_identity_digest(),
        "geometry.collaborative"
    );
}

#[test]
fn legality_review_rejects_declarations_from_a_different_admitted_world() {
    let left = admitted_handle("collaborative");
    let right = admitted_handle("restricted");
    let declaration = left
        .declare(Declaration::<LegalFamily>::new("edge:42"))
        .expect("declaration should admit");

    match right.review_legality_checked(declaration) {
        ForgeQueryDeclarationLegalityChecked::Illegal(
            crate::application::ForgeQueryDeclarationLegalityDenial::WrongAdmittedWorld { .. },
        ) => {}
        other => panic!(
            "expected wrong-world denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn legality_review_distinguishes_role_and_surface_failures() {
    let handle = admitted_handle("collaborative");

    let bad_role = handle
        .declare(Declaration::<IllegalRoleFamily>::new("edge:42"))
        .expect("declaration should admit");
    match handle.review_legality_checked(bad_role) {
        ForgeQueryDeclarationLegalityChecked::Illegal(
            crate::application::ForgeQueryDeclarationLegalityDenial::IllegalRoleClaim { .. },
        ) => {}
        other => panic!(
            "expected role denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }

    let bad_surface = handle
        .declare(Declaration::<IllegalDispositionFamily>::new("edge:42"))
        .expect("declaration should admit");
    match handle.review_legality_checked(bad_surface) {
        ForgeQueryDeclarationLegalityChecked::Illegal(
            crate::application::ForgeQueryDeclarationLegalityDenial::IllegalSurfaceDisposition {
                ..
            },
        ) => {}
        other => panic!(
            "expected disposition denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn legality_boundary_can_defer_even_after_family_admission() {
    let handle = admitted_handle("collaborative");
    let declaration = handle
        .declare(Declaration::<DeferredLegalityFamily>::new("edge:42"))
        .expect("declaration should admit");

    match handle.review_legality_checked(declaration) {
        ForgeQueryDeclarationLegalityChecked::Illegal(
            crate::application::ForgeQueryDeclarationLegalityDenial::DeferredByLegalityBoundary {
                ..
            },
        ) => {}
        other => panic!(
            "expected deferred legality denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn declare_and_review_preserves_admission_vs_legality_split() {
    let handle = admitted_handle("collaborative");

    match handle.declare_and_review(Declaration::<DurableAdmissionFamily>::new("edge:42")) {
        Err(ForgeQueryDeclarationAdmissionOrLegalityError::Admission(admission)) => {
            assert!(matches!(
                admission,
                crate::application::ForgeQueryDeclarationAdmissionError::Deferred(_)
            ));
        }
        _ => panic!("expected admission denial"),
    }

    match handle.declare_and_review(Declaration::<IllegalRoleFamily>::new("edge:42")) {
        Err(ForgeQueryDeclarationAdmissionOrLegalityError::Legality(
            crate::application::ForgeQueryDeclarationLegalityDenial::IllegalRoleClaim { .. },
        )) => {}
        _ => panic!("expected legality denial"),
    }
}
