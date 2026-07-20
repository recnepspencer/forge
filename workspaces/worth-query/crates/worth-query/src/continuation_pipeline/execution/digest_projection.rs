use crate::authoring::AspectFieldKey;

pub(super) fn terminal_declaration_aspect_projection_for_digest(key: &AspectFieldKey) -> String {
    format!("{}.{}", key.aspect().as_str(), key.field().as_str())
}

pub(super) fn hex_byte(byte: u8) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(2);
    encoded.push(HEX[(byte >> 4) as usize] as char);
    encoded.push(HEX[(byte & 0x0f) as usize] as char);
    encoded
}
