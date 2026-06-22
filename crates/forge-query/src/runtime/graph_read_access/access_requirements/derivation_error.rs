#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadAccessRequirementDerivationError {
    ReadGraphDigestMismatch {
        access_shape_read_graph_digest: String,
        selectivity_shape_read_graph_digest: String,
    },
    AccessShapeDigestMismatch {
        access_shape_digest: String,
        selectivity_shape_access_shape_digest: String,
    },
}

impl ForgeQueryGraphReadAccessRequirementDerivationError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadGraphDigestMismatch { .. } => "read_graph_digest_mismatch",
            Self::AccessShapeDigestMismatch { .. } => "access_shape_digest_mismatch",
        }
    }
}
