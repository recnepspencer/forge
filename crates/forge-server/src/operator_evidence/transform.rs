#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerEvidenceTransform {
    OperatorDefault,
}

impl ForgeServerEvidenceTransform {
    pub const fn operator_default() -> Self {
        Self::OperatorDefault
    }
}
