#![forbid(unsafe_code)]
//!
//! S0 handoff gate proof evidence cannot be synthesized from raw counts by
//! ordinary callers:
//!
//! ```compile_fail
//! use worth_store_s0_reclassification::{
//!     S0CurrentResidueScanEvidence, S0FoundationalAdoptionEvidence,
//!     S0HandoffGateProofEvidence, S0NativeHarnessEvidence, S0PublicFacadeEvidence,
//!     S0TerminalProjectionBoundaryEvidence,
//! };
//!
//! let current = S0CurrentResidueScanEvidence::new(1).unwrap();
//! let terminal = S0TerminalProjectionBoundaryEvidence::new(1).unwrap();
//! let adoption = S0FoundationalAdoptionEvidence::new(6).unwrap();
//! let facade = S0PublicFacadeEvidence::new(3).unwrap();
//! let harness = S0NativeHarnessEvidence::new(2).unwrap();
//! let _WORTHd = S0HandoffGateProofEvidence::new(
//!     current, terminal, adoption, facade, harness,
//! );
//! ```

use worth_store_claim_boundaries::StoreCapabilityTier;

mod handoff_claims;
mod handoff_gate_proof_scan;

pub use handoff_claims::{
    certify_current_s0_handoff_gate_proof_evidence, S0CurrentResidueScanEvidence,
    S0FoundationalAdoptionEvidence, S0HandoffDeniedInputKind, S0HandoffGateProofEvidence,
    S0HandoffGateProofEvidenceDenial, S0NativeHarnessEvidence, S0PublicFacadeEvidence,
    S0TerminalProjectionBoundaryEvidence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendReclassification {
    admitted_tier: StoreCapabilityTier,
}

impl BackendReclassification {
    pub const fn new(admitted_tier: StoreCapabilityTier) -> Self {
        Self { admitted_tier }
    }

    pub const fn admitted_tier(&self) -> StoreCapabilityTier {
        self.admitted_tier
    }
}
