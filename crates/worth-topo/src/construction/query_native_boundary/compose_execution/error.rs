use forge_query::facade::ForgeQueryRuntimeError;

use super::super::birth_synopsis::TopologyPrimitiveConstructionBirthFamily;

#[derive(Debug)]
pub enum TopologyPrimitiveConstructionBirthComposeExecutionError {
    Runtime(ForgeQueryRuntimeError),
    MissingGraphObligationEvidence {
        family: TopologyPrimitiveConstructionBirthFamily,
    },
}

impl std::fmt::Display for TopologyPrimitiveConstructionBirthComposeExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Runtime(error) => write!(f, "{error}"),
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
