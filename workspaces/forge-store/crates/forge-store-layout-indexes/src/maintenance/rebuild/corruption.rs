#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutCorruptionClassification {
    _owner_issued: (),
}

impl LayoutCorruptionClassification {
    pub(crate) const fn derived_projection_rebuild_to_parity() -> Self {
        Self { _owner_issued: () }
    }
}
