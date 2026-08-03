#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryCompatibilityCounters {
    /// Foundational comparison front-door calls, including export comparison.
    pub canonical_comparisons: usize,
    /// Portable operation dimensions inspected by their installation owner.
    pub portable_contract_comparisons: usize,
    /// Variable-width semantic items submitted to reached owner comparisons.
    pub portable_variable_items_submitted: usize,
    /// Conditional nodes submitted to the portable owner comparator.
    pub portable_conditional_nodes_submitted: usize,
    /// Retained authority items actually inspected.
    pub retained_authority_checks: usize,
    /// Owner-issued receipts inspected while closing required-domain turnover.
    pub required_domain_rebind_receipts_inspected: usize,
    pub conditional_lowerings_compared: usize,
    pub conditional_foundational_comparisons: usize,
    pub conditional_bridge_contract_comparisons: usize,
    pub conditional_liveness_checks: usize,
    pub conditional_correspondences_inspected: usize,
    pub conditional_targets_inspected: usize,
    pub conditional_provider_roles_inspected: usize,
    pub conditional_signal_semantic_dimensions_inspected: usize,
    pub conditional_signal_affinity_dimensions_inspected: usize,
    pub conditional_bridge_affinity_dimensions_inspected: usize,
    pub lower_runtime_contacts: usize,
    pub execution_calls: usize,
    pub maintenance_calls: usize,
}
