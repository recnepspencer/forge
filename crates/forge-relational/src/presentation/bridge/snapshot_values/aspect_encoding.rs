use forge_foundational::facade::AspectValue;

pub(super) fn encode_snapshot_aspect_value(value: &AspectValue) -> Vec<u8> {
    crate::aspect_wire::encode_aspect_value(value)
}
