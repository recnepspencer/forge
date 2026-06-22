use super::{BasisEligibilityDisposition, BasisOperationLaneRequest, NormalizedBasisFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisVisibility {
    CurrentHead,
    BranchScoped,
    SnapshotScoped,
    Historical,
    PreviewScoped,
}

impl BasisVisibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHead => "current_head",
            Self::BranchScoped => "branch_scoped",
            Self::SnapshotScoped => "snapshot_scoped",
            Self::Historical => "historical",
            Self::PreviewScoped => "preview_scoped",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecyclePosture {
    Authoritative,
    MutablePreparation,
    HistoricalReplay,
    AdvisoryPreview,
}

impl BasisLifecyclePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::MutablePreparation => "mutable_preparation",
            Self::HistoricalReplay => "historical_replay",
            Self::AdvisoryPreview => "advisory_preview",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnboundLowerRuntimeEvidencePlaceholder {
    BridgeTruthViewAuthority,
    BridgeContinuityAuthority,
    BridgeSubscriptionAuthority,
    BridgePreviewSubscriptionAuthority,
    BridgeWritebackAuthority,
    BridgeCausalEnvelopeAuthority,
    RelationalTruthAuthority,
    SignalObservationAuthority,
}

impl UnboundLowerRuntimeEvidencePlaceholder {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BridgeTruthViewAuthority => "bridge_truth_view_authority",
            Self::BridgeContinuityAuthority => "bridge_continuity_authority",
            Self::BridgeSubscriptionAuthority => "bridge_subscription_authority",
            Self::BridgePreviewSubscriptionAuthority => "bridge_preview_subscription_authority",
            Self::BridgeWritebackAuthority => "bridge_writeback_authority",
            Self::BridgeCausalEnvelopeAuthority => "bridge_causal_envelope_authority",
            Self::RelationalTruthAuthority => "relational_truth_authority",
            Self::SignalObservationAuthority => "signal_observation_authority",
        }
    }
}

pub(super) fn visibility_for_family(family: &NormalizedBasisFamily) -> BasisVisibility {
    match family {
        NormalizedBasisFamily::CurrentHead => BasisVisibility::CurrentHead,
        NormalizedBasisFamily::BranchHead => BasisVisibility::BranchScoped,
        NormalizedBasisFamily::BranchSnapshot | NormalizedBasisFamily::RuntimeSnapshot => {
            BasisVisibility::SnapshotScoped
        }
        NormalizedBasisFamily::HistoricalSnapshot | NormalizedBasisFamily::HistoricalCommit => {
            BasisVisibility::Historical
        }
        NormalizedBasisFamily::Preview | NormalizedBasisFamily::PreviewDerivedHistorical => {
            BasisVisibility::PreviewScoped
        }
    }
}

pub(super) fn lifecycle_posture_for_admission(
    lane: &BasisOperationLaneRequest,
    disposition: &BasisEligibilityDisposition,
) -> BasisLifecyclePosture {
    match disposition {
        BasisEligibilityDisposition::Advisory => BasisLifecyclePosture::AdvisoryPreview,
        BasisEligibilityDisposition::Success => match lane {
            BasisOperationLaneRequest::MutationPreparation => {
                BasisLifecyclePosture::MutablePreparation
            }
            BasisOperationLaneRequest::Replay => BasisLifecyclePosture::HistoricalReplay,
            _ => BasisLifecyclePosture::Authoritative,
        },
    }
}

pub(super) fn placeholders_for_lane(
    lane: &BasisOperationLaneRequest,
    disposition: &BasisEligibilityDisposition,
) -> &'static [UnboundLowerRuntimeEvidencePlaceholder] {
    match (lane, disposition) {
        (BasisOperationLaneRequest::Observation, _) => &[
            UnboundLowerRuntimeEvidencePlaceholder::BridgeTruthViewAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::RelationalTruthAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::SignalObservationAuthority,
        ],
        (BasisOperationLaneRequest::Inspection, _) => &[
            UnboundLowerRuntimeEvidencePlaceholder::BridgeContinuityAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::RelationalTruthAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::SignalObservationAuthority,
        ],
        (
            BasisOperationLaneRequest::SubscriptionDeclaration
            | BasisOperationLaneRequest::SubscriptionActivation,
            BasisEligibilityDisposition::Advisory,
        ) => &[UnboundLowerRuntimeEvidencePlaceholder::BridgePreviewSubscriptionAuthority],
        (
            BasisOperationLaneRequest::SubscriptionDeclaration
            | BasisOperationLaneRequest::SubscriptionActivation,
            BasisEligibilityDisposition::Success,
        ) => &[UnboundLowerRuntimeEvidencePlaceholder::BridgeSubscriptionAuthority],
        (BasisOperationLaneRequest::Materialization, _) => &[
            UnboundLowerRuntimeEvidencePlaceholder::BridgeTruthViewAuthority,
            UnboundLowerRuntimeEvidencePlaceholder::BridgeCausalEnvelopeAuthority,
        ],
        (BasisOperationLaneRequest::MutationPreparation, _) => {
            &[UnboundLowerRuntimeEvidencePlaceholder::BridgeWritebackAuthority]
        }
        _ => &[],
    }
}

pub(super) fn permitted_lanes_for_family(
    family: &NormalizedBasisFamily,
) -> &'static [BasisOperationLaneRequest] {
    match family {
        NormalizedBasisFamily::CurrentHead => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Materialization,
            BasisOperationLaneRequest::SubscriptionDeclaration,
            BasisOperationLaneRequest::SubscriptionActivation,
            BasisOperationLaneRequest::Certification,
            BasisOperationLaneRequest::MutationPreparation,
        ],
        NormalizedBasisFamily::BranchHead => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Materialization,
            BasisOperationLaneRequest::SubscriptionDeclaration,
            BasisOperationLaneRequest::SubscriptionActivation,
            BasisOperationLaneRequest::Certification,
            BasisOperationLaneRequest::MutationPreparation,
        ],
        NormalizedBasisFamily::BranchSnapshot | NormalizedBasisFamily::RuntimeSnapshot => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Materialization,
            BasisOperationLaneRequest::Certification,
        ],
        NormalizedBasisFamily::HistoricalSnapshot | NormalizedBasisFamily::HistoricalCommit => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Materialization,
            BasisOperationLaneRequest::Replay,
            BasisOperationLaneRequest::Certification,
        ],
        NormalizedBasisFamily::Preview => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::PreviewCloseout,
        ],
        NormalizedBasisFamily::PreviewDerivedHistorical => &[
            BasisOperationLaneRequest::Observation,
            BasisOperationLaneRequest::Inspection,
            BasisOperationLaneRequest::Certification,
        ],
    }
}
