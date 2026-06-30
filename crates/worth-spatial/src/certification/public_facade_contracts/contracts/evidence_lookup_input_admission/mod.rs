use worth_spatial::facade::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupStageReceiptFamilyIdentity,
};
use worth_spatial::facade::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, EvidenceLookupAdmittedInput, EvidenceLookupInputAdmissionCounters,
    EvidenceLookupInputAdmissionError, EvidenceLookupInputAdmissionErrorKind,
    EvidenceLookupInputAdmissionRequest, EvidenceLookupProductSeparationProof,
    EvidenceLookupQueryAdmissionEvidenceSet, EvidenceLookupQueryAdmissionSupport,
    EvidenceLookupQuerySupportState, EvidenceLookupStageReceiptAdmission,
    EvidenceLookupTopologyAdmissionSupport, EvidenceLookupTopologySupportState,
};
use worth_spatial::facade::workload_vocabulary::{
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStage,
};

#[test]
fn spatial_public_api_exports_lookup_input_admission_boundary() {
    let _: fn(
        &worth_spatial::facade::evidence_lookup_family_catalog::EvidenceLookupFamilyCatalogCloseout,
        EvidenceLookupInputAdmissionRequest<'_>,
    ) -> Result<EvidenceLookupAdmittedInput, EvidenceLookupInputAdmissionError> =
        admit_evidence_lookup_input;

    let _: for<'a> fn(
        &'a SpatialGeometryEvidenceTouchAuthority,
    ) -> EvidenceLookupInputAdmissionRequest<'a> = request_from_spatial_touch_authority;
    let _: fn(
        EvidenceLookupInputAdmissionRequest<'static>,
        WorkloadEvidenceStage,
        EvidenceLookupStageReceiptFamilyIdentity,
    ) -> EvidenceLookupInputAdmissionRequest<'static> =
        EvidenceLookupInputAdmissionRequest::with_stage_receipt_family;

    let catalog = current_evidence_lookup_family_catalog().expect("catalog closes");
    assert_eq!(catalog.counters().family_count(), 3);
    let query_evidence = EvidenceLookupQueryAdmissionEvidenceSet::from_family_catalog(&catalog);
    assert_eq!(query_evidence.evidence_count(), 2);
    let imported_query_evidence = catalog
        .declarations()
        .iter()
        .find_map(|family| family.query_posture().imported_evidence())
        .expect("catalog exposes imported query posture");
    let query_evidence = EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(
        imported_query_evidence,
    );
    assert_eq!(query_evidence.evidence_count(), 1);
}

fn request_from_spatial_touch_authority<'a>(
    authority: &'a SpatialGeometryEvidenceTouchAuthority,
) -> EvidenceLookupInputAdmissionRequest<'a> {
    EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(authority)
}

#[test]
fn spatial_public_api_exposes_admission_envelope_read_contract() {
    let _: fn(&EvidenceLookupAdmittedInput) -> &str = EvidenceLookupAdmittedInput::admission_digest;
    let _: fn(&EvidenceLookupAdmittedInput) -> &str = EvidenceLookupAdmittedInput::catalog_digest;
    let _: fn(&EvidenceLookupAdmittedInput) -> &str =
        EvidenceLookupAdmittedInput::spatial_touch_digest;
    let _: fn(&EvidenceLookupAdmittedInput) -> &str =
        EvidenceLookupAdmittedInput::stage_receipt_digest;
    let _: fn(&EvidenceLookupAdmittedInput) -> &EvidenceLookupInputAdmissionCounters =
        EvidenceLookupAdmittedInput::counters;
    let _: fn(&EvidenceLookupAdmittedInput) -> bool =
        EvidenceLookupAdmittedInput::claims_lookup_product_construction;
    let _: fn(&EvidenceLookupAdmittedInput) -> bool =
        EvidenceLookupAdmittedInput::claims_lookup_execution;
    let _: fn(&EvidenceLookupAdmittedInput) -> EvidenceLookupProductSeparationProof =
        EvidenceLookupAdmittedInput::product_separation;
    let _: fn(
        &SpatialGeometryEvidenceTouchAuthority,
        EvidenceLookupStageReceiptFamilyIdentity,
    ) -> EvidenceLookupStageReceiptAdmission =
        EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority;
    let _: fn(&EvidenceLookupStageReceiptAdmission) -> &str =
        EvidenceLookupStageReceiptAdmission::stage_receipt_digest;
    let _: fn(&EvidenceLookupStageReceiptAdmission) -> &str =
        EvidenceLookupStageReceiptAdmission::spatial_touch_digest;
}

#[test]
fn spatial_public_api_exposes_typed_denial_and_support_posture() {
    let _: fn(&EvidenceLookupInputAdmissionError) -> EvidenceLookupInputAdmissionErrorKind =
        EvidenceLookupInputAdmissionError::kind;
    let _: fn(&EvidenceLookupInputAdmissionError) -> &str =
        EvidenceLookupInputAdmissionError::detail;
    let _: fn(&EvidenceLookupInputAdmissionError) -> &EvidenceLookupInputAdmissionCounters =
        EvidenceLookupInputAdmissionError::counters;
    let _: fn(&EvidenceLookupQueryAdmissionSupport) -> &EvidenceLookupQuerySupportState =
        EvidenceLookupQueryAdmissionSupport::state;
    let _: fn(&EvidenceLookupTopologyAdmissionSupport) -> &EvidenceLookupTopologySupportState =
        EvidenceLookupTopologyAdmissionSupport::state;
    let _: fn(&EvidenceLookupProductSeparationProof) -> bool =
        EvidenceLookupProductSeparationProof::claims_lookup_product_construction;
    let _: fn(&EvidenceLookupProductSeparationProof) -> bool =
        EvidenceLookupProductSeparationProof::claims_lookup_execution;
    let _: fn(&EvidenceLookupProductSeparationProof) -> bool =
        EvidenceLookupProductSeparationProof::claims_query_descriptor_authority;
    let _: fn(&EvidenceLookupProductSeparationProof) -> bool =
        EvidenceLookupProductSeparationProof::claims_topology_product_authority;
}
