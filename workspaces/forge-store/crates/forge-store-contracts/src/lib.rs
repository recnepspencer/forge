//! Store contract vocabulary.
//!
//! External callers cannot mint physical authority witnesses directly:
//!
//! ```compile_fail
//! use forge_store_contracts::{
//!     PhysicalAuthorityScope, StorePhysicalAuthorityWitness, ROADMAP_2_S1_SCOPE,
//! };
//!
//! let _forged = StorePhysicalAuthorityWitness {
//!     roadmap_scope: ROADMAP_2_S1_SCOPE,
//!     authority_scope: PhysicalAuthorityScope::PhysicalEvidenceExport,
//! };
//! ```

#![forbid(unsafe_code)]

mod artifact_identity;
mod contract_error;
mod handoff_readiness;
mod physical_authority;
mod roadmap_scope;

pub use artifact_identity::{StableArtifactId, StableDigest};
pub use contract_error::{StoreContractError, StoreContractResult};
pub use handoff_readiness::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, S0HandoffArtifactKind,
};
pub use physical_authority::{PhysicalAuthorityScope, StorePhysicalAuthorityWitness};
pub use roadmap_scope::{RoadmapScope, ROADMAP_2_S1_SCOPE, ROADMAP_2_SCOPE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableArtifactClass {
    Authoritative,
    DerivedDurable,
    Ephemeral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedAccuracyClass {
    Exact,
    Conservative,
    Approximate,
    Heuristic,
    Advisory,
}
