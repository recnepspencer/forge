use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionConsumerContractIdentity, BridgeSubscriptionCounters,
    BridgeSubscriptionSharingEligibilityIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionConsumerContractFamily {
    CanonicalDelivery,
    ReplayAudit,
}

impl BridgeSubscriptionConsumerContractFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalDelivery => "canonical_delivery",
            Self::ReplayAudit => "replay_audit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionConsumerPacingCapability {
    Immediate,
    LagBounded,
}

impl BridgeSubscriptionConsumerPacingCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::LagBounded => "lag_bounded",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionConsumerBackpressurePosture {
    PacingOnly,
    IndependentCursorRequired,
}

impl BridgeSubscriptionConsumerBackpressurePosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacingOnly => "pacing_only",
            Self::IndependentCursorRequired => "independent_cursor_required",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionConsumerDiagnosticsRetention {
    MinimalReference,
    RetainedDetail,
}

impl BridgeSubscriptionConsumerDiagnosticsRetention {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MinimalReference => "minimal_reference",
            Self::RetainedDetail => "retained_detail",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionConsumerContractRejectionKind {
    ReplayAuditRequiresRetainedDiagnostics,
    IndependentCursorRequiresLagBoundedPacing,
}

impl BridgeSubscriptionConsumerContractRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReplayAuditRequiresRetainedDiagnostics => {
                "replay_audit_requires_retained_diagnostics"
            }
            Self::IndependentCursorRequiresLagBoundedPacing => {
                "independent_cursor_requires_lag_bounded_pacing"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionConsumerContractRejection {
    rejection_kind: BridgeSubscriptionConsumerContractRejectionKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionConsumerContractRejection {
    fn new(rejection_kind: BridgeSubscriptionConsumerContractRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-consumer-contract-rejection|kind={}",
            rejection_kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCounters::from_consumer_contract_rejection(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-consumer-contract-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionConsumerContractRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionSharingEligibilityWitness {
    sharing_eligibility_identity: BridgeSubscriptionSharingEligibilityIdentity,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionSharingEligibilityWitness {
    fn new(canonical_basis: Arc<str>) -> Self {
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            sharing_eligibility_identity: BridgeSubscriptionSharingEligibilityIdentity::new(
                format!("bridge-subscription-sharing-eligibility-id:sha256:{digest:x}"),
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-sharing-eligibility:sha256:{digest:x}"
            )),
        }
    }

    pub fn sharing_eligibility_identity(&self) -> &BridgeSubscriptionSharingEligibilityIdentity {
        &self.sharing_eligibility_identity
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionConsumerContract {
    consumer_contract_identity: BridgeSubscriptionConsumerContractIdentity,
    family: BridgeSubscriptionConsumerContractFamily,
    pacing_capability: BridgeSubscriptionConsumerPacingCapability,
    backpressure_posture: BridgeSubscriptionConsumerBackpressurePosture,
    coalescing_admitted: bool,
    diagnostics_retention: BridgeSubscriptionConsumerDiagnosticsRetention,
    sharing_eligibility: BridgeSubscriptionSharingEligibilityWitness,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionConsumerContract {
    pub(crate) fn admit(
        family: BridgeSubscriptionConsumerContractFamily,
        pacing_capability: BridgeSubscriptionConsumerPacingCapability,
        backpressure_posture: BridgeSubscriptionConsumerBackpressurePosture,
        coalescing_admitted: bool,
        diagnostics_retention: BridgeSubscriptionConsumerDiagnosticsRetention,
    ) -> Result<Self, BridgeSubscriptionConsumerContractRejection> {
        if family == BridgeSubscriptionConsumerContractFamily::ReplayAudit
            && diagnostics_retention
                != BridgeSubscriptionConsumerDiagnosticsRetention::RetainedDetail
        {
            return Err(BridgeSubscriptionConsumerContractRejection::new(
                BridgeSubscriptionConsumerContractRejectionKind::ReplayAuditRequiresRetainedDiagnostics,
            ));
        }
        if backpressure_posture
            == BridgeSubscriptionConsumerBackpressurePosture::IndependentCursorRequired
            && pacing_capability != BridgeSubscriptionConsumerPacingCapability::LagBounded
        {
            return Err(BridgeSubscriptionConsumerContractRejection::new(
                BridgeSubscriptionConsumerContractRejectionKind::IndependentCursorRequiresLagBoundedPacing,
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-consumer-contract|family={}|pacing={}|backpressure={}|coalescing={}|diagnostics={}",
            family.as_str(),
            pacing_capability.as_str(),
            backpressure_posture.as_str(),
            coalescing_admitted,
            diagnostics_retention.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let sharing_eligibility =
            BridgeSubscriptionSharingEligibilityWitness::new(canonical_basis.clone());
        Ok(Self {
            consumer_contract_identity: BridgeSubscriptionConsumerContractIdentity::new(format!(
                "bridge-subscription-consumer-contract-id:sha256:{digest:x}"
            )),
            family,
            pacing_capability,
            backpressure_posture,
            coalescing_admitted,
            diagnostics_retention,
            sharing_eligibility,
            counters: BridgeSubscriptionCounters::from_consumer_contract_admission(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-consumer-contract:sha256:{digest:x}"
            )),
        })
    }

    pub fn consumer_contract_identity(&self) -> &BridgeSubscriptionConsumerContractIdentity {
        &self.consumer_contract_identity
    }

    pub fn family(&self) -> BridgeSubscriptionConsumerContractFamily {
        self.family
    }

    pub fn pacing_capability(&self) -> BridgeSubscriptionConsumerPacingCapability {
        self.pacing_capability
    }

    pub fn backpressure_posture(&self) -> BridgeSubscriptionConsumerBackpressurePosture {
        self.backpressure_posture
    }

    pub fn coalescing_admitted(&self) -> bool {
        self.coalescing_admitted
    }

    pub fn diagnostics_retention(&self) -> BridgeSubscriptionConsumerDiagnosticsRetention {
        self.diagnostics_retention
    }

    pub fn sharing_eligibility(&self) -> &BridgeSubscriptionSharingEligibilityWitness {
        &self.sharing_eligibility
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
