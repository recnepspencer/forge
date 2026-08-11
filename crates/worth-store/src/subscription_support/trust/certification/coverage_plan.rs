use super::super::epochs::{SupportCertificationEpoch, SupportOperationalLedgerEpoch};
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::row_requirement::SupportCertificationRowRequirement;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCertificationCoveragePlan {
    operational_ledger_epoch: SupportOperationalLedgerEpoch,
    certification_epoch: SupportCertificationEpoch,
    required_rows: Vec<SupportCertificationRowRequirement>,
}

impl SubscriptionSupportCertificationCoveragePlan {
    pub fn new(
        operational_ledger_epoch: SupportOperationalLedgerEpoch,
        certification_epoch: SupportCertificationEpoch,
        mut required_rows: Vec<SupportCertificationRowRequirement>,
    ) -> Result<Self, SupportTrustFailure> {
        if required_rows.is_empty() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification coverage plans require at least one row",
            ));
        }
        required_rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        if required_rows
            .windows(2)
            .any(|pair| pair[0].row_id == pair[1].row_id)
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification coverage plans cannot require duplicate row ids",
            ));
        }
        Ok(Self {
            operational_ledger_epoch,
            certification_epoch,
            required_rows,
        })
    }

    pub fn required_rows(&self) -> &[SupportCertificationRowRequirement] {
        &self.required_rows
    }

    pub fn certification_epoch(&self) -> SupportCertificationEpoch {
        self.certification_epoch
    }

    pub fn operational_ledger_epoch(&self) -> SupportOperationalLedgerEpoch {
        self.operational_ledger_epoch
    }
}
