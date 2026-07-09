use std::sync::Arc;

use worth_signal::facade::{
    ResourceRetryReason, ResourceRevalidationDenialClass, ResourceRevalidationReport,
};

use super::super::request_identity::{
    admit_from_existing_signal_request, AdmittedBridgeAsyncRequestIdentity,
    BridgeAsyncRequestAdmissionRequest, BridgeAsyncRequestFamilyAdmission,
};
use super::class::BridgeAsyncForwardCausalityClass;
use super::counters::BridgeAsyncForwardCausalityCounters;
use super::evidence::{BridgeAsyncRetryLineageRequest, BridgeAsyncRevalidationLineageRequest};
use super::lineage::{BridgeAsyncRetryLineage, BridgeAsyncRevalidationLineage};
use super::rejection::{
    map_request_identity_rejection, rejected, BridgeAsyncForwardCausalityRejection,
    BridgeAsyncForwardCausalityRejectionKind,
};
use crate::source::async_declaration::request_identity::state::BridgeSignalRuntime;

pub fn admit_retry_lineage(
    runtime: &mut BridgeSignalRuntime,
    request: BridgeAsyncRetryLineageRequest,
) -> Result<BridgeAsyncRetryLineage, BridgeAsyncForwardCausalityRejection> {
    let prior = request.prior.clone();
    if request.timeout_report.is_some() {
        let scheduled = request
            .retry_schedule_report
            .as_ref()
            .and_then(|report| report.scheduled_retry())
            .cloned()
            .ok_or_else(|| {
                rejected(
                    BridgeAsyncForwardCausalityRejectionKind::RetryScheduleMissing,
                    "timeout-triggered retry requires one admitted retry schedule",
                )
            })?;
        let admitted_retry = request
            .retry_admission_report
            .as_ref()
            .and_then(|report| report.admitted_retry())
            .cloned()
            .ok_or_else(|| {
                rejected(
                    BridgeAsyncForwardCausalityRejectionKind::RetryAdmissionMissing,
                    "timeout-triggered retry requires one admitted retry request",
                )
            })?;
        if scheduled.previous() != prior.request_handle() {
            return Err(rejected(
                BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerSignalHandleMismatch,
                "scheduled retry must point back to the prior bridge async request handle",
            ));
        }
        if admitted_retry.scheduled().previous() != prior.request_handle() {
            return Err(rejected(
                BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerSignalHandleMismatch,
                "admitted retry must retain the exact prior bridge async request handle",
            ));
        }
        let timed_out = request
            .timeout_report
            .as_ref()
            .and_then(|report| report.timed_out_request())
            .ok_or_else(|| {
                rejected(
                    BridgeAsyncForwardCausalityRejectionKind::TimeoutEvidenceMissing,
                    "timeout-triggered retry requires an admitted timeout report",
                )
            })?;
        if timed_out.handle() != prior.request_handle() {
            return Err(rejected(
                BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerSignalHandleMismatch,
                "timeout-triggered retry must refer to the exact prior request handle",
            ));
        }
        if scheduled.reason() != ResourceRetryReason::TimedOut {
            return Err(rejected(
                BridgeAsyncForwardCausalityRejectionKind::RetryScheduleMissing,
                "timeout-triggered retry must retain a timed-out retry reason",
            ));
        }
        let retry_request = BridgeAsyncRequestAdmissionRequest::rebind(
            prior.lowered(),
            prior.basis_binding(),
            prior.family_admission(),
        )
        .map_err(map_request_identity_rejection)?;
        let newer = admit_from_existing_signal_request(
            runtime,
            retry_request,
            admitted_retry.admitted_request(),
            None,
        )
        .map_err(map_request_identity_rejection)?;
        return finalize_retry_lineage(
            prior,
            newer,
            BridgeAsyncForwardCausalityClass::RetryAfterTimeout,
            scheduled.reason(),
            scheduled.retry_ordinal().get(),
            admitted_retry.admitted_request().attempt().get(),
            admitted_retry.ready_wake().id().get(),
            scheduled.policy_decision_digest().as_str(),
            request
                .timeout_report
                .as_ref()
                .and_then(|report| report.timed_out_request())
                .map(|report| report.ready_wake().id().get().to_string())
                .unwrap_or_else(|| "-".to_owned()),
            "-".to_owned(),
        );
    } else {
        let cancelled = request
            .cancellation_report
            .as_ref()
            .and_then(|report| report.cancelled_request())
            .ok_or_else(|| {
                rejected(
                    BridgeAsyncForwardCausalityRejectionKind::CancellationEvidenceMissing,
                    "cancellation-triggered retry requires an admitted cancellation report",
                )
            })?;
        if cancelled.handle() != prior.request_handle() {
            return Err(rejected(
                BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerSignalHandleMismatch,
                "cancellation-triggered retry must refer to the exact prior request handle",
            ));
        }
        let newer = request.newer_request.clone().ok_or_else(|| {
            rejected(
                BridgeAsyncForwardCausalityRejectionKind::RetryAdmissionMissing,
                "cancellation-triggered retry requires one newer admitted request identity",
            )
        })?;
        return finalize_retry_lineage(
            prior,
            newer,
            BridgeAsyncForwardCausalityClass::RetryAfterCancellation,
            ResourceRetryReason::HostRequested,
            0,
            0,
            0,
            cancelled.policy_decision_digest().as_str(),
            "-".to_owned(),
            format!("{:?}", cancelled.reason()),
        );
    }
}

