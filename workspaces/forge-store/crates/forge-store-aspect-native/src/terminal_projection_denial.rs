use forge_foundational::JsonCompatibilityLoweringDenial;

use crate::StoreAspectNativeDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreTerminalProjectionDenial {
    MissingProjectedAspectValue,
    ContractIdentityMismatch,
    UnsupportedTerminalProjectionValue(&'static str),
    JsonCompatibilityDenied(JsonCompatibilityLoweringDenial),
    StoreAuthorityDenied(StoreAspectNativeDenial),
}
