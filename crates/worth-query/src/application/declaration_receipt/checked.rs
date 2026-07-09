use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use worth_foundational::facade::MaterializedFoundationalProfileSet;

use super::{
    artifact::WorthQueryDeclarationReceipt,
    denial::{
        WorthQueryDeclarationReceiptDeferred, WorthQueryDeclarationReceiptDenied,
        WorthQueryDeclarationReceiptFailed,
    },
    input::WorthQueryDeclarationReceiptInput,
    materialize::{
        default_receipt_materialized_profile, deferred_receipt, denied_receipt, failed_receipt,
        receipt_from_plan,
    },
};
use crate::application::WorthQueryDeclarationEntryOrchestrationMaterializationTier;

pub enum WorthQueryDeclarationReceiptChecked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Issued(WorthQueryDeclarationReceipt<D, I>),
    Deferred(WorthQueryDeclarationReceiptDeferred<D, I>),
    Denied(WorthQueryDeclarationReceiptDenied<D, I>),
    Failed(WorthQueryDeclarationReceiptFailed<D, I>),
}

pub(crate) fn worth_query_checked_declaration_receipt<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    input: WorthQueryDeclarationReceiptInput<D, I>,
) -> WorthQueryDeclarationReceiptChecked<D, I> {
    worth_query_checked_declaration_receipt_with_materialized_profile(
        input,
        default_receipt_materialized_profile(),
        WorthQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
    )
}

pub(crate) fn worth_query_checked_declaration_receipt_with_materialized_profile<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    input: WorthQueryDeclarationReceiptInput<D, I>,
    materialized_profile: &MaterializedFoundationalProfileSet,
    receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier,
) -> WorthQueryDeclarationReceiptChecked<D, I> {
    match input {
        WorthQueryDeclarationReceiptInput::PlannedRoute(plan) => {
            match receipt_from_plan(plan, materialized_profile, receipt_tier) {
                Ok(receipt) => WorthQueryDeclarationReceiptChecked::Issued(receipt),
                Err((plan, cause)) => {
                    let (
                        _progressed,
                        evidence,
                        route_intent,
                        route_set,
                        class,
                        _automation_requires_explicit_handoff,
                        _route_aspect_contract,
                        _route_aspect_fit,
                        _route_aspect_publication,
                        _future_projection,
                        _explanation,
                        _decl,
                        _digest,
                    ) = plan.into_parts();
                    let planned_route_reference = route_set
                        .primary_route()
                        .map(|route| format!("planned-route:{}", route.family().as_str()))
                        .or_else(|| {
                            route_set
                                .route_families()
                                .first()
                                .map(|family| format!("planned-route:{}", family.as_str()))
                        });
                    let extra_route_truths = vec![format!("planned-class:{class:?}")];
                    let receipt = denied_receipt(
                        evidence,
                        route_intent,
                        None,
                        None,
                        cause,
                        planned_route_reference,
                        extra_route_truths,
                        materialized_profile,
                        receipt_tier,
                    )
                    .expect("unsupported receipt kinds should still materialize denied receipts");
                    WorthQueryDeclarationReceiptChecked::Denied(
                        WorthQueryDeclarationReceiptDenied::from_receipt_cause(
                            receipt,
                            route_intent,
                            cause,
                        ),
                    )
                }
            }
        }
        WorthQueryDeclarationReceiptInput::DeferredRoute(plan) => {
            let (_progressed, evidence, route_intent, contract, reason) = plan.into_parts();
            let receipt = deferred_receipt(
                evidence,
                route_intent,
                contract,
                reason,
                materialized_profile,
                receipt_tier,
            )
            .expect("deferred route truth should always materialize a deferred receipt");
            WorthQueryDeclarationReceiptChecked::Deferred(
                WorthQueryDeclarationReceiptDeferred::new(receipt, route_intent, reason),
            )
        }
        WorthQueryDeclarationReceiptInput::DeniedRoute(plan) => {
            let (_progressed, evidence, route_intent, contract, cause) = plan.into_parts();
            let receipt = denied_receipt(
                evidence,
                route_intent,
                Some(contract),
                Some(cause),
                crate::application::WorthQueryDeclarationReceiptDenialCause::RouteIntegrityMismatch,
                None,
                Vec::new(),
                materialized_profile,
                receipt_tier,
            )
            .expect("denied route truth should always materialize a denied receipt");
            WorthQueryDeclarationReceiptChecked::Denied(
                WorthQueryDeclarationReceiptDenied::from_route_cause(receipt, route_intent, cause),
            )
        }
        WorthQueryDeclarationReceiptInput::FailedRoute(plan) => {
            let (_progressed, evidence, route_intent, contract, reason) = plan.into_parts();
            let receipt = failed_receipt(
                evidence,
                route_intent,
                contract,
                reason,
                materialized_profile,
                receipt_tier,
            )
            .expect("failed route truth should always materialize a failed receipt");
            WorthQueryDeclarationReceiptChecked::Failed(WorthQueryDeclarationReceiptFailed::new(
                receipt,
                route_intent,
                reason,
            ))
        }
    }
}
