use worth_foundational::facade::{
    claim_receipt_evidence_boundary_surface, materialize_descriptive_boundary_surface,
    FoundationalBoundaryMaterializationSeam, FoundationalBoundaryMaterializationSource,
    FoundationalBoundaryReceiptSurface, MaterializedFoundationalProfileSet,
};

use crate::application::{
    declaration_publication::declaration_publication_for_tier, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectPublication,
    WorthQueryDeclarationEntryOrchestrationMaterializationTier,
    WorthQueryDeclarationFoundationalEvidence, WorthQueryDeclarationInput,
    WorthQueryDeclarationRoutePlan, WorthQueryDeclarationRoutePlanDenialCause,
    WorthQueryDomainEntryMarker,
};

use super::super::{
    artifact::{
        WorthQueryDeclarationReceipt, WorthQueryDeclarationReceiptClass,
        WorthQueryDeclarationReceiptKind,
    },
    denial::WorthQueryDeclarationReceiptDenialCause,
    explain::WorthQueryDeclarationReceiptExplanation,
};
use super::truth::{
    completed_boundary_text, evidence_aspect_coverage, governing_reason_from_plan,
    retained_truths_from_evidence, retained_truths_from_plan, route_reference_for_non_success,
    route_scoped_aspect_contract,
};

struct ReceiptCrossingAspectState {
    contract: WorthQueryDeclarationAspectContract,
    coverage: WorthQueryDeclarationAspectCoverage,
    publication: WorthQueryDeclarationAspectPublication,
}

pub(crate) fn receipt_from_plan<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    plan: WorthQueryDeclarationRoutePlan<D, I>,
    materialized_profile: &MaterializedFoundationalProfileSet,
    receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> Result<
    WorthQueryDeclarationReceipt<D, I>,
    (
        WorthQueryDeclarationRoutePlan<D, I>,
        WorthQueryDeclarationReceiptDenialCause,
    ),
> {
    receipt_from_plan_with_tier(plan, materialized_profile, receipt_tier)
}

pub(crate) fn receipt_from_plan_with_tier<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    plan: WorthQueryDeclarationRoutePlan<D, I>,
    materialized_profile: &MaterializedFoundationalProfileSet,
    receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> Result<
    WorthQueryDeclarationReceipt<D, I>,
    (
        WorthQueryDeclarationRoutePlan<D, I>,
        WorthQueryDeclarationReceiptDenialCause,
    ),
> {
    let kind = match kind_for_plan(&plan) {
        Ok(kind) => kind,
        Err(cause) => return Err((plan, cause)),
    };
    let crossing_aspects = planned_receipt_crossing_aspects(&plan, receipt_tier);
    let explanation = WorthQueryDeclarationReceiptExplanation::new(
        "successful crossing recorded",
        plan.primary_route()
            .map(|route| format!("primary-route:{}", route.family().as_str())),
        retained_truths_from_plan(&plan),
        governing_reason_from_plan(&plan),
    );
    Ok(build_receipt(
        WorthQueryDeclarationReceiptClass::CoveredCrossing,
        kind,
        Some(plan),
        None,
        None,
        explanation,
        crossing_aspects,
        None,
        None,
        None,
        materialized_profile,
    ))
}

pub(crate) fn deferred_receipt<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    evidence: WorthQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<crate::application::WorthQueryDeclarationRouteIntent>,
    route_contract: crate::application::WorthQueryDeclarationRouteContract,
    reason: &'static str,
    materialized_profile: &MaterializedFoundationalProfileSet,
    receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptDenialCause> {
    let crossing_aspects = evidence_crossing_aspects(&evidence, receipt_tier);
    let explanation = WorthQueryDeclarationReceiptExplanation::new(
        "deferred crossing recorded",
        route_reference_for_non_success(Some(route_contract), route_intent),
        retained_truths_from_evidence(&evidence, Some(route_contract), route_intent),
        reason.to_string(),
    );
    Ok(build_receipt(
        WorthQueryDeclarationReceiptClass::DeferredCrossing,
        WorthQueryDeclarationReceiptKind::Deferred,
        None,
        Some(evidence),
        None,
        explanation,
        crossing_aspects,
        Some(route_contract),
        route_intent,
        None,
        materialized_profile,
    ))
}

pub(crate) fn denied_receipt<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    evidence: WorthQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<crate::application::WorthQueryDeclarationRouteIntent>,
    route_contract: Option<crate::application::WorthQueryDeclarationRouteContract>,
    route_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: WorthQueryDeclarationReceiptDenialCause,
    route_reference_override: Option<String>,
    extra_route_truths: Vec<String>,
    materialized_profile: &MaterializedFoundationalProfileSet,
    receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptDenialCause> {
    let crossing_aspects = evidence_crossing_aspects(&evidence, receipt_tier);
    let reason = route_cause
        .map(|cause| cause.reason().to_string())
        .unwrap_or_else(|| receipt_cause.reason().to_string());
    let mut retained_truths =
        retained_truths_from_evidence(&evidence, route_contract, route_intent);
    retained_truths.extend(extra_route_truths);
    let explanation = WorthQueryDeclarationReceiptExplanation::new(
        "denied crossing recorded",
        route_reference_override
            .or_else(|| route_reference_for_non_success(route_contract, route_intent)),
        retained_truths,
        reason,
    );
    Ok(build_receipt(
        WorthQueryDeclarationReceiptClass::DeniedCrossing,
        WorthQueryDeclarationReceiptKind::Denied,
        None,
        Some(evidence),
        route_cause,
        explanation,
        crossing_aspects,
        route_contract,
        route_intent,
        Some(receipt_cause),
        materialized_profile,
    ))
}

