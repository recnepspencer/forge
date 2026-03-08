use forge_core::KernelError;

use crate::configuration::facade::FingerprintDetail;
use crate::engine::contract::InvariantKind;
use crate::proof::{
    ValidationCheckpoint, ValidationConfig, ValidationResult,
};

use super::SpecEnvelope;

impl SpecEnvelope {
    pub fn spec_fingerprint(&self) -> u128 {
        self.spec.spec_hash()
    }

    pub fn projection_fingerprint(&self) -> Result<u128, KernelError> {
        self.fingerprint_now(FingerprintDetail::Full)
    }

    pub fn fingerprint(&self, detail: FingerprintDetail) -> Result<u128, KernelError> {
        self.fingerprint_now(detail)
    }

    pub fn validate_invariant(
        &self,
        kind: &InvariantKind,
        config: &ValidationConfig,
    ) -> Result<(), KernelError> {
        match kind {
            InvariantKind::ManifoldEdges => {
                if !config.is_active(ValidationCheckpoint::PostFeature) {
                    return Ok(());
                }
                self.ensure_invariant_validated(kind)
            }
            InvariantKind::G1Continuity => Ok(()),
            InvariantKind::NoSelfIntersection => Ok(()),
            InvariantKind::NoSliverFaces => Ok(()),
        }
    }

    pub fn validate_structure(&self) -> Result<(), KernelError> {
        self.ensure_structure_validated()
    }

    pub fn run_checkpoint(
        &self,
        config: &ValidationConfig,
        checkpoint: ValidationCheckpoint,
    ) -> Result<ValidationResult, KernelError> {
        self.checkpoint_result_now(config, checkpoint)
    }
}