fn finalize_retry_lineage(
    prior: AdmittedBridgeAsyncRequestIdentity,
    newer: AdmittedBridgeAsyncRequestIdentity,
    class: BridgeAsyncForwardCausalityClass,
    retry_reason: ResourceRetryReason,
    retry_ordinal: u64,
    next_attempt: u64,
    ready_wake: u64,
    policy_digest: &str,
    timeout_trigger: String,
    cancellation_trigger: String,
) -> Result<BridgeAsyncRetryLineage, BridgeAsyncForwardCausalityRejection> {
    if prior.lowered().declaration_identity() != newer.lowered().declaration_identity() {
        return Err(rejected(
            BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerDeclarationMismatch,
            "retry lineage must stay within one bridge async declaration identity",
        ));
    }
    if prior.lowered().lowering_identity() != newer.lowered().lowering_identity() {
        return Err(rejected(
            BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerLoweringMismatch,
            "retry lineage must stay within one lowered bridge async source identity",
        ));
    }
    if prior.family_admission() != newer.family_admission()
        && !matches!(
            (prior.family_admission(), newer.family_admission()),
            (
                BridgeAsyncRequestFamilyAdmission::SubscriptionBacked { .. },
                BridgeAsyncRequestFamilyAdmission::SubscriptionBacked { .. }
            )
        )
    {
        return Err(rejected(
            BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerFamilyMismatch,
            "retry lineage must stay within one bridge async family",
        ));
    }
    if prior.basis_binding().truth_view_basis().digest()
        != newer.basis_binding().truth_view_basis().digest()
    {
        return Err(rejected(
            BridgeAsyncForwardCausalityRejectionKind::BasisDriftForbiddenForRetry,
            "retry lineage cannot rebind to a different truth-view basis",
        ));
    }
    if subscription_instance_digest(&prior) != subscription_instance_digest(&newer) {
        return Err(rejected(
            BridgeAsyncForwardCausalityRejectionKind::SubscriptionInstanceDriftForbiddenForRetry,
            "retry lineage cannot drift subscription instance identity",
        ));
    }
    let counters = match class {
        BridgeAsyncForwardCausalityClass::RetryAfterTimeout => {
            BridgeAsyncForwardCausalityCounters::one_retry_after_timeout()
        }
        BridgeAsyncForwardCausalityClass::RetryAfterCancellation => {
            BridgeAsyncForwardCausalityCounters::one_retry_after_cancellation()
        }
        _ => unreachable!(),
    };
    Ok(BridgeAsyncRetryLineage::new(
        prior.clone(),
        newer.clone(),
        class,
        counters,
        Arc::from(format!(
            "bridge-async-forward-causality|class={class:?}|prior={}|newer={}|prior-basis={}|newer-basis={}|subscription-instance={}|retry-reason={:?}|retry-ordinal={}|next-attempt={}|ready-wake={}|policy={}|timeout-trigger={}|cancellation-trigger={}",
            prior.request_identity().as_str(),
            newer.request_identity().as_str(),
            prior.basis_binding().truth_view_basis().digest(),
            newer.basis_binding().truth_view_basis().digest(),
            subscription_instance_digest(&prior).unwrap_or("-"),
            retry_reason,
            retry_ordinal,
            next_attempt,
            ready_wake,
            policy_digest,
            timeout_trigger,
            cancellation_trigger,
        )),
    ))
}

