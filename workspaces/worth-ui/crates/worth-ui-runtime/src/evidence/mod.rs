mod dummy_measurement_family;
mod evidence_expansion;
mod evidence_family_summary;
mod evidence_handle;
mod evidence_identity;
mod evidence_materialized_detail;
mod evidence_reference;
mod evidence_slice;
mod evidence_slice_assembly;
mod evidence_slice_ordering;
mod evidence_slice_ref;
#[cfg(test)]
mod host_measurement_request_shape_digest;
mod inspection_cost_receipt;
mod measurement_basis;
mod measurement_basis_assembly;
mod measurement_basis_certification;
mod measurement_basis_certification_scenario;
#[cfg(test)]
mod measurement_basis_certification_tests;
mod measurement_basis_denial;
#[cfg(test)]
mod measurement_basis_hostile_tests;
mod measurement_basis_identity;
#[cfg(test)]
mod measurement_basis_tests;
mod measurement_coordinate_space;
mod measurement_dependency_lineage;
mod measurement_dependency_map;
#[cfg(test)]
mod measurement_dependency_map_tests;
mod measurement_evidence_category;
mod measurement_evidence_input;
mod measurement_generation_compatibility;
mod measurement_inspection_receipt;
#[cfg(test)]
mod measurement_inspection_tests;
mod measurement_neighborhood_class_hint;
#[cfg(test)]
mod measurement_projection_variant_test_support;
mod measurement_result;
#[cfg(test)]
mod measurement_result_identity_digest;
mod measurement_rounding_posture;
mod measurement_unit_posture;
mod obligation_evidence_receipt;
mod obligation_reason_projection;
mod projection_fact_receipt;
#[cfg(test)]
mod projection_fact_receipt_tests;
#[cfg(test)]
pub(crate) mod projection_fact_test_support;
#[cfg(test)]
pub(crate) mod projection_query_context_test_support;
mod query_measurement_fact_family_digest;

pub use evidence_expansion::UiEvidenceExpansion;
pub use evidence_family_summary::UiEvidenceFamilySummary;
pub use evidence_handle::UiEvidenceHandle;
pub use evidence_identity::UiEvidenceIdentity;
pub use evidence_materialized_detail::UiEvidenceMaterializedDetail;
pub use evidence_reference::UiEvidenceRef;
pub use evidence_slice::UiEvidenceSlice;
pub(crate) use evidence_slice_assembly::{UiEvidenceSliceAssembly, UiEvidenceSliceAssemblyInput};
pub(crate) use evidence_slice_ordering::order_refs;
pub use evidence_slice_ref::UiEvidenceSliceRef;
#[cfg(test)]
pub(crate) use host_measurement_request_shape_digest::host_measurement_request_shape_digest;
pub(crate) use inspection_cost_receipt::UiInspectionCostMetrics;
pub use measurement_basis::{
    admit_measurement_basis, UiMeasurementBasis, UiMeasurementBasisGeneration,
    UiMeasurementBasisPosture,
};
pub use measurement_basis_certification::{
    certify_measurement_basis_determinism, UiMeasurementBasisCertificationReport,
    UiMeasurementBasisDeterminismPosture,
};
pub use measurement_basis_certification_scenario::{
    certify_measurement_basis_determinism_for_scenarios,
    UiMeasurementBasisCertificationHostRequest, UiMeasurementBasisCertificationOutcome,
    UiMeasurementBasisCertificationScenario, UiMeasurementBasisCertificationScenarioError,
};
pub use measurement_basis_denial::{UiMeasurementBasisDenial, UiMeasurementEvidenceSlot};
pub use measurement_coordinate_space::UiMeasurementCoordinateSpace;
pub use measurement_dependency_lineage::{
    UiMeasurementDependencyLineage, UiMeasurementDependencyLineageEntry,
    UiMeasurementDependencyLineageKind,
};
pub(crate) use measurement_dependency_map::{
    derive_measurement_dependency_map, derive_measurement_neighborhood_class_hint,
};
pub use measurement_dependency_map::{UiMeasurementDependencyMap, UiMeasurementDependencyMapEntry};
pub use measurement_evidence_category::UiMeasurementEvidenceCategory;
pub use measurement_evidence_input::MeasurementEvidenceInput;
pub use measurement_generation_compatibility::UiMeasurementGenerationCompatibility;
pub(crate) use measurement_inspection_receipt::{
    project_measurement_inspection_compatibility_view, project_measurement_inspection_denial_view,
    project_measurement_inspection_view,
};
pub use measurement_neighborhood_class_hint::UiMeasurementNeighborhoodClassHint;
pub use measurement_result::{UiCurrentMeasurementResult, UiMeasurementResult, UiMeasurementValue};
#[cfg(test)]
pub(crate) use measurement_result_identity_digest::measurement_result_identity_digest;
pub use measurement_rounding_posture::UiMeasurementRoundingPosture;
pub use measurement_unit_posture::UiMeasurementUnitPosture;
pub use obligation_evidence_receipt::UiInspectionObligationEvidenceReceipt;
pub use obligation_reason_projection::UiInspectionObligationReasonProjection;
pub(crate) use projection_fact_receipt::admit_declared_measurement_projection_fact_receipt;
pub use projection_fact_receipt::{
    consume_declared_measurement_projection_facts, UiProjectionFactReceipt,
    UiProjectionFactReceiptDenial,
};
pub(crate) use query_measurement_fact_family_digest::query_measurement_fact_family_set_digest;
pub use worth_ui_inspection::{
    UiEvidenceAuthorityArtifactIdentity, UiEvidenceAuthorityBinding, UiEvidenceAuthorityGeneration,
    UiEvidenceAuthorityKind, UiEvidenceExpansionOutcome, UiEvidenceFamily,
    UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture, UiEvidenceSliceOmission,
    UiInspectionCostReceipt,
};

