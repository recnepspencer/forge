use forge_core::KernelError;
use forge_topo::projection::compute_projected_topology_hash;

use crate::configuration::facade::FingerprintDetail;
use crate::engine::contract::InvariantKind;
use crate::proof::{
    run_spec_envelope_checkpoint, validate_spec_envelope_structure, ValidationCheckpoint,
    ValidationConfig, ValidationResult,
};

use super::SpecEnvelope;

impl SpecEnvelope {
    pub fn spec_fingerprint(&self) -> u128 {
        self.spec.spec_hash()
    }

    pub fn projection_fingerprint(&self) -> Result<u128, KernelError> {
        Ok(compute_projected_topology_hash(self.projection()?))
    }

    pub fn fingerprint(&self, detail: FingerprintDetail) -> Result<u128, KernelError> {
        match detail {
            FingerprintDetail::Standard => Ok(self.spec_fingerprint()),
            FingerprintDetail::Full => self.projection_fingerprint(),
        }
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
                self.validate_structure()
            }
            InvariantKind::G1Continuity => Ok(()),
            InvariantKind::NoSelfIntersection => Ok(()),
            InvariantKind::NoSliverFaces => Ok(()),
        }
    }

    pub fn validate_structure(&self) -> Result<(), KernelError> {
        validate_spec_envelope_structure(self)
    }

    pub fn run_checkpoint(
        &self,
        config: &ValidationConfig,
        checkpoint: ValidationCheckpoint,
    ) -> Result<ValidationResult, KernelError> {
        run_spec_envelope_checkpoint(self, config, checkpoint)
    }
}
