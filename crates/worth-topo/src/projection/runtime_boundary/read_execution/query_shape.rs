use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, OrderingSelector,
    RootEntityKey, ScalarPredicateValue,
};

use crate::projection::read_views::domain::error::TopologyDomainQueryError;

pub(crate) const TOPOLOGY_ENTITY_ROOT: &str = "TopologyEntity";
const IDENTITY_ASPECT: &str = "identity";
const IDENTITY_FIELD: &str = "id";
const TOPOLOGY_ASPECT: &str = "topology";
const TOPOLOGY_KIND_FIELD: &str = "kind";

pub(crate) fn topology_entity_root() -> Result<RootEntityKey, TopologyDomainQueryError> {
    RootEntityKey::new(TOPOLOGY_ENTITY_ROOT).map_err(|error| {
        TopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })
}

pub(crate) fn identity_selector() -> Result<AspectFieldSelector, TopologyDomainQueryError> {
    aspect_field(IDENTITY_ASPECT, IDENTITY_FIELD)
}

pub(crate) fn topology_kind_selector() -> Result<AspectFieldSelector, TopologyDomainQueryError> {
    aspect_field(TOPOLOGY_ASPECT, TOPOLOGY_KIND_FIELD)
}

pub(crate) fn identity_anchor_predicate(
    identity: &str,
) -> Result<EqualityPredicate, TopologyDomainQueryError> {
    EqualityPredicate::new(
        IDENTITY_ASPECT,
        IDENTITY_FIELD,
        ScalarPredicateValue::String(identity.to_string()),
    )
    .map_err(|error| TopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}")))
}

pub(crate) fn identity_ordering() -> Result<OrderingSelector, TopologyDomainQueryError> {
    OrderingSelector::ascending(IDENTITY_ASPECT, IDENTITY_FIELD).map_err(|error| {
        TopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })
}

pub(crate) fn identity_result_field() -> Result<AuthoredResultShapeField, TopologyDomainQueryError>
{
    result_field(IDENTITY_ASPECT, IDENTITY_FIELD, IDENTITY_FIELD)
}

pub(crate) fn topology_kind_result_field(
) -> Result<AuthoredResultShapeField, TopologyDomainQueryError> {
    result_field(TOPOLOGY_ASPECT, TOPOLOGY_KIND_FIELD, TOPOLOGY_KIND_FIELD)
}

fn aspect_field(
    aspect: impl Into<String>,
    field: impl Into<String>,
) -> Result<AspectFieldSelector, TopologyDomainQueryError> {
    AspectFieldSelector::new(aspect, field).map_err(|error| {
        TopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })
}

fn result_field(
    aspect: impl Into<String>,
    field: impl Into<String>,
    delivered_name: impl Into<String>,
) -> Result<AuthoredResultShapeField, TopologyDomainQueryError> {
    AuthoredResultShapeField::new(aspect, field, delivered_name).map_err(|error| {
        TopologyDomainQueryError::canonical_lowering_resolution(format!("{error:?}"))
    })
}
