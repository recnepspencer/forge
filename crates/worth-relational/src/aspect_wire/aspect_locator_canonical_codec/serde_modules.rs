use worth_foundational::facade::{AspectFieldLocator, AspectValueLocator, BoundarySourceLocator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;

use super::reading::{
    decode_aspect_field_locator, decode_aspect_value_locator, decode_boundary_source_locator,
};
use super::writing::{
    encode_aspect_field_locator, encode_aspect_value_locator, encode_boundary_source_locator,
};

pub(crate) mod serde_canonical_aspect_value_locator {
    use super::*;
    use serde::de::Error;

    pub(crate) fn serialize<S>(
        locator: &AspectValueLocator,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        encode_aspect_value_locator(locator).serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<AspectValueLocator, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        decode_aspect_value_locator(&bytes).map_err(D::Error::custom)
    }
}

pub(crate) mod serde_canonical_aspect_field_locator {
    use super::*;
    use serde::de::Error;

    pub(crate) fn serialize<S>(
        locator: &AspectFieldLocator,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        encode_aspect_field_locator(locator).serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<AspectFieldLocator, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        decode_aspect_field_locator(&bytes).map_err(D::Error::custom)
    }
}

pub(crate) mod serde_optional_canonical_aspect_field_locator {
    use super::*;
    use serde::de::Error;

    pub(crate) fn serialize<S>(
        locator: &Option<AspectFieldLocator>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        locator
            .as_ref()
            .map(encode_aspect_field_locator)
            .serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<AspectFieldLocator>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Option::<Vec<u8>>::deserialize(deserializer)?;
        bytes
            .as_deref()
            .map(decode_aspect_field_locator)
            .transpose()
            .map_err(D::Error::custom)
    }
}

pub(crate) mod serde_canonical_aspect_field_locator_arc_slice {
    use super::*;
    use serde::de::Error;

    pub(crate) fn serialize<S>(
        locators: &Arc<[AspectFieldLocator]>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        locators
            .iter()
            .map(encode_aspect_field_locator)
            .collect::<Vec<_>>()
            .serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Arc<[AspectFieldLocator]>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<Vec<u8>>::deserialize(deserializer)?
            .iter()
            .map(|bytes| decode_aspect_field_locator(bytes))
            .collect::<Result<Vec<_>, _>>()
            .map(Arc::from)
            .map_err(D::Error::custom)
    }
}

pub(crate) mod serde_canonical_boundary_source_locator {
    use super::*;
    use serde::de::Error;

    pub(crate) fn serialize<S>(
        locator: &BoundarySourceLocator,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        encode_boundary_source_locator(locator)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<BoundarySourceLocator, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        decode_boundary_source_locator(&bytes).map_err(D::Error::custom)
    }
}
