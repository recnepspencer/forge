use super::batch_plan::SupportDomainCertificationBatchPlan;
use super::digest::stable_digest;
use super::domain_counter::SupportDomainCertificationCounterSnapshot;
use super::domain_row::SupportDomainCertificationRow;
use super::domain_validation::{validate_domain_counters, validate_required_domain_rows};
use crate::subscription_support::trust::failure::SupportTrustFailure;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportDomainCertificationBundle {
    rows: Vec<SupportDomainCertificationRow>,
    batch_plan: SupportDomainCertificationBatchPlan,
    counter_snapshot: SupportDomainCertificationCounterSnapshot,
    domain_certification_digest: String,
}

impl SupportDomainCertificationBundle {
    pub fn new(
        mut rows: Vec<SupportDomainCertificationRow>,
        batch_plan: SupportDomainCertificationBatchPlan,
        counter_snapshot: SupportDomainCertificationCounterSnapshot,
    ) -> Result<Self, SupportTrustFailure> {
        rows.sort_by_key(SupportDomainCertificationRow::scenario);
        validate_required_domain_rows(&rows)?;
        validate_domain_counters(&rows, &batch_plan, counter_snapshot)?;
        let mut bundle = Self {
            rows,
            batch_plan,
            counter_snapshot,
            domain_certification_digest: String::new(),
        };
        bundle.domain_certification_digest = stable_digest(&SupportDomainBundleDigestBasis {
            rows: &bundle.rows,
            batch_plan: &bundle.batch_plan,
            counter_snapshot: bundle.counter_snapshot,
        })?;
        Ok(bundle)
    }

    pub fn rows(&self) -> &[SupportDomainCertificationRow] {
        &self.rows
    }

    pub fn counter_snapshot(&self) -> SupportDomainCertificationCounterSnapshot {
        self.counter_snapshot
    }

    pub fn domain_certification_digest(&self) -> &str {
        &self.domain_certification_digest
    }
}

#[derive(Serialize)]
struct SupportDomainBundleDigestBasis<'a> {
    rows: &'a [SupportDomainCertificationRow],
    batch_plan: &'a SupportDomainCertificationBatchPlan,
    counter_snapshot: SupportDomainCertificationCounterSnapshot,
}
