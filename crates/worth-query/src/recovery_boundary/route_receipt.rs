use crate::application::{
    WorthQueryDeclarationEntryOrchestrationStage, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceiptChecked, WorthQueryDeclarationReceiptDenialCause,
    WorthQueryDeclarationRoutePlanChecked, WorthQueryDeclarationRoutePlanDenialCause,
    WorthQueryDomainEntryMarker,
};

use super::brief::{
    WorthQueryRecoveryAction, WorthQueryRecoveryAuthoritySurface, WorthQueryRecoveryBrief,
    WorthQueryRecoveryStopFamily, WorthQueryRecoveryStopKind,
};
use super::explanation::WorthQueryRecoveryExplanation;
use super::family::WorthQueryRecoverySourceFamily;

pub fn worth_query_recovery_brief_from_declaration_route_plan_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryDeclarationRoutePlanChecked<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    match checked {
        WorthQueryDeclarationRoutePlanChecked::Planned(_) => None,
        WorthQueryDeclarationRoutePlanChecked::Deferred(value) => Some(route_plan_brief(
            WorthQueryRecoveryStopKind::Deferred,
            WorthQueryRecoveryAuthoritySurface::SupportReadiness,
            WorthQueryRecoveryAction::RetryLater,
            value.reason(),
            value
                .progressed_declaration()
                .progression_digest()
                .to_string(),
            value.route_contract().reason(),
            None,
        )),
        WorthQueryDeclarationRoutePlanChecked::Denied(value) => Some(route_plan_denied_brief(
            value.cause(),
            value.reason(),
            value
                .progressed_declaration()
                .progression_digest()
                .to_string(),
            value.route_contract().reason(),
        )),
        WorthQueryDeclarationRoutePlanChecked::Failed(value) => Some(route_plan_brief(
            WorthQueryRecoveryStopKind::Failed,
            WorthQueryRecoveryAuthoritySurface::FailureEscalation,
            WorthQueryRecoveryAction::EscalateFailure,
            value.reason(),
            value
                .progressed_declaration()
                .progression_digest()
                .to_string(),
            value.route_contract().reason(),
            None,
        )),
    }
}

pub fn worth_query_recovery_brief_from_declaration_receipt_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryDeclarationReceiptChecked<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    match checked {
        WorthQueryDeclarationReceiptChecked::Issued(_) => None,
        WorthQueryDeclarationReceiptChecked::Deferred(value) => Some(receipt_brief(
            WorthQueryRecoveryStopKind::Deferred,
            WorthQueryRecoveryAuthoritySurface::SupportReadiness,
            WorthQueryRecoveryAction::RetryLater,
            value.reason(),
            canonical_digest_token(value.receipt().receipt_digest()),
            value.receipt().explain().governing_reason().to_string(),
            None,
        )),
        WorthQueryDeclarationReceiptChecked::Denied(value) => {
            if let Some(route_cause) = value.route_cause() {
                Some(receipt_route_denied_brief(
                    route_cause,
                    value.reason(),
                    canonical_digest_token(value.receipt().receipt_digest()),
                    value.receipt().explain().governing_reason().to_string(),
                ))
            } else {
                Some(receipt_denied_brief(
                    value
                        .receipt_cause()
                        .expect("receipt denial must retain one denial cause"),
                    value.reason(),
                    canonical_digest_token(value.receipt().receipt_digest()),
                    value.receipt().explain().governing_reason().to_string(),
                ))
            }
        }
        WorthQueryDeclarationReceiptChecked::Failed(value) => Some(receipt_brief(
            WorthQueryRecoveryStopKind::Failed,
            WorthQueryRecoveryAuthoritySurface::FailureEscalation,
            WorthQueryRecoveryAction::EscalateFailure,
            value.reason(),
            canonical_digest_token(value.receipt().receipt_digest()),
            value.receipt().explain().governing_reason().to_string(),
            None,
        )),
    }
}

