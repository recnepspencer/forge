use crate::identity::hash_parts;

use super::admission_budget::QuerySubscriptionAdmissionBudget;
use super::admission_diagnostics::{
    QuerySubscriptionAdmissionDiagnosticOutcome, QuerySubscriptionAdmissionDiagnosticStage,
    QuerySubscriptionAdmissionDiagnostics,
};
use super::admission_error::{
    QuerySubscriptionAdmissionDenialKind, QuerySubscriptionAdmissionError,
};
use super::bridge_lowering::BridgeSubscriptionLoweringPlan;
use super::counters::QuerySubscriptionDeclarationCounters;
use super::diagnostic::{QuerySubscriptionDiagnosticEvidence, QuerySubscriptionDiagnosticStage};
use super::support::QuerySubscriptionSupportProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionAdmissionArtifact {
    admission_digest: String,
    query_declaration_digest: String,
    bridge_declaration_digest: String,
    basis_binding_digest: String,
    signal_strategy_digest: String,
    admission_budget: QuerySubscriptionAdmissionBudget,
    diagnostics: QuerySubscriptionAdmissionDiagnostics,
    support_profile: QuerySubscriptionSupportProfile,
    counters: QuerySubscriptionDeclarationCounters,
}

impl QuerySubscriptionAdmissionArtifact {
    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        &self.bridge_declaration_digest
    }

    pub fn basis_binding_digest(&self) -> &str {
        &self.basis_binding_digest
    }

    pub fn signal_strategy_digest(&self) -> &str {
        &self.signal_strategy_digest
    }

    pub fn admission_budget(&self) -> &QuerySubscriptionAdmissionBudget {
        &self.admission_budget
    }

    pub fn diagnostics(&self) -> &QuerySubscriptionAdmissionDiagnostics {
        &self.diagnostics
    }

    pub fn support_profile(&self) -> &QuerySubscriptionSupportProfile {
        &self.support_profile
    }

    pub fn counters(&self) -> &QuerySubscriptionDeclarationCounters {
        &self.counters
    }
}

pub fn admit_query_subscription(
    lowering: BridgeSubscriptionLoweringPlan,
    admission_budget: QuerySubscriptionAdmissionBudget,
) -> Result<QuerySubscriptionAdmissionArtifact, QuerySubscriptionAdmissionError> {
    let mut counters = lowering.counters().clone();
    let source_digest = lowering.bridge_declaration_digest();

    if exceeds_admission_budget(&admission_budget) {
        counters.admission_denial_count = 1;
        counters.work_budget_denial_count = 1;
        return Err(admission_error(
            QuerySubscriptionAdmissionDenialKind::AdmissionBudgetExceeded,
            "subscription admission exceeds its explicit admission budget",
            QuerySubscriptionAdmissionDiagnosticStage::AdmissionBudget,
            source_digest,
            counters,
        ));
    }

    if admission_budget.durable_reload_requested() {
        counters.admission_denial_count = 1;
        counters.durable_overclaim_denial_count = 1;
        counters.declaration_time_checkpoint_denial_count = 1;
        return Err(admission_error(
            QuerySubscriptionAdmissionDenialKind::DurableReloadOverclaim,
            "durable subscription reload remains later-milestone debt",
            QuerySubscriptionAdmissionDiagnosticStage::DurableReloadOverclaim,
            source_digest,
            counters,
        ));
    }

    if admission_budget.active_lifecycle_allocation_requested() {
        counters.admission_denial_count = 1;
        counters.active_state_allocation_denial_count = 1;
        return Err(admission_error(
            QuerySubscriptionAdmissionDenialKind::ActiveLifecycleAllocationForbidden,
            "Milestone 9.1 admission may not allocate active subscription lifecycle state",
            QuerySubscriptionAdmissionDiagnosticStage::ActiveLifecycleAllocation,
            source_digest,
            counters,
        ));
    }

    counters.admission_count = 1;
    let diagnostics = QuerySubscriptionAdmissionDiagnostics::new(
        QuerySubscriptionAdmissionDiagnosticStage::RuntimeBackedAdmission,
        QuerySubscriptionAdmissionDiagnosticOutcome::Admitted,
        "runtime-backed query subscription declaration admitted for activation handoff",
        source_digest,
    );
    let support_profile = QuerySubscriptionSupportProfile::admitted(source_digest);
    let mut admission_digest_parts = vec![
        "query_subscription_admission_artifact_v1".to_string(),
        format!("query_declaration:{}", lowering.query_declaration_digest()),
        format!(
            "bridge_declaration:{}",
            lowering.bridge_declaration_digest()
        ),
        format!("basis:{}", lowering.basis_request().digest()),
        format!(
            "signal_strategy:{}",
            lowering.signal_strategy_request().digest()
        ),
        format!("diagnostics:{}", diagnostics.digest()),
        format!("support:{}", support_profile.digest()),
        format!(
            "budget:declaration:{}",
            admission_budget.declaration_input_width_limit()
        ),
        format!(
            "budget:bridge:{}",
            admission_budget.bridge_plan_width_limit()
        ),
        format!(
            "budget:basis:{}",
            admission_budget.basis_binding_width_limit()
        ),
        format!(
            "budget:signal:{}",
            admission_budget.signal_strategy_width_limit()
        ),
        format!(
            "budget:activation:{}",
            admission_budget.activation_input_width_limit()
        ),
    ];
    admission_digest_parts.extend(admission_counter_digest_parts(&counters));
    let admission_digest = hash_parts(&admission_digest_parts);

    Ok(QuerySubscriptionAdmissionArtifact {
        admission_digest,
        query_declaration_digest: lowering.query_declaration_digest().to_string(),
        bridge_declaration_digest: lowering.bridge_declaration_digest().to_string(),
        basis_binding_digest: lowering.basis_request().digest().to_string(),
        signal_strategy_digest: lowering.signal_strategy_request().digest().to_string(),
        admission_budget,
        diagnostics,
        support_profile,
        counters,
    })
}

