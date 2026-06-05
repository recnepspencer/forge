use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionTemporalAsyncCertificationCloseoutRejectionKind {
    MissingRequiredSuiteEvidence,
    UnsupportedBasisNotTyped,
    UnsupportedNeighborNotTyped,
    TemporalAsyncParityBandIncomplete,
    ReferenceWorkloadNotSufficient,
}

impl BridgeSubscriptionTemporalAsyncCertificationCloseoutRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingRequiredSuiteEvidence => "missing_required_suite_evidence",
            Self::UnsupportedBasisNotTyped => "unsupported_basis_not_typed",
            Self::UnsupportedNeighborNotTyped => "unsupported_neighbor_not_typed",
            Self::TemporalAsyncParityBandIncomplete => "temporal_async_parity_band_incomplete",
            Self::ReferenceWorkloadNotSufficient => "reference_workload_not_sufficient",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection {
    rejection_kind: BridgeSubscriptionTemporalAsyncCertificationCloseoutRejectionKind,
    detail: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection {
    pub(crate) fn new(
        rejection_kind: BridgeSubscriptionTemporalAsyncCertificationCloseoutRejectionKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        let detail = detail.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-temporal-async-certification-closeout-rejection|kind={}|detail={detail}",
            rejection_kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            detail,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-temporal-async-certification-closeout-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(
        &self,
    ) -> BridgeSubscriptionTemporalAsyncCertificationCloseoutRejectionKind {
        self.rejection_kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