fn route_plan_denied_brief(
    cause: WorthQueryDeclarationRoutePlanDenialCause,
    reason: &'static str,
    retained_digest: String,
    route_governing_reason: &'static str,
) -> WorthQueryRecoveryBrief {
    match cause {
        WorthQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld => route_plan_brief(
            WorthQueryRecoveryStopKind::WrongWorld,
            WorthQueryRecoveryAuthoritySurface::AdmittedOperatingWorld,
            WorthQueryRecoveryAction::CorrectWorld,
            reason,
            retained_digest,
            route_governing_reason,
            Some(cause),
        ),
        WorthQueryDeclarationRoutePlanDenialCause::IntentRequired
        | WorthQueryDeclarationRoutePlanDenialCause::IntentForbidden
        | WorthQueryDeclarationRoutePlanDenialCause::IntentConflictsWithRouteContract => {
            route_plan_brief(
                WorthQueryRecoveryStopKind::DeclarationDenied,
                WorthQueryRecoveryAuthoritySurface::InputNarrowing,
                WorthQueryRecoveryAction::NarrowInput,
                reason,
                retained_digest,
                route_governing_reason,
                Some(cause),
            )
        }
        WorthQueryDeclarationRoutePlanDenialCause::EvidenceMismatch
        | WorthQueryDeclarationRoutePlanDenialCause::MissingRequiredAspect
        | WorthQueryDeclarationRoutePlanDenialCause::AspectConflict
        | WorthQueryDeclarationRoutePlanDenialCause::NoAllowedRoutes
        | WorthQueryDeclarationRoutePlanDenialCause::ForbiddenRouteCombination => route_plan_brief(
            WorthQueryRecoveryStopKind::DeclarationDenied,
            WorthQueryRecoveryAuthoritySurface::DeclarationMeaning,
            WorthQueryRecoveryAction::RepairDeclarationMeaning,
            reason,
            retained_digest,
            route_governing_reason,
            Some(cause),
        ),
    }
}

fn route_plan_brief(
    stop_kind: WorthQueryRecoveryStopKind,
    authority_surface: WorthQueryRecoveryAuthoritySurface,
    recommended_action: WorthQueryRecoveryAction,
    reason: &'static str,
    retained_digest: String,
    route_governing_reason: &'static str,
    route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
) -> WorthQueryRecoveryBrief {
    WorthQueryRecoveryBrief::new(
        WorthQueryRecoveryStopFamily::DeclarationRoutePlan,
        stop_kind,
        authority_surface,
        recommended_action,
        reason,
        WorthQueryRecoveryExplanation::new_with_source_family(
            crate::ordinary_outcome::WorthQueryOrdinaryCheckedTopology::orchestration(
                WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                Some(retained_digest),
                None,
            ),
            WorthQueryRecoverySourceFamily::DeclarationRoutePlan,
        )
        .with_route_context(route_governing_reason, route_denial_cause),
    )
}

