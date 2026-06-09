use super::request::PrimitiveConstructionPhaseError;
#[derive(Debug)]
pub enum PrimitiveConstructionResultError {
    Phase(PrimitiveConstructionPhaseError),
}

impl std::fmt::Display for PrimitiveConstructionResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Phase(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionResultError {}

#[cfg(test)]
pub use crate::construction::tests::support::prepared_result::{
    prepare_primitive_construction_result, PreparedPrimitiveConstructionResult,
};
