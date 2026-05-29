use std::collections::BTreeMap;

use forge_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};

use super::{
    decode_aspect_value, encode_length_prefixed_aspect_value, encode_string, encode_u32,
    AspectValueCanonicalCodecError,
};
use crate::transactions::data::{AspectFieldPatch, AspectFieldPatchTarget};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectFieldPatchCodecError {
    detail: String,
}

impl AspectFieldPatchCodecError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for AspectFieldPatchCodecError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for AspectFieldPatchCodecError {}

impl From<AspectValueCanonicalCodecError> for AspectFieldPatchCodecError {
    fn from(error: AspectValueCanonicalCodecError) -> Self {
        Self::new(error.detail())
    }
}

pub(crate) fn encode_aspect_field_patch_canonical_bytes(
    patch: &AspectFieldPatch,
) -> Result<Vec<u8>, AspectFieldPatchCodecError> {
    let mut bytes = Vec::new();
    encode_u32(&mut bytes, patch.len() as u32);
    for (target, value) in patch.iter() {
        encode_aspect_field_patch_target(&mut bytes, target);
        encode_length_prefixed_aspect_value(&mut bytes, value)?;
    }
    Ok(bytes)
}

pub(crate) fn decode_aspect_field_patch_canonical_bytes(
    bytes: &[u8],
) -> Result<AspectFieldPatch, AspectFieldPatchCodecError> {
    let mut reader = AspectFieldPatchReader::new(bytes);
    let field_count = reader.read_u32()? as usize;
    let mut targets = BTreeMap::new();
    for _ in 0..field_count {
        let target = reader.read_target()?;
        let value = reader.read_length_prefixed_aspect_value()?;
        targets.insert(target, value);
    }
    reader.finish()?;
    Ok(AspectFieldPatch::new(targets))
}

pub(crate) fn encode_aspect_field_patch_target_canonical_bytes(
    target: &AspectFieldPatchTarget,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_aspect_field_patch_target(&mut bytes, target);
    bytes
}

pub(crate) fn decode_aspect_field_patch_target_canonical_bytes(
    bytes: &[u8],
) -> Result<AspectFieldPatchTarget, AspectFieldPatchCodecError> {
    let mut reader = AspectFieldPatchReader::new(bytes);
    let target = reader.read_target()?;
    reader.finish()?;
    Ok(target)
}

fn encode_aspect_field_patch_target(bytes: &mut Vec<u8>, target: &AspectFieldPatchTarget) {
    encode_string(bytes, target.aspect_key().as_str());
    encode_field_path(bytes, target.field_path());
}

fn encode_field_path(bytes: &mut Vec<u8>, path: &CanonicalFieldPath) {
    encode_u32(bytes, path.fields().len() as u32);
    for field in path.fields() {
        encode_string(bytes, field.as_str());
    }
}

struct AspectFieldPatchReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> AspectFieldPatchReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn finish(self) -> Result<(), AspectFieldPatchCodecError> {
        if self.cursor == self.bytes.len() {
            Ok(())
        } else {
            Err(AspectFieldPatchCodecError::new(format!(
                "aspect field patch codec left {} trailing bytes",
                self.bytes.len() - self.cursor
            )))
        }
    }

    fn read_field_path(&mut self) -> Result<CanonicalFieldPath, AspectFieldPatchCodecError> {
        let field_count = self.read_u32()? as usize;
        let mut fields = Vec::with_capacity(field_count);
        for _ in 0..field_count {
            let encoded_field_key = self.read_string()?;
            let field = FieldKey::new(encoded_field_key).ok_or_else(|| {
                AspectFieldPatchCodecError::new(
                    "aspect field patch target is not a valid field key",
                )
            })?;
            fields.push(field);
        }
        CanonicalFieldPath::new(fields).ok_or_else(|| {
            AspectFieldPatchCodecError::new("aspect field patch target path is empty")
        })
    }

    fn read_target(&mut self) -> Result<AspectFieldPatchTarget, AspectFieldPatchCodecError> {
        let aspect_key = self.read_aspect_key()?;
        let field_path = self.read_field_path()?;
        Ok(AspectFieldPatchTarget::new(aspect_key, field_path))
    }

    fn read_aspect_key(&mut self) -> Result<AspectKey, AspectFieldPatchCodecError> {
        let aspect_key = self.read_string()?;
        AspectKey::new(aspect_key).ok_or_else(|| {
            AspectFieldPatchCodecError::new("aspect field patch target aspect key is not valid")
        })
    }

    fn read_string(&mut self) -> Result<String, AspectFieldPatchCodecError> {
        let length = self.read_u32()? as usize;
        let bytes = self.read_exact(length)?;
        std::str::from_utf8(bytes)
            .map(str::to_owned)
            .map_err(|error| {
                AspectFieldPatchCodecError::new(format!(
                    "invalid utf-8 string in aspect field patch codec: {error}"
                ))
            })
    }

    fn read_u32(&mut self) -> Result<u32, AspectFieldPatchCodecError> {
        let mut buffer = [0_u8; 4];
        buffer.copy_from_slice(self.read_exact(4)?);
        Ok(u32::from_le_bytes(buffer))
    }

    fn read_length_prefixed_aspect_value(
        &mut self,
    ) -> Result<AspectValue, AspectFieldPatchCodecError> {
        let length = self.read_u32()? as usize;
        let bytes = self.read_exact(length)?;
        decode_aspect_value(bytes).map_err(AspectFieldPatchCodecError::from)
    }

    fn read_exact(&mut self, length: usize) -> Result<&'a [u8], AspectFieldPatchCodecError> {
        if self.cursor + length > self.bytes.len() {
            return Err(AspectFieldPatchCodecError::new(format!(
                "aspect field patch codec expected {length} bytes at offset {} but input ended early",
                self.cursor
            )));
        }
        let start = self.cursor;
        self.cursor += length;
        Ok(&self.bytes[start..self.cursor])
    }
}