fn admission_error(
    denial_kind: QuerySubscriptionAdmissionDenialKind,
    message: &'static str,
    stage: QuerySubscriptionAdmissionDiagnosticStage,
    source_digest: &str,
    counters: QuerySubscriptionDeclarationCounters,
) -> QuerySubscriptionAdmissionError {
    let diagnostics = QuerySubscriptionAdmissionDiagnostics::new(
        stage,
        QuerySubscriptionAdmissionDiagnosticOutcome::Denied,
        message,
        source_digest,
    );
    let pipeline_diagnostic = QuerySubscriptionDiagnosticEvidence::denied(
        admission_pipeline_stage(stage),
        message,
        source_digest,
        counters.digest(),
    );
    let support_profile = QuerySubscriptionSupportProfile::denied(source_digest);
    QuerySubscriptionAdmissionError::new(
        denial_kind,
        message,
        diagnostics,
        pipeline_diagnostic,
        support_profile,
        counters,
    )
}

fn admission_pipeline_stage(
    stage: QuerySubscriptionAdmissionDiagnosticStage,
) -> QuerySubscriptionDiagnosticStage {
    match stage {
        QuerySubscriptionAdmissionDiagnosticStage::RuntimeBackedAdmission => {
            QuerySubscriptionDiagnosticStage::RuntimeBackedAdmission
        }
        QuerySubscriptionAdmissionDiagnosticStage::AdmissionBudget => {
            QuerySubscriptionDiagnosticStage::AdmissionBudget
        }
        QuerySubscriptionAdmissionDiagnosticStage::DurableReloadOverclaim => {
            QuerySubscriptionDiagnosticStage::DurableReloadOverclaim
        }
        QuerySubscriptionAdmissionDiagnosticStage::ActiveLifecycleAllocation => {
            QuerySubscriptionDiagnosticStage::ActiveLifecycleAllocation
        }
        QuerySubscriptionAdmissionDiagnosticStage::ActivationReadiness => {
            QuerySubscriptionDiagnosticStage::ActivationReadiness
        }
    }
}

fn exceeds_admission_budget(budget: &QuerySubscriptionAdmissionBudget) -> bool {
    budget.declaration_input_width_limit() < 1
        || budget.bridge_plan_width_limit() < 1
        || budget.basis_binding_width_limit() < 1
        || budget.signal_strategy_width_limit() < 1
        || budget.activation_input_width_limit() < 1
}