fn receipt_route_denied_brief(
    route_cause: WorthQueryDeclarationRoutePlanDenialCause,
    reason: &'static str,
    retained_digest: String,
    receipt_governing_reason: String,
) -> WorthQueryRecoveryBrief {
    let (authority_surface, recommended_action) = match route_cause {
        WorthQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld => (
            WorthQueryRecoveryAuthoritySurface::AdmittedOperatingWorld,
            WorthQueryRecoveryAction::CorrectWorld,
        ),
        WorthQueryDeclarationRoutePlanDenialCause::IntentRequired
        | WorthQueryDeclarationRoutePlanDenialCause::IntentForbidden
        | WorthQueryDeclarationRoutePlanDenialCause::IntentConflictsWithRouteContract => (
            WorthQueryRecoveryAuthoritySurface::InputNarrowing,
            WorthQueryRecoveryAction::NarrowInput,
        ),
        WorthQueryDeclarationRoutePlanDenialCause::EvidenceMismatch
        | WorthQueryDeclarationRoutePlanDenialCause::MissingRequiredAspect
        | WorthQueryDeclarationRoutePlanDenialCause::AspectConflict
        | WorthQueryDeclarationRoutePlanDenialCause::NoAllowedRoutes
        | WorthQueryDeclarationRoutePlanDenialCause::ForbiddenRouteCombination => (
            WorthQueryRecoveryAuthoritySurface::DeclarationMeaning,
            WorthQueryRecoveryAction::RepairDeclarationMeaning,
        ),
    };
    WorthQueryRecoveryBrief::new(
        WorthQueryRecoveryStopFamily::DeclarationReceipt,
        if route_cause == WorthQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld {
            WorthQueryRecoveryStopKind::WrongWorld
        } else {
            WorthQueryRecoveryStopKind::DeclarationDenied
        },
        authority_surface,
        recommended_action,
        reason,
        WorthQueryRecoveryExplanation::new_with_source_family(
            crate::ordinary_outcome::WorthQueryOrdinaryCheckedTopology::orchestration(
                WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(retained_digest),
                None,
            ),
            WorthQueryRecoverySourceFamily::DeclarationReceipt,
        )
        .with_route_context(route_cause.reason(), Some(route_cause))
        .with_receipt_context(receipt_governing_reason, None),
    )
}

fn receipt_denied_brief(
    receipt_cause: WorthQueryDeclarationReceiptDenialCause,
    reason: &'static str,
    retained_digest: String,
    receipt_governing_reason: String,
) -> WorthQueryRecoveryBrief {
    let (authority_surface, recommended_action) = match receipt_cause {
        WorthQueryDeclarationReceiptDenialCause::UnsupportedReceiptKind => (
            WorthQueryRecoveryAuthoritySurface::SupportReadiness,
            WorthQueryRecoveryAction::CheckSupport,
        ),
        WorthQueryDeclarationReceiptDenialCause::MissingRoutePlan
        | WorthQueryDeclarationReceiptDenialCause::ReceiptMaterializationMismatch
        | WorthQueryDeclarationReceiptDenialCause::RouteIntegrityMismatch => (
            WorthQueryRecoveryAuthoritySurface::DeclarationMeaning,
            WorthQueryRecoveryAction::InspectCheckedLane,
        ),
    };
    WorthQueryRecoveryBrief::new(
        WorthQueryRecoveryStopFamily::DeclarationReceipt,
        WorthQueryRecoveryStopKind::DeclarationDenied,
        authority_surface,
        recommended_action,
        reason,
        WorthQueryRecoveryExplanation::new_with_source_family(
            crate::ordinary_outcome::WorthQueryOrdinaryCheckedTopology::orchestration(
                WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(retained_digest),
                None,
            ),
            WorthQueryRecoverySourceFamily::DeclarationReceipt,
        )
        .with_receipt_context(receipt_governing_reason, Some(receipt_cause)),
    )
}

fn receipt_brief(
    stop_kind: WorthQueryRecoveryStopKind,
    authority_surface: WorthQueryRecoveryAuthoritySurface,
    recommended_action: WorthQueryRecoveryAction,
    reason: &'static str,
    retained_digest: String,
    receipt_governing_reason: String,
    receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
) -> WorthQueryRecoveryBrief {
    WorthQueryRecoveryBrief::new(
        WorthQueryRecoveryStopFamily::DeclarationReceipt,
        stop_kind,
        authority_surface,
        recommended_action,
        reason,
        WorthQueryRecoveryExplanation::new_with_source_family(
            crate::ordinary_outcome::WorthQueryOrdinaryCheckedTopology::orchestration(
                WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(retained_digest),
                None,
            ),
            WorthQueryRecoverySourceFamily::DeclarationReceipt,
        )
        .with_receipt_context(receipt_governing_reason, receipt_denial_cause),
    )
}

fn canonical_digest_token(digest: &worth_foundational::facade::CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
