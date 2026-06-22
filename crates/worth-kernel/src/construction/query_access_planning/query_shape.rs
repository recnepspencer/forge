use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, AuthoringError, EqualityPredicate,
    OrderingSelector, QuerySchemaView, RelationName, ScalarPredicateValue, SchemaFieldKind,
    SchemaFieldView, SchemaRelationView,
};

use super::access_denial::PrimitiveConstructionQueryAccessError;

const ROOT: &str = "TopologyEntity";
const IDENTITY_ASPECT: &str = "identity";
const DIGEST_FIELD: &str = "id";
const TOPOLOGY_ASPECT: &str = "topology";
const CLASS_FIELD: &str = "kind";

pub(crate) fn root_collection() -> &'static str {
    ROOT
}

pub(crate) fn construction_access_schema(
    max_depth: u8,
) -> Result<QuerySchemaView, PrimitiveConstructionQueryAccessError> {
    if max_depth == 0 {
        return Err(PrimitiveConstructionQueryAccessError::Lowering(
            "construction graph read traversal depth must be non-zero".to_string(),
        ));
    }
    let dependency_relation = topology::facade::topology_half_edge_next_relation_name()
        .map_err(|error| map_authoring_error("topology-relation", error))?;
    Ok(QuerySchemaView::new(
        format!("worth-kernel.primitive-construction.graph-read:{max_depth}"),
        [
            SchemaFieldView::new(IDENTITY_ASPECT, DIGEST_FIELD, SchemaFieldKind::String),
            SchemaFieldView::new(TOPOLOGY_ASPECT, CLASS_FIELD, SchemaFieldKind::String)
                .presence_predicate_queryable(),
        ],
        [SchemaRelationView::new(
            dependency_relation.as_str(),
            max_depth,
        )],
    ))
}

pub(crate) fn dependency_relation() -> Result<RelationName, PrimitiveConstructionQueryAccessError> {
    topology::facade::topology_half_edge_next_relation_name()
        .map_err(|error| map_authoring_error("topology-relation", error))
}

pub(crate) fn digest_selector() -> Result<AspectFieldSelector, PrimitiveConstructionQueryAccessError>
{
    AspectFieldSelector::new(IDENTITY_ASPECT, DIGEST_FIELD)
        .map_err(|error| map_authoring_error("digest-selector", error))
}

pub(crate) fn topology_class_selector(
) -> Result<AspectFieldSelector, PrimitiveConstructionQueryAccessError> {
    AspectFieldSelector::new(TOPOLOGY_ASPECT, CLASS_FIELD)
        .map_err(|error| map_authoring_error("topology-class-selector", error))
}

pub(crate) fn birth_digest_predicate(
    birth_digest: &str,
) -> Result<EqualityPredicate, PrimitiveConstructionQueryAccessError> {
    EqualityPredicate::new(
        IDENTITY_ASPECT,
        DIGEST_FIELD,
        ScalarPredicateValue::String(birth_digest.to_string()),
    )
    .map_err(|error| map_authoring_error("birth-digest-predicate", error))
}

pub(crate) fn digest_ordering() -> Result<OrderingSelector, PrimitiveConstructionQueryAccessError> {
    OrderingSelector::ascending(IDENTITY_ASPECT, DIGEST_FIELD)
        .map_err(|error| map_authoring_error("digest-ordering", error))
}

pub(crate) fn digest_result_field(
) -> Result<AuthoredResultShapeField, PrimitiveConstructionQueryAccessError> {
    AuthoredResultShapeField::new(IDENTITY_ASPECT, DIGEST_FIELD, DIGEST_FIELD)
        .map_err(|error| map_authoring_error("digest-result-field", error))
}

pub(crate) fn topology_class_result_field(
) -> Result<AuthoredResultShapeField, PrimitiveConstructionQueryAccessError> {
    AuthoredResultShapeField::new(TOPOLOGY_ASPECT, CLASS_FIELD, CLASS_FIELD)
        .map_err(|error| map_authoring_error("topology-class-result-field", error))
}

pub(crate) fn map_authoring_error(
    context: &'static str,
    error: AuthoringError,
) -> PrimitiveConstructionQueryAccessError {
    PrimitiveConstructionQueryAccessError::Lowering(format!(
        "{context}:{}",
        authoring_error_code(&error)
    ))
}

fn authoring_error_code(error: &AuthoringError) -> &'static str {
    match error {
        AuthoringError::EmptyRootEntityKey => "empty-root-entity-key",
        AuthoringError::EmptyProjectionSelector => "empty-projection-selector",
        AuthoringError::EmptyOrderingSelector => "empty-ordering-selector",
        AuthoringError::EmptyProjectionSet => "empty-projection-set",
        AuthoringError::EmptyTraversalRelation => "empty-traversal-relation",
        AuthoringError::UnsupportedTraversalDepth { .. } => "unsupported-traversal-depth",
        AuthoringError::EmptyResultFieldSource => "empty-result-field-source",
        AuthoringError::EmptyDeliveredFieldName => "empty-delivered-field-name",
        AuthoringError::EmptyResultShapeFieldSet => "empty-result-shape-field-set",
    }
}
