use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectLocator, AspectValueLocator, BoundarySourceLocator,
    CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{encode_string, encode_u32};

const WHOLE_ASPECT: u8 = 1;
const STRUCT_FIELD: u8 = 2;

const AUTHORITY_AUTHORITATIVE: u8 = 1;
const AUTHORITY_DERIVED: u8 = 2;
const AUTHORITY_PROJECTED: u8 = 3;
const AUTHORITY_SUPPORT_ONLY: u8 = 4;
const AUTHORITY_PLANNED: u8 = 5;
const AUTHORITY_RECEIPT_BEARING: u8 = 6;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AspectValueLocatorCanonicalCodecError {
    detail: String,
}

impl AspectValueLocatorCanonicalCodecError {
    fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

impl std::fmt::Display for AspectValueLocatorCanonicalCodecError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.detail.fmt(formatter)
    }
}

impl std::error::Error for AspectValueLocatorCanonicalCodecError {}

pub(crate) fn encode_aspect_value_locator(locator: &AspectValueLocator) -> Vec<u8> {
    let mut bytes = Vec::new();
    match locator {
        AspectValueLocator::WholeAspect(aspect) => {
            bytes.push(WHOLE_ASPECT);
            encode_authority(&mut bytes, aspect.authority());
            encode_string(&mut bytes, aspect.aspect_key().as_str());
        }
        AspectValueLocator::StructField(field) => {
            bytes.push(STRUCT_FIELD);
            encode_authority(&mut bytes, field.aspect().authority());
            encode_string(&mut bytes, field.aspect().aspect_key().as_str());
            encode_u32(&mut bytes, field.field_path().fields().len() as u32);
            for field_key in field.field_path().fields() {
                encode_string(&mut bytes, field_key.as_str());
            }
        }
    }
    bytes
}

pub(crate) fn decode_aspect_value_locator(
    bytes: &[u8],
) -> Result<AspectValueLocator, AspectValueLocatorCanonicalCodecError> {
    let mut reader = AspectValueLocatorReader::new(bytes);
    let tag = reader.read_u8()?;
    let authority = reader.read_authority()?;
    let aspect_key = reader.read_aspect_key()?;
    let locator = match tag {
        WHOLE_ASPECT => AspectValueLocator::whole_aspect(AspectLocator::new(authority, aspect_key)),
        STRUCT_FIELD => {
            let field_path = reader.read_field_path()?;
            AspectValueLocator::struct_field(AspectFieldLocator::new(
                authority, aspect_key, field_path,
            ))
        }
        other => {
            return Err(AspectValueLocatorCanonicalCodecError::new(format!(
                "unknown aspect value locator tag {other}"
            )))
        }
    };
    reader.finish()?;
    Ok(locator)
}

pub(crate) fn encode_aspect_field_locator(locator: &AspectFieldLocator) -> Vec<u8> {
    encode_aspect_value_locator(&AspectValueLocator::struct_field(locator.clone()))
}

pub(crate) fn decode_aspect_field_locator(
    bytes: &[u8],
) -> Result<AspectFieldLocator, AspectValueLocatorCanonicalCodecError> {
    match decode_aspect_value_locator(bytes)? {
        AspectValueLocator::StructField(locator) => Ok(locator),
        AspectValueLocator::WholeAspect(_) => Err(AspectValueLocatorCanonicalCodecError::new(
            "expected aspect field locator bytes, found whole-aspect locator",
        )),
    }
}

pub(crate) fn encode_boundary_source_locator(
    locator: &BoundarySourceLocator,
) -> Result<Vec<u8>, AspectValueLocatorCanonicalCodecError> {
    match locator {
        BoundarySourceLocator::Aspect(aspect) => Ok(encode_aspect_value_locator(
            &AspectValueLocator::whole_aspect(aspect.clone()),
        )),
        BoundarySourceLocator::AspectField(field) => Ok(encode_aspect_value_locator(
            &AspectValueLocator::struct_field(field.clone()),
        )),
        BoundarySourceLocator::BoundaryArtifact(_) => {
            Err(AspectValueLocatorCanonicalCodecError::new(
                "canonical aspect source locator bytes do not encode boundary artifact locators",
            ))
        }
    }
}

pub(crate) fn decode_boundary_source_locator(
    bytes: &[u8],
) -> Result<BoundarySourceLocator, AspectValueLocatorCanonicalCodecError> {
    match decode_aspect_value_locator(bytes)? {
        AspectValueLocator::WholeAspect(aspect) => Ok(BoundarySourceLocator::aspect(aspect)),
        AspectValueLocator::StructField(field) => Ok(BoundarySourceLocator::aspect_field(field)),
    }
}

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

