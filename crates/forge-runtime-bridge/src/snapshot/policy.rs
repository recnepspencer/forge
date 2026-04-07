use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::input::envelope::TruthBranchIdentity;
use crate::snapshot::{
    BridgeDeliveryIntent, BridgeReplayMode, BridgeTruthViewSelectorIdentity,
    HistoricalEvaluationDeclaration, HistoricalEvaluationDeclarationIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TruthViewRetentionAdmission {
    SnapshotResident,
    HistoricalLookupRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TruthViewSourceCapability {
    DirectSnapshotRead,
    HistoricalLookupAndSnapshotRead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TruthViewReplayCompatibility {
    ReplayPermitted,
    ReplayRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TruthViewPolicyRejectionKind {
    UnsupportedTruthViewSelector,
    UnavailableTruthView,
    SourceCapabilityMismatch,
    BranchMismatch,
    ReplayNotPermitted,
    UnresolvedPolicyConflict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTruthViewPolicyRejection {
    declaration_identity: HistoricalEvaluationDeclarationIdentity,
    selector_identity: BridgeTruthViewSelectorIdentity,
    branch_identity: TruthBranchIdentity,
    replay_mode: BridgeReplayMode,
    delivery_intent: BridgeDeliveryIntent,
    kind: TruthViewPolicyRejectionKind,
    detail: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTruthViewPolicyRejection {
    pub(crate) fn new(
        declaration: &HistoricalEvaluationDeclaration,
        kind: TruthViewPolicyRejectionKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        let detail = detail.into();
        let canonical_basis = Arc::<str>::from(format!(
            "truth-view-policy-rejection|declaration={}|selector={}|branch={}|replay:{:?}|delivery:{:?}|kind:{kind:?}|detail:{}",
            declaration.declaration_identity().as_str(),
            declaration.selector().selector_identity().as_str(),
            declaration.selector().branch_identity().as_str(),
            declaration.replay_mode(),
            declaration.delivery_intent(),
            detail.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration_identity: declaration.declaration_identity().clone(),
            selector_identity: declaration.selector().selector_identity().clone(),
            branch_identity: declaration.selector().branch_identity().clone(),
            replay_mode: declaration.replay_mode(),
            delivery_intent: declaration.delivery_intent(),
            kind,
            detail,
            canonical_basis,
            digest: Arc::from(format!("truth-view-policy-rejection:sha256:{digest:x}")),
        }
    }

    pub fn declaration_identity(&self) -> &HistoricalEvaluationDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn selector_identity(&self) -> &BridgeTruthViewSelectorIdentity {
        &self.selector_identity
    }

    pub fn branch_identity(&self) -> &TruthBranchIdentity {
        &self.branch_identity
    }

    pub fn replay_mode(&self) -> BridgeReplayMode {
        self.replay_mode
    }

    pub fn delivery_intent(&self) -> BridgeDeliveryIntent {
        self.delivery_intent
    }

    pub fn kind(&self) -> TruthViewPolicyRejectionKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedTruthViewPolicy {
    declaration_identity: HistoricalEvaluationDeclarationIdentity,
    selector_identity: BridgeTruthViewSelectorIdentity,
    branch_identity: TruthBranchIdentity,
    retention_admission: TruthViewRetentionAdmission,
    source_capability: TruthViewSourceCapability,
    replay_compatibility: TruthViewReplayCompatibility,
    replay_mode: BridgeReplayMode,
    delivery_intent: BridgeDeliveryIntent,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl ResolvedTruthViewPolicy {
    pub(crate) fn admitted(
        declaration: &HistoricalEvaluationDeclaration,
        retention_admission: TruthViewRetentionAdmission,
        source_capability: TruthViewSourceCapability,
        replay_compatibility: TruthViewReplayCompatibility,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "resolved-truth-view-policy|declaration={}|selector={}|branch={}|retention:{retention_admission:?}|source:{source_capability:?}|replay-compatibility:{replay_compatibility:?}|replay-mode:{:?}|delivery:{:?}",
            declaration.declaration_identity().as_str(),
            declaration.selector().selector_identity().as_str(),
            declaration.selector().branch_identity().as_str(),
            declaration.replay_mode(),
            declaration.delivery_intent(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration_identity: declaration.declaration_identity().clone(),
            selector_identity: declaration.selector().selector_identity().clone(),
            branch_identity: declaration.selector().branch_identity().clone(),
            retention_admission,
            source_capability,
            replay_compatibility,
            replay_mode: declaration.replay_mode(),
            delivery_intent: declaration.delivery_intent(),
            canonical_basis,
            digest: Arc::from(format!("resolved-truth-view-policy:sha256:{digest:x}")),
        }
    }

    pub fn declaration_identity(&self) -> &HistoricalEvaluationDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn selector_identity(&self) -> &BridgeTruthViewSelectorIdentity {
        &self.selector_identity
    }

    pub fn branch_identity(&self) -> &TruthBranchIdentity {
        &self.branch_identity
    }

    pub fn retention_admission(&self) -> TruthViewRetentionAdmission {
        self.retention_admission
    }

    pub fn source_capability(&self) -> TruthViewSourceCapability {
        self.source_capability
    }

    pub fn replay_compatibility(&self) -> TruthViewReplayCompatibility {
        self.replay_compatibility
    }

    pub fn replay_mode(&self) -> BridgeReplayMode {
        self.replay_mode
    }

    pub fn delivery_intent(&self) -> BridgeDeliveryIntent {
        self.delivery_intent
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeTruthViewPolicyResolution {
    Admitted(ResolvedTruthViewPolicy),
    Rejected(BridgeTruthViewPolicyRejection),
}

impl BridgeTruthViewPolicyResolution {
    pub fn declaration_identity(&self) -> &HistoricalEvaluationDeclarationIdentity {
        match self {
            Self::Admitted(policy) => policy.declaration_identity(),
            Self::Rejected(rejection) => rejection.declaration_identity(),
        }
    }

    pub fn selector_identity(&self) -> &BridgeTruthViewSelectorIdentity {
        match self {
            Self::Admitted(policy) => policy.selector_identity(),
            Self::Rejected(rejection) => rejection.selector_identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BridgeTruthViewPolicyRejection, BridgeTruthViewPolicyResolution, ResolvedTruthViewPolicy,
        TruthViewPolicyRejectionKind, TruthViewReplayCompatibility, TruthViewRetentionAdmission,
        TruthViewSourceCapability,
    };
    use crate::input::envelope::TruthBranchIdentity;
    use crate::policy::BridgeDiagnosticsTier;
    use crate::snapshot::{
        BridgeDeliveryIntent, BridgeReplayMode, BridgeTruthViewSelector,
        HistoricalEvaluationDeclaration, TruthSnapshotIdentity,
    };

    #[test]
    fn admitted_policy_is_canonical_for_same_inputs() {
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::DeliverInvalidation,
        );
        let left = ResolvedTruthViewPolicy::admitted(
            &declaration,
            TruthViewRetentionAdmission::SnapshotResident,
            TruthViewSourceCapability::DirectSnapshotRead,
            TruthViewReplayCompatibility::ReplayPermitted,
        );
        let right = ResolvedTruthViewPolicy::admitted(
            &declaration,
            TruthViewRetentionAdmission::SnapshotResident,
            TruthViewSourceCapability::DirectSnapshotRead,
            TruthViewReplayCompatibility::ReplayPermitted,
        );

        assert_eq!(left, right);
        assert!(left
            .canonical_basis()
            .contains("retention:SnapshotResident|source:DirectSnapshotRead"));
    }

    #[test]
    fn rejection_preserves_declaration_identity() {
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("analysis")),
            BridgeReplayMode::Required,
            BridgeDiagnosticsTier::Exhaustive,
            BridgeDeliveryIntent::PrepareOnly,
        );
        let rejection = BridgeTruthViewPolicyRejection::new(
            &declaration,
            TruthViewPolicyRejectionKind::UnavailableTruthView,
            "retained branch head was unavailable",
        );
        let resolution = BridgeTruthViewPolicyResolution::Rejected(rejection.clone());

        assert_eq!(
            rejection.declaration_identity(),
            declaration.declaration_identity()
        );
        assert_eq!(
            resolution.selector_identity(),
            declaration.selector().selector_identity()
        );
        assert!(rejection.detail().contains("unavailable"));
    }
}
