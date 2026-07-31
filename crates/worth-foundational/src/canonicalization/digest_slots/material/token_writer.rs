use super::writer::{CanonicalMaterialResult, CanonicalMaterialWriter};

pub(super) fn append_token(
    material: &mut CanonicalMaterialWriter,
    label: &str,
    value: &str,
) -> CanonicalMaterialResult {
    material.append(label)?;
    material.append("#")?;
    material.append(&value.len().to_string())?;
    material.append(":")?;
    material.append(value)?;
    material.append(";")
}

pub(super) fn append_bytes(
    material: &mut CanonicalMaterialWriter,
    label: &str,
    value: &[u8],
) -> CanonicalMaterialResult {
    material.append(label)?;
    material.append("#")?;
    material.append(&value.len().to_string())?;
    material.append(":")?;
    for byte in value {
        material.append(&format!("{byte:02x}"))?;
    }
    material.append(";")
}

pub(super) fn append_u32(
    material: &mut CanonicalMaterialWriter,
    label: &str,
    value: u32,
) -> CanonicalMaterialResult {
    append_token(material, label, &value.to_string())
}

pub(super) fn append_u64(
    material: &mut CanonicalMaterialWriter,
    label: &str,
    value: u64,
) -> CanonicalMaterialResult {
    append_token(material, label, &value.to_string())
}

pub(super) fn append_i32(
    material: &mut CanonicalMaterialWriter,
    label: &str,
    value: i32,
) -> CanonicalMaterialResult {
    append_token(material, label, &value.to_string())
}

pub(super) fn append_i64(
    material: &mut CanonicalMaterialWriter,
    label: &str,
    value: i64,
) -> CanonicalMaterialResult {
    append_token(material, label, &value.to_string())
}
