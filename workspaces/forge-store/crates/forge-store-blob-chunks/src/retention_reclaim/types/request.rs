use crate::retention_reclaim::residue::BlobReclaimResidueKind;
use crate::retention_reclaim::types::admission::BlobRetentionReclaimAdmission;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionReclaimRequest {
    admission: BlobRetentionReclaimAdmission,
    residue_kind: BlobReclaimResidueKind,
}

impl BlobRetentionReclaimRequest {
    pub fn for_admission(admission: BlobRetentionReclaimAdmission) -> Self {
        Self {
            admission,
            residue_kind: BlobReclaimResidueKind::FailedReclaimBytes,
        }
    }

    pub fn with_abandoned_resume_residue(mut self) -> Self {
        self.residue_kind = BlobReclaimResidueKind::AbandonedResumeSessionBytes;
        self
    }

    pub const fn admission(&self) -> &BlobRetentionReclaimAdmission {
        &self.admission
    }

    pub const fn residue_kind(&self) -> BlobReclaimResidueKind {
        self.residue_kind
    }

    pub(crate) fn into_parts(self) -> (BlobRetentionReclaimAdmission, BlobReclaimResidueKind) {
        (self.admission, self.residue_kind)
    }
}
