use forge_foundational::facade::{
    claim_receipt_evidence_boundary_surface, materialize_descriptive_boundary_surface, profiles,
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundaryMaterializationSeam,
    FoundationalBoundaryMaterializationSource, FoundationalBoundaryReceiptSurface,
    MaterializedFoundationalProfileSet, RetentionDeliveryProfile, SupportPostureProfile,
};
use forge_proof::TransitionOutcome;

use crate::application::{
    ForgeQueryDeclarationFoundationalEvidence, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRoutePlan, ForgeQueryDeclarationRoutePlanDenialCause,
    ForgeQueryDomainEntryMarker,
};

use super::{
    artifact::{
        ForgeQueryDeclarationReceipt, ForgeQueryDeclarationReceiptClass,
        ForgeQueryDeclarationReceiptKind,
    },
    denial::ForgeQueryDeclarationReceiptDenialCause,
    explain::ForgeQueryDeclarationReceiptExplanation,
};
use crate::application::{
    materialized_profile_for_tier as orchestration_materialized_profile_for_tier,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
};

pub(crate) fn receipt_from_plan<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    plan: ForgeQueryDeclarationRoutePlan<D, I>,
    materialized_profile: &MaterializedFoundationalProfileSet,
) -> Result<
    ForgeQueryDeclarationReceipt<D, I>,
    (
        ForgeQueryDeclarationRoutePlan<D, I>,
        ForgeQueryDeclarationReceiptDenialCause,
    ),
> {
    let kind = match kind_for_plan(&plan) {
        Ok(kind) => kind,
        Err(cause) => return Err((plan, cause)),
    };
    let explanation = ForgeQueryDeclarationReceiptExplanation::new(
        "successful crossing recorded",
        plan.primary_route()
            .map(|route| format!("primary-route:{}", route.family().as_str())),
        retained_truths_from_plan(&plan),
        governing_reason_from_plan(&plan),
    );
    Ok(build_receipt(
        ForgeQueryDeclarationReceiptClass::CoveredCrossing,
        kind,
        Some(plan),
        None,
        None,
        explanation,
        None,
        None,
        None,
        None,
        materialized_profile,
    ))
}

pub(crate) fn deferred_receipt<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<crate::application::ForgeQueryDeclarationRouteIntent>,
    route_contract: crate::application::ForgeQueryDeclarationRouteContract,
    reason: &'static str,
    materialized_profile: &MaterializedFoundationalProfileSet,
) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptDenialCause> {
    let explanation = ForgeQueryDeclarationReceiptExplanation::new(
        "deferred crossing recorded",
        route_reference_for_non_success(Some(route_contract), route_intent),
        retained_truths_from_evidence(&evidence, Some(route_contract), route_intent),
        reason.to_string(),
    );
    Ok(build_receipt(
        ForgeQueryDeclarationReceiptClass::DeferredCrossing,
        ForgeQueryDeclarationReceiptKind::Deferred,
        None,
        Some(evidence),
        None,
        explanation,
        Some(route_contract),
        route_intent,
        None,
        None,
        materialized_profile,
    ))
}

pub(crate) fn denied_receipt<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<crate::application::ForgeQueryDeclarationRouteIntent>,
    route_contract: Option<crate::application::ForgeQueryDeclarationRouteContract>,
    route_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: ForgeQueryDeclarationReceiptDenialCause,
    route_reference_override: Option<String>,
    extra_route_truths: Vec<String>,
    materialized_profile: &MaterializedFoundationalProfileSet,
) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptDenialCause> {
    let reason = route_cause
        .map(|cause| cause.reason().to_string())
        .unwrap_or_else(|| receipt_cause.reason().to_string());
    let mut retained_truths =
        retained_truths_from_evidence(&evidence, route_contract, route_intent);
    retained_truths.extend(extra_route_truths);
    let explanation = ForgeQueryDeclarationReceiptExplanation::new(
        "denied crossing recorded",
        route_reference_override
            .or_else(|| route_reference_for_non_success(route_contract, route_intent)),
        retained_truths,
        reason,
    );
    Ok(build_receipt(
        ForgeQueryDeclarationReceiptClass::DeniedCrossing,
        ForgeQueryDeclarationReceiptKind::Denied,
        None,
        Some(evidence),
        route_cause,
        explanation,
        route_contract,
        route_intent,
        route_cause,
        Some(receipt_cause),
        materialized_profile,
    ))
}

