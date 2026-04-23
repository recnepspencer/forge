use crate::identity::hash_parts;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QuerySubscriptionDeclarationCounters {
    pub(super) family_selection_count: u64,
    pub(super) family_denial_count: u64,
    pub(super) family_registry_lookup_count: u64,
    pub(super) view_family_registry_lookup_count: u64,
    pub(super) equivalence_digest_part_count: u64,
    pub(super) admission_dimension_denial_count: u64,
    pub(super) work_budget_denial_count: u64,
    pub(super) unknown_cost_denial_count: u64,
    pub(super) raw_cdc_fallback_denial_count: u64,
    pub(super) host_observer_inference_denial_count: u64,
    pub(super) relationship_proof_drift_denial_count: u64,
    pub(super) declaration_count: u64,
    pub(super) declaration_denial_count: u64,
    pub(super) declared_slice_count: u64,
    pub(super) deduplicated_slice_count: u64,
    pub(super) slice_deduplication_input_count: u64,
    pub(super) slice_sort_comparison_count: u64,
    pub(super) masked_slice_denial_count: u64,
    pub(super) delivery_intent_denial_count: u64,
    pub(super) declaration_digest_part_count: u64,
    pub(super) bridge_lowering_count: u64,
    pub(super) bridge_family_denial_count: u64,
    pub(super) bridge_fallback_denial_count: u64,
    pub(super) bridge_family_registry_lookup_count: u64,
    pub(super) bridge_slice_count: u64,
    pub(super) bridge_slice_denial_count: u64,
    pub(super) bridge_slice_registry_lookup_count: u64,
    pub(super) basis_binding_request_count: u64,
    pub(super) basis_binding_denial_count: u64,
    pub(super) signal_strategy_request_count: u64,
    pub(super) admission_count: u64,
    pub(super) admission_denial_count: u64,
    pub(super) durable_overclaim_denial_count: u64,
    pub(super) activation_input_count: u64,
    pub(super) active_state_allocation_denial_count: u64,
    pub(super) declaration_time_checkpoint_denial_count: u64,
    pub(super) scratch_allocation_count: u64,
    pub(super) forbidden_heap_allocation_denial_count: u64,
}

impl QuerySubscriptionDeclarationCounters {
    pub fn digest(&self) -> String {
        hash_parts(&[
            format!("family_selection:{}", self.family_selection_count),
            format!("family_denial:{}", self.family_denial_count),
            format!(
                "family_registry_lookup:{}",
                self.family_registry_lookup_count
            ),
            format!(
                "view_family_registry_lookup:{}",
                self.view_family_registry_lookup_count
            ),
            format!(
                "equivalence_digest_part:{}",
                self.equivalence_digest_part_count
            ),
            format!(
                "admission_dimension_denial:{}",
                self.admission_dimension_denial_count
            ),
            format!("work_budget_denial:{}", self.work_budget_denial_count),
            format!("unknown_cost_denial:{}", self.unknown_cost_denial_count),
            format!(
                "raw_cdc_fallback_denial:{}",
                self.raw_cdc_fallback_denial_count
            ),
            format!(
                "host_observer_inference_denial:{}",
                self.host_observer_inference_denial_count
            ),
            format!(
                "relationship_proof_drift_denial:{}",
                self.relationship_proof_drift_denial_count
            ),
            format!("declaration:{}", self.declaration_count),
            format!("declaration_denial:{}", self.declaration_denial_count),
            format!("declared_slice:{}", self.declared_slice_count),
            format!("deduplicated_slice:{}", self.deduplicated_slice_count),
            format!(
                "slice_deduplication_input:{}",
                self.slice_deduplication_input_count
            ),
            format!("slice_sort_comparison:{}", self.slice_sort_comparison_count),
            format!("masked_slice_denial:{}", self.masked_slice_denial_count),
            format!(
                "delivery_intent_denial:{}",
                self.delivery_intent_denial_count
            ),
            format!(
                "declaration_digest_part:{}",
                self.declaration_digest_part_count
            ),
            format!("bridge_lowering:{}", self.bridge_lowering_count),
            format!("bridge_family_denial:{}", self.bridge_family_denial_count),
            format!(
                "bridge_fallback_denial:{}",
                self.bridge_fallback_denial_count
            ),
            format!(
                "bridge_family_registry_lookup:{}",
                self.bridge_family_registry_lookup_count
            ),
            format!("bridge_slice:{}", self.bridge_slice_count),
            format!("bridge_slice_denial:{}", self.bridge_slice_denial_count),
            format!(
                "bridge_slice_registry_lookup:{}",
                self.bridge_slice_registry_lookup_count
            ),
            format!("basis_binding_request:{}", self.basis_binding_request_count),
            format!("basis_binding_denial:{}", self.basis_binding_denial_count),
            format!(
                "signal_strategy_request:{}",
                self.signal_strategy_request_count
            ),
            format!("admission:{}", self.admission_count),
            format!("admission_denial:{}", self.admission_denial_count),
            format!(
                "durable_overclaim_denial:{}",
                self.durable_overclaim_denial_count
            ),
            format!("activation_input:{}", self.activation_input_count),
            format!(
                "active_state_allocation_denial:{}",
                self.active_state_allocation_denial_count
            ),
            format!(
                "declaration_time_checkpoint_denial:{}",
                self.declaration_time_checkpoint_denial_count
            ),
            format!("scratch_allocation:{}", self.scratch_allocation_count),
            format!(
                "forbidden_heap_allocation_denial:{}",
                self.forbidden_heap_allocation_denial_count
            ),
        ])
    }