pub(crate) fn evidence_identity(family: UiEvidenceFamily, digest: u64) -> UiEvidenceIdentity {
    UiEvidenceIdentity::new(family, digest)
}

pub(crate) fn evidence_handle(
    family: UiEvidenceFamily,
    identity: UiEvidenceIdentity,
    handle_digest: u64,
) -> UiEvidenceHandle {
    UiEvidenceHandle::new(family, identity, handle_digest)
}

pub(crate) fn evidence_authority_binding(
    authority_kind: UiEvidenceAuthorityKind,
    authority_digest: u64,
    authority_generation: UiEvidenceAuthorityGeneration,
    world: Option<worth_ui_inspection::UiInspectionSupportWorld>,
) -> UiEvidenceAuthorityBinding {
    UiEvidenceAuthorityBinding::new(
        UiEvidenceAuthorityArtifactIdentity::new(authority_kind, authority_digest),
        authority_generation,
        world,
    )
}

pub(crate) fn evidence_ref(
    family: UiEvidenceFamily,
    identity: UiEvidenceIdentity,
    authority_binding: UiEvidenceAuthorityBinding,
    materialization_posture: UiEvidenceMaterializationPosture,
    retention_posture: UiEvidenceRetentionPosture,
    handle: UiEvidenceHandle,
) -> UiEvidenceRef {
    UiEvidenceRef::new(
        family,
        identity,
        authority_binding,
        materialization_posture,
        retention_posture,
        handle,
    )
}

pub(crate) fn with_retention_posture(
    evidence_ref: UiEvidenceRef,
    retention_posture: UiEvidenceRetentionPosture,
) -> UiEvidenceRef {
    UiEvidenceRef::new(
        evidence_ref.family(),
        evidence_ref.identity(),
        evidence_ref.authority_binding(),
        evidence_ref.materialization_posture(),
        retention_posture,
        evidence_ref.handle(),
    )
}

pub(crate) fn evidence_family_summary(
    family: UiEvidenceFamily,
    ref_count: usize,
) -> UiEvidenceFamilySummary {
    UiEvidenceFamilySummary::new(family, ref_count)
}

pub(crate) fn evidence_slice(
    authority_generation: UiEvidenceAuthorityGeneration,
    refs: Box<[UiEvidenceRef]>,
    family_summaries: Box<[UiEvidenceFamilySummary]>,
    materialized_detail: Option<UiEvidenceMaterializedDetail>,
    omission: Option<UiEvidenceSliceOmission>,
) -> UiEvidenceSlice {
    UiEvidenceSlice::new(
        authority_generation,
        refs,
        family_summaries,
        materialized_detail,
        omission,
    )
}

pub(crate) fn preflight_evidence_expansion(
    current_generation: UiEvidenceAuthorityGeneration,
    evidence_ref: UiEvidenceRef,
    requested_richness: worth_ui_inspection::UiEvidenceRichness,
) -> Option<UiEvidenceExpansion> {
    if matches!(
        evidence_ref.retention_posture(),
        UiEvidenceRetentionPosture::DiscardedWithTombstone
    ) {
        return Some(UiEvidenceExpansion::new(
            evidence_ref,
            requested_richness,
            UiEvidenceExpansionOutcome::Discarded {
                retention: evidence_ref.retention_posture(),
            },
            None,
            Box::new([]),
            None,
        ));
    }

    if evidence_ref.authority_generation() != current_generation {
        return Some(UiEvidenceExpansion::new(
            evidence_ref,
            requested_richness,
            UiEvidenceExpansionOutcome::WrongGeneration {
                requested_generation: evidence_ref.authority_generation(),
                current_generation,
            },
            None,
            Box::new([]),
            None,
        ));
    }

    if !matches!(
        evidence_ref.materialization_posture(),
        UiEvidenceMaterializationPosture::SummaryAvailable
            | UiEvidenceMaterializationPosture::DetailAvailable
    ) {
        return Some(UiEvidenceExpansion::new(
            evidence_ref,
            requested_richness,
            UiEvidenceExpansionOutcome::NotMaterialized {
                posture: evidence_ref.materialization_posture(),
            },
            None,
            Box::new([]),
            None,
        ));
    }

    None
}
