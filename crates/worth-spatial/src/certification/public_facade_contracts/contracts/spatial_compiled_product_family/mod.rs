use worth_spatial::facade::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog as public_evidence_lookup_family_catalog;
use worth_spatial::facade::spatial_compiled_product_family::{
    current_spatial_compiled_product_family_catalog, select_spatial_compiled_product_family,
    SelectedSpatialCompiledProductFamily, SpatialCompiledProductConsumer,
    SpatialCompiledProductFamilyAdmittedInput, SpatialCompiledProductFamilyCatalog,
    SpatialCompiledProductFamilyCatalogCounters, SpatialCompiledProductFamilyDeclaration,
    SpatialCompiledProductFamilyError, SpatialCompiledProductFamilyErrorKind,
    SpatialCompiledProductFamilyIdentity, SpatialCompiledProductLoweredIdentity,
};

#[test]
fn spatial_public_api_exports_compiled_product_family_boundary() {
    let _: fn() -> SpatialCompiledProductFamilyCatalog =
        current_spatial_compiled_product_family_catalog;
    let _: fn(
        &SpatialCompiledProductFamilyCatalog,
        SpatialCompiledProductFamilyAdmittedInput,
    )
        -> Result<SelectedSpatialCompiledProductFamily, SpatialCompiledProductFamilyError> =
        select_spatial_compiled_product_family;
}

#[test]
fn spatial_public_api_exposes_declared_family_read_contract() {
    let _: fn(&SpatialCompiledProductFamilyCatalog) -> &str =
        SpatialCompiledProductFamilyCatalog::catalog_digest;
    let _: fn(&SpatialCompiledProductFamilyCatalog) -> SpatialCompiledProductFamilyCatalogCounters =
        SpatialCompiledProductFamilyCatalog::counters;
    let _: fn(
        &SpatialCompiledProductFamilyCatalog,
        SpatialCompiledProductFamilyIdentity,
    ) -> Option<&SpatialCompiledProductFamilyDeclaration> =
        SpatialCompiledProductFamilyCatalog::family;
    let _: fn(
        &SpatialCompiledProductFamilyCatalog,
        SpatialCompiledProductConsumer,
    ) -> Option<&SpatialCompiledProductFamilyDeclaration> =
        SpatialCompiledProductFamilyCatalog::family_for_consumer;
    let _: fn(&SpatialCompiledProductFamilyCatalogCounters) -> usize =
        SpatialCompiledProductFamilyCatalogCounters::family_count;
    let _: fn(&SpatialCompiledProductFamilyCatalogCounters) -> usize =
        SpatialCompiledProductFamilyCatalogCounters::declared_family_count;
    let _: fn(&SpatialCompiledProductFamilyCatalogCounters) -> usize =
        SpatialCompiledProductFamilyCatalogCounters::supported_consumer_count;
    let _: fn(&SpatialCompiledProductFamilyDeclaration) -> SpatialCompiledProductFamilyIdentity =
        SpatialCompiledProductFamilyDeclaration::identity;
    let _: fn(&SpatialCompiledProductFamilyDeclaration) -> &[SpatialCompiledProductConsumer] =
        SpatialCompiledProductFamilyDeclaration::supported_consumers;
    let _: fn(&SpatialCompiledProductFamilyDeclaration) -> &str =
        SpatialCompiledProductFamilyDeclaration::family_digest;
    let _: fn(&SpatialCompiledProductFamilyAdmittedInput) -> SpatialCompiledProductConsumer =
        SpatialCompiledProductFamilyAdmittedInput::consumer;
    let _: fn(&SpatialCompiledProductFamilyAdmittedInput) -> SpatialCompiledProductFamilyIdentity =
        SpatialCompiledProductFamilyAdmittedInput::family_identity;
    let _: fn(&SelectedSpatialCompiledProductFamily) -> &SpatialCompiledProductFamilyDeclaration =
        SelectedSpatialCompiledProductFamily::declaration;
    let _: fn(&SelectedSpatialCompiledProductFamily) -> &SpatialCompiledProductFamilyAdmittedInput =
        SelectedSpatialCompiledProductFamily::admitted_input;
    let _: fn(
        &SelectedSpatialCompiledProductFamily,
    )
        -> Result<SpatialCompiledProductLoweredIdentity, SpatialCompiledProductFamilyError> =
        SelectedSpatialCompiledProductFamily::compile_product_identity;
    let _: fn(&SpatialCompiledProductLoweredIdentity) -> SpatialCompiledProductFamilyIdentity =
        SpatialCompiledProductLoweredIdentity::family_identity;
    let _: fn(&SpatialCompiledProductLoweredIdentity) -> &str =
        SpatialCompiledProductLoweredIdentity::family_digest;
}

#[test]
fn spatial_public_api_exposes_compiled_product_family_error_boundary() {
    let _: fn(&SpatialCompiledProductFamilyError) -> SpatialCompiledProductFamilyErrorKind =
        SpatialCompiledProductFamilyError::kind;
    let _: fn(&SpatialCompiledProductFamilyError) -> &str =
        SpatialCompiledProductFamilyError::detail;
}

#[test]
fn spatial_public_facade_runtime_lowering_matches_real_evidence_lookup_path() {
    let family_catalog = public_evidence_lookup_family_catalog().expect("public family catalog");
    let covered_family = family_catalog
        .declarations()
        .iter()
        .find(|family| !family.topology_input_posture().requires_topology_receipt())
        .expect("covered family declaration");
    let path = crate::workload_platform::evidence_lookup_stage_cutover::current_path::admit_current_family_stage_cutover_path(
        &family_catalog,
        covered_family,
        covered_family.stage_applicability().stages()[0],
    )
    .expect("current cutover path");
    let spatial_catalog = current_spatial_compiled_product_family_catalog();
    let selected = select_spatial_compiled_product_family(&spatial_catalog, {
        crate::workload_platform::compiled_product_admission::admit_spatial_compiled_product_input(
            &spatial_catalog,
            crate::workload_platform::compiled_product_admission::SpatialCompiledProductAdmissionRequest::for_evidence_lookup_product(
                SpatialCompiledProductConsumer::EvidenceLookupPublicCloseout,
                path.selected_plan(),
                path.index_product(),
            ),
        )
        .expect("crate-owned spatial admitted input")
        .family_admitted_input()
    })
    .expect("public facade selected family");
    let lowered = selected
        .compile_product_identity()
        .expect("public facade lowered identity");

    assert_eq!(
        selected.declaration().identity(),
        SpatialCompiledProductFamilyIdentity::EvidenceLookupDerivedSupport
    );
    assert_eq!(
        lowered.compiled_product_identity().identity_digest(),
        path.index_product().compiled_product_identity_digest()
    );
    assert_eq!(
        lowered.equivalence_policy_identity().identity_digest(),
        path.index_product().equivalence_policy_identity_digest()
    );
}
