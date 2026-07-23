use super::admitted_capability::AdmittedBasisCapability;
use super::counters::BasisEligibilityCounters;
use super::lanes::{
    BasisOperationLane, CertificationLaneWitness, EffectAuthoringLaneWitness,
    InspectionLaneWitness, MaterializationLaneWitness, MutationPreparationLaneWitness,
    ObservationLaneWitness, PreviewCloseoutLaneWitness, ReplayLaneWitness,
    SubscriptionActivationLaneWitness, SubscriptionDeclarationLaneWitness,
};
use super::proofs::{
    AdvisoryBasisEligibility, BasisEligibility, DeferredBasisEligibility, DeniedBasisCapability,
    NormalizedBasisIntent,
};
use super::support::{support_posture_for, BasisSupportPosture};
use super::taxonomy::{
    BasisFamily, BasisLifecyclePosture, BasisVisibilityPosture, DeniedBasisCapabilityKind,
};

pub fn evaluate_basis_observation_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<BasisEligibility<ObservationLaneWitness>, DeniedBasisCapability> {
    evaluate_eligibility(normalized, ObservationLaneWitness::new())
}

pub fn evaluate_basis_mutation_preparation_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<BasisEligibility<MutationPreparationLaneWitness>, DeniedBasisCapability> {
    evaluate_eligibility(normalized, MutationPreparationLaneWitness::new())
}

pub fn evaluate_basis_replay_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<BasisEligibility<ReplayLaneWitness>, DeniedBasisCapability> {
    evaluate_eligibility(normalized, ReplayLaneWitness::new())
}

pub fn evaluate_basis_inspection_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<BasisEligibility<InspectionLaneWitness>, DeniedBasisCapability> {
    evaluate_eligibility(normalized, InspectionLaneWitness::new())
}

pub fn evaluate_basis_inspection_advisory_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<AdvisoryBasisEligibility<InspectionLaneWitness>, DeniedBasisCapability> {
    evaluate_basis_advisory_eligibility(normalized, InspectionLaneWitness::new())
}

pub fn evaluate_basis_materialization_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<BasisEligibility<MaterializationLaneWitness>, DeniedBasisCapability> {
    evaluate_eligibility(normalized, MaterializationLaneWitness::new())
}

pub fn evaluate_basis_subscription_declaration_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<BasisEligibility<SubscriptionDeclarationLaneWitness>, DeniedBasisCapability> {
    evaluate_eligibility(normalized, SubscriptionDeclarationLaneWitness::new())
}

pub fn evaluate_basis_subscription_activation_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<BasisEligibility<SubscriptionActivationLaneWitness>, DeniedBasisCapability> {
    evaluate_eligibility(normalized, SubscriptionActivationLaneWitness::new())
}

pub fn evaluate_basis_preview_closeout_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<BasisEligibility<PreviewCloseoutLaneWitness>, DeniedBasisCapability> {
    evaluate_eligibility(normalized, PreviewCloseoutLaneWitness::new())
}

pub(crate) fn evaluate_basis_certification_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<BasisEligibility<CertificationLaneWitness>, DeniedBasisCapability> {
    evaluate_eligibility(normalized, CertificationLaneWitness::new())
}

pub fn evaluate_basis_effect_authoring_deferred_eligibility(
    normalized: NormalizedBasisIntent,
) -> Result<DeferredBasisEligibility<EffectAuthoringLaneWitness>, DeniedBasisCapability> {
    evaluate_basis_deferred_eligibility(normalized, EffectAuthoringLaneWitness::new())
}

fn evaluate_basis_advisory_eligibility<L: BasisOperationLane>(
    normalized: NormalizedBasisIntent,
    lane: L,
) -> Result<AdvisoryBasisEligibility<L>, DeniedBasisCapability> {
    evaluate_denials(&normalized, L::lane_name())?;
    Ok(AdvisoryBasisEligibility::new(normalized, lane))
}

fn evaluate_basis_deferred_eligibility<L: BasisOperationLane>(
    normalized: NormalizedBasisIntent,
    lane: L,
) -> Result<DeferredBasisEligibility<L>, DeniedBasisCapability> {
    evaluate_denials(&normalized, L::lane_name())?;
    match support_posture_for(normalized.family(), L::lane_name()) {
        BasisSupportPosture::Deferred => Ok(DeferredBasisEligibility::new(
            normalized.clone(),
            lane,
            deferred_denial_kind(&normalized),
            "future-neighbor basis support is deferred and leaves zero operational residue",
        )),
        _ => Err(denial(
            DeniedBasisCapabilityKind::OperationIneligible,
            &normalized,
            "basis family is not eligible for the requested operation lane",
            0,
            0,
            0,
        )),
    }
}

fn evaluate_eligibility<L: BasisOperationLane>(
    normalized: NormalizedBasisIntent,
    lane: L,
) -> Result<BasisEligibility<L>, DeniedBasisCapability> {
    evaluate_denials(&normalized, L::lane_name())?;
    match support_posture_for(normalized.family(), L::lane_name()) {
        BasisSupportPosture::Admitted => Ok(BasisEligibility::new(normalized, lane)),
        BasisSupportPosture::Advisory => Err(denial(
            DeniedBasisCapabilityKind::OperationIneligible,
            &normalized,
            "advisory basis cannot be silently promoted to admitted capability",
            0,
            0,
            0,
        )),
        BasisSupportPosture::Deferred => Err(deferred_denial(&normalized)),
        BasisSupportPosture::Denied | BasisSupportPosture::Unsupported => Err(denial(
            DeniedBasisCapabilityKind::OperationIneligible,
            &normalized,
            "basis family is not eligible for the requested operation lane",
            0,
            0,
            0,
        )),
    }
}

