use forge_runtime_bridge::facade::{
    AdmittedBridgeSubscription, BridgeDeliveredContinuityResult, BridgeTruthViewEvaluation,
    BridgeTruthViewKind,
};

use crate::identity::hash_parts;

use super::{
    denied_basis_capability_for_lower_runtime_mismatch,
    denied_basis_capability_for_lower_runtime_unsupported, AdmittedBasisCapability,
    BasisCapabilityAdmission, BasisEligibilityCounters, DeniedBasisCapability,
    InspectionBasisCapability, NormalizedBasisFamily, ObservationBasisCapability,
    SubscriptionActivationBasisCapability, SubscriptionDeclarationBasisCapability,
};

const BRIDGE_AUTHORITY: &str = "forge_runtime_bridge";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeLowerRuntimeEvidenceKind {
    TruthViewEvaluation,
    ContinuityDelivery,
    SubscriptionAdmission,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeLowerRuntimeEvidenceReference {
    kind: BridgeLowerRuntimeEvidenceKind,
    detail: BridgeLowerRuntimeEvidenceDetail,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum BridgeLowerRuntimeEvidenceDetail {
    TruthViewEvaluation {
        record_identity: String,
        selector_identity: String,
        authority_digest: String,
        snapshot_identity: String,
    },
    ContinuityDelivery {
        route_identity: String,
        continuity_identity: String,
        continuity_resolution_digest: String,
        source_snapshot_identity: String,
    },
    SubscriptionAdmission {
        admitted_subscription_identity: String,
        basis_identity: String,
        strategy_identity: String,
        subscription_digest: String,
    },
}

impl BridgeLowerRuntimeEvidenceReference {
    pub fn kind(&self) -> BridgeLowerRuntimeEvidenceKind {
        self.kind
    }

    pub fn record_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                record_identity, ..
            } => Some(record_identity.as_str()),
            _ => None,
        }
    }

    pub fn selector_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                selector_identity, ..
            } => Some(selector_identity.as_str()),
            _ => None,
        }
    }

    pub fn authority_digest(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                authority_digest, ..
            } => Some(authority_digest.as_str()),
            _ => None,
        }
    }

    pub fn snapshot_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                snapshot_identity, ..
            } => Some(snapshot_identity.as_str()),
            _ => None,
        }
    }

    pub fn route_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery { route_identity, .. } => {
                Some(route_identity.as_str())
            }
            _ => None,
        }
    }

    pub fn continuity_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                continuity_identity,
                ..
            } => Some(continuity_identity.as_str()),
            _ => None,
        }
    }

    pub fn continuity_resolution_digest(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                continuity_resolution_digest,
                ..
            } => Some(continuity_resolution_digest.as_str()),
            _ => None,
        }
    }

    pub fn source_snapshot_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                source_snapshot_identity,
                ..
            } => Some(source_snapshot_identity.as_str()),
            _ => None,
        }
    }

    pub fn admitted_subscription_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                admitted_subscription_identity,
                ..
            } => Some(admitted_subscription_identity.as_str()),
            _ => None,
        }
    }

    pub fn basis_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission { basis_identity, .. } => {
                Some(basis_identity.as_str())
            }
            _ => None,
        }
    }

    pub fn strategy_identity(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                strategy_identity, ..
            } => Some(strategy_identity.as_str()),
            _ => None,
        }
    }

    pub fn subscription_digest(&self) -> Option<&str> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                subscription_digest,
                ..
            } => Some(subscription_digest.as_str()),
            _ => None,
        }
    }

    fn truth_view(
        record_identity: String,
        selector_identity: String,
        authority_digest: String,
        snapshot_identity: String,
    ) -> Self {
        Self {
            kind: BridgeLowerRuntimeEvidenceKind::TruthViewEvaluation,
            detail: BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                record_identity,
                selector_identity,
                authority_digest,
                snapshot_identity,
            },
        }
    }

    fn continuity(
        route_identity: String,
        continuity_identity: String,
        continuity_resolution_digest: String,
        source_snapshot_identity: String,
    ) -> Self {
        Self {
            kind: BridgeLowerRuntimeEvidenceKind::ContinuityDelivery,
            detail: BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                route_identity,
                continuity_identity,
                continuity_resolution_digest,
                source_snapshot_identity,
            },
        }
    }

    fn subscription(
        admitted_subscription_identity: String,
        basis_identity: String,
        strategy_identity: String,
        subscription_digest: String,
    ) -> Self {
        Self {
            kind: BridgeLowerRuntimeEvidenceKind::SubscriptionAdmission,
            detail: BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                admitted_subscription_identity,
                basis_identity,
                strategy_identity,
                subscription_digest,
            },
        }
    }

    fn digest_parts(&self) -> Vec<String> {
        match &self.detail {
            BridgeLowerRuntimeEvidenceDetail::TruthViewEvaluation {
                record_identity,
                selector_identity,
                authority_digest,
                snapshot_identity,
            } => vec![
                "kind:truth_view_evaluation".to_string(),
                format!("record_identity:{record_identity}"),
                format!("selector_identity:{selector_identity}"),
                format!("authority_digest:{authority_digest}"),
                format!("snapshot_identity:{snapshot_identity}"),
            ],
            BridgeLowerRuntimeEvidenceDetail::ContinuityDelivery {
                route_identity,
                continuity_identity,
                continuity_resolution_digest,
                source_snapshot_identity,
            } => vec![
                "kind:continuity_delivery".to_string(),
                format!("route_identity:{route_identity}"),
                format!("continuity_identity:{continuity_identity}"),
                format!("continuity_resolution_digest:{continuity_resolution_digest}"),
                format!("source_snapshot_identity:{source_snapshot_identity}"),
            ],
            BridgeLowerRuntimeEvidenceDetail::SubscriptionAdmission {
                admitted_subscription_identity,
                basis_identity,
                strategy_identity,
                subscription_digest,
            } => vec![
                "kind:subscription_admission".to_string(),
                format!("admitted_subscription_identity:{admitted_subscription_identity}"),
                format!("basis_identity:{basis_identity}"),
                format!("strategy_identity:{strategy_identity}"),
                format!("subscription_digest:{subscription_digest}"),
            ],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBoundObservationBasis {
    capability: ObservationBasisCapability,
    evidence: BridgeLowerRuntimeEvidenceReference,
    binding_digest: String,
    counters: BasisEligibilityCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBoundInspectionBasis {
    capability: InspectionBasisCapability,
    evidence: BridgeLowerRuntimeEvidenceReference,
    binding_digest: String,
    counters: BasisEligibilityCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBoundSubscriptionDeclarationBasis {
    capability: SubscriptionDeclarationBasisCapability,
    evidence: BridgeLowerRuntimeEvidenceReference,
    binding_digest: String,
    counters: BasisEligibilityCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBoundSubscriptionActivationBasis {
    capability: SubscriptionActivationBasisCapability,
    evidence: BridgeLowerRuntimeEvidenceReference,
    binding_digest: String,
    counters: BasisEligibilityCounters,
}

macro_rules! impl_bound_accessors {
    ($name:ident) => {
        impl $name {
            pub fn capability(&self) -> &super::BasisCapabilityAdmission {
                self.capability.admission()
            }

            pub fn authority_name(&self) -> &'static str {
                BRIDGE_AUTHORITY
            }

            pub fn evidence(&self) -> &BridgeLowerRuntimeEvidenceReference {
                &self.evidence
            }

            pub fn binding_digest(&self) -> &str {
                &self.binding_digest
            }

            pub fn counters(&self) -> &BasisEligibilityCounters {
                &self.counters
            }
        }
    };
}

impl_bound_accessors!(LowerRuntimeBoundObservationBasis);
impl_bound_accessors!(LowerRuntimeBoundInspectionBasis);
impl_bound_accessors!(LowerRuntimeBoundSubscriptionDeclarationBasis);
impl_bound_accessors!(LowerRuntimeBoundSubscriptionActivationBasis);

pub fn readmit_bridge_truth_view_evidence(
    capability: ObservationBasisCapability,
    evaluation: &BridgeTruthViewEvaluation,
) -> Result<LowerRuntimeBoundObservationBasis, DeniedBasisCapability> {
    let admitted = admitted_capability_from_lane(capability.admission())?.clone();
    let selector = evaluation.record().declaration().selector();
    match_truth_view_selector(
        &admitted,
        selector.view_kind(),
        selector.branch_identity().as_str(),
        selector.commit_identity().map(|identity| identity.as_str()),
        selector
            .snapshot_identity()
            .map(|identity| identity.as_str()),
    )?;

    let counters = admitted.counters().clone().with_lower_runtime_check(0);
    let evidence = BridgeLowerRuntimeEvidenceReference::truth_view(
        evaluation.record().record_identity().as_str().to_string(),
        selector.selector_identity().as_str().to_string(),
        evaluation
            .record()
            .decision_log()
            .authority_digest()
            .to_string(),
        evaluation.snapshot_identity().as_str().to_string(),
    );
    Ok(LowerRuntimeBoundObservationBasis {
        capability,
        binding_digest: binding_digest(admitted.capability_digest(), &evidence),
        evidence,
        counters,
    })
}

pub fn readmit_bridge_continuity_evidence(
    capability: InspectionBasisCapability,
    continuity: &BridgeDeliveredContinuityResult,
) -> Result<LowerRuntimeBoundInspectionBasis, DeniedBasisCapability> {
    let admitted = admitted_capability_from_lane(capability.admission())?.clone();
    let record = continuity.canonical_record();
    match_continuity_record(
        &admitted,
        record.route_record().source_branch().as_str(),
        record.route_record().source_commit().as_str(),
        record.source_snapshot().as_str(),
    )?;

    let counters = admitted.counters().clone().with_lower_runtime_check(0);
    let evidence = BridgeLowerRuntimeEvidenceReference::continuity(
        record.route_identity().as_str().to_string(),
        continuity.continuity_identity().as_str().to_string(),
        record.continuity_resolution_digest().to_string(),
        record.source_snapshot().as_str().to_string(),
    );
    Ok(LowerRuntimeBoundInspectionBasis {
        capability,
        binding_digest: binding_digest(admitted.capability_digest(), &evidence),
        evidence,
        counters,
    })
}

pub fn readmit_bridge_subscription_declaration_evidence(
    capability: SubscriptionDeclarationBasisCapability,
    admitted_subscription: &AdmittedBridgeSubscription,
) -> Result<LowerRuntimeBoundSubscriptionDeclarationBasis, DeniedBasisCapability> {
    let admitted = admitted_capability_from_lane(capability.admission())?.clone();
    match_subscription_binding(&admitted, admitted_subscription)?;

    let counters = admitted.counters().clone().with_lower_runtime_check(0);
    let evidence = subscription_evidence(admitted_subscription);
    Ok(LowerRuntimeBoundSubscriptionDeclarationBasis {
        capability,
        binding_digest: binding_digest(admitted.capability_digest(), &evidence),
        evidence,
        counters,
    })
}

pub fn readmit_bridge_subscription_activation_evidence(
    capability: SubscriptionActivationBasisCapability,
    admitted_subscription: &AdmittedBridgeSubscription,
) -> Result<LowerRuntimeBoundSubscriptionActivationBasis, DeniedBasisCapability> {
    let admitted = admitted_capability_from_lane(capability.admission())?.clone();
    match_subscription_binding(&admitted, admitted_subscription)?;

    let counters = admitted.counters().clone().with_lower_runtime_check(0);
    let evidence = subscription_evidence(admitted_subscription);
    Ok(LowerRuntimeBoundSubscriptionActivationBasis {
        capability,
        binding_digest: binding_digest(admitted.capability_digest(), &evidence),
        evidence,
        counters,
    })
}

fn admitted_capability_from_lane(
    admission: &BasisCapabilityAdmission,
) -> Result<&AdmittedBasisCapability, DeniedBasisCapability> {
    match admission {
        BasisCapabilityAdmission::Admitted(capability) => Ok(capability),
        BasisCapabilityAdmission::Advisory(capability) => {
            Err(denied_basis_capability_for_lower_runtime_unsupported(
                capability.normalized_basis_intent_digest(),
                capability.family(),
                capability.operation_lane(),
                capability.counters().clone(),
                BRIDGE_AUTHORITY,
            ))
        }
    }
}

fn match_truth_view_selector(
    admitted: &AdmittedBasisCapability,
    view_kind: BridgeTruthViewKind,
    branch_identity: &str,
    commit_identity: Option<&str>,
    snapshot_identity: Option<&str>,
) -> Result<(), DeniedBasisCapability> {
    let (expected, observed) = match admitted.family() {
        NormalizedBasisFamily::BranchHead => (
            format!("branch_head:{}", admitted.scope_label()),
            format!("branch_head:{branch_identity}"),
        ),
        NormalizedBasisFamily::BranchSnapshot => (
            format!("branch_snapshot:{}", admitted.scope_label()),
            format!(
                "branch_snapshot:{}@{}",
                branch_identity,
                snapshot_identity.unwrap_or("-")
            ),
        ),
        NormalizedBasisFamily::RuntimeSnapshot | NormalizedBasisFamily::HistoricalSnapshot => (
            format!("snapshot:{}", admitted.scope_label()),
            format!("snapshot:{}", snapshot_identity.unwrap_or("-")),
        ),
        NormalizedBasisFamily::HistoricalCommit => (
            format!("historical_commit:{}", admitted.scope_label()),
            format!("historical_commit:{}", commit_identity.unwrap_or("-")),
        ),
        _ => {
            return Err(denied_basis_capability_for_lower_runtime_unsupported(
                admitted.normalized_basis_intent_digest(),
                admitted.family(),
                admitted.operation_lane(),
                admitted.counters().clone(),
                BRIDGE_AUTHORITY,
            ))
        }
    };

    let expected_kind = match admitted.family() {
        NormalizedBasisFamily::BranchHead => BridgeTruthViewKind::BranchHead,
        NormalizedBasisFamily::BranchSnapshot => BridgeTruthViewKind::BranchSnapshot,
        NormalizedBasisFamily::RuntimeSnapshot => BridgeTruthViewKind::CommittedSnapshot,
        NormalizedBasisFamily::HistoricalSnapshot => BridgeTruthViewKind::BranchSnapshot,
        NormalizedBasisFamily::HistoricalCommit => BridgeTruthViewKind::HistoricalCommit,
        _ => unreachable!(),
    };

    if view_kind != expected_kind || expected != observed {
        return Err(denied_basis_capability_for_lower_runtime_mismatch(
            admitted.normalized_basis_intent_digest(),
            admitted.family(),
            admitted.operation_lane(),
            admitted.counters().clone(),
            BRIDGE_AUTHORITY,
            expected,
            observed,
        ));
    }
    Ok(())
}

fn match_continuity_record(
    admitted: &AdmittedBasisCapability,
    branch_identity: &str,
    commit_identity: &str,
    snapshot_identity: &str,
) -> Result<(), DeniedBasisCapability> {
    let (expected, observed) = match admitted.family() {
        NormalizedBasisFamily::BranchHead => (
            format!("branch_head:{}", admitted.scope_label()),
            format!("branch_head:{branch_identity}"),
        ),
        NormalizedBasisFamily::BranchSnapshot => (
            format!("branch_snapshot:{}", admitted.scope_label()),
            format!("branch_snapshot:{branch_identity}@{snapshot_identity}"),
        ),
        NormalizedBasisFamily::RuntimeSnapshot | NormalizedBasisFamily::HistoricalSnapshot => (
            format!("snapshot:{}", admitted.scope_label()),
            format!("snapshot:{snapshot_identity}"),
        ),
        NormalizedBasisFamily::HistoricalCommit => (
            format!("historical_commit:{}", admitted.scope_label()),
            format!("historical_commit:{commit_identity}"),
        ),
        _ => {
            return Err(denied_basis_capability_for_lower_runtime_unsupported(
                admitted.normalized_basis_intent_digest(),
                admitted.family(),
                admitted.operation_lane(),
                admitted.counters().clone(),
                BRIDGE_AUTHORITY,
            ))
        }
    };
    if expected != observed {
        return Err(denied_basis_capability_for_lower_runtime_mismatch(
            admitted.normalized_basis_intent_digest(),
            admitted.family(),
            admitted.operation_lane(),
            admitted.counters().clone(),
            BRIDGE_AUTHORITY,
            expected,
            observed,
        ));
    }
    Ok(())
}

fn match_subscription_binding(
    admitted: &AdmittedBasisCapability,
    admitted_subscription: &AdmittedBridgeSubscription,
) -> Result<(), DeniedBasisCapability> {
    let binding = admitted_subscription.basis_binding();
    let (expected, observed) = match admitted.family() {
        NormalizedBasisFamily::BranchHead => (
            format!("branch_head:{}", admitted.scope_label()),
            format!(
                "branch_head:{}",
                binding
                    .branch_identity()
                    .map(|id| id.as_str())
                    .unwrap_or("-")
            ),
        ),
        _ => {
            return Err(denied_basis_capability_for_lower_runtime_unsupported(
                admitted.normalized_basis_intent_digest(),
                admitted.family(),
                admitted.operation_lane(),
                admitted.counters().clone(),
                BRIDGE_AUTHORITY,
            ))
        }
    };
    if expected != observed {
        return Err(denied_basis_capability_for_lower_runtime_mismatch(
            admitted.normalized_basis_intent_digest(),
            admitted.family(),
            admitted.operation_lane(),
            admitted.counters().clone(),
            BRIDGE_AUTHORITY,
            expected,
            observed,
        ));
    }
    Ok(())
}

fn subscription_evidence(
    admitted_subscription: &AdmittedBridgeSubscription,
) -> BridgeLowerRuntimeEvidenceReference {
    BridgeLowerRuntimeEvidenceReference::subscription(
        admitted_subscription
            .admitted_subscription_identity()
            .as_str()
            .to_string(),
        admitted_subscription
            .basis_binding()
            .basis_identity()
            .as_str()
            .to_string(),
        admitted_subscription
            .signal_strategy()
            .strategy_identity()
            .as_str()
            .to_string(),
        admitted_subscription.digest().to_string(),
    )
}

fn binding_digest(
    capability_digest: &str,
    evidence: &BridgeLowerRuntimeEvidenceReference,
) -> String {
    let mut parts = vec![format!("capability_digest:{capability_digest}")];
    parts.extend(evidence.digest_parts());
    hash_parts(&parts)
}
