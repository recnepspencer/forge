mod evidence_expansion;
mod evidence_family_summary;
mod evidence_handle;
mod evidence_identity;
mod evidence_materialized_detail;
mod evidence_reference;
mod evidence_slice_assembly;
mod evidence_slice_ordering;
mod evidence_slice;
mod evidence_slice_ref;
mod inspection_cost_receipt;
mod obligation_evidence_receipt;
mod obligation_reason_projection;

pub(crate) use evidence_slice_assembly::{UiEvidenceSliceAssembly, UiEvidenceSliceAssemblyInput};
pub(crate) use evidence_slice_ordering::order_refs;
pub(crate) use inspection_cost_receipt::UiInspectionCostMetrics;
pub use evidence_expansion::UiEvidenceExpansion;
pub use evidence_family_summary::UiEvidenceFamilySummary;
pub use evidence_handle::UiEvidenceHandle;
pub use evidence_identity::UiEvidenceIdentity;
pub use evidence_materialized_detail::UiEvidenceMaterializedDetail;
pub use evidence_reference::UiEvidenceRef;
pub use evidence_slice::UiEvidenceSlice;
pub use evidence_slice_ref::UiEvidenceSliceRef;
pub use obligation_evidence_receipt::UiInspectionObligationEvidenceReceipt;
pub use obligation_reason_projection::UiInspectionObligationReasonProjection;
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