fn admission_counter_digest_parts(counters: &QuerySubscriptionDeclarationCounters) -> Vec<String> {
    vec![
        format!(
            "counter:family_selection:{}",
            counters.family_selection_count()
        ),
        format!("counter:family_denial:{}", counters.family_denial_count()),
        format!(
            "counter:family_registry_lookup:{}",
            counters.family_registry_lookup_count()
        ),
        format!(
            "counter:view_family_registry_lookup:{}",
            counters.view_family_registry_lookup_count()
        ),
        format!(
            "counter:equivalence_digest_part:{}",
            counters.equivalence_digest_part_count()
        ),
        format!(
            "counter:admission_dimension_denial:{}",
            counters.admission_dimension_denial_count()
        ),
        format!(
            "counter:work_budget_denial:{}",
            counters.work_budget_denial_count()
        ),
        format!(
            "counter:unknown_cost_denial:{}",
            counters.unknown_cost_denial_count()
        ),
        format!(
            "counter:raw_cdc_fallback_denial:{}",
            counters.raw_cdc_fallback_denial_count()
        ),
        format!(
            "counter:host_observer_inference_denial:{}",
            counters.host_observer_inference_denial_count()
        ),
        format!(
            "counter:relationship_proof_drift_denial:{}",
            counters.relationship_proof_drift_denial_count()
        ),
        format!("counter:declaration:{}", counters.declaration_count()),
        format!(
            "counter:declaration_denial:{}",
            counters.declaration_denial_count()
        ),
        format!("counter:declared_slice:{}", counters.declared_slice_count()),
        format!(
            "counter:deduplicated_slice:{}",
            counters.deduplicated_slice_count()
        ),
        format!(
            "counter:slice_deduplication_input:{}",
            counters.slice_deduplication_input_count()
        ),
        format!(
            "counter:slice_sort_comparison:{}",
            counters.slice_sort_comparison_count()
        ),
        format!(
            "counter:masked_slice_denial:{}",
            counters.masked_slice_denial_count()
        ),
        format!(
            "counter:delivery_intent_denial:{}",
            counters.delivery_intent_denial_count()
        ),
        format!(
            "counter:declaration_digest_part:{}",
            counters.declaration_digest_part_count()
        ),
        format!(
            "counter:bridge_lowering:{}",
            counters.bridge_lowering_count()
        ),
        format!(
            "counter:bridge_family_denial:{}",
            counters.bridge_family_denial_count()
        ),
        format!(
            "counter:bridge_fallback_denial:{}",
            counters.bridge_fallback_denial_count()
        ),
        format!(
            "counter:bridge_family_registry_lookup:{}",
            counters.bridge_family_registry_lookup_count()
        ),
        format!("counter:bridge_slice:{}", counters.bridge_slice_count()),
        format!(
            "counter:bridge_slice_denial:{}",
            counters.bridge_slice_denial_count()
        ),
        format!(
            "counter:bridge_slice_registry_lookup:{}",
            counters.bridge_slice_registry_lookup_count()
        ),
        format!(
            "counter:basis_binding_request:{}",
            counters.basis_binding_request_count()
        ),
        format!(
            "counter:basis_binding_denial:{}",
            counters.basis_binding_denial_count()
        ),
        format!(
            "counter:signal_strategy_request:{}",
            counters.signal_strategy_request_count()
        ),
        format!("counter:admission:{}", counters.admission_count()),
        format!(
            "counter:admission_denial:{}",
            counters.admission_denial_count()
        ),
        format!(
            "counter:durable_overclaim_denial:{}",
            counters.durable_overclaim_denial_count()
        ),
        format!(
            "counter:activation_input:{}",
            counters.activation_input_count()
        ),
        format!(
            "counter:active_state_allocation_denial:{}",
            counters.active_state_allocation_denial_count()
        ),
        format!(
            "counter:declaration_time_checkpoint_denial:{}",
            counters.declaration_time_checkpoint_denial_count()
        ),
        format!(
            "counter:scratch_allocation:{}",
            counters.scratch_allocation_count()
        ),
        format!(
            "counter:forbidden_heap_allocation_denial:{}",
            counters.forbidden_heap_allocation_denial_count()
        ),
    ]
}
