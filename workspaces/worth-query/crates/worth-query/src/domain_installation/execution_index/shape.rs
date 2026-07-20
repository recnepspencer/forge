#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthQueryInstalledDomainExecutionIndexShape {
    pub(crate) invariant_count: usize,
    pub(crate) graph_obligation_count: usize,
    pub(crate) operation_count: usize,
    pub(crate) domain_operation_count: usize,
    pub(crate) operation_graph_participation_count: usize,
    pub(crate) operation_required_domain_count: usize,
    pub(crate) declaration_family_count: usize,
    pub(crate) contribution_policy_count: usize,
}

pub(super) struct WorthQueryExecutionIndexShapeInputs<'a> {
    pub(super) graph_read_operations:
        &'a BTreeMap<WorthQueryGraphReadOperationKey, InstalledGraphReadOperation>,
    pub(super) domain_operations: &'a HashMap<(TypeId, TypeId, TypeId), InstalledDomainOperation>,
    pub(super) operation_graph_participations:
        &'a HashMap<(TypeId, TypeId, TypeId), Vec<WorthQueryInstalledOperationGraphBinding>>,
    pub(super) operation_required_domains:
        &'a HashMap<(TypeId, TypeId, TypeId), Vec<WorthQueryInstalledOperationRequiredDomain>>,
    pub(super) declaration_families: &'a BTreeMap<InstalledDeclarationFamilySlot, String>,
    pub(super) contribution_policies: &'a BTreeMap<InstalledDomainOwner, Vec<String>>,
    pub(super) invariant_slots: &'a BTreeMap<InstalledInvariantSlot, String>,
    pub(super) graph_obligation_identity_parts: &'a [String],
}

pub(super) fn execution_index_shape(
    inputs: WorthQueryExecutionIndexShapeInputs<'_>,
) -> WorthQueryInstalledDomainExecutionIndexShape {
    WorthQueryInstalledDomainExecutionIndexShape {
        invariant_count: inputs.invariant_slots.len(),
        graph_obligation_count: inputs.graph_obligation_identity_parts.len(),
        operation_count: inputs.graph_read_operations.len(),
        domain_operation_count: inputs.domain_operations.len(),
        operation_graph_participation_count: inputs
            .operation_graph_participations
            .values()
            .map(Vec::len)
            .sum(),
        operation_required_domain_count: inputs
            .operation_required_domains
            .values()
            .map(Vec::len)
            .sum(),
        declaration_family_count: inputs.declaration_families.len(),
        contribution_policy_count: inputs.contribution_policies.values().map(Vec::len).sum(),
    }
}
use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};

use crate::authoring::WorthQueryGraphReadOperationKey;

use super::{
    InstalledDeclarationFamilySlot, InstalledDomainOperation, InstalledDomainOwner,
    InstalledGraphReadOperation, InstalledInvariantSlot, WorthQueryInstalledOperationGraphBinding,
    WorthQueryInstalledOperationRequiredDomain,
};
