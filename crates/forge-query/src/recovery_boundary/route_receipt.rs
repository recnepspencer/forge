use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDeclarationRoutePlanDenialCause,
    ForgeQueryDomainEntryMarker,
};

use super::brief::{
    ForgeQueryRecoveryAction, ForgeQueryRecoveryAuthoritySurface, ForgeQueryRecoveryBrief,
    ForgeQueryRecoveryStopFamily, ForgeQueryRecoveryStopKind,
};
use super::explanation::ForgeQueryRecoveryExplanation;
use super::family::ForgeQueryRecoverySourceFamily;

pub fn forge_query_recovery_brief_from_declaration_route_plan_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationRoutePlanChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    match checked {
        ForgeQueryDeclarationRoutePlanChecked::Planned(_) => None,
        ForgeQueryDeclarationRoutePlanChecked::Deferred(value) => Some(route_plan_brief(
            ForgeQueryRecoveryStopKind::Deferred,
            ForgeQueryRecoveryAuthoritySurface::SupportReadiness,
            ForgeQueryRecoveryAction::RetryLater,
            value.reason(),
            value
                .progressed_declaration()
                .progression_digest()
                .to_string(),
            value.route_contract().reason(),
            None,
        )),
        ForgeQueryDeclarationRoutePlanChecked::Denied(value) => Some(route_plan_denied_brief(
            value.cause(),
            value.reason(),
            value
                .progressed_declaration()
                .progression_digest()
                .to_string(),
            value.route_contract().reason(),
        )),
        ForgeQueryDeclarationRoutePlanChecked::Failed(value) => Some(route_plan_brief(
            ForgeQueryRecoveryStopKind::Failed,
            ForgeQueryRecoveryAuthoritySurface::FailureEscalation,
            ForgeQueryRecoveryAction::EscalateFailure,
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

pub fn forge_query_recovery_brief_from_declaration_receipt_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationReceiptChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    match checked {
        ForgeQueryDeclarationReceiptChecked::Issued(_) => None,
        ForgeQueryDeclarationReceiptChecked::Deferred(value) => Some(receipt_brief(
            ForgeQueryRecoveryStopKind::Deferred,
            ForgeQueryRecoveryAuthoritySurface::SupportReadiness,
            ForgeQueryRecoveryAction::RetryLater,
            value.reason(),
            canonical_digest_token(value.receipt().receipt_digest()),
            value.receipt().explain().governing_reason().to_string(),
            None,
        )),
        ForgeQueryDeclarationReceiptChecked::Denied(value) => {
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
        ForgeQueryDeclarationReceiptChecked::Failed(value) => Some(receipt_brief(
            ForgeQueryRecoveryStopKind::Failed,
            ForgeQueryRecoveryAuthoritySurface::FailureEscalation,
            ForgeQueryRecoveryAction::EscalateFailure,
            value.reason(),
            canonical_digest_token(value.receipt().receipt_digest()),
            value.receipt().explain().governing_reason().to_string(),
            None,
        )),
    }
}

fn route_plan_denied_brief(
    cause: ForgeQueryDeclarationRoutePlanDenialCause,
    reason: &'static str,
    retained_digest: String,
    route_governing_reason: &'static str,
) -> ForgeQueryRecoveryBrief {
    match cause {
        ForgeQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld => route_plan_brief(
            ForgeQueryRecoveryStopKind::WrongWorld,
            ForgeQueryRecoveryAuthoritySurface::AdmittedOperatingWorld,
            ForgeQueryRecoveryAction::CorrectWorld,
            reason,
            retained_digest,
            route_governing_reason,
            Some(cause),
        ),
        ForgeQueryDeclarationRoutePlanDenialCause::IntentRequired
        | ForgeQueryDeclarationRoutePlanDenialCause::IntentForbidden
        | ForgeQueryDeclarationRoutePlanDenialCause::IntentConflictsWithRouteContract => {
            route_plan_brief(
                ForgeQueryRecoveryStopKind::DeclarationDenied,
                ForgeQueryRecoveryAuthoritySurface::InputNarrowing,
                ForgeQueryRecoveryAction::NarrowInput,
                reason,
                retained_digest,
                route_governing_reason,
                Some(cause),
            )
        }
        ForgeQueryDeclarationRoutePlanDenialCause::EvidenceMismatch
        | ForgeQueryDeclarationRoutePlanDenialCause::MissingRequiredAspect
        | ForgeQueryDeclarationRoutePlanDenialCause::AspectConflict
        | ForgeQueryDeclarationRoutePlanDenialCause::NoAllowedRoutes
        | ForgeQueryDeclarationRoutePlanDenialCause::ForbiddenRouteCombination => route_plan_brief(
            ForgeQueryRecoveryStopKind::DeclarationDenied,
            ForgeQueryRecoveryAuthoritySurface::DeclarationMeaning,
            ForgeQueryRecoveryAction::RepairDeclarationMeaning,
            reason,
            retained_digest,
            route_governing_reason,
            Some(cause),
        ),
    }
}

fn route_plan_brief(
    stop_kind: ForgeQueryRecoveryStopKind,
    authority_surface: ForgeQueryRecoveryAuthoritySurface,
    recommended_action: ForgeQueryRecoveryAction,
    reason: &'static str,
    retained_digest: String,
    route_governing_reason: &'static str,
    route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
) -> ForgeQueryRecoveryBrief {
    ForgeQueryRecoveryBrief::new(
        ForgeQueryRecoveryStopFamily::DeclarationRoutePlan,
        stop_kind,
        authority_surface,
        recommended_action,
        reason,
        ForgeQueryRecoveryExplanation::new_with_source_family(
            crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::orchestration(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                Some(retained_digest),
                None,
            ),
            ForgeQueryRecoverySourceFamily::DeclarationRoutePlan,
        )
        .with_route_context(route_governing_reason, route_denial_cause),
    )
}

fn receipt_route_denied_brief(
    route_cause: ForgeQueryDeclarationRoutePlanDenialCause,
    reason: &'static str,
    retained_digest: String,
    receipt_governing_reason: String,
) -> ForgeQueryRecoveryBrief {
    let (authority_surface, recommended_action) = match route_cause {
        ForgeQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld => (
            ForgeQueryRecoveryAuthoritySurface::AdmittedOperatingWorld,
            ForgeQueryRecoveryAction::CorrectWorld,
        ),
        ForgeQueryDeclarationRoutePlanDenialCause::IntentRequired
        | ForgeQueryDeclarationRoutePlanDenialCause::IntentForbidden
        | ForgeQueryDeclarationRoutePlanDenialCause::IntentConflictsWithRouteContract => (
            ForgeQueryRecoveryAuthoritySurface::InputNarrowing,
            ForgeQueryRecoveryAction::NarrowInput,
        ),
        ForgeQueryDeclarationRoutePlanDenialCause::EvidenceMismatch
        | ForgeQueryDeclarationRoutePlanDenialCause::MissingRequiredAspect
        | ForgeQueryDeclarationRoutePlanDenialCause::AspectConflict
        | ForgeQueryDeclarationRoutePlanDenialCause::NoAllowedRoutes
        | ForgeQueryDeclarationRoutePlanDenialCause::ForbiddenRouteCombination => (
            ForgeQueryRecoveryAuthoritySurface::DeclarationMeaning,
            ForgeQueryRecoveryAction::RepairDeclarationMeaning,
        ),
    };
    ForgeQueryRecoveryBrief::new(
        ForgeQueryRecoveryStopFamily::DeclarationReceipt,
        if route_cause == ForgeQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld {
            ForgeQueryRecoveryStopKind::WrongWorld
        } else {
            ForgeQueryRecoveryStopKind::DeclarationDenied
        },
        authority_surface,
        recommended_action,
        reason,
        ForgeQueryRecoveryExplanation::new_with_source_family(
            crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::orchestration(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(retained_digest),
                None,
            ),
            ForgeQueryRecoverySourceFamily::DeclarationReceipt,
        )
        .with_route_context(route_cause.reason(), Some(route_cause))
        .with_receipt_context(receipt_governing_reason, None),
    )
}

fn receipt_denied_brief(
    receipt_cause: ForgeQueryDeclarationReceiptDenialCause,
    reason: &'static str,
    retained_digest: String,
    receipt_governing_reason: String,
) -> ForgeQueryRecoveryBrief {
    let (authority_surface, recommended_action) = match receipt_cause {
        ForgeQueryDeclarationReceiptDenialCause::UnsupportedReceiptKind => (
            ForgeQueryRecoveryAuthoritySurface::SupportReadiness,
            ForgeQueryRecoveryAction::CheckSupport,
        ),
        ForgeQueryDeclarationReceiptDenialCause::MissingRoutePlan
        | ForgeQueryDeclarationReceiptDenialCause::ReceiptMaterializationMismatch
        | ForgeQueryDeclarationReceiptDenialCause::RouteIntegrityMismatch => (
            ForgeQueryRecoveryAuthoritySurface::DeclarationMeaning,
            ForgeQueryRecoveryAction::InspectCheckedLane,
        ),
    };
    ForgeQueryRecoveryBrief::new(
        ForgeQueryRecoveryStopFamily::DeclarationReceipt,
        ForgeQueryRecoveryStopKind::DeclarationDenied,
        authority_surface,
        recommended_action,
        reason,
        ForgeQueryRecoveryExplanation::new_with_source_family(
            crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::orchestration(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(retained_digest),
                None,
            ),
            ForgeQueryRecoverySourceFamily::DeclarationReceipt,
        )
        .with_receipt_context(receipt_governing_reason, Some(receipt_cause)),
    )
}

fn receipt_brief(
    stop_kind: ForgeQueryRecoveryStopKind,
    authority_surface: ForgeQueryRecoveryAuthoritySurface,
    recommended_action: ForgeQueryRecoveryAction,
    reason: &'static str,
    retained_digest: String,
    receipt_governing_reason: String,
    receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
) -> ForgeQueryRecoveryBrief {
    ForgeQueryRecoveryBrief::new(
        ForgeQueryRecoveryStopFamily::DeclarationReceipt,
        stop_kind,
        authority_surface,
        recommended_action,
        reason,
        ForgeQueryRecoveryExplanation::new_with_source_family(
            crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::orchestration(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(retained_digest),
                None,
            ),
            ForgeQueryRecoverySourceFamily::DeclarationReceipt,
        )
        .with_receipt_context(receipt_governing_reason, receipt_denial_cause),
    )
}

fn canonical_digest_token(digest: &forge_foundational::facade::CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
