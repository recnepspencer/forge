use worth_foundational::facade::AspectValue;
use worth_relational::facade::runtime::ProjectionAspectScope;
use worth_relational::facade::storage::RecordLifecycleState;

use super::super::{
    read_execution::{
        read_execution_denial, WorthQueryApplicationReadExecutionDenial,
        WorthQueryApplicationReadExecutionDenialKind,
    },
    WorthQueryAdmittedApplicationQueryPlan,
};

pub(super) fn read_scope_identity<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    graph: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout,
    plan: &WorthQueryAdmittedApplicationQueryPlan<
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
) -> Result<
    (
        AspectValue,
        crate::domain_computation::provider_session::WorthQueryObservedGraphReadWork,
    ),
    WorthQueryApplicationReadExecutionDenial,
> {
    let live = plan
        .query
        .live()
        .ok_or_else(|| projection_denial(plan.query.name()))?;
    let identity = live.scope_identity();
    let expected_kind = graph
        .entity_kind(identity.entity())
        .ok_or_else(|| projection_denial(identity.result_path()))?;
    let view = runtime
        .read_truth()
        .project_snapshot(plan.basis.snapshot_handle())
        .ok_or_else(|| projection_denial(identity.result_path()))?;
    let scope = ProjectionAspectScope::fields(
        identity.aspect_key().clone(),
        [identity.field_key().clone()],
    );
    let value = view
        .entity_record_with_projection_scope(plan.scope.entity_id(), scope, |record| {
            (record.kind_id() == expected_kind && record.lifecycle() == RecordLifecycleState::Live)
                .then(|| {
                    record
                        .aspect_field_value(identity.aspect_key(), identity.field_key())
                        .cloned()
                })
                .flatten()
        })
        .ok_or_else(|| projection_denial(identity.result_path()))?;
    Ok((
        value,
        crate::domain_computation::provider_session::WorthQueryObservedGraphReadWork {
            examined_candidates: 1,
            projected_records: 1,
            projected_fields: 1,
            relation_records_examined: 0,
            ordering_comparisons: 0,
        },
    ))
}

fn projection_denial(subject: impl Into<String>) -> WorthQueryApplicationReadExecutionDenial {
    read_execution_denial(
        WorthQueryApplicationReadExecutionDenialKind::ProjectionUnavailable,
        subject,
    )
}
