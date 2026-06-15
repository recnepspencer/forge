#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneReductionRequestError {
    OperandPairIdentityMismatch {
        expected_operand_pair_identity: String,
        actual_operand_pair_identity: String,
    },
    DeclarationOperandPairIdentityMismatch {
        expected_operand_pair_identity: String,
        actual_operand_pair_identity: String,
    },
}

impl PlanarBooleanCommonPlaneReductionRequestError {
    pub fn human_reason(&self) -> &'static str {
        match self {
            Self::OperandPairIdentityMismatch { .. } => {
                "common-plane reduction request requires a construction receipt for the same admitted boolean operand pair"
            }
            Self::DeclarationOperandPairIdentityMismatch { .. } => {
                "common-plane reduction request requires a 7.0 declaration receipt for the same admitted boolean operand pair"
            }
        }
    }

    pub fn expected_operand_pair_identity(&self) -> Option<&str> {
        match self {
            Self::OperandPairIdentityMismatch {
                expected_operand_pair_identity,
                ..
            }
            | Self::DeclarationOperandPairIdentityMismatch {
                expected_operand_pair_identity,
                ..
            } => Some(expected_operand_pair_identity),
        }
    }

    pub fn actual_operand_pair_identity(&self) -> Option<&str> {
        match self {
            Self::OperandPairIdentityMismatch {
                actual_operand_pair_identity,
                ..
            }
            | Self::DeclarationOperandPairIdentityMismatch {
                actual_operand_pair_identity,
                ..
            } => Some(actual_operand_pair_identity),
        }
    }
}
