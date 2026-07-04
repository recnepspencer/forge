use crate::declaration::stable_text_digest;
use worth_ui_inspection::UiEvidenceSliceOmission;

use super::{
    UiEvidenceAuthorityGeneration, UiEvidenceFamilySummary, UiEvidenceMaterializedDetail,
    UiEvidenceRef, UiEvidenceSliceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiEvidenceSlice {
    slice_ref: UiEvidenceSliceRef,
    authority_generation: UiEvidenceAuthorityGeneration,
    refs: Box<[UiEvidenceRef]>,
    family_summaries: Box<[UiEvidenceFamilySummary]>,
    materialized_detail: Option<UiEvidenceMaterializedDetail>,
    omission: Option<UiEvidenceSliceOmission>,
}

impl UiEvidenceSlice {
    pub(crate) fn new(
        authority_generation: UiEvidenceAuthorityGeneration,
        refs: Box<[UiEvidenceRef]>,
        family_summaries: Box<[UiEvidenceFamilySummary]>,
        materialized_detail: Option<UiEvidenceMaterializedDetail>,
        omission: Option<UiEvidenceSliceOmission>,
    ) -> Self {
        let slice_digest = refs.iter().fold(
            stable_text_digest("ui-evidence-slice")
                ^ authority_generation.as_u64().rotate_left(7)
                ^ (family_summaries.len() as u64).rotate_left(13),
            |digest, evidence_ref| {
                digest
                    ^ evidence_ref.identity().digest().rotate_left(17)
                    ^ evidence_ref.authority_generation().as_u64().rotate_left(29)
            },
        );
        Self {
            slice_ref: UiEvidenceSliceRef::new(slice_digest, authority_generation),
            authority_generation,
            refs,
            family_summaries,
            materialized_detail,
            omission,
        }
    }

    pub fn slice_ref(&self) -> UiEvidenceSliceRef {
        self.slice_ref
    }

    pub fn authority_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.authority_generation
    }

    pub fn refs(&self) -> &[UiEvidenceRef] {
        &self.refs
    }

    pub fn family_summaries(&self) -> &[UiEvidenceFamilySummary] {
        &self.family_summaries
    }

    pub fn materialized_detail(&self) -> Option<&UiEvidenceMaterializedDetail> {
        self.materialized_detail.as_ref()
    }

    pub fn omission(&self) -> Option<UiEvidenceSliceOmission> {
        self.omission
    }
}
