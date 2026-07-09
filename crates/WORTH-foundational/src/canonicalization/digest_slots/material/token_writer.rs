pub(super) fn append_token(material: &mut String, label: &str, value: &str) {
    material.push_str(label);
    material.push('#');
    material.push_str(&value.len().to_string());
    material.push(':');
    material.push_str(value);
    material.push(';');
}

pub(super) fn append_bytes(material: &mut String, label: &str, value: &[u8]) {
    material.push_str(label);
    material.push('#');
    material.push_str(&value.len().to_string());
    material.push(':');
    for byte in value {
        material.push_str(&format!("{byte:02x}"));
    }
    material.push(';');
}

pub(super) fn append_u32(material: &mut String, label: &str, value: u32) {
    append_token(material, label, &value.to_string());
}

pub(super) fn append_u64(material: &mut String, label: &str, value: u64) {
    append_token(material, label, &value.to_string());
}

pub(super) fn append_i32(material: &mut String, label: &str, value: i32) {
    append_token(material, label, &value.to_string());
}

pub(super) fn append_i64(material: &mut String, label: &str, value: i64) {
    append_token(material, label, &value.to_string());
}
