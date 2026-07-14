use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::suite_id::BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId;
use crate::subscription::certification::{
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationFailureBoundary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict {
    ParityBandProven,
    EquivalentParityProven,
    DivergenceProven,
    TypedRejectionProven,
    SufficiencyProven,
    CloseoutProven,
}

impl BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityBandProven => "parity_band_proven",
            Self::EquivalentParityProven => "equivalent_parity_proven",
            Self::DivergenceProven => "divergence_proven",
            Self::TypedRejectionProven => "typed_rejection_proven",
            Self::SufficiencyProven => "sufficiency_proven",
            Self::CloseoutProven => "closeout_proven",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow {
    suite_id: BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId,
    verdict: BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict,
    evidence_digest: Arc<str>,
    primary_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow {
    pub(crate) fn new(
        suite_id: BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId,
        verdict: BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict,
        evidence_digest: &str,
        primary_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-temporal-async-certification-support-matrix-row|suite={}|verdict={}|evidence={}|failure={}",
            suite_id.as_str(),
            verdict.as_str(),
            evidence_digest,
            primary_failure_boundary
                .map(BridgeSubscriptionCertificationFailureBoundary::as_str)
                .unwrap_or("none"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            suite_id,
            verdict,
            evidence_digest: Arc::from(evidence_digest),
            primary_failure_boundary,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-temporal-async-certification-support-matrix-row:sha256:{digest:x}"
            )),
        }
    }

    pub fn suite_id(&self) -> BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId {
        self.suite_id
    }
    pub fn verdict(&self) -> BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict {
        self.verdict
    }
    pub fn evidence_digest(&self) -> &str {
        self.evidence_digest.as_ref()
    }
    pub fn primary_failure_boundary(
        &self,
    ) -> Option<BridgeSubscriptionCertificationFailureBoundary> {
        self.primary_failure_boundary
    }
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionTemporalAsyncCertificationSupportMatrix {
    rows: Vec<BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow>,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionTemporalAsyncCertificationSupportMatrix {
    pub(crate) fn from_rows(
        rows: Vec<BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow>,
    ) -> Self {
        let row_digest_basis = rows
            .iter()
            .map(BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::digest)
            .collect::<Vec<_>>()
            .join(",");
        let counters = BridgeSubscriptionCertificationCounterSnapshot::from_phase_18_support_matrix(
            rows.len(),
        );
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-temporal-async-certification-support-matrix|rows={row_digest_basis}|counters={}",
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rows,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-temporal-async-certification-support-matrix:sha256:{digest:x}"
            )),
        }
    }

    pub fn rows(&self) -> &[BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow] {
        &self.rows
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