pub fn admit_revalidation_lineage(
    runtime: &mut BridgeSignalRuntime,
    request: BridgeAsyncRevalidationLineageRequest,
) -> Result<BridgeAsyncRevalidationLineage, BridgeAsyncForwardCausalityRejection> {
    let prior = request.prior.clone();
    let resource_report = request.signal_report.resource_report();
    if let Some(denied) = resource_report.denied_revalidation() {
        return match denied.class() {
            ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch
            | ResourceRevalidationDenialClass::ActiveHandleProofMismatch => Err(rejected(
                BridgeAsyncForwardCausalityRejectionKind::StaleSignalGenerationRejected,
                "revalidation lineage cannot admit from stale expected-active signal generation",
            )),
            _ => Err(rejected(
                BridgeAsyncForwardCausalityRejectionKind::RevalidationAdmissionMissing,
                "revalidation report denied before bridge lineage could classify it",
            )),
        };
    }
    let admitted = resource_report.admitted_revalidation().ok_or_else(|| {
        rejected(
            BridgeAsyncForwardCausalityRejectionKind::RevalidationAdmissionMissing,
            "revalidation report must admit one replacement request",
        )
    })?;
    let family_admission = match prior.family_admission() {
        BridgeAsyncRequestFamilyAdmission::RequestResponse => {
            if request.current_subscription_instance.is_some() {
                return Err(rejected(
                    BridgeAsyncForwardCausalityRejectionKind::SubscriptionInstanceDriftForbiddenForRetry,
                    "request-response revalidation cannot carry a subscription instance",
                ));
            }
            BridgeAsyncRequestFamilyAdmission::RequestResponse
        }
        BridgeAsyncRequestFamilyAdmission::SubscriptionBacked { .. } => {
            let subscription_instance = request
                .current_subscription_instance
                .clone()
                .ok_or_else(|| rejected(BridgeAsyncForwardCausalityRejectionKind::SubscriptionInstanceRequiredForSubscriptionBackedFamily, "subscription-backed revalidation requires an explicit current subscription instance"))?;
            BridgeAsyncRequestFamilyAdmission::SubscriptionBacked {
                subscription_instance,
            }
        }
    };
    let rebind = BridgeAsyncRequestAdmissionRequest::rebind(
        prior.lowered(),
        &crate::source::ValidatedBridgeAsyncRequestBasisBinding::bind(
            prior.lowered(),
            request.current_truth_view_basis.clone(),
        ),
        &family_admission,
    )
    .map_err(map_request_identity_rejection)?;
    let newer = admit_from_existing_signal_request(
        runtime,
        rebind,
        admitted.admitted_request(),
        request.signal_report.async_decision_digest(),
    )
    .map_err(map_request_identity_rejection)?;
    let class = classify_revalidation_class(&prior, &newer, resource_report)?;
    let counters = match class {
        BridgeAsyncForwardCausalityClass::RevalidationAfterTruthBasisDrift => {
            BridgeAsyncForwardCausalityCounters::one_revalidation_after_truth_basis_drift()
        }
        BridgeAsyncForwardCausalityClass::RevalidationAfterPreviewBasisDrift => {
            BridgeAsyncForwardCausalityCounters::one_revalidation_after_preview_basis_drift()
        }
        BridgeAsyncForwardCausalityClass::RevalidationAfterSubscriptionInstanceDrift => {
            BridgeAsyncForwardCausalityCounters::one_revalidation_after_subscription_instance_drift(
            )
        }
        _ => unreachable!(),
    };
    Ok(BridgeAsyncRevalidationLineage::new(
        prior.clone(),
        newer.clone(),
        class,
        counters,
        Arc::from(format!(
            "bridge-async-forward-causality|class={class:?}|prior={}|newer={}|prior-basis={}|newer-basis={}|prior-subscription={}|newer-subscription={}|freshness-class={:?}|freshness-digest={}|expected-active={}|forced-active={}|decision={}",
            prior.request_identity().as_str(),
            newer.request_identity().as_str(),
            prior.basis_binding().truth_view_basis().digest(),
            newer.basis_binding().truth_view_basis().digest(),
            subscription_instance_digest(&prior).unwrap_or("-"),
            subscription_instance_digest(&newer).unwrap_or("-"),
            admitted.freshness_decision().class(),
            admitted.freshness_decision().freshness_digest(),
            admitted.expected_active().map(|handle| format!("{}#{}", handle.request_id().get(), handle.generation().get())).unwrap_or_else(|| "-".to_owned()),
            admitted.forced_active_handle().map(|handle| format!("{}#{}", handle.request_id().get(), handle.generation().get())).unwrap_or_else(|| "-".to_owned()),
            admitted.decision_digest().as_str(),
        )),
    ))
}