fn encode_authority(bytes: &mut Vec<u8>, authority: LocatorAuthority) {
    bytes.push(match authority {
        LocatorAuthority::Authoritative => AUTHORITY_AUTHORITATIVE,
        LocatorAuthority::Derived => AUTHORITY_DERIVED,
        LocatorAuthority::Projected => AUTHORITY_PROJECTED,
        LocatorAuthority::SupportOnly => AUTHORITY_SUPPORT_ONLY,
        LocatorAuthority::Planned => AUTHORITY_PLANNED,
        LocatorAuthority::ReceiptBearing => AUTHORITY_RECEIPT_BEARING,
    });
}

struct AspectValueLocatorReader<'bytes> {
    bytes: &'bytes [u8],
    cursor: usize,
}

impl<'bytes> AspectValueLocatorReader<'bytes> {
    fn new(bytes: &'bytes [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn finish(self) -> Result<(), AspectValueLocatorCanonicalCodecError> {
        if self.cursor == self.bytes.len() {
            Ok(())
        } else {
            Err(AspectValueLocatorCanonicalCodecError::new(format!(
                "aspect value locator codec left {} trailing bytes",
                self.bytes.len() - self.cursor
            )))
        }
    }

    fn read_authority(
        &mut self,
    ) -> Result<LocatorAuthority, AspectValueLocatorCanonicalCodecError> {
        match self.read_u8()? {
            AUTHORITY_AUTHORITATIVE => Ok(LocatorAuthority::Authoritative),
            AUTHORITY_DERIVED => Ok(LocatorAuthority::Derived),
            AUTHORITY_PROJECTED => Ok(LocatorAuthority::Projected),
            AUTHORITY_SUPPORT_ONLY => Ok(LocatorAuthority::SupportOnly),
            AUTHORITY_PLANNED => Ok(LocatorAuthority::Planned),
            AUTHORITY_RECEIPT_BEARING => Ok(LocatorAuthority::ReceiptBearing),
            other => Err(AspectValueLocatorCanonicalCodecError::new(format!(
                "unknown aspect value locator authority tag {other}"
            ))),
        }
    }

    fn read_aspect_key(&mut self) -> Result<AspectKey, AspectValueLocatorCanonicalCodecError> {
        let raw_aspect_key = self.read_string()?;
        AspectKey::new(&raw_aspect_key).ok_or_else(|| {
            AspectValueLocatorCanonicalCodecError::new(format!(
                "invalid aspect value locator aspect key `{raw_aspect_key}`"
            ))
        })
    }

    fn read_field_path(
        &mut self,
    ) -> Result<CanonicalFieldPath, AspectValueLocatorCanonicalCodecError> {
        let field_count = self.read_u32()? as usize;
        let mut fields = Vec::with_capacity(field_count);
        for _ in 0..field_count {
            let raw_field_key = self.read_string()?;
            fields.push(FieldKey::new(&raw_field_key).ok_or_else(|| {
                AspectValueLocatorCanonicalCodecError::new(format!(
                    "invalid aspect value locator field key `{raw_field_key}`"
                ))
            })?);
        }
        CanonicalFieldPath::new(fields).ok_or_else(|| {
            AspectValueLocatorCanonicalCodecError::new("empty aspect value locator field path")
        })
    }

    fn read_string(&mut self) -> Result<String, AspectValueLocatorCanonicalCodecError> {
        let length = self.read_u32()? as usize;
        let bytes = self.read_exact(length)?;
        std::str::from_utf8(bytes)
            .map(str::to_string)
            .map_err(|error| {
                AspectValueLocatorCanonicalCodecError::new(format!(
                    "invalid UTF-8 in aspect value locator string: {error}"
                ))
            })
    }

    fn read_u8(&mut self) -> Result<u8, AspectValueLocatorCanonicalCodecError> {
        Ok(self.read_exact(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, AspectValueLocatorCanonicalCodecError> {
        let mut buffer = [0_u8; 4];
        buffer.copy_from_slice(self.read_exact(4)?);
        Ok(u32::from_le_bytes(buffer))
    }

    fn read_exact(
        &mut self,
        length: usize,
    ) -> Result<&'bytes [u8], AspectValueLocatorCanonicalCodecError> {
        if self.cursor + length > self.bytes.len() {
            return Err(AspectValueLocatorCanonicalCodecError::new(format!(
                "aspect value locator codec expected {length} bytes at offset {} but input ended early",
                self.cursor
            )));
        }
        let start = self.cursor;
        self.cursor += length;
        Ok(&self.bytes[start..self.cursor])
    }
}
