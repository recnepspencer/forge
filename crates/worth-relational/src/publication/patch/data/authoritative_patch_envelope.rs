use crate::publication::patch::data::{
    PatchDetail, PatchOrdering, PatchPublicationMode, PatchStreamPosition,
    PublishedAuthoritativePatch, RecordStructuralChange,
};
use crate::transactions::data::RecordRef;
use serde::{Deserialize, Serialize};
use worth_foundational::facade::AspectKey;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishedAuthoritativeRecordPatch {
    pub target: RecordRef,
    pub structural_change: RecordStructuralChange,
    pub authoritative_patch: PublishedAuthoritativePatch,
    pub contains_opaque_aspect: bool,
    pub detail: PatchDetail,
}

impl PublishedAuthoritativeRecordPatch {
    pub fn authoritative_changed_aspect_keys(&self) -> impl Iterator<Item = &AspectKey> {
        self.authoritative_patch.changed_aspect_keys()
    }

    pub fn authoritative_changed_aspects(&self) -> Vec<AspectKey> {
        self.authoritative_patch.changed_aspects()
    }

    pub fn canonicalized(&self) -> Self {
        Self {
            target: self.target.clone(),
            structural_change: self.structural_change,
            authoritative_patch: self.authoritative_patch.canonicalized(),
            contains_opaque_aspect: self.contains_opaque_aspect,
            detail: self.detail.canonicalized(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishedAuthoritativePatchEnvelope {
    pub ordering: PatchOrdering,
    pub publication_mode: PatchPublicationMode,
    pub position: PatchStreamPosition,
    pub authoritative_record_patches: Vec<PublishedAuthoritativeRecordPatch>,
}

impl PublishedAuthoritativePatchEnvelope {
    pub fn canonicalized(&self) -> Self {
        Self {
            ordering: self.ordering,
            publication_mode: self.publication_mode,
            position: self.position,
            authoritative_record_patches: self
                .authoritative_record_patches
                .iter()
                .map(PublishedAuthoritativeRecordPatch::canonicalized)
                .collect(),
        }
    }
}
