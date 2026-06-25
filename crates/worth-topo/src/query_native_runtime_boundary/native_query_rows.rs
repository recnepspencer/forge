use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey, InternedString};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryEntity, ForgeQueryEntityIdentity, ForgeQueryRetainedFieldPath,
};
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthTopologyNativeQueryRowError {
    EmptyFieldPath,
    InvalidFieldPathSegment(String),
    NonRelationalQueryIdentity,
    WrongRelationalIdentityKind,
}

impl std::fmt::Display for WorthTopologyNativeQueryRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyFieldPath => f.write_str("native query row field path may not be empty"),
            Self::InvalidFieldPathSegment(segment) => {
                write!(f, "invalid native query row field path segment `{segment}`")
            }
            Self::NonRelationalQueryIdentity => {
                f.write_str("query row identity does not carry relational record parts")
            }
            Self::WrongRelationalIdentityKind => {
                f.write_str("query row identity carries the wrong relational record kind")
            }
        }
    }
}

impl std::error::Error for WorthTopologyNativeQueryRowError {}

pub(crate) fn native_entity_row(
    identity: ForgeQueryEntityIdentity,
    field_values: impl IntoIterator<Item = (CanonicalFieldPath, AspectValue)>,
) -> ForgeQueryEntity {
    ForgeQueryEntity::from_native_field_values(identity, field_values.into_iter().collect())
}

pub(crate) fn native_field_path(
    segments: impl IntoIterator<Item = impl Into<String>>,
) -> Result<CanonicalFieldPath, WorthTopologyNativeQueryRowError> {
    let fields = segments
        .into_iter()
        .map(|segment| {
            let segment = segment.into();
            FieldKey::new(segment.clone()).ok_or(
                WorthTopologyNativeQueryRowError::InvalidFieldPathSegment(segment),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    CanonicalFieldPath::new(fields).ok_or(WorthTopologyNativeQueryRowError::EmptyFieldPath)
}

pub(crate) fn native_field_path_for_touch(
    aspect_touch: &ForgeQueryAspectTouch,
) -> Result<CanonicalFieldPath, WorthTopologyNativeQueryRowError> {
    let segments = std::iter::once(aspect_touch.native_aspect_key().as_str().to_string()).chain(
        aspect_touch
            .native_field_path()
            .into_iter()
            .flat_map(|field_path| field_path.fields().iter())
            .map(|field| field.as_str().to_string()),
    );
    native_field_path(segments)
}

pub(crate) fn native_row_value_for_touch<'a>(
    row: &'a ForgeQueryEntity,
    aspect_touch: &ForgeQueryAspectTouch,
) -> Option<&'a AspectValue> {
    let field_path = native_field_path_for_touch(aspect_touch).ok()?;
    row.scalar_value_at(&field_path)
}

pub(crate) fn native_retained_field_path(
    segments: impl IntoIterator<Item = impl Into<String>>,
) -> Result<ForgeQueryRetainedFieldPath, WorthTopologyNativeQueryRowError> {
    native_field_path(segments).map(ForgeQueryRetainedFieldPath::from_canonical_field_path)
}

pub(crate) fn native_string(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}

pub(crate) fn native_i64(value: i64) -> AspectValue {
    AspectValue::Int64(value)
}

pub(crate) fn native_null() -> AspectValue {
    AspectValue::Null
}

pub(crate) fn row_text_at<'a>(
    row: &'a ForgeQueryEntity,
    segments: impl IntoIterator<Item = impl Into<String>>,
) -> Option<&'a str> {
    let field_path = native_field_path(segments).ok()?;
    match row.scalar_value_at(&field_path)? {
        AspectValue::String(InternedString::Raw(value)) => Some(value.as_str()),
        _ => None,
    }
}

pub(crate) fn query_entity_id_from_identity(
    identity: &ForgeQueryEntityIdentity,
) -> Result<EntityId, WorthTopologyNativeQueryRowError> {
    let parts = identity
        .relational_record_parts()
        .ok_or(WorthTopologyNativeQueryRowError::NonRelationalQueryIdentity)?;
    if parts.kind() != RelationalBridgeRecordIdentityKind::Entity {
        return Err(WorthTopologyNativeQueryRowError::WrongRelationalIdentityKind);
    }
    Ok(EntityId::new(
        PartitionId::new(parts.partition_id()),
        parts.local_slot(),
        parts.generation(),
    ))
}

pub(crate) fn query_relation_id_from_identity(
    identity: &ForgeQueryEntityIdentity,
) -> Result<RelationId, WorthTopologyNativeQueryRowError> {
    let parts = identity
        .relational_record_parts()
        .ok_or(WorthTopologyNativeQueryRowError::NonRelationalQueryIdentity)?;
    if parts.kind() != RelationalBridgeRecordIdentityKind::Relation {
        return Err(WorthTopologyNativeQueryRowError::WrongRelationalIdentityKind);
    }
    Ok(RelationId::new(
        PartitionId::new(parts.partition_id()),
        parts.local_slot(),
        parts.generation(),
    ))
}