fn classify_revalidation_class(
    prior: &AdmittedBridgeAsyncRequestIdentity,
    newer: &AdmittedBridgeAsyncRequestIdentity,
    report: &ResourceRevalidationReport,
) -> Result<BridgeAsyncForwardCausalityClass, BridgeAsyncForwardCausalityRejection> {
    let admitted = report.admitted_revalidation().ok_or_else(|| {
        rejected(
            BridgeAsyncForwardCausalityRejectionKind::RevalidationAdmissionMissing,
            "revalidation report must admit one replacement request",
        )
    })?;
    if admitted.forced_active_handle().is_some() {
        return Err(rejected(
            BridgeAsyncForwardCausalityRejectionKind::StaleSignalGenerationRejected,
            "revalidation lineage cannot admit from forced-active signal generation drift",
        ));
    }
    if admitted.admitted_request().handle() != newer.request_handle() {
        return Err(rejected(
            BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerSignalHandleMismatch,
            "revalidation report must align to the newer bridge async request handle",
        ));
    }
    if prior.lowered().declaration_identity() != newer.lowered().declaration_identity() {
        return Err(rejected(
            BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerDeclarationMismatch,
            "revalidation lineage must stay within one bridge async declaration identity",
        ));
    }
    if prior.family_admission() != newer.family_admission()
        && !matches!(
            (prior.family_admission(), newer.family_admission()),
            (
                BridgeAsyncRequestFamilyAdmission::SubscriptionBacked { .. },
                BridgeAsyncRequestFamilyAdmission::SubscriptionBacked { .. }
            )
        )
    {
        return Err(rejected(
            BridgeAsyncForwardCausalityRejectionKind::PriorAndNewerFamilyMismatch,
            "revalidation lineage must stay within one bridge async family",
        ));
    }
    if prior.basis_binding().truth_view_basis().digest()
        != newer.basis_binding().truth_view_basis().digest()
    {
        let prior_basis = prior.basis_binding().truth_view_basis();
        let newer_basis = newer.basis_binding().truth_view_basis();
        if prior_basis.preview_active_subscription_identity().is_some()
            && newer_basis.preview_active_subscription_identity().is_some()
        {
            return Ok(BridgeAsyncForwardCausalityClass::RevalidationAfterPreviewBasisDrift);
        }
        return Ok(BridgeAsyncForwardCausalityClass::RevalidationAfterTruthBasisDrift);
    }
    if subscription_instance_digest(prior) != subscription_instance_digest(newer) {
        return Ok(BridgeAsyncForwardCausalityClass::RevalidationAfterSubscriptionInstanceDrift);
    }
    Err(rejected(
        BridgeAsyncForwardCausalityRejectionKind::BasisDriftRequiredForRevalidation,
        "revalidation lineage requires truth-view basis drift or subscription instance drift",
    ))
}

fn subscription_instance_digest(identity: &AdmittedBridgeAsyncRequestIdentity) -> Option<&str> {
    identity
        .subscription_instance()
        .map(|instance| instance.digest())
}