fn evaluate_denials(
    normalized: &NormalizedBasisIntent,
    operation_lane: &'static str,
) -> Result<(), DeniedBasisCapability> {
    let denial_work = EligibilityDenialWorkCounters::for_normalized_basis(normalized);

    if let Some(kind) = policy_or_scope_denial_kind(normalized) {
        return Err(denial(
            kind,
            normalized,
            "policy or tenant/schema posture denies the requested basis lane",
            denial_work.policy_check_count,
            denial_work.tenant_schema_check_count,
            denial_work.lower_runtime_evidence_check_count,
        ));
    }

    if preview_basis_has_drifted(normalized) {
        return Err(denial(
            DeniedBasisCapabilityKind::PreviewDrifted,
            normalized,
            "stale preview basis denies before operation artifacts exist",
            denial_work.policy_check_count,
            denial_work.tenant_schema_check_count,
            denial_work.lower_runtime_evidence_check_count,
        ));
    }

    if historical_replay_lacks_retained_support(normalized, operation_lane) {
        return Err(denial(
            DeniedBasisCapabilityKind::HistoricalReplayUnsupported,
            normalized,
            "historical replay is unsupported for this retained basis",
            denial_work.policy_check_count,
            denial_work.tenant_schema_check_count,
            denial_work.lower_runtime_evidence_check_count,
        ));
    }

    if lower_runtime_binding_is_missing(normalized) {
        return Err(denial(
            DeniedBasisCapabilityKind::LowerRuntimeBindingMissing,
            normalized,
            "lower-runtime basis binding is required before eligibility can admit",
            denial_work.policy_check_count,
            denial_work.tenant_schema_check_count,
            denial_work.lower_runtime_evidence_check_count,
        ));
    }

    Ok(())
}

struct EligibilityDenialWorkCounters {
    policy_check_count: usize,
    tenant_schema_check_count: usize,
    lower_runtime_evidence_check_count: usize,
}

impl EligibilityDenialWorkCounters {
    fn for_normalized_basis(normalized: &NormalizedBasisIntent) -> Self {
        Self {
            policy_check_count: usize::from(matches!(
                normalized.visibility(),
                BasisVisibilityPosture::PolicyMasked | BasisVisibilityPosture::Advisory
            )),
            tenant_schema_check_count: usize::from(matches!(
                normalized.family(),
                BasisFamily::TenantScoped | BasisFamily::PolicyScoped
            )),
            lower_runtime_evidence_check_count: usize::from(
                normalized.lower_runtime_binding_digest().is_some(),
            ),
        }
    }
}

fn policy_or_scope_denial_kind(
    normalized: &NormalizedBasisIntent,
) -> Option<DeniedBasisCapabilityKind> {
    (normalized.visibility() == BasisVisibilityPosture::PolicyMasked).then(|| {
        normalized
            .eligibility_denial_cause()
            .map(|cause| cause.denied_capability_kind())
            .unwrap_or(DeniedBasisCapabilityKind::PolicyMasked)
    })
}

fn preview_basis_has_drifted(normalized: &NormalizedBasisIntent) -> bool {
    normalized.lifecycle() == BasisLifecyclePosture::PreviewStale
}

fn historical_replay_lacks_retained_support(
    normalized: &NormalizedBasisIntent,
    operation_lane: &'static str,
) -> bool {
    operation_lane == "replay"
        && matches!(
            normalized.family(),
            BasisFamily::HistoricalSnapshot | BasisFamily::HistoricalCommit
        )
        && normalized.visibility() == BasisVisibilityPosture::Advisory
}

fn lower_runtime_binding_is_missing(normalized: &NormalizedBasisIntent) -> bool {
    normalized
        .lower_runtime_binding_digest()
        .is_some_and(|digest| digest.starts_with("missing-runtime-snapshot:"))
}

fn deferred_denial(normalized: &NormalizedBasisIntent) -> DeniedBasisCapability {
    let kind = deferred_denial_kind(normalized);
    denial(
        kind,
        normalized,
        "future-neighbor basis support is deferred and leaves zero operational residue",
        0,
        0,
        1,
    )
}

fn deferred_denial_kind(normalized: &NormalizedBasisIntent) -> DeniedBasisCapabilityKind {
    match normalized.family() {
        BasisFamily::DurableReload => DeniedBasisCapabilityKind::DurableOverclaim,
        BasisFamily::StoreBacked => DeniedBasisCapabilityKind::StoreBackedDeferred,
        _ => DeniedBasisCapabilityKind::OperationIneligible,
    }
}

fn denial(
    kind: DeniedBasisCapabilityKind,
    normalized: &NormalizedBasisIntent,
    message: &'static str,
    policy_check_count: usize,
    tenant_schema_check_count: usize,
    lower_runtime_evidence_check_count: usize,
) -> DeniedBasisCapability {
    DeniedBasisCapability::new(
        kind,
        normalized,
        message,
        BasisEligibilityCounters::eligibility(
            policy_check_count,
            tenant_schema_check_count,
            lower_runtime_evidence_check_count,
            0,
        ),
    )
}

pub fn admit_basis_capability<L: BasisOperationLane>(
    eligibility: BasisEligibility<L>,
) -> AdmittedBasisCapability<L> {
    AdmittedBasisCapability::new(eligibility)
}
