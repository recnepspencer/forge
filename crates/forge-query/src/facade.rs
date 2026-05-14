//! Public API boundary for `forge-query`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

mod exports_foundation;
mod exports_policy;
mod exports_runtime;

pub mod foundation {
    pub use super::exports_foundation::*;
}

pub mod policy {
    pub use super::exports_policy::*;
    pub use crate::query_basis_lifecycle::{
        admit_basis_capability, AdmittedBasisCapability, BasisAuthorityPosture, BasisEligibility,
        BasisEligibilityCounters, BasisIntentDenial, BasisIntentDenialKind, BasisLifecyclePosture,
        DeniedBasisCapability, DeniedBasisCapabilityKind, NormalizedBasisIntent, RawBasisIntent,
        ScopedCertificationBasis, ScopedInspectionBasis, ScopedMaterializationBasis,
        ScopedMutationPreparationBasis, ScopedObservationBasis, ScopedPreviewCloseoutBasis,
        ScopedReplayBasis, ScopedSubscriptionActivationBasis, ScopedSubscriptionDeclarationBasis,
    };
}

pub mod runtime {
    pub use super::exports_runtime::*;
}

pub use exports_foundation::*;
pub use exports_policy::*;
pub use exports_runtime::*;
