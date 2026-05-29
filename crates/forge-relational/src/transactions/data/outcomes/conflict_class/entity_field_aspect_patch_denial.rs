use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AuthoritativePatchApplicationDenial,
    AuthoritativePatchConstructionDenial, CanonicalFieldPath, ContractValidationDenial, FieldKey,
    LocatorAuthority,
};
use serde::{Deserialize, Serialize};

use crate::identity::data::KindId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityFieldAspectPatchDenial {
    MissingAspectPlan {
        kind_id: KindId,
    },
    UndeclaredEntityAspectTarget {
        #[serde(with = "aspect_field_locator_serde")]
        field_locator: AspectFieldLocator,
    },
    EntityAspectFieldPathMismatch {
        #[serde(with = "aspect_field_locator_serde")]
        field_locator: AspectFieldLocator,
    },
    UnsupportedNestedEntityFieldPath {
        path: Vec<FieldKey>,
    },
    ContractValidationDenied {
        #[serde(with = "aspect_field_locator_serde")]
        field_locator: AspectFieldLocator,
        denial: ContractValidationDenial,
    },
    PatchConstructionDenied {
        #[serde(with = "optional_aspect_field_locator_serde")]
        field_locator: Option<AspectFieldLocator>,
        denial: AuthoritativePatchConstructionDenial,
    },
    FieldPatchApplicationDenied {
        #[serde(with = "aspect_field_locator_serde")]
        field_locator: AspectFieldLocator,
        denial: AuthoritativePatchApplicationDenial,
    },
    WholeAspectPatchApplicationDenied {
        aspect_key: AspectKey,
        denial: AuthoritativePatchApplicationDenial,
    },
    MissingAuthoritativeAspectState {
        aspect_key: Option<AspectKey>,
    },
    EmptyAuthoritativePatchPlan,
}

mod aspect_field_locator_serde {
    use super::{aspect_field_locator_from_parts, AspectFieldLocator, AspectKey, FieldKey};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(locator: &AspectFieldLocator, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (locator.aspect().aspect_key(), locator.field_path().fields()).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<AspectFieldLocator, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (aspect_key, fields) = <(AspectKey, Vec<FieldKey>)>::deserialize(deserializer)?;
        aspect_field_locator_from_parts(aspect_key, fields).map_err(serde::de::Error::custom)
    }
}

mod optional_aspect_field_locator_serde {
    use super::{aspect_field_locator_from_parts, AspectFieldLocator, AspectKey, FieldKey};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(
        locator: &Option<AspectFieldLocator>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        locator
            .as_ref()
            .map(|locator| (locator.aspect().aspect_key(), locator.field_path().fields()))
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<AspectFieldLocator>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let locator_parts = Option::<(AspectKey, Vec<FieldKey>)>::deserialize(deserializer)?;
        locator_parts
            .map(|(aspect_key, fields)| {
                aspect_field_locator_from_parts(aspect_key, fields)
                    .map_err(serde::de::Error::custom)
            })
            .transpose()
    }
}

fn aspect_field_locator_from_parts(
    aspect_key: AspectKey,
    fields: Vec<FieldKey>,
) -> Result<AspectFieldLocator, &'static str> {
    let field_path = CanonicalFieldPath::new(fields)
        .ok_or("entity field aspect denial path must not be empty")?;
    Ok(AspectFieldLocator::new(
        LocatorAuthority::Planned,
        aspect_key,
        field_path,
    ))
}
