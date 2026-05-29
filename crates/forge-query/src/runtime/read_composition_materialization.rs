use std::collections::{BTreeMap, BTreeSet};

use crate::authoring::ScalarPredicateValue;
use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativePredicateFilter};
use crate::memory_workspace::ForgeQueryEntity;
use crate::query_context::QueryContextExecutionArtifact;
use crate::runtime::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadDenial, ForgeQueryReadDenialKind,
    ForgeQueryReadGraph, ForgeQueryRuntime,
};
use crate::schema_view::QuerySchemaView;

pub(in crate::runtime) fn materialize_read_rows(
    runtime: &mut ForgeQueryRuntime,
    read_graph: &ForgeQueryReadGraph,
) -> Result<Vec<ForgeQueryEntity>, ForgeQueryReadDenial> {
    let view_name = format!("runtime.read.materialized:{}", read_graph.digest());
    let request = read_graph.declarative_request().clone();
    ensure_materialized_read_view(runtime, &view_name, read_graph)?;
    let source_rows = runtime.backend.live_entities(&view_name);
    Ok(
        materialize_rows_from_request(read_graph, &request, &source_rows)
            .unwrap_or_else(|| synthetic_rows_for_request(&request)),
    )
}

pub(in crate::runtime) fn materialize_query_context_rows(
    context_execution: &QueryContextExecutionArtifact,
) -> Vec<ForgeQueryEntity> {
    context_execution
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| ForgeQueryEntity {
            identity: format!(
                "query-context:{}:{}",
                context_execution.family().as_str(),
                index
            ),
            payload: serde_json::json!({
                "query_context": {
                    "basis_digest": context_execution.basis_digest(),
                    "query_digest": context_execution.query_digest(),
                    "row": row,
                    "result_digest": context_execution.result_digest(),
                },
                "relations": {},
            }),
        })
        .collect()
}

fn ensure_materialized_read_view(
    runtime: &mut ForgeQueryRuntime,
    view_name: &str,
    read_graph: &ForgeQueryReadGraph,
) -> Result<(), ForgeQueryReadDenial> {
    let request = read_graph.declarative_request();
    ensure_materialized_read_view_name_available(
        &runtime.materialized_read_views,
        view_name,
        request,
    )?;
    admit_materialized_read_view(runtime, view_name, request, read_graph.schema_view())?;
    declare_materialized_read_view(runtime, view_name, request, read_graph.schema_view())
}

fn ensure_materialized_read_view_name_available(
    materialized_read_views: &BTreeMap<String, DeclarativeLiveQueryRequest>,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
) -> Result<(), ForgeQueryReadDenial> {
    if let Some(existing) = materialized_read_views.get(view_name) {
        if existing != request {
            return Err(ForgeQueryReadDenial::new(
                ForgeQueryReadDenialKind::ExecutionDenied,
                "materialized read view name collision with mismatched declarative request",
            ));
        }
    }
    Ok(())
}

fn admit_materialized_read_view(
    runtime: &ForgeQueryRuntime,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
    schema_view: &QuerySchemaView,
) -> Result<(), ForgeQueryReadDenial> {
    let admission_receipt = runtime
        .backend
        .admit_live_view_declaration(view_name, request, schema_view)
        .map_err(execution_denial_from_workspace_error)?;
    if let Some(message) = admission_receipt.drift_from_request(view_name, request) {
        return Err(ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::ExecutionDenied,
            message,
        ));
    }
    Ok(())
}

fn declare_materialized_read_view(
    runtime: &mut ForgeQueryRuntime,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
    schema_view: &QuerySchemaView,
) -> Result<(), ForgeQueryReadDenial> {
    if runtime.materialized_read_views.contains_key(view_name) {
        return Ok(());
    }
    runtime
        .backend
        .declare_live_view(view_name.to_string(), request.clone(), schema_view.clone())
        .map_err(execution_denial_from_workspace_error)?;
    runtime
        .materialized_read_views
        .insert(view_name.to_string(), request.clone());
    Ok(())
}

fn execution_denial_from_workspace_error(
    error: crate::memory_workspace::ForgeQueryWorkspaceError,
) -> ForgeQueryReadDenial {
    ForgeQueryReadDenial::new(ForgeQueryReadDenialKind::ExecutionDenied, error.to_string())
}

