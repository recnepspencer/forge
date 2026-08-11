use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyCertificationCounterSnapshot {
    required_row_count: u64,
    certified_row_count: u64,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

impl SubscriptionSupportAccuracyCertificationCounterSnapshot {
    pub fn new(
        required_row_count: u64,
        certified_row_count: u64,
        forbidden_exact_overclaim_count: u64,
        global_scan_debt_count: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if forbidden_exact_overclaim_count != 0 || global_scan_debt_count != 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy suite counters require zero exact-overclaim and global-scan debt",
            ));
        }
        Ok(Self {
            required_row_count,
            certified_row_count,
            forbidden_exact_overclaim_count,
            global_scan_debt_count,
        })
    }

    pub fn required_row_count(&self) -> u64 {
        self.required_row_count
    }

    pub fn certified_row_count(&self) -> u64 {
        self.certified_row_count
    }
}
