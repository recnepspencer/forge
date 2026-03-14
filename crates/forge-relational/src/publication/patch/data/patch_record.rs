use crate::publication::patch::data::{
    AspectKey, PatchCompatibilityClass, PatchDetail, PatchOrdering, PatchPublicationMode,
    PatchStreamPosition,
};
use crate::transactions::data::RecordRef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchRecordKind {
    Created,
    Updated,
    Deleted,
    RetainedForAudit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchRecord {
    pub kind: PatchRecordKind,
    pub target: RecordRef,
    pub aspects: Vec<AspectKey>,
    pub detail: PatchDetail,
}

impl PatchRecord {
    pub fn canonicalized(&self) -> Self {
        let mut aspects = self.aspects.clone();
        if !aspects.windows(2).all(|window| window[0] < window[1]) {
            aspects.sort();
            aspects.dedup();
        }
        Self {
            kind: self.kind.clone(),
            target: self.target.clone(),
            aspects,
            detail: self.detail.canonicalized(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalPatchRecord {
    pub ordering: PatchOrdering,
    pub publication_mode: PatchPublicationMode,
    pub position: PatchStreamPosition,
    pub compatibility: PatchCompatibilityClass,
    pub records: Vec<PatchRecord>,
}

impl RelationalPatchRecord {
    pub fn canonicalized(&self) -> Self {
        Self {
            ordering: self.ordering,
            publication_mode: self.publication_mode,
            position: self.position,
            compatibility: self.compatibility,
            records: self
                .records
                .iter()
                .map(PatchRecord::canonicalized)
                .collect(),
        }
    }
}
