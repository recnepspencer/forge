use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectLocator, AspectValue, AspectValueLocator,
    CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use serde::{Deserialize, Serialize};

use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeValueMaterialization {
    EqualityWitnessDigest,
    SnapshotPinnedRead,
    InternedCanonicalValueHandle,
    EagerInlineAspectValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeValueSourceSide {
    Source,
    Target,
    Base,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedAspectValue {
    pub policy: MergeValueMaterialization,
    pub evidence: MaterializedAspectValueEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterializedAspectValueEvidence {
    EqualityWitnessDigest(String),
    PinnedVisibleAspect {
        side: MergeValueSourceSide,
        record: RecordRef,
        #[serde(with = "serde_aspect_value_locator")]
        locator: AspectValueLocator,
    },
    InlineAspectValue(
        #[serde(with = "crate::aspect_wire::serde_canonical_aspect_value")] AspectValue,
    ),
}

pub(crate) fn aspect_reference(
    side: MergeValueSourceSide,
    record: RecordRef,
    aspect_key: AspectKey,
) -> MaterializedAspectValue {
    MaterializedAspectValue {
        policy: MergeValueMaterialization::SnapshotPinnedRead,
        evidence: MaterializedAspectValueEvidence::PinnedVisibleAspect {
            side,
            record,
            locator: authoritative_whole_aspect_value_locator(aspect_key),
        },
    }
}

pub(crate) fn materialized_value_aspect_key(locator: &AspectValueLocator) -> &AspectKey {
    match locator {
        AspectValueLocator::WholeAspect(aspect) => aspect.aspect_key(),
        AspectValueLocator::StructField(field) => field.aspect().aspect_key(),
    }
}

fn authoritative_whole_aspect_value_locator(aspect_key: AspectKey) -> AspectValueLocator {
    AspectValueLocator::whole_aspect(AspectLocator::new(
        LocatorAuthority::Authoritative,
        aspect_key,
    ))
}

mod serde_aspect_value_locator {
    use super::*;
    use serde::de::Error;

    pub fn serialize<S>(locator: &AspectValueLocator, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializableAspectValueLocator::from(locator).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<AspectValueLocator, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        SerializableAspectValueLocator::deserialize(deserializer)?
            .try_into()
            .map_err(D::Error::custom)
    }

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum SerializableAspectValueLocator {
        WholeAspect {
            authority: SerializableLocatorAuthority,
            aspect_key: AspectKey,
        },
        StructField {
            authority: SerializableLocatorAuthority,
            aspect_key: AspectKey,
            field_path: Vec<FieldKey>,
        },
    }

    impl From<&AspectValueLocator> for SerializableAspectValueLocator {
        fn from(locator: &AspectValueLocator) -> Self {
            match locator {
                AspectValueLocator::WholeAspect(aspect) => Self::WholeAspect {
                    authority: SerializableLocatorAuthority::from(aspect.authority()),
                    aspect_key: aspect.aspect_key().clone(),
                },
                AspectValueLocator::StructField(field) => Self::StructField {
                    authority: SerializableLocatorAuthority::from(field.aspect().authority()),
                    aspect_key: field.aspect().aspect_key().clone(),
                    field_path: field.field_path().fields().to_vec(),
                },
            }
        }
    }

    impl TryFrom<SerializableAspectValueLocator> for AspectValueLocator {
        type Error = &'static str;

        fn try_from(value: SerializableAspectValueLocator) -> Result<Self, Self::Error> {
            match value {
                SerializableAspectValueLocator::WholeAspect {
                    authority,
                    aspect_key,
                } => Ok(AspectValueLocator::whole_aspect(AspectLocator::new(
                    authority.into(),
                    aspect_key,
                ))),
                SerializableAspectValueLocator::StructField {
                    authority,
                    aspect_key,
                    field_path,
                } => {
                    let field_path =
                        CanonicalFieldPath::new(field_path).ok_or("empty aspect field path")?;
                    Ok(AspectValueLocator::struct_field(AspectFieldLocator::new(
                        authority.into(),
                        aspect_key,
                        field_path,
                    )))
                }
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum SerializableLocatorAuthority {
        Authoritative,
        Derived,
        Projected,
        SupportOnly,
        Planned,
        ReceiptBearing,
    }

    impl From<LocatorAuthority> for SerializableLocatorAuthority {
        fn from(value: LocatorAuthority) -> Self {
            match value {
                LocatorAuthority::Authoritative => Self::Authoritative,
                LocatorAuthority::Derived => Self::Derived,
                LocatorAuthority::Projected => Self::Projected,
                LocatorAuthority::SupportOnly => Self::SupportOnly,
                LocatorAuthority::Planned => Self::Planned,
                LocatorAuthority::ReceiptBearing => Self::ReceiptBearing,
            }
        }
    }

    impl From<SerializableLocatorAuthority> for LocatorAuthority {
        fn from(value: SerializableLocatorAuthority) -> Self {
            match value {
                SerializableLocatorAuthority::Authoritative => Self::Authoritative,
                SerializableLocatorAuthority::Derived => Self::Derived,
                SerializableLocatorAuthority::Projected => Self::Projected,
                SerializableLocatorAuthority::SupportOnly => Self::SupportOnly,
                SerializableLocatorAuthority::Planned => Self::Planned,
                SerializableLocatorAuthority::ReceiptBearing => Self::ReceiptBearing,
            }
        }
    }
}
