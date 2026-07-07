use crate::{
    scenario::S7BlobHarnessScenarioMetadata, OracleFamilyKind, PhysicalSimulationScenarioFamily,
    ShortcutRejectionObservationKind,
};

use super::{
    OracleDenial, OracleVerdictBasis, PhysicalProofOracle, PhysicalProofOracleKind,
    PhysicalProofOracleVerdict,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobNoSidecarPathOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobNoCrossScopeDedupeOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobConstantMemoryOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobByteEqualityOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkOrderingOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobDigestChecksumDistinctionOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobReachabilityOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobHeavyQualificationEvidenceOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobHeavyCleanupOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobHeavyPatternLaneOracle;

impl PhysicalProofOracle for BlobByteEqualityOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobByteEquality
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHarnessEvidence
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        let observation = require_blob_observation(&basis)?;
        Ok(if observation.byte_equality_verified() {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

impl PhysicalProofOracle for BlobChunkOrderingOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobChunkOrdering
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHarnessEvidence
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        let observation = require_blob_observation(&basis)?;
        Ok(if observation.chunk_ordering_verified() {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

impl PhysicalProofOracle for BlobDigestChecksumDistinctionOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobDigestChecksumDistinction
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHarnessEvidence
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        let observation = require_blob_observation(&basis)?;
        Ok(if observation.digest_checksum_distinction_verified() {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

impl PhysicalProofOracle for BlobReachabilityOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobReachability
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHarnessEvidence
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        let observation = require_blob_observation(&basis)?;
        Ok(if observation.reachability_verified() {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

impl PhysicalProofOracle for BlobNoSidecarPathOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobNoSidecarPath
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHarnessEvidence
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        if !basis.has_shortcut_rejection(ShortcutRejectionObservationKind::WholeObjectHelperDenied)
        {
            return Err(OracleDenial::MissingRequiredShortcutRejectionObservation {
                required: ShortcutRejectionObservationKind::WholeObjectHelperDenied,
            });
        }
        let observation = require_blob_observation(&basis)?;
        Ok(if observation.no_sidecar_path_verified() {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

impl PhysicalProofOracle for BlobNoCrossScopeDedupeOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobNoCrossScopeDedupe
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHarnessEvidence
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        require_blob_basis(&basis)?;
        let observation = require_blob_observation(&basis)?;
        let satisfied = observation.cross_scope_dedupe_guarded();
        Ok(if satisfied {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

impl PhysicalProofOracle for BlobConstantMemoryOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobConstantMemory
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHarnessEvidence
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        require_blob_basis(&basis)?;
        let observation = require_blob_observation(&basis)?;
        let satisfied = observation.constant_memory_envelope_held();
        Ok(if satisfied {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

impl PhysicalProofOracle for BlobHeavyQualificationEvidenceOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobHeavyQualificationEvidence
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHeavyQualification
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        require_blob_basis(&basis)?;
        let observation = require_blob_observation(&basis)?;
        Ok(if observation.heavy_evidence_verified() {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

impl PhysicalProofOracle for BlobHeavyCleanupOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobHeavyCleanup
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHeavyQualification
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        require_blob_basis(&basis)?;
        let observation = require_blob_observation(&basis)?;
        Ok(if observation.heavy_cleanup_verified() {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

impl PhysicalProofOracle for BlobHeavyPatternLaneOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::BlobHeavyPatternLane
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S7BlobHeavyQualification
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        require_blob_basis(&basis)?;
        let observation = require_blob_observation(&basis)?;
        Ok(if observation.heavy_pattern_lane_verified() {
            PhysicalProofOracleVerdict::satisfied(self.family_kind(), self.oracle_kind(), basis, [])
        } else {
            PhysicalProofOracleVerdict::failed(self.family_kind(), self.oracle_kind(), basis, [])
        })
    }
}

fn require_blob_basis(
    basis: &OracleVerdictBasis,
) -> Result<S7BlobHarnessScenarioMetadata, OracleDenial> {
    if basis.scenario_family() != PhysicalSimulationScenarioFamily::S7BlobHarnessSeed {
        return Err(OracleDenial::PlanTraceIdentityMismatch);
    }
    basis
        .s7_blob_harness_metadata()
        .ok_or(OracleDenial::MissingS7BlobHarnessMetadata)
}

fn require_blob_observation(
    basis: &OracleVerdictBasis,
) -> Result<crate::S7BlobHarnessOracleObservation, OracleDenial> {
    require_blob_basis(basis)?;
    basis
        .s7_blob_harness_observation()
        .ok_or(OracleDenial::MissingS7BlobHarnessObservation)
}