pub(crate) fn failed_receipt<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    evidence: WorthQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<crate::application::WorthQueryDeclarationRouteIntent>,
    route_contract: crate::application::WorthQueryDeclarationRouteContract,
    reason: &'static str,
    materialized_profile: &MaterializedFoundationalProfileSet,
    receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> Result<WorthQueryDeclarationReceipt<D, I>, WorthQueryDeclarationReceiptDenialCause> {
    let crossing_aspects = evidence_crossing_aspects(&evidence, receipt_tier);
    let explanation = WorthQueryDeclarationReceiptExplanation::new(
        "failed crossing recorded",
        route_reference_for_non_success(Some(route_contract), route_intent),
        retained_truths_from_evidence(&evidence, Some(route_contract), route_intent),
        reason.to_string(),
    );
    Ok(build_receipt(
        WorthQueryDeclarationReceiptClass::FailedCrossing,
        WorthQueryDeclarationReceiptKind::Failed,
        None,
        Some(evidence),
        None,
        explanation,
        crossing_aspects,
        Some(route_contract),
        route_intent,
        None,
        materialized_profile,
    ))
}

#[allow(clippy::too_many_arguments)]
fn build_receipt<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    class: WorthQueryDeclarationReceiptClass,
    kind: WorthQueryDeclarationReceiptKind,
    route_plan: Option<WorthQueryDeclarationRoutePlan<D, I>>,
    evidence: Option<WorthQueryDeclarationFoundationalEvidence<D, I>>,
    route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    explanation: WorthQueryDeclarationReceiptExplanation,
    crossing_aspects: ReceiptCrossingAspectState,
    route_contract: Option<crate::application::WorthQueryDeclarationRouteContract>,
    route_intent: Option<crate::application::WorthQueryDeclarationRouteIntent>,
    receipt_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
    materialized_profile: &MaterializedFoundationalProfileSet,
) -> WorthQueryDeclarationReceipt<D, I> {
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
    let receipt_digest = super::super::digest::derive_receipt_digest(
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
        route_denial_cause.map(|cause| cause.reason()),
        receipt_cause.map(|cause| cause.reason()),
        &crossing_aspects.contract,
        &crossing_aspects.coverage,
        &crossing_aspects.publication,
    );
    WorthQueryDeclarationReceipt::new(
        class,
        kind,
        route_plan,
        evidence,
        route_denial_cause,
        explanation,
        crossing_aspects.contract,
        crossing_aspects.coverage,
        crossing_aspects.publication,
        descriptive_receipt,
        boundary_receipt,
        receipt_digest,
    )
}

fn planned_receipt_crossing_aspects<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    plan: &WorthQueryDeclarationRoutePlan<D, I>,
    receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> ReceiptCrossingAspectState {
    let coverage = plan.foundational_evidence().aspect_coverage().clone();
    let contract = plan.aspect_contract().clone();
    let publication = declaration_publication_for_tier(&contract, &coverage, receipt_tier);
    ReceiptCrossingAspectState {
        contract,
        coverage,
        publication,
    }
}

fn evidence_crossing_aspects<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    evidence: &WorthQueryDeclarationFoundationalEvidence<D, I>,
    receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> ReceiptCrossingAspectState {
    let contract = route_scoped_aspect_contract(evidence.aspect_contract());
    let coverage = evidence_aspect_coverage(evidence);
    let publication = declaration_publication_for_tier(&contract, &coverage, receipt_tier);
    ReceiptCrossingAspectState {
        contract,
        coverage,
        publication,
    }
}

fn kind_for_plan<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    plan: &WorthQueryDeclarationRoutePlan<D, I>,
) -> Result<WorthQueryDeclarationReceiptKind, WorthQueryDeclarationReceiptDenialCause> {
    match plan.class() {
        crate::application::WorthQueryDeclarationRoutePlanClass::RelationalOnly => {
            Ok(WorthQueryDeclarationReceiptKind::Relational)
        }
        crate::application::WorthQueryDeclarationRoutePlanClass::BridgeOnly => {
            Ok(WorthQueryDeclarationReceiptKind::Bridge)
        }
        crate::application::WorthQueryDeclarationRoutePlanClass::Mixed => {
            Ok(WorthQueryDeclarationReceiptKind::Mixed)
        }
        crate::application::WorthQueryDeclarationRoutePlanClass::SignalOnly => {
            Err(WorthQueryDeclarationReceiptDenialCause::UnsupportedReceiptKind)
        }
    }
}
