use crate::identity::hash_parts;

use super::activation::SubscriptionActivationInput;
use super::active_budget::ActiveSubscriptionWorkBudget;
use super::active_counters::ActiveSubscriptionCounters;
use super::active_digest::ActiveSubscriptionLaneDigest;
use super::active_error::{
    ActiveSubscriptionLifecycleDenialKind, ActiveSubscriptionLifecycleError,
};
use super::active_lane::ActiveSubscriptionLaneAdmission;
use super::active_posture::{
    ActiveLaneLookupClass, ActiveSubscriptionDeliveryPosture, ActiveSubscriptionLifecyclePosture,
};
use super::delivery_density::ActiveDeliveryDensityPosture;
use super::performance_receipt::SubscriptionPerformanceReceipt;

pub fn admit_active_subscription_lane(
    activation: SubscriptionActivationInput,
    budget: ActiveSubscriptionWorkBudget,
) -> Result<ActiveSubscriptionLaneAdmission, ActiveSubscriptionLifecycleError> {
    let source_digest = activation.activation_digest().to_string();
    let mut counters = ActiveSubscriptionCounters::default();

    if budget.exceeds_phase_one_budget() {
        counters.active_lane_allocation_denial_count = 1;
        return Err(ActiveSubscriptionLifecycleError::new(
            ActiveSubscriptionLifecycleDenialKind::WorkBudgetExceeded,
            "active subscription lane admission exceeds its explicit Phase 1 budget",
            source_digest,
            counters,
        ));
    }

    if budget.durable_checkpoint_requested() {
        counters.durable_checkpoint_overclaim_denial_count = 1;
        return Err(ActiveSubscriptionLifecycleError::new(
            ActiveSubscriptionLifecycleDenialKind::DurableCheckpointOverclaim,
            "durable active subscription checkpoints remain later-milestone debt",
            source_digest,
            counters,
        ));
    }

    if budget.store_backed_restart_requested() {
        counters.store_backed_restart_overclaim_denial_count = 1;
        return Err(ActiveSubscriptionLifecycleError::new(
            ActiveSubscriptionLifecycleDenialKind::StoreBackedRestartOverclaim,
            "store-backed restart-stable active subscription handles remain later-milestone debt",
            source_digest,
            counters,
        ));
    }

    if budget.allocation_posture().is_heap_denied()
        || !budget.allocation_posture().admits_lifecycle_phase()
    {
        counters.heap_allocation_denial_count = 1;
        counters.active_lane_allocation_denial_count = 1;
        return Err(ActiveSubscriptionLifecycleError::new(
            ActiveSubscriptionLifecycleDenialKind::HeapAllocationForbidden,
            "active lane allocation must use an admitted lifecycle allocation posture",
            source_digest,
            counters,
        ));
    }

    if budget.allocation_posture().is_heap_debt() {
        counters.heap_allocation_debt_count = 1;
    }

    if budget.lookup_class() == &ActiveLaneLookupClass::LinearScanDebtExplicit {
        counters.active_lane_linear_scan_debt_count = 1;
    }

    if budget.lookup_class() == &ActiveLaneLookupClass::LinearScanDenied {
        counters.active_lane_linear_scan_denial_count = 1;
        return Err(ActiveSubscriptionLifecycleError::new(
            ActiveSubscriptionLifecycleDenialKind::LinearScanLookupForbidden,
            "active lane admission must use an indexed lookup class",
            source_digest,
            counters,
        ));
    }

    counters.active_lane_admission_count = 1;
    counters.active_lane_lookup_class_count = 1;
    let performance_receipt = SubscriptionPerformanceReceipt::new(
        3,
        budget.registry_lookup_width() + budget.fanout_width() + budget.allocation_scope_width(),
        ActiveDeliveryDensityPosture::SparseDelta,
        budget.allocation_posture(),
        &source_digest,
    );
    counters.subscription_performance_receipt_count = 1;
    counters.subscription_budget_consumption_width = performance_receipt.consumed_width();
    counters.subscription_budget_remaining_width = performance_receipt.remaining_width();
    let lifecycle_posture = ActiveSubscriptionLifecyclePosture::SingleConsumer;
    let delivery_posture = ActiveSubscriptionDeliveryPosture::QueryShapedPatch;
    let lane_digest = ActiveSubscriptionLaneDigest::new(hash_parts(&[
        "active_subscription_lane_v1".to_string(),
        format!("activation:{}", activation.activation_digest()),
        format!("admission:{}", activation.admission_digest()),
        format!(
            "query_declaration:{}",
            activation.query_declaration_digest()
        ),
        format!(
            "bridge_declaration:{}",
            activation.bridge_declaration_digest()
        ),
        format!("basis:{}", activation.basis_binding_digest()),
        format!("signal_strategy:{}", activation.signal_strategy_digest()),
        format!("lifecycle:{}", lifecycle_posture.as_str()),
        format!("delivery:{}", delivery_posture.as_str()),
        format!("lookup:{}", budget.lookup_class().as_str()),
        format!("allocation:{}", budget.allocation_policy().as_str()),
        format!("budget:registry:{}", budget.registry_lookup_width()),
        format!("budget:fanout:{}", budget.fanout_width()),
        format!("budget:allocation:{}", budget.allocation_scope_width()),
        format!(
            "performance:{}",
            performance_receipt.performance_receipt_digest()
        ),
        format!("counters:{}", counters.digest()),
    ]));

    Ok(ActiveSubscriptionLaneAdmission {
        lane_digest,
        activation_digest: activation.activation_digest().to_string(),
        admission_digest: activation.admission_digest().to_string(),
        query_declaration_digest: activation.query_declaration_digest().to_string(),
        bridge_declaration_digest: activation.bridge_declaration_digest().to_string(),
        basis_binding_digest: activation.basis_binding_digest().to_string(),
        signal_strategy_digest: activation.signal_strategy_digest().to_string(),
        lifecycle_posture,
        delivery_posture,
        lookup_class: *budget.lookup_class(),
        allocation_policy: *budget.allocation_policy(),
        budget,
        performance_receipt,
        counters,
    })
}
