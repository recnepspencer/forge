use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectLocator, BoundarySourceLocator, CanonicalFieldPath,
    FieldKey, LocatorAuthority,
};
use serde::{Deserialize, Serialize};

pub(super) fn authoritative_aspect_source_locator(aspect_key: AspectKey) -> BoundarySourceLocator {
    BoundarySourceLocator::aspect(AspectLocator::new(
        LocatorAuthority::SupportOnly,
        aspect_key,
    ))
}

pub(super) fn authoritative_aspect_field_source_locator(
    aspect_key: AspectKey,
    field: FieldKey,
) -> BoundarySourceLocator {
    BoundarySourceLocator::aspect_field(AspectFieldLocator::new(
        LocatorAuthority::SupportOnly,
        aspect_key,
        CanonicalFieldPath::single(field),
    ))
}

pub(super) fn source_locator_aspect_label(locator: &BoundarySourceLocator) -> String {
    match locator {
        BoundarySourceLocator::Aspect(aspect) => aspect.aspect_key().as_str().to_string(),
        BoundarySourceLocator::AspectField(field) => {
            field.aspect().aspect_key().as_str().to_string()
        }
        BoundarySourceLocator::BoundaryArtifact(artifact) => format!("{artifact:?}"),
    }
}

pub(super) fn source_locator_field_label(locator: &BoundarySourceLocator) -> Option<String> {
    match locator {
        BoundarySourceLocator::AspectField(field) => Some(
            crate::transactions::data::canonical_field_path_label(field.field_path()),
        ),
        BoundarySourceLocator::Aspect(_) | BoundarySourceLocator::BoundaryArtifact(_) => None,
    }
}

pub(super) mod serde_boundary_source_locator {
    use super::{deserialize_source_locator, serialize_source_locator, BoundarySourceLocator};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(locator: &BoundarySourceLocator, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_source_locator(locator, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BoundarySourceLocator, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_source_locator(deserializer)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BoundarySourceLocatorParts {
    kind: BoundarySourceLocatorKind,
    aspect_key: AspectKey,
    fields: Option<Vec<FieldKey>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum BoundarySourceLocatorKind {
    Aspect,
    AspectField,
}

fn serialize_source_locator<S>(
    locator: &BoundarySourceLocator,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let parts = match locator {
        BoundarySourceLocator::Aspect(aspect) => BoundarySourceLocatorParts {
            kind: BoundarySourceLocatorKind::Aspect,
            aspect_key: aspect.aspect_key().clone(),
            fields: None,
        },
        BoundarySourceLocator::AspectField(field) => BoundarySourceLocatorParts {
            kind: BoundarySourceLocatorKind::AspectField,
            aspect_key: field.aspect().aspect_key().clone(),
            fields: Some(field.field_path().fields().to_vec()),
        },
        BoundarySourceLocator::BoundaryArtifact(_) => {
            return Err(serde::ser::Error::custom(
                "authoritative aspect state denials do not serialize boundary artifact locators",
            ));
        }
    };
    parts.serialize(serializer)
}

fn deserialize_source_locator<'de, D>(deserializer: D) -> Result<BoundarySourceLocator, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let parts = BoundarySourceLocatorParts::deserialize(deserializer)?;
    match parts.kind {
        BoundarySourceLocatorKind::Aspect => {
            Ok(authoritative_aspect_source_locator(parts.aspect_key))
        }
        BoundarySourceLocatorKind::AspectField => {
            let fields = parts.fields.ok_or_else(|| {
                serde::de::Error::custom("authoritative aspect field locator requires fields")
            })?;
            let field_path = CanonicalFieldPath::new(fields).ok_or_else(|| {
                serde::de::Error::custom(
                    "authoritative aspect field locator path must not be empty",
                )
            })?;
            Ok(BoundarySourceLocator::aspect_field(
                AspectFieldLocator::new(
                    LocatorAuthority::SupportOnly,
                    parts.aspect_key,
                    field_path,
                ),
            ))
        }
    }
}