fn materialize_rows_from_request(
    read_graph: &ForgeQueryReadGraph,
    request: &DeclarativeLiveQueryRequest,
    rows: &[ForgeQueryEntity],
) -> Option<Vec<ForgeQueryEntity>> {
    let anchor_identity = identity_anchor(request)?;
    let row_index = row_index(rows);
    let anchor_row = row_index.get(anchor_identity)?.clone();
    let selected = if read_graph
        .built_in_operators()
        .contains(&ForgeQueryReadBuiltInOperator::SharedEndpoint)
        || read_graph
            .built_in_operators()
            .contains(&ForgeQueryReadBuiltInOperator::SharedAttachment)
    {
        collect_shared_neighborhood_rows(&anchor_row, rows, request)
    } else if request.traversal().is_empty() {
        vec![anchor_row]
    } else {
        collect_traversal_rows(&anchor_row, &row_index, request)
    };
    Some(order_rows(selected, request))
}

fn collect_shared_neighborhood_rows(
    anchor_row: &ForgeQueryEntity,
    rows: &[ForgeQueryEntity],
    request: &DeclarativeLiveQueryRequest,
) -> Vec<ForgeQueryEntity> {
    let anchor_targets = request
        .traversal()
        .iter()
        .filter_map(|selector| relation_target(anchor_row, selector.relation()))
        .collect::<BTreeSet<_>>();
    rows.iter()
        .filter(|row| {
            row.identity == anchor_row.identity
                || request.traversal().iter().any(|selector| {
                    relation_target(row, selector.relation())
                        .is_some_and(|target| anchor_targets.contains(target))
                })
        })
        .cloned()
        .collect()
}

fn collect_traversal_rows(
    anchor_row: &ForgeQueryEntity,
    row_index: &BTreeMap<String, ForgeQueryEntity>,
    request: &DeclarativeLiveQueryRequest,
) -> Vec<ForgeQueryEntity> {
    let mut selected = BTreeMap::from([(anchor_row.identity.clone(), anchor_row.clone())]);
    for selector in request.traversal() {
        let mut current = anchor_row.identity.clone();
        for _ in 0..usize::from(selector.depth()) {
            let Some(next_identity) = row_index
                .get(&current)
                .and_then(|row| relation_target(row, selector.relation()))
            else {
                break;
            };
            let Some(next_row) = row_index.get(next_identity) else {
                break;
            };
            selected.insert(next_row.identity.clone(), next_row.clone());
            current = next_identity.to_string();
        }
    }
    selected.into_values().collect()
}

fn order_rows(
    mut rows: Vec<ForgeQueryEntity>,
    request: &DeclarativeLiveQueryRequest,
) -> Vec<ForgeQueryEntity> {
    if request
        .ordering()
        .iter()
        .any(|ordering| ordering.aspect() == "identity" && ordering.field() == "id")
    {
        rows.sort_by(|left, right| left.identity.cmp(&right.identity));
    }
    rows
}

fn identity_anchor(request: &DeclarativeLiveQueryRequest) -> Option<&str> {
    request
        .predicate_filters()
        .iter()
        .find_map(|predicate| match predicate {
            DeclarativePredicateFilter::Equality(filter)
                if filter.aspect() == "identity" && filter.field() == "id" =>
            {
                match filter.value() {
                    ScalarPredicateValue::String(value) => Some(value.as_str()),
                    _ => None,
                }
            }
            _ => None,
        })
}

fn relation_target<'a>(row: &'a ForgeQueryEntity, relation: &str) -> Option<&'a str> {
    row.payload.get("relations")?.get(relation)?.as_str()
}

fn row_index(rows: &[ForgeQueryEntity]) -> BTreeMap<String, ForgeQueryEntity> {
    rows.iter()
        .cloned()
        .map(|row| (row.identity.clone(), row))
        .collect()
}

fn synthetic_rows_for_request(request: &DeclarativeLiveQueryRequest) -> Vec<ForgeQueryEntity> {
    let anchor_identity = identity_anchor(request).unwrap_or("synthetic-anchor");
    vec![ForgeQueryEntity {
        identity: anchor_identity.to_string(),
        payload: serde_json::json!({
            "read": { "synthetic": true },
            "relations": {}
        }),
    }]
}
