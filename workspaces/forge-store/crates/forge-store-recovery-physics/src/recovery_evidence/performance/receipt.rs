use forge_foundational::{
    FoundationalAttachedCounterBackedPerformanceReceipt, FoundationalAuthoritativePerformanceClaim,
    FoundationalCertifiedPerformanceBundle, FoundationalCounterBackedPerformanceReceipt,
    FoundationalPerformanceCounterRow, FoundationalPolicyAdmissionReceipt,
};

use crate::RecoveryCounterSnapshot;

use super::super::denial::RecoveryEvidenceDenial;
use super::super::executed_evidence_source::RecoveryPhysicsEvidenceSource;
use super::counter_backed_receipt::counter_backed_receipt;
use super::counter_rows::recovery_performance_counter_rows;
use super::policy_admission::policy_admission_receipt;
use super::support_certification::{
    certified_support_expansion, readmitted_support_expansion, support_expansion_report,
};
use super::surfaces::{recovery_performance_surfaces, RecoveryPerformanceSurface};

pub type RecoveryAttachedCounterBackedPerformanceReceipt =
    FoundationalAttachedCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;
pub type RecoveryMaterializedPerformanceReport =
    forge_foundational::FoundationalMaterializedPerformanceReport<
        RecoveryAttachedCounterBackedPerformanceReceipt,
    >;
pub type RecoveryCertifiedPerformanceBundle =
    FoundationalCertifiedPerformanceBundle<RecoveryMaterializedPerformanceReport>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryCounterPerformanceReceipt {
    rows: Vec<FoundationalPerformanceCounterRow>,
    exact_counter_assertions: usize,
    counter_backed:
        FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
    policy_admission: FoundationalPolicyAdmissionReceipt,
}

impl RecoveryCounterPerformanceReceipt {
    pub fn from_source(source: &RecoveryPhysicsEvidenceSource) -> Self {
        Self::from_counters(source.counters())
    }

    pub(crate) fn from_counters(counters: RecoveryCounterSnapshot) -> Self {
        let rows = recovery_performance_counter_rows(counters);
        let counter_backed = counter_backed_receipt(rows.clone());
        let policy_admission = policy_admission_receipt(counters);
        Self {
            exact_counter_assertions: rows.len(),
            rows,
            counter_backed,
            policy_admission,
        }
    }

    pub fn rows(&self) -> &[FoundationalPerformanceCounterRow] {
        &self.rows
    }

    pub const fn exact_counter_assertions(&self) -> usize {
        self.exact_counter_assertions
    }

    pub const fn counter_backed(
        &self,
    ) -> &FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>
    {
        &self.counter_backed
    }

    pub const fn policy_admission(&self) -> &FoundationalPolicyAdmissionReceipt {
        &self.policy_admission
    }

    pub fn support_expansion_report(&self) -> RecoveryMaterializedPerformanceReport {
        support_expansion_report(&self.counter_backed)
    }

    pub fn certified_support_expansion(
        &self,
    ) -> Result<RecoveryCertifiedPerformanceBundle, RecoveryEvidenceDenial> {
        certified_support_expansion(&self.counter_backed)
    }

    pub fn readmitted_support_expansion(
        &self,
    ) -> Result<RecoveryCertifiedPerformanceBundle, RecoveryEvidenceDenial> {
        readmitted_support_expansion(&self.counter_backed)
    }

    pub fn surfaces(&self) -> Vec<RecoveryPerformanceSurface> {
        recovery_performance_surfaces(
            self.rows.len(),
            &self.policy_admission,
            &self.counter_backed,
        )
    }
}
