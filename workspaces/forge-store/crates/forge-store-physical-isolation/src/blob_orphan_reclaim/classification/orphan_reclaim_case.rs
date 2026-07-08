use crate::ReclaimReachabilityRemovalReceipt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OrphanReclaimCase {
    DeniedMissingRemovalEvidence,
    DeniedIdentityNotCovered,
    CoverageAdmitted {
        receipt: ReclaimReachabilityRemovalReceipt,
    },
}