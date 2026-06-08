use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use forge_foundational::facade::MaterializedFoundationalProfileSet;

use super::{
    artifact::ForgeQueryDeclarationReceipt,
    denial::{
        ForgeQueryDeclarationReceiptDeferred, ForgeQueryDeclarationReceiptDenied,
        ForgeQueryDeclarationReceiptFailed,
    },
    input::ForgeQueryDeclarationReceiptInput,
    materialize::{
        default_receipt_materialized_profile, deferred_receipt, denied_receipt, failed_receipt,
        receipt_from_plan,
    },
};
use crate::application::ForgeQueryDeclarationEntryOrchestrationMaterializationTier;

pub enum ForgeQueryDeclarationReceiptChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Issued(ForgeQueryDeclarationReceipt<D, I>),
    Deferred(ForgeQueryDeclarationReceiptDeferred<D, I>),
    Denied(ForgeQueryDeclarationReceiptDenied<D, I>),
    Failed(ForgeQueryDeclarationReceiptFailed<D, I>),
}

pub(crate) fn forge_query_checked_declaration_receipt<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationReceiptInput<D, I>,
) -> ForgeQueryDeclarationReceiptChecked<D, I> {
    forge_query_checked_declaration_receipt_with_materialized_profile(
        input,
        default_receipt_materialized_profile(),
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
    )
}

pub(crate) fn forge_query_checked_declaration_receipt_with_materialized_profile<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationReceiptInput<D, I>,
    materialized_profile: &MaterializedFoundationalProfileSet,
    receipt_tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
) -> ForgeQueryDeclarationReceiptChecked<D, I> {
    match input {
        ForgeQueryDeclarationReceiptInput::PlannedRoute(plan) => {
            match receipt_from_plan(plan, materialized_profile, receipt_tier) {
                Ok(receipt) => ForgeQueryDeclarationReceiptChecked::Issued(receipt),
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
                    ForgeQueryDeclarationReceiptChecked::Denied(
                        ForgeQueryDeclarationReceiptDenied::from_receipt_cause(
                            receipt,
                            route_intent,
                            cause,
                        ),
                    )
                }
            }
        }
        ForgeQueryDeclarationReceiptInput::DeferredRoute(plan) => {
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
            ForgeQueryDeclarationReceiptChecked::Deferred(
                ForgeQueryDeclarationReceiptDeferred::new(receipt, route_intent, reason),
            )
        }
        ForgeQueryDeclarationReceiptInput::DeniedRoute(plan) => {
            let (_progressed, evidence, route_intent, contract, cause) = plan.into_parts();
            let receipt = denied_receipt(
                evidence,
                route_intent,
                Some(contract),
                Some(cause),
                crate::application::ForgeQueryDeclarationReceiptDenialCause::RouteIntegrityMismatch,
                None,
                Vec::new(),
                materialized_profile,
                receipt_tier,
            )
            .expect("denied route truth should always materialize a denied receipt");
            ForgeQueryDeclarationReceiptChecked::Denied(
                ForgeQueryDeclarationReceiptDenied::from_route_cause(receipt, route_intent, cause),
            )
        }
        ForgeQueryDeclarationReceiptInput::FailedRoute(plan) => {
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
            ForgeQueryDeclarationReceiptChecked::Failed(ForgeQueryDeclarationReceiptFailed::new(
                receipt,
                route_intent,
                reason,
            ))
        }
    }
}
