//! Store claim-boundary vocabulary.
//!
//! External callers cannot mint platform-grade witnesses directly:
//!
//! ```compile_fail
//! use worth_store_claim_boundaries::{
//!     BackendFamily, LegacyBackendFamily, PlatformGradeClaimWitness,
//! };
//! use worth_store_contracts::ROADMAP_2_S1_SCOPE;
//!
//! let _WORTHd = PlatformGradeClaimWitness {
//!     backend_family: BackendFamily::legacy(LegacyBackendFamily::Heap),
//!     scope: ROADMAP_2_S1_SCOPE,
//! };
//! ```
//!
//! External callers also cannot manufacture forbidden-claim reports by calling
//! the crate-private constructor:
//!
//! ```compile_fail
//! use worth_store_claim_boundaries::{
//!     BackendFamily, ForbiddenPlatformClaim, ForbiddenPlatformClaimReason,
//!     LegacyBackendFamily, StoreBackendCapabilityTier,
//! };
//! use worth_store_contracts::ROADMAP_2_S1_SCOPE;
//!
//! let _WORTHd = ForbiddenPlatformClaim::new(
//!     BackendFamily::legacy(LegacyBackendFamily::Heap),
//!     StoreBackendCapabilityTier::PlatformGrade,
//!     ROADMAP_2_S1_SCOPE,
//!     ForbiddenPlatformClaimReason::LegacyBackendCannotClaimPhysicalFoundation,
//! );
//! ```

#![forbid(unsafe_code)]

mod backend_family;
mod claim_request;
mod counters;
mod forbidden_claim;
mod promotion;
mod tiers;

pub use backend_family::{BackendFamily, LegacyBackendFamily};
pub use claim_request::{
    BackendClaimRequest, ClaimBoundary, ClaimPromotionRejection, ClassifiedBackendClaim,
};
pub use counters::PhysicalShortcutCounterName;
pub use forbidden_claim::{
    ForbiddenPlatformClaim, ForbiddenPlatformClaimReason, LegacyBackendClassificationReport,
};
pub use promotion::PlatformGradeClaimWitness;
pub use tiers::{
    BackendTierMarker, BootstrapBackend, CompatibilityBackend, PhysicalFoundationBackend,
    PlatformGradeBackend, SemanticCertificationBackend, StoreBackendCapabilityTier,
};

pub type StoreCapabilityTier = StoreBackendCapabilityTier;
