use forge_runtime_bridge::facade::{
    AdmittedBridgeSubscription, BridgeDeliveredContinuityResult, BridgeTruthViewEvaluation,
    BridgeTruthViewKind, TruthBranchIdentity, TruthCommitIdentity, TruthSnapshotIdentity,
};

use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::identity_authority::{
    admit_query_feeder_authority_identity, QueryFeederAuthorityIdentity, QueryFeederIdentityKind,
};

use super::binding_evidence::binding_identity;
use super::{
    denied_basis_capability_for_lower_runtime_mismatch,
    denied_basis_capability_for_lower_runtime_unsupported, AdmittedBasisCapability,
    BasisCapabilityAdmission, BasisEligibilityCounters, BridgeLowerRuntimeEvidenceReference,
    DeniedBasisCapability, InspectionBasisCapability, NormalizedBasisFamily,
    ObservationBasisCapability, SubscriptionActivationBasisCapability,
    SubscriptionDeclarationBasisCapability,
};

const BRIDGE_AUTHORITY: &str = "forge_runtime_bridge";

type LowerRuntimeBindingAuthority =
    QueryFeederAuthorityIdentity<ForgeQueryEvidenceIdentity, QueryFeederIdentityKind>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBoundObservationBasis {
    capability: ObservationBasisCapability,
    evidence: BridgeLowerRuntimeEvidenceReference,
    binding_authority: LowerRuntimeBindingAuthority,
    counters: BasisEligibilityCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBoundInspectionBasis {
    capability: InspectionBasisCapability,
    evidence: BridgeLowerRuntimeEvidenceReference,
    binding_authority: LowerRuntimeBindingAuthority,
    counters: BasisEligibilityCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBoundSubscriptionDeclarationBasis {
    capability: SubscriptionDeclarationBasisCapability,
    evidence: BridgeLowerRuntimeEvidenceReference,
    binding_authority: LowerRuntimeBindingAuthority,
    counters: BasisEligibilityCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerRuntimeBoundSubscriptionActivationBasis {
    capability: SubscriptionActivationBasisCapability,
    evidence: BridgeLowerRuntimeEvidenceReference,
    binding_authority: LowerRuntimeBindingAuthority,
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

            pub fn binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
                self.binding_authority.value()
            }

            pub fn binding_authority(&self) -> &LowerRuntimeBindingAuthority {
                &self.binding_authority
            }

            pub fn binding_for_reporting(&self) -> &str {
                self.binding_authority.value().as_str()
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
        Some(truth_branch_label(selector.branch_identity())),
        selector.commit_identity().map(truth_commit_label),
        selector.snapshot_identity().map(truth_snapshot_label),
    )?;

    let counters = admitted.counters().clone().with_lower_runtime_check(0);
    let evidence = BridgeLowerRuntimeEvidenceReference::truth_view(
        evaluation.record().record_identity().bridge_admission_evidence(),
        selector.selector_identity().bridge_admission_evidence(),
        evaluation
            .record()
            .decision_log()
            .authority_digest()
            .to_string(),
        evaluation.snapshot_identity().bridge_admission_evidence(),
    );
    Ok(LowerRuntimeBoundObservationBasis {
        capability,
        binding_authority: admit_query_feeder_authority_identity(binding_identity(
            admitted.capability_digest(),
            &evidence,
        )),
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
        Some(truth_branch_label(record.route_record().source_branch())),
        Some(truth_commit_label(record.route_record().source_commit())),
        Some(truth_snapshot_label(record.source_snapshot())),
    )?;

    let counters = admitted.counters().clone().with_lower_runtime_check(0);
    let evidence = BridgeLowerRuntimeEvidenceReference::continuity(
        record.route_identity().bridge_admission_evidence(),
        continuity.continuity_identity().bridge_admission_evidence(),
        record.continuity_resolution_digest().to_string(),
        record.source_snapshot().bridge_admission_evidence(),
    );
    Ok(LowerRuntimeBoundInspectionBasis {
        capability,
        binding_authority: admit_query_feeder_authority_identity(binding_identity(
            admitted.capability_digest(),
            &evidence,
        )),
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
        binding_authority: admit_query_feeder_authority_identity(binding_identity(
            admitted.capability_digest(),
            &evidence,
        )),
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
        binding_authority: admit_query_feeder_authority_identity(binding_identity(
            admitted.capability_digest(),
            &evidence,
        )),
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
    branch_identity: Option<String>,
    commit_identity: Option<String>,
    snapshot_identity: Option<String>,
) -> Result<(), DeniedBasisCapability> {
    let (expected, observed) = match admitted.family() {
        NormalizedBasisFamily::BranchHead => (
            format!("branch_head:{}", admitted.scope_label()),
            format!("branch_head:{}", branch_identity.as_deref().unwrap_or("-")),
        ),
        NormalizedBasisFamily::BranchSnapshot => (
            format!("branch_snapshot:{}", admitted.scope_label()),
            format!(
                "branch_snapshot:{}@{}",
                branch_identity.as_deref().unwrap_or("-"),
                snapshot_identity.as_deref().unwrap_or("-")
            ),
        ),
        NormalizedBasisFamily::RuntimeSnapshot | NormalizedBasisFamily::HistoricalSnapshot => (
            format!("snapshot:{}", admitted.scope_label()),
            format!("snapshot:{}", snapshot_identity.as_deref().unwrap_or("-")),
        ),
        NormalizedBasisFamily::HistoricalCommit => (
            format!("historical_commit:{}", admitted.scope_label()),
            format!(
                "historical_commit:{}",
                commit_identity.as_deref().unwrap_or("-")
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
    branch_identity: Option<String>,
    commit_identity: Option<String>,
    snapshot_identity: Option<String>,
) -> Result<(), DeniedBasisCapability> {
    let (expected, observed) = match admitted.family() {
        NormalizedBasisFamily::BranchHead => (
            format!("branch_head:{}", admitted.scope_label()),
            format!("branch_head:{}", branch_identity.as_deref().unwrap_or("-")),
        ),
        NormalizedBasisFamily::BranchSnapshot => (
            format!("branch_snapshot:{}", admitted.scope_label()),
            format!(
                "branch_snapshot:{}@{}",
                branch_identity.as_deref().unwrap_or("-"),
                snapshot_identity.as_deref().unwrap_or("-")
            ),
        ),
        NormalizedBasisFamily::RuntimeSnapshot | NormalizedBasisFamily::HistoricalSnapshot => (
            format!("snapshot:{}", admitted.scope_label()),
            format!("snapshot:{}", snapshot_identity.as_deref().unwrap_or("-")),
        ),
        NormalizedBasisFamily::HistoricalCommit => (
            format!("historical_commit:{}", admitted.scope_label()),
            format!(
                "historical_commit:{}",
                commit_identity.as_deref().unwrap_or("-")
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
                    .map(truth_branch_label)
                    .as_deref()
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
            .bridge_admission_evidence(),
        admitted_subscription
            .basis_binding()
            .basis_identity()
            .bridge_admission_evidence(),
        admitted_subscription
            .signal_strategy()
            .strategy_identity()
            .bridge_admission_evidence(),
        admitted_subscription.digest().to_string(),
    )
}

fn truth_branch_label(identity: &TruthBranchIdentity) -> String {
    identity.bridge_admission_evidence().terminal_projection_for_reporting().to_string()
}

fn truth_commit_label(identity: &TruthCommitIdentity) -> String {
    identity.bridge_admission_evidence().terminal_projection_for_reporting().to_string()
}

fn truth_snapshot_label(identity: &TruthSnapshotIdentity) -> String {
    identity.bridge_admission_evidence().terminal_projection_for_reporting().to_string()
}