pub(crate) fn failed_receipt<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<crate::application::ForgeQueryDeclarationRouteIntent>,
    route_contract: crate::application::ForgeQueryDeclarationRouteContract,
    reason: &'static str,
    materialized_profile: &MaterializedFoundationalProfileSet,
) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptDenialCause> {
    let explanation = ForgeQueryDeclarationReceiptExplanation::new(
        "failed crossing recorded",
        route_reference_for_non_success(Some(route_contract), route_intent),
        retained_truths_from_evidence(&evidence, Some(route_contract), route_intent),
        reason.to_string(),
    );
    Ok(build_receipt(
        ForgeQueryDeclarationReceiptClass::FailedCrossing,
        ForgeQueryDeclarationReceiptKind::Failed,
        None,
        Some(evidence),
        None,
        explanation,
        Some(route_contract),
        route_intent,
        None,
        None,
        materialized_profile,
    ))
}

fn build_receipt<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    class: ForgeQueryDeclarationReceiptClass,
    kind: ForgeQueryDeclarationReceiptKind,
    route_plan: Option<ForgeQueryDeclarationRoutePlan<D, I>>,
    evidence: Option<ForgeQueryDeclarationFoundationalEvidence<D, I>>,
    route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    explanation: ForgeQueryDeclarationReceiptExplanation,
    route_contract: Option<crate::application::ForgeQueryDeclarationRouteContract>,
    route_intent: Option<crate::application::ForgeQueryDeclarationRouteIntent>,
    route_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    materialized_profile: &MaterializedFoundationalProfileSet,
) -> ForgeQueryDeclarationReceipt<D, I> {
    let evidence_ref = match (route_plan.as_ref(), evidence.as_ref()) {
        (Some(plan), None) => plan.foundational_evidence(),
        (None, Some(evidence)) => evidence,
        _ => {
            panic!("receipt construction must be sourced from exactly one retained evidence owner")
        }
    };
    let surface = FoundationalBoundaryReceiptSurface::new(
        completed_boundary_text(kind, evidence_ref.declaration_family_key()),
        route_plan
            .as_ref()
            .map_or(1, |plan| plan.route_count().max(1)),
    )
    .expect("receipt surface should be constructible from retained crossing truth");
    let boundary_receipt = materialize_descriptive_boundary_surface(
        claim_receipt_evidence_boundary_surface(surface),
        FoundationalBoundaryMaterializationSource::CompatibilityLowered,
        FoundationalBoundaryMaterializationSeam::BoundaryExchange,
        materialized_profile.clone(),
    )
    .expect("retained crossing truth should materialize a foundational boundary receipt");
    let descriptive_receipt = evidence_ref.receipt().cloned();
    let version = evidence_ref
        .subject()
        .canonical_declaration()
        .version()
        .foundational()
        .clone();
    let receipt_digest = super::digest::derive_receipt_digest(
        version,
        evidence_ref.handle_identity_digest(),
        evidence_ref.operating_context_identity_digest(),
        evidence_ref.declaration_family_key(),
        evidence_ref.declaration_digest(),
        evidence_ref.progression_digest(),
        route_plan.as_ref().map(|plan| plan.route_plan_digest()),
        class,
        kind,
        evidence_ref.class(),
        evidence_ref.support_digest(),
        evidence_ref.legality_digest(),
        route_contract.map(|contract| contract.reason()),
        route_intent.map(|intent| intent.as_str()),
        route_cause.map(|cause| cause.reason()),
        receipt_cause.map(|cause| cause.reason()),
    );
    ForgeQueryDeclarationReceipt::new(
        class,
        kind,
        route_plan,
        evidence,
        route_denial_cause,
        explanation,
        descriptive_receipt,
        boundary_receipt,
        receipt_digest,
    )
}

fn kind_for_plan<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    plan: &ForgeQueryDeclarationRoutePlan<D, I>,
) -> Result<ForgeQueryDeclarationReceiptKind, ForgeQueryDeclarationReceiptDenialCause> {
    match plan.class() {
        crate::application::ForgeQueryDeclarationRoutePlanClass::RelationalOnly => {
            Ok(ForgeQueryDeclarationReceiptKind::Relational)
        }
        crate::application::ForgeQueryDeclarationRoutePlanClass::BridgeOnly => {
            Ok(ForgeQueryDeclarationReceiptKind::Bridge)
        }
        crate::application::ForgeQueryDeclarationRoutePlanClass::Mixed => {
            Ok(ForgeQueryDeclarationReceiptKind::Mixed)
        }
        crate::application::ForgeQueryDeclarationRoutePlanClass::SignalOnly => {
            Err(ForgeQueryDeclarationReceiptDenialCause::UnsupportedReceiptKind)
        }
    }
}

