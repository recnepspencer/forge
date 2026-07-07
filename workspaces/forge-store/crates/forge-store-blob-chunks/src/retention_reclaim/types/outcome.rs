use crate::retention_reclaim::permit::BlobRetentionReclaimPermit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobRetentionReclaimOutcome {
    Permitted(BlobRetentionReclaimPermit),
}

impl BlobRetentionReclaimOutcome {
    pub fn into_permit(self) -> BlobRetentionReclaimPermit {
        match self {
            Self::Permitted(permit) => permit,
        }
    }
}
