use forge_store_physical_format::PhysicalEpoch;

use super::{
    AdmittedCoverageBasis, LayoutCoverageWitness, LayoutMaterializationSourceIdentity,
    LayoutMaterializationState, MaterializationDenial, PhysicalCoverageBasis,
};

#[test]
fn equal_watermarks_from_different_physical_sources_cannot_form_coverage() {
    let original = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let advanced = crate::bootstrap::test_support::advanced_bootstrap_catalog_read_admission();
    let basis = PhysicalCoverageBasis::root_epoch(PhysicalEpoch::from_raw(7).unwrap());
    let lower = AdmittedCoverageBasis::admit(
        LayoutMaterializationSourceIdentity::from_catalog(&original),
        &basis,
    );
    let upper = AdmittedCoverageBasis::admit(
        LayoutMaterializationSourceIdentity::from_catalog(&advanced),
        &basis,
    );
    let state = LayoutMaterializationState::exact_through_physical_basis(
        crate::layout_declarations().seed_family().family(),
    );

    assert_eq!(
        LayoutCoverageWitness::from_admitted_bases(state, lower, upper, None),
        Err(MaterializationDenial::CoverageSourceMismatch),
    );
}

#[test]
fn every_coverage_witness_retains_a_concrete_source_identity() {
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let security = crate::keyspace::tests_support::admitted_scope(
        forge_store_security::StoreKeyScope::PageEnvelope,
        forge_store_security::StoreTenantScope::TenantPhysicalBoundary,
        forge_store_security::StoreAuthenticityRequirement::required(
            forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        forge_store_security::StoreCustodyPosture::InternalStoreCustody,
    );
    let declaration = crate::layout_declarations()
        .declaration(forge_store_contracts::DurableArtifactFamilyId::PhysicalPage)
        .unwrap();
    let family = crate::layout_declarations()
        .admit_physical_artifact_family(declaration, security.witnesses())
        .unwrap();
    let materialization = crate::access_planning()
        .admit_current_catalog_root_materialization(family, &catalog)
        .unwrap();

    assert_eq!(
        materialization.coverage().source(),
        materialization.source()
    );
}