pub(crate) fn default_receipt_materialized_profile() -> &'static MaterializedFoundationalProfileSet
{
    static ONCE: std::sync::OnceLock<MaterializedFoundationalProfileSet> =
        std::sync::OnceLock::new();
    ONCE.get_or_init(build_default_receipt_materialized_profile)
}

fn build_default_receipt_materialized_profile() -> MaterializedFoundationalProfileSet {
    let requested = profiles()
        .set()
        .diagnostic_richness(DiagnosticRichnessProfile::Standard)
        .support_posture(SupportPostureProfile::SupportReady)
        .compatibility_posture(CompatibilityPostureProfile::CompatibilityLowered)
        .admission_readiness(AdmissionReadinessProfile::Admitted)
        .retention_delivery(RetentionDeliveryProfile::Retained)
        .certification_posture(CertificationPostureProfile::Uncertified)
        .request()
        .expect("static receipt profile should compose");
    let admitted = match profiles().progression().admit_same(requested) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("receipt profile admission should succeed: {outcome:?}"),
    };
    match profiles().progression().materialize_same(admitted) {
        TransitionOutcome::Success(value) => *value.payload(),
        outcome => panic!("receipt profile materialization should succeed: {outcome:?}"),
    }
}

pub(crate) fn receipt_materialized_profile_for_tier(
    tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
) -> MaterializedFoundationalProfileSet {
    orchestration_materialized_profile_for_tier(tier)
}

fn completed_boundary_text(kind: ForgeQueryDeclarationReceiptKind, family: &str) -> String {
    format!(
        "{} declaration boundary receipt for {}",
        match kind {
            ForgeQueryDeclarationReceiptKind::Relational => "relational",
            ForgeQueryDeclarationReceiptKind::Bridge => "bridge",
            ForgeQueryDeclarationReceiptKind::Mixed => "mixed",
            ForgeQueryDeclarationReceiptKind::Deferred => "deferred",
            ForgeQueryDeclarationReceiptKind::Denied => "denied",
            ForgeQueryDeclarationReceiptKind::Failed => "failed",
        },
        family
    )
}

fn retained_truths_from_plan<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    plan: &ForgeQueryDeclarationRoutePlan<D, I>,
) -> Vec<String> {
    let mut truths = retained_truths_from_evidence(plan.foundational_evidence(), None, None);
    truths.push(format!("route-plan:{}", plan.route_plan_digest()));
    truths.push(format!("route-count:{}", plan.route_count()));
    for family in plan.route_families() {
        truths.push(format!("route-family:{}", family.as_str()));
    }
    truths
}

fn retained_truths_from_evidence<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    evidence: &ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_contract: Option<crate::application::ForgeQueryDeclarationRouteContract>,
    route_intent: Option<crate::application::ForgeQueryDeclarationRouteIntent>,
) -> Vec<String> {
    let mut truths = vec![
        format!("handle:{}", evidence.handle_identity_digest()),
        format!(
            "operating_context:{}",
            evidence.operating_context_identity_digest()
        ),
        format!("declaration:{}", evidence.declaration_digest()),
        format!("support:{}", evidence.support_digest()),
    ];
    if let Some(progression) = evidence.progression_digest() {
        truths.push(format!("progression:{progression}"));
    }
    if let Some(contract) = route_contract {
        truths.push(format!("route-contract:{}", contract.reason()));
        for family in contract.allowed_route_families() {
            truths.push(format!("route-family:{}", family.as_str()));
        }
    }
    if let Some(intent) = route_intent {
        truths.push(format!("route-intent:{}", intent.as_str()));
    }
    truths
}

fn governing_reason_from_plan<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    plan: &ForgeQueryDeclarationRoutePlan<D, I>,
) -> String {
    let mut parts = vec![plan.explain().route_contract_reason().to_string()];
    parts.extend(plan.explain().route_segment_reasons().iter().cloned());
    if let Some(intent_reason) = plan.explain().intent_reason() {
        parts.push(intent_reason.to_string());
    }
    parts.join("; ")
}

fn route_reference_for_non_success(
    route_contract: Option<crate::application::ForgeQueryDeclarationRouteContract>,
    route_intent: Option<crate::application::ForgeQueryDeclarationRouteIntent>,
) -> Option<String> {
    let contract_token = route_contract.map(|contract| {
        contract
            .allowed_route_families()
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>()
            .join("+")
    });
    match (contract_token, route_intent) {
        (Some(family_tokens), Some(intent)) => Some(format!(
            "contract:{family_tokens}|intent:{}",
            intent.as_str()
        )),
        (Some(family_tokens), None) => Some(format!("contract:{family_tokens}")),
        (None, Some(intent)) => Some(format!("intent:{}", intent.as_str())),
        (None, None) => None,
    }
}
