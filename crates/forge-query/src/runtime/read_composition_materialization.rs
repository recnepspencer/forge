use std::collections::{BTreeMap, BTreeSet};

use crate::authoring::{AspectFieldKey, RelationName, ScalarPredicateValue};
use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativePredicateFilter};
use crate::memory_workspace::ForgeQueryEntity;
use crate::query_context::QueryContextExecutionArtifact;
use crate::runtime::{
    ForgeQueryLiveArtifactTarget, ForgeQueryReadBuiltInOperator, ForgeQueryReadDenial,
    ForgeQueryReadDenialKind, ForgeQueryReadGraph, ForgeQueryRuntime,
};
use crate::schema_view::QuerySchemaView;
use forge_foundational::facade::{
    AspectKey, AspectValue, CanonicalFieldPath, FieldKey, InternedString,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ForgeQueryReadMaterializedRowIdentity {
    value: String,
}

impl ForgeQueryReadMaterializedRowIdentity {
    fn from_label(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

pub(in crate::runtime) fn materialize_read_rows(
    runtime: &mut ForgeQueryRuntime,
    read_graph: &ForgeQueryReadGraph,
) -> Result<Vec<ForgeQueryEntity>, ForgeQueryReadDenial> {
    let view_name = format!("runtime.read.materialized:{}", read_graph.digest());
    let target = ForgeQueryLiveArtifactTarget::from_view_name(view_name.clone());
    let request = read_graph.declarative_request().clone();
    ensure_materialized_read_view(runtime, &target, &view_name, read_graph)?;
    let source_rows = runtime.backend.live_entities_for_target(&target);
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
        .map(|(index, row)| {
            ForgeQueryEntity::from_native_field_values(
                crate::memory_workspace::admit_authored_entity_label(format!(
                    "query-context:{}:{}",
                    context_execution.family().as_str(),
                    index
                )),
                query_context_row_values(context_execution, row),
            )
        })
        .collect()
}

fn query_context_row_values(
    context_execution: &QueryContextExecutionArtifact,
    row: &str,
) -> BTreeMap<CanonicalFieldPath, AspectValue> {
    BTreeMap::from([
        (
            native_field_path("query_context.basis_digest"),
            crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(
                context_execution.basis_digest().to_string(),
            ),
        ),
        (
            native_field_path("query_context.query_digest"),
            crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(
                context_execution.query_digest().to_string(),
            ),
        ),
        (
            native_field_path("query_context.result_digest"),
            crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(
                context_execution.result_digest().to_string(),
            ),
        ),
        (
            native_field_path("query_context.row"),
            crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(row.to_string()),
        ),
    ])
}

fn ensure_materialized_read_view(
    runtime: &mut ForgeQueryRuntime,
    target: &ForgeQueryLiveArtifactTarget,
    view_name: &str,
    read_graph: &ForgeQueryReadGraph,
) -> Result<(), ForgeQueryReadDenial> {
    let request = read_graph.declarative_request();
    ensure_materialized_read_target_available(&runtime.materialized_read_views, target, request)?;
    admit_materialized_read_view(runtime, view_name, request, read_graph.schema_view())?;
    declare_materialized_read_view(
        runtime,
        target,
        view_name,
        request,
        read_graph.schema_view(),
    )
}

fn ensure_materialized_read_target_available(
    materialized_read_views: &BTreeMap<ForgeQueryLiveArtifactTarget, DeclarativeLiveQueryRequest>,
    target: &ForgeQueryLiveArtifactTarget,
    request: &DeclarativeLiveQueryRequest,
) -> Result<(), ForgeQueryReadDenial> {
    if let Some(existing) = materialized_read_views.get(target) {
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
    target: &ForgeQueryLiveArtifactTarget,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
    schema_view: &QuerySchemaView,
) -> Result<(), ForgeQueryReadDenial> {
    if runtime.materialized_read_views.contains_key(target) {
        return Ok(());
    }
    runtime
        .backend
        .declare_live_view(view_name.to_string(), request.clone(), schema_view.clone())
        .map_err(execution_denial_from_workspace_error)?;
    runtime
        .materialized_read_views
        .insert(target.clone(), request.clone());
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
    let anchor_identity =
        identity_anchor(request).map(ForgeQueryReadMaterializedRowIdentity::from_label)?;
    let row_index = row_index(rows);
    let anchor_row = row_index.get(&anchor_identity)?.clone();
    let selected = if read_graph
        .built_in_operators()
        .contains(&ForgeQueryReadBuiltInOperator::SharedEndpoint)
        || read_graph
            .built_in_operators()
            .contains(&ForgeQueryReadBuiltInOperator::SharedAttachment)
    {
        collect_shared_neighborhood_rows(&anchor_row, rows, request)?
    } else if request.traversal().is_empty() {
        vec![anchor_row]
    } else {
        collect_traversal_rows(&anchor_row, &row_index, request)?
    };
    Some(order_rows(selected, request))
}

fn collect_shared_neighborhood_rows(
    anchor_row: &ForgeQueryEntity,
    rows: &[ForgeQueryEntity],
    request: &DeclarativeLiveQueryRequest,
) -> Option<Vec<ForgeQueryEntity>> {
    let anchor_identity = row_identity_label(anchor_row)?;
    let anchor_targets = request
        .traversal()
        .iter()
        .filter_map(|selector| relation_target(anchor_row, selector.relation_name()))
        .collect::<BTreeSet<_>>();
    Some(
        rows.iter()
            .filter(|row| {
                row_identity_label(row)
                    .as_ref()
                    .is_some_and(|identity| identity == &anchor_identity)
                    || request.traversal().iter().any(|selector| {
                        relation_target(row, selector.relation_name())
                            .is_some_and(|target| anchor_targets.contains(&target))
                    })
            })
            .cloned()
            .collect(),
    )
}

fn collect_traversal_rows(
    anchor_row: &ForgeQueryEntity,
    row_index: &BTreeMap<ForgeQueryReadMaterializedRowIdentity, ForgeQueryEntity>,
    request: &DeclarativeLiveQueryRequest,
) -> Option<Vec<ForgeQueryEntity>> {
    let anchor_identity = row_identity_label(anchor_row)?;
    let mut selected = BTreeMap::from([(anchor_identity.clone(), anchor_row.clone())]);
    for selector in request.traversal() {
        let mut current = anchor_identity.clone();
        for _ in 0..usize::from(selector.depth()) {
            let Some(next_identity) = row_index
                .get(&current)
                .and_then(|row| relation_target(row, selector.relation_name()))
            else {
                break;
            };
            let Some(next_row) = row_index.get(&next_identity) else {
                break;
            };
            let Some(next_identity_label) = row_identity_label(next_row) else {
                break;
            };
            selected.insert(next_identity_label, next_row.clone());
            current = next_identity;
        }
    }
    Some(selected.into_values().collect())
}

fn order_rows(
    mut rows: Vec<ForgeQueryEntity>,
    request: &DeclarativeLiveQueryRequest,
) -> Vec<ForgeQueryEntity> {
    if request
        .ordering()
        .iter()
        .any(|ordering| is_identity_field_key(ordering.source_field_key()))
    {
        rows.sort_by_key(row_identity_label);
    }
    rows
}

fn identity_anchor(request: &DeclarativeLiveQueryRequest) -> Option<&str> {
    request
        .predicate_filters()
        .iter()
        .find_map(|predicate| match predicate {
            DeclarativePredicateFilter::Equality(filter)
                if is_identity_field_key(filter.source_field_key()) =>
            {
                match filter.value() {
                    ScalarPredicateValue::String(value) => Some(value.as_str()),
                    _ => None,
                }
            }
            _ => None,
        })
}

fn relation_target(
    row: &ForgeQueryEntity,
    relation: &RelationName,
) -> Option<ForgeQueryReadMaterializedRowIdentity> {
    row.scalar_value_at(&relation_target_field_path(relation))
        .and_then(as_string_scalar)
        .map(ForgeQueryReadMaterializedRowIdentity::from_label)
}

fn relation_target_field_path(relation: &RelationName) -> CanonicalFieldPath {
    CanonicalFieldPath::new(
        std::iter::once("relations")
            .chain(relation.as_str().split('.'))
            .map(FieldKey::new)
            .collect::<Option<Vec<_>>>()
            .expect("relation target path segments must be foundational field keys"),
    )
    .expect("relation target field path must not be empty")
}

fn row_index(
    rows: &[ForgeQueryEntity],
) -> BTreeMap<ForgeQueryReadMaterializedRowIdentity, ForgeQueryEntity> {
    rows.iter()
        .cloned()
        .filter_map(|row| row_identity_label(&row).map(|identity| (identity, row)))
        .collect()
}

fn row_identity_label(row: &ForgeQueryEntity) -> Option<ForgeQueryReadMaterializedRowIdentity> {
    row.scalar_value_at(&native_field_path("identity.id"))
        .and_then(as_string_scalar)
        .map(ForgeQueryReadMaterializedRowIdentity::from_label)
}

fn is_identity_field_key(field: &AspectFieldKey) -> bool {
    field.native_aspect_key() == identity_aspect_key()
        && field.native_field_key() == identity_field_key()
}

fn identity_aspect_key() -> AspectKey {
    AspectKey::new("identity").expect("identity aspect key should be foundational")
}

fn identity_field_key() -> FieldKey {
    FieldKey::new("id").expect("identity field key should be foundational")
}

fn as_string_scalar(value: &AspectValue) -> Option<&str> {
    match value {
        AspectValue::String(InternedString::Raw(value)) => Some(value.as_str()),
        _ => None,
    }
}

fn synthetic_rows_for_request(request: &DeclarativeLiveQueryRequest) -> Vec<ForgeQueryEntity> {
    let anchor_identity = identity_anchor(request).unwrap_or("synthetic-anchor");
    vec![ForgeQueryEntity::from_native_field_values(
        crate::memory_workspace::admit_authored_entity_label(anchor_identity),
        BTreeMap::from([
            (
                native_field_path("identity.id"),
                crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(
                    anchor_identity.to_string(),
                ),
            ),
            (native_field_path("read.synthetic"), AspectValue::Bool(true)),
        ]),
    )]
}

fn native_field_path(path: impl AsRef<str>) -> CanonicalFieldPath {
    CanonicalFieldPath::new(
        path.as_ref()
            .split('.')
            .map(FieldKey::new)
            .collect::<Option<Vec<_>>>()
            .expect("synthetic row field path segments must be valid foundational field keys"),
    )
    .expect("synthetic row field path must not be empty")
}