    pub fn family_selection_count(&self) -> u64 {
        self.family_selection_count
    }

    pub fn family_denial_count(&self) -> u64 {
        self.family_denial_count
    }

    pub fn family_registry_lookup_count(&self) -> u64 {
        self.family_registry_lookup_count
    }

    pub fn view_family_registry_lookup_count(&self) -> u64 {
        self.view_family_registry_lookup_count
    }

    pub fn equivalence_digest_part_count(&self) -> u64 {
        self.equivalence_digest_part_count
    }

    pub fn admission_dimension_denial_count(&self) -> u64 {
        self.admission_dimension_denial_count
    }

    pub fn work_budget_denial_count(&self) -> u64 {
        self.work_budget_denial_count
    }

    pub fn unknown_cost_denial_count(&self) -> u64 {
        self.unknown_cost_denial_count
    }

    pub fn raw_cdc_fallback_denial_count(&self) -> u64 {
        self.raw_cdc_fallback_denial_count
    }

    pub fn host_observer_inference_denial_count(&self) -> u64 {
        self.host_observer_inference_denial_count
    }

    pub fn relationship_proof_drift_denial_count(&self) -> u64 {
        self.relationship_proof_drift_denial_count
    }

    pub fn declaration_count(&self) -> u64 {
        self.declaration_count
    }

    pub fn declaration_denial_count(&self) -> u64 {
        self.declaration_denial_count
    }

    pub fn declared_slice_count(&self) -> u64 {
        self.declared_slice_count
    }

    pub fn deduplicated_slice_count(&self) -> u64 {
        self.deduplicated_slice_count
    }

    pub fn slice_deduplication_input_count(&self) -> u64 {
        self.slice_deduplication_input_count
    }

    pub fn slice_sort_comparison_count(&self) -> u64 {
        self.slice_sort_comparison_count
    }

    pub fn masked_slice_denial_count(&self) -> u64 {
        self.masked_slice_denial_count
    }

    pub fn delivery_intent_denial_count(&self) -> u64 {
        self.delivery_intent_denial_count
    }

    pub fn declaration_digest_part_count(&self) -> u64 {
        self.declaration_digest_part_count
    }

    pub fn bridge_lowering_count(&self) -> u64 {
        self.bridge_lowering_count
    }

    pub fn bridge_family_denial_count(&self) -> u64 {
        self.bridge_family_denial_count
    }

    pub fn bridge_fallback_denial_count(&self) -> u64 {
        self.bridge_fallback_denial_count
    }

    pub fn bridge_family_registry_lookup_count(&self) -> u64 {
        self.bridge_family_registry_lookup_count
    }

    pub fn bridge_slice_count(&self) -> u64 {
        self.bridge_slice_count
    }

    pub fn bridge_slice_denial_count(&self) -> u64 {
        self.bridge_slice_denial_count
    }

    pub fn bridge_slice_registry_lookup_count(&self) -> u64 {
        self.bridge_slice_registry_lookup_count
    }

    pub fn basis_binding_request_count(&self) -> u64 {
        self.basis_binding_request_count
    }

    pub fn basis_binding_denial_count(&self) -> u64 {
        self.basis_binding_denial_count
    }

    pub fn signal_strategy_request_count(&self) -> u64 {
        self.signal_strategy_request_count
    }

    pub fn admission_count(&self) -> u64 {
        self.admission_count
    }

    pub fn admission_denial_count(&self) -> u64 {
        self.admission_denial_count
    }

    pub fn durable_overclaim_denial_count(&self) -> u64 {
        self.durable_overclaim_denial_count
    }

    pub fn activation_input_count(&self) -> u64 {
        self.activation_input_count
    }

    pub fn active_state_allocation_denial_count(&self) -> u64 {
        self.active_state_allocation_denial_count
    }

    pub fn declaration_time_checkpoint_denial_count(&self) -> u64 {
        self.declaration_time_checkpoint_denial_count
    }

    pub fn scratch_allocation_count(&self) -> u64 {
        self.scratch_allocation_count
    }

    pub fn forbidden_heap_allocation_denial_count(&self) -> u64 {
        self.forbidden_heap_allocation_denial_count
    }
}
