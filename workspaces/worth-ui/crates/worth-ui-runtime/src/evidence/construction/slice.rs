use crate::evidence::shared::evidence_family_summary::UiEvidenceFamilySummary;
use crate::evidence::shared::evidence_materialized_detail::UiEvidenceMaterializedDetail;
use crate::evidence::shared::evidence_reference::UiEvidenceRef;
use crate::evidence::shared::evidence_slice::UiEvidenceSlice;
use worth_ui_inspection::{
    UiEvidenceAuthorityGeneration, UiEvidenceFamily, UiEvidenceSliceOmission,
};

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
