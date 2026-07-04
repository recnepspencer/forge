use super::classification::{
    CompiledProductReuseDisposition, CompiledProductReuseSemanticCategory,
};
use super::row::CompiledProductReuseSurfaceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompiledProductReuseInventoryError {
    DuplicateSurface(CompiledProductReuseSurfaceIdentity),
    MissingRequiredSurface(CompiledProductReuseSurfaceIdentity),
    MissingCoveredCategory(CompiledProductReuseSemanticCategory),
    InvalidOrdinaryDisposition {
        surface: CompiledProductReuseSurfaceIdentity,
        disposition: CompiledProductReuseDisposition,
    },
    InvalidNonOrdinaryDisposition {
        surface: CompiledProductReuseSurfaceIdentity,
    },
    MissingExitCondition(CompiledProductReuseSurfaceIdentity),
    UncoveredSourcePattern(String),
    SourceScanFailure(String),
}
