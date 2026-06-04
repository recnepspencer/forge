use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeSubscriptionReferenceWorkloadFamilyKind, BridgeSubscriptionReferenceWorkloadLaneKind,
    BridgeSubscriptionReferenceWorkloadLaneRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionSourceArtifactKind {
    Declaration,
    BasisBinding,
    AdmittedSubscription,
    Lifecycle,
    ActiveDelivery,
    DeliveryWindow,
    Fanout,
    Checkpoint,
    Resume,
    Continuation,
    Preview,
    RetainedReplay,
    StrategyLowering,
    Failure,
    LaneIdentity,
}

impl BridgeSubscriptionSourceArtifactKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Declaration => "declaration",
            Self::BasisBinding => "basis_binding",
            Self::AdmittedSubscription => "admitted_subscription",
            Self::Lifecycle => "lifecycle",
            Self::ActiveDelivery => "active_delivery",
            Self::DeliveryWindow => "delivery_window",
            Self::Fanout => "fanout",
            Self::Checkpoint => "checkpoint",
            Self::Resume => "resume",
            Self::Continuation => "continuation",
            Self::Preview => "preview",
            Self::RetainedReplay => "retained_replay",
            Self::StrategyLowering => "strategy_lowering",
            Self::Failure => "failure",
            Self::LaneIdentity => "lane_identity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionSourceArtifactScenario {
    CertificationReport,
    ReferenceWorkload,
    MultiFailurePrecedence,
    OrderingHostility,
    SchemaParity,
    StaleCheckpoint,
    CertificationBundle,
    OfflineAudit,
    ManifestCostAndSchema,
    ComparisonReports,
    PrecedenceOrderingAndBundle,
}

impl BridgeSubscriptionSourceArtifactScenario {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertificationReport => "certification-report",
            Self::ReferenceWorkload => "reference-workload",
            Self::MultiFailurePrecedence => "multi-failure-precedence",
            Self::OrderingHostility => "ordering-hostility",
            Self::SchemaParity => "schema-parity",
            Self::StaleCheckpoint => "stale-checkpoint",
            Self::CertificationBundle => "certification-bundle",
            Self::OfflineAudit => "offline-audit",
            Self::ManifestCostAndSchema => "manifest-cost-and-schema",
            Self::ComparisonReports => "comparison-reports",
            Self::PrecedenceOrderingAndBundle => "precedence-ordering-and-bundle",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionSourceArtifactRole {
    Stable,
    Control,
    Hostile,
    Fresh,
    Stale,
    Parity,
    Divergent,
    Left,
    Right,
    DuplicateScan,
    MinimalDiagnostics,
    RichDiagnostics,
    SharedFanout,
    DivergentFanout,
    AdmittedContinuation,
    DeniedContinuation,
    ExactFieldLens,
    CollectionMembershipIndex,
    HostileStrategyLowering,
}

impl BridgeSubscriptionSourceArtifactRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Control => "control",
            Self::Hostile => "hostile",
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Parity => "parity",
            Self::Divergent => "divergent",
            Self::Left => "left",
            Self::Right => "right",
            Self::DuplicateScan => "duplicate-scan",
            Self::MinimalDiagnostics => "minimal-diagnostics",
            Self::RichDiagnostics => "rich-diagnostics",
            Self::SharedFanout => "shared-fanout",
            Self::DivergentFanout => "divergent-fanout",
            Self::AdmittedContinuation => "admitted-continuation",
            Self::DeniedContinuation => "denied-continuation",
            Self::ExactFieldLens => "exact-field-lens",
            Self::CollectionMembershipIndex => "collection-membership-index",
            Self::HostileStrategyLowering => "hostile-strategy-lowering",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionSourceArtifactEvidence {
    artifact_kind: BridgeSubscriptionSourceArtifactKind,
    scenario: BridgeSubscriptionSourceArtifactScenario,
    role: BridgeSubscriptionSourceArtifactRole,
    lane_kind: Option<BridgeSubscriptionReferenceWorkloadLaneKind>,
    family_kind: Option<BridgeSubscriptionReferenceWorkloadFamilyKind>,
}

impl BridgeSubscriptionSourceArtifactEvidence {
    pub fn scenario(
        artifact_kind: BridgeSubscriptionSourceArtifactKind,
        scenario: BridgeSubscriptionSourceArtifactScenario,
        role: BridgeSubscriptionSourceArtifactRole,
    ) -> Self {
        Self {
            artifact_kind,
            scenario,
            role,
            lane_kind: None,
            family_kind: None,
        }
    }

    pub(crate) fn reference_workload_lane(
        artifact_kind: BridgeSubscriptionSourceArtifactKind,
        request: BridgeSubscriptionReferenceWorkloadLaneRequest,
        role: BridgeSubscriptionSourceArtifactRole,
    ) -> Self {
        Self {
            artifact_kind,
            scenario: BridgeSubscriptionSourceArtifactScenario::ReferenceWorkload,
            role,
            lane_kind: Some(request.lane_kind()),
            family_kind: Some(request.family_kind()),
        }
    }

    pub(crate) fn reference_workload_family(
        artifact_kind: BridgeSubscriptionSourceArtifactKind,
        family_kind: BridgeSubscriptionReferenceWorkloadFamilyKind,
        role: BridgeSubscriptionSourceArtifactRole,
    ) -> Self {
        Self {
            artifact_kind,
            scenario: BridgeSubscriptionSourceArtifactScenario::ReferenceWorkload,
            role,
            lane_kind: None,
            family_kind: Some(family_kind),
        }
    }

    pub(super) fn artifact_kind(&self) -> BridgeSubscriptionSourceArtifactKind {
        self.artifact_kind
    }

    pub(super) fn identity(&self) -> Arc<str> {
        Arc::from(self.canonical_identity_basis())
    }

    pub(super) fn digest(&self) -> Arc<str> {
        let canonical_basis = self.canonical_digest_basis();
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Arc::from(format!(
            "bridge-subscription-source-artifact-evidence:sha256:{digest:x}"
        ))
    }

    fn canonical_identity_basis(&self) -> String {
        format!(
            "{}:{}:{}{}{}",
            self.scenario.as_str(),
            self.artifact_kind.as_str(),
            self.role.as_str(),
            self.lane_identity_fragment(),
            self.family_identity_fragment(),
        )
    }

    fn canonical_digest_basis(&self) -> String {
        format!(
            "bridge-subscription-source-artifact-evidence|scenario={}|kind={}|role={}{}{}",
            self.scenario.as_str(),
            self.artifact_kind.as_str(),
            self.role.as_str(),
            self.lane_digest_fragment(),
            self.family_digest_fragment(),
        )
    }

    fn lane_identity_fragment(&self) -> String {
        self.lane_kind
            .map(|lane| format!(":lane={}", lane.as_str()))
            .unwrap_or_default()
    }

    fn family_identity_fragment(&self) -> String {
        self.family_kind
            .map(|family| format!(":family={}", family.as_str()))
            .unwrap_or_default()
    }

    fn lane_digest_fragment(&self) -> String {
        self.lane_kind
            .map(|lane| format!("|lane={}", lane.as_str()))
            .unwrap_or_default()
    }

    fn family_digest_fragment(&self) -> String {
        self.family_kind
            .map(|family| format!("|family={}", family.as_str()))
            .unwrap_or_default()
    }
}
