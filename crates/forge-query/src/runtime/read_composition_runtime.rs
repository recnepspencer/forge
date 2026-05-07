use crate::basis::{
    preflight_execution_basis, resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode,
    ExecutionBasisIntent, SnapshotLineageClass,
};
use crate::execution::execute_preflight_bundle;
use crate::runtime::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadDenial, ForgeQueryReadDenialKind,
    ForgeQueryReadGraph, ForgeQueryReadResult, ForgeQueryReadScopeClass, ForgeQueryRuntime,
};

pub(super) fn classify_scope_shape_with_operators(
    validated: &crate::validation::ValidatedQueryBundle,
    built_in_operators: &[ForgeQueryReadBuiltInOperator],
) -> ForgeQueryReadScopeClass {
    let traversal = validated.query().traversal();
    let traversal_depth_limit = traversal
        .iter()
        .map(|entry| entry.depth())
        .max()
        .unwrap_or(0);
    let non_anchor_predicate_count = validated
        .query()
        .predicates()
        .entries()
        .iter()
        .filter(|predicate| !is_identity_anchor_predicate(predicate))
        .count();

    if built_in_operators.contains(&ForgeQueryReadBuiltInOperator::FrontierSearch) {
        ForgeQueryReadScopeClass::ExplicitBroadSearch
    } else if non_anchor_predicate_count > 0 {
        ForgeQueryReadScopeClass::ExplicitBroadSearch
    } else if built_in_operators.contains(&ForgeQueryReadBuiltInOperator::SuccessorWalk)
        || built_in_operators.contains(&ForgeQueryReadBuiltInOperator::DirectEdge)
        || built_in_operators.contains(&ForgeQueryReadBuiltInOperator::SharedEndpoint)
        || built_in_operators.contains(&ForgeQueryReadBuiltInOperator::SharedAttachment)
    {
        ForgeQueryReadScopeClass::LocalNeighborhood
    } else if traversal_depth_limit > 1 {
        ForgeQueryReadScopeClass::AnchoredExpansion
    } else {
        ForgeQueryReadScopeClass::LocalNeighborhood
    }
}

fn is_identity_anchor_predicate(predicate: &crate::validation::ValidatedPredicateEntry) -> bool {
    predicate.aspect() == "identity"
        && predicate.field() == "id"
        && predicate.predicate_family() == "equality"
        && predicate.value_kind() == "String"
}

pub(super) fn runtime_basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

pub(in crate::runtime) fn execute_runtime_current_read_graph(
    runtime: &ForgeQueryRuntime,
    read_graph: &ForgeQueryReadGraph,
) -> Result<ForgeQueryReadResult, ForgeQueryReadDenial> {
    let snapshot_token = runtime.snapshot_token();
    let identity = crate::basis::ResolvedSnapshotIdentity::new(
        BasisAuthorityFamily::Runtime,
        None,
        snapshot_token.clone(),
        read_graph.schema_basis().clone(),
        SnapshotLineageClass::CurrentHead,
    );
    let basis = resolve_snapshot_basis(
        runtime_basis_intent(),
        identity,
        BasisResolutionMode::RuntimeDirect,
    )
    .map_err(|error| {
        ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::BasisResolutionDenied,
            format!("{error:?}"),
        )
    })?;
    let preflight =
        preflight_execution_basis(read_graph.execution_plan().clone(), basis).map_err(|error| {
            ForgeQueryReadDenial::new(
                ForgeQueryReadDenialKind::BasisPreflightDenied,
                format!("{error:?}"),
            )
        })?;
    let execution = execute_preflight_bundle(&preflight).map_err(|error| {
        ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::ExecutionDenied,
            format!("{error:?}"),
        )
    })?;
    let receipt = crate::runtime::ForgeQueryReadReceipt::from_execution(
        read_graph,
        snapshot_token,
        &execution,
    );
    Ok(ForgeQueryReadResult::new(
        execution.payload().to_vec(),
        receipt,
    ))
}
