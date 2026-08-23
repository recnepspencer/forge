use super::super::{
    WorthQueryInvariantExecutionContract, WorthQueryOperationCollectionContract,
    WorthQueryOperationEffectContract, WorthQueryOperationGraphReadContract,
    WorthQueryOperationInvariantContract, WorthQueryOperationParameterContract,
    WorthQueryOperationTouchContract,
};

pub(super) fn parameters(contract: &WorthQueryOperationParameterContract) -> usize {
    match contract {
        WorthQueryOperationParameterContract::NotRequired => 0,
        WorthQueryOperationParameterContract::Declared { fields } => fields.len(),
    }
}

pub(super) fn canonical_query(
    query: &worth_query_declaration::facade::canonicalization::CanonicalQueryBundle,
) -> usize {
    let query_artifact = query.query();
    query_artifact.projection().len()
        + query_artifact.predicates().len()
        + query_artifact.ordering().len()
        + query_artifact.traversal().len()
        + query_artifact.identity_bindings().len()
        + query.result_shape().fields().len()
}

pub(super) fn collection(contract: &WorthQueryOperationCollectionContract) -> usize {
    match contract {
        WorthQueryOperationCollectionContract::NotCollection => 0,
        WorthQueryOperationCollectionContract::Collection {
            ordering_fields,
            grouping,
            ..
        } => {
            ordering_fields.len()
                + match grouping {
                    super::super::WorthQueryOperationGroupingContract::Ungrouped => 0,
                    super::super::WorthQueryOperationGroupingContract::Grouped {
                        grouping_fields,
                    } => grouping_fields.len(),
                }
                + 2
        }
    }
}

pub(super) fn graph_reads(contract: &WorthQueryOperationGraphReadContract) -> usize {
    match contract {
        WorthQueryOperationGraphReadContract::NotRequired => 0,
        WorthQueryOperationGraphReadContract::Declared { roles } => {
            roles.iter().map(|role| 1 + role.read_scopes().len()).sum()
        }
        WorthQueryOperationGraphReadContract::DeclaredDomain { roles } => {
            roles.iter().map(|role| 1 + role.semantic_reads.len()).sum()
        }
    }
}

pub(super) fn touches(contract: &WorthQueryOperationTouchContract) -> usize {
    match contract {
        WorthQueryOperationTouchContract::NotRequired => 0,
        WorthQueryOperationTouchContract::Declared {
            graph_roles,
            scopes,
        } => graph_roles.len() + scopes.len(),
    }
}

pub(super) fn effects(contract: &WorthQueryOperationEffectContract) -> usize {
    match contract {
        WorthQueryOperationEffectContract::NotRequired => 0,
        WorthQueryOperationEffectContract::Declared { effect_families } => effect_families.len(),
    }
}

pub(super) fn invariants(contract: &WorthQueryOperationInvariantContract) -> usize {
    match contract {
        WorthQueryOperationInvariantContract::NotRequired => 0,
        WorthQueryOperationInvariantContract::Declared { invariant_slots } => invariant_slots.len(),
    }
}

pub(super) fn invariant_execution(contract: &WorthQueryInvariantExecutionContract) -> usize {
    match contract {
        WorthQueryInvariantExecutionContract::NotRequired => 0,
        WorthQueryInvariantExecutionContract::Declared { requirements } => requirements
            .iter()
            .map(|requirement| 7 + requirement.state_load_families().len())
            .sum(),
    }
}
