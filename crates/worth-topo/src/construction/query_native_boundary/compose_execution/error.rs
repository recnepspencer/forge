use forge_query::facade::ForgeQueryRuntimeError;

use super::super::birth_synopsis::TopologyPrimitiveConstructionBirthFamily;

#[derive(Debug)]
pub enum TopologyPrimitiveConstructionBirthComposeExecutionError {
    Runtime(ForgeQueryRuntimeError),
    TouchedBasisDescriptor {
        reason: String,
    },
    TouchedBasisMismatch {
        expected_basis_digest: String,
        actual_basis_digest: String,
    },
    MissingGraphObligationEvidence {
        family: TopologyPrimitiveConstructionBirthFamily,
    },
}

impl std::fmt::Display for TopologyPrimitiveConstructionBirthComposeExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Runtime(error) => write!(f, "{error}"),
            Self::TouchedBasisDescriptor { reason } => write!(
                f,
                "primitive construction birth touched-basis descriptor could not be built before compose execution: {reason}"
            ),
            Self::TouchedBasisMismatch {
                expected_basis_digest,
                actual_basis_digest,
            } => write!(
                f,
                "primitive construction birth compose requires declared touched-basis digest `{expected_basis_digest}` before Query graph execution, got `{actual_basis_digest}`"
            ),
            Self::MissingGraphObligationEvidence { family } => write!(
                f,
                "primitive construction birth compose for `{}` did not retain graph obligation evidence",
                family.as_str()
            ),
        }
    }
}

impl std::error::Error for TopologyPrimitiveConstructionBirthComposeExecutionError {}

impl From<ForgeQueryRuntimeError> for TopologyPrimitiveConstructionBirthComposeExecutionError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::Runtime(value)
    }
}
