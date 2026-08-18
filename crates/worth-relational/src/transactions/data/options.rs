use crate::branch::RelationalLegacyBranchBinding;
use crate::history::data::BranchId;
use crate::schema::data::{ProposedSchemaTransition, SchemaReconciliationPolicy};

/// Runtime-owned transaction inputs.
///
/// A transaction cannot be created from a branch id, a serialized options
/// value, or an ambient default. The binding is minted by the owning runtime
/// from one exact branch cell and carries the currentness observation used by
/// validation and publication. The remaining knobs describe execution
/// policy; they never select a branch or resolve a head.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionOptions {
    pub(crate) allow_nested_savepoints: bool,
    pub(crate) diagnostics_required: bool,
    pub(crate) deterministic_merge_required: bool,
    pub(crate) branch_binding: RelationalLegacyBranchBinding,
    pub(crate) merge_parent_bindings: Vec<RelationalLegacyBranchBinding>,
    pub(crate) proposed_schema_transition: Option<ProposedSchemaTransition>,
    pub(crate) schema_reconciliation_policy: Option<SchemaReconciliationPolicy>,
}

impl TransactionOptions {
    pub(crate) fn from_owner_binding(binding: RelationalLegacyBranchBinding) -> Self {
        Self {
            allow_nested_savepoints: true,
            diagnostics_required: true,
            deterministic_merge_required: true,
            branch_binding: binding,
            merge_parent_bindings: Vec::new(),
            proposed_schema_transition: None,
            schema_reconciliation_policy: None,
        }
    }

    pub(crate) fn branch_binding(&self) -> &RelationalLegacyBranchBinding {
        &self.branch_binding
    }

    pub(crate) fn target_branch(&self) -> &BranchId {
        self.branch_binding.identity().branch_id()
    }

    pub(crate) fn merge_parent_bindings(&self) -> &[RelationalLegacyBranchBinding] {
        &self.merge_parent_bindings
    }

    pub(crate) fn merge_parent_branch_ids(&self) -> Vec<BranchId> {
        self.merge_parent_bindings
            .iter()
            .map(|binding| binding.identity().branch_id().clone())
            .collect()
    }

    /// Attach immutable merge provenance supplied by the owner-controlled
    /// merge planner. Every binding carries the exact owner observation and
    /// branch-local truth version and is checked again before parent
    /// observations are admitted.
    pub(crate) fn with_merge_parent_bindings(
        mut self,
        bindings: Vec<RelationalLegacyBranchBinding>,
    ) -> Self {
        self.merge_parent_bindings = bindings;
        self
    }

    pub(crate) fn allow_nested_savepoints(&self) -> bool {
        self.allow_nested_savepoints
    }

    pub(crate) fn proposed_schema_transition(&self) -> Option<&ProposedSchemaTransition> {
        self.proposed_schema_transition.as_ref()
    }

    pub(crate) fn schema_reconciliation_policy(&self) -> Option<&SchemaReconciliationPolicy> {
        self.schema_reconciliation_policy.as_ref()
    }

    pub fn with_schema_transition(
        mut self,
        proposed_schema_transition: ProposedSchemaTransition,
        schema_reconciliation_policy: Option<SchemaReconciliationPolicy>,
    ) -> Self {
        self.proposed_schema_transition = Some(proposed_schema_transition);
        self.schema_reconciliation_policy = schema_reconciliation_policy;
        self
    }
}
