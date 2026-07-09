#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerEvidenceTransform {
    OperatorDefault,
}

impl WorthServerEvidenceTransform {
    pub const fn operator_default() -> Self {
        Self::OperatorDefault
    }
}
