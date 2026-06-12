use crate::basis::{
    preflight_execution_basis, resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode,
    ExecutionBasisIntent, SnapshotLineageClass,
};
use crate::declarative_live::canonicalize_declarative_request;
use crate::execution::execute_preflight_bundle;
use crate::projection_consumption::ProjectionMaterializedFactPosture;
use crate::query_context::{
    execute_query_basis_context, AdmittedQueryBasisContext, QueryContextFamily,
};
use crate::runtime::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadDenial, ForgeQueryReadDenialKind,
    ForgeQueryReadGraph, ForgeQueryReadResult, ForgeQueryReadScopeClass, ForgeQueryRuntime,
};

use super::materialized_fact_posture::materialized_fact_posture_from_live_subscription_state;
use super::read_composition_materialization::{
    materialize_query_context_rows, materialize_read_rows,
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
    runtime: &mut ForgeQueryRuntime,
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
    let rows = materialize_read_rows(runtime, read_graph)?;
    let receipt = crate::runtime::ForgeQueryReadReceipt::from_materialized_rows(
        read_graph,
        snapshot_token,
        &execution,
        &rows,
    )
    .with_materialized_fact_posture(materialized_fact_posture_for_read_graph(
        runtime,
        read_graph,
        execution.report().basis_digest().as_str(),
    ));
    Ok(ForgeQueryReadResult::new(rows, receipt))
}

pub(in crate::runtime) fn execute_runtime_basis_context_read_graph(
    runtime: &mut ForgeQueryRuntime,
    read_graph: &ForgeQueryReadGraph,
    context: &AdmittedQueryBasisContext,
) -> Result<ForgeQueryReadResult, ForgeQueryReadDenial> {
    ensure_context_matches_read_graph(read_graph, context)?;
    let context_execution = execute_query_basis_context(context).map_err(|error| {
        ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::BasisPreflightDenied,
            format!("{error:?}"),
        )
    })?;
    let context_execution = context_execution.with_materialized_fact_posture(
        materialized_fact_posture_for_read_graph(runtime, read_graph, context.basis_digest()),
    );
    let snapshot_token = runtime.snapshot_token();
    let rows = if context_allows_runtime_materialization(snapshot_token.as_str(), context) {
        materialize_read_rows(runtime, read_graph)?
    } else {
        materialize_query_context_rows(&context_execution)
    };
    let receipt = crate::runtime::ForgeQueryReadReceipt::from_query_context_execution(
        read_graph,
        snapshot_token,
        &context_execution,
        &rows,
    );
    Ok(ForgeQueryReadResult::new(rows, receipt))
}

fn materialized_fact_posture_for_read_graph(
    runtime: &ForgeQueryRuntime,
    read_graph: &ForgeQueryReadGraph,
    basis_digest: &str,
) -> Option<ProjectionMaterializedFactPosture> {
    let lower_declaration_digest =
        canonicalize_declarative_request(read_graph.declarative_request())
            .ok()?
            .query()
            .digest()
            .as_str()
            .to_string();
    let mut exact_request_matches = runtime
        .live_subscriptions
        .values()
        .filter(|state| state.request == *read_graph.declarative_request());
    let state = if let Some(state) = exact_request_matches.next() {
        if exact_request_matches.next().is_some() {
            return None;
        }
        state
    } else {
        let mut canonical_matches = runtime
            .live_subscriptions
            .values()
            .filter(|state| state.installation.query_digest() == lower_declaration_digest);
        let state = canonical_matches.next()?;
        if canonical_matches.next().is_some() {
            return None;
        }
        state
    };
    Some(materialized_fact_posture_from_live_subscription_state(
        state,
        basis_digest,
    ))
}

fn ensure_context_matches_read_graph(
    read_graph: &ForgeQueryReadGraph,
    context: &AdmittedQueryBasisContext,
) -> Result<(), ForgeQueryReadDenial> {
    if context.query_digest() == read_graph.query_digest() {
        return Ok(());
    }
    Err(ForgeQueryReadDenial::new(
        ForgeQueryReadDenialKind::BasisPreflightDenied,
        "admitted query basis context does not match reusable read-family query digest",
    ))
}

fn context_allows_runtime_materialization(
    runtime_snapshot_token: &str,
    context: &AdmittedQueryBasisContext,
) -> bool {
    match context.family() {
        QueryContextFamily::CurrentBranchHead => true,
        QueryContextFamily::HistoricalSnapshot => {
            context.declared_basis_label() == runtime_snapshot_token
        }
        QueryContextFamily::BranchHead
        | QueryContextFamily::HistoricalCommit
        | QueryContextFamily::PreviewDerivedHistorical
        | QueryContextFamily::DiffComparison => false,
    }
}
