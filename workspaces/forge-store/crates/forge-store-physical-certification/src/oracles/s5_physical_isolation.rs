use crate::{OracleFamilyKind, PhysicalSimulationScenarioFamily};

use super::{
    OracleDenial, OracleVerdictBasis, PhysicalProofOracle, PhysicalProofOracleKind,
    PhysicalProofOracleVerdict,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S5PhysicalIsolationInterleavingOracle;

impl PhysicalProofOracle for S5PhysicalIsolationInterleavingOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::S5PhysicalIsolationInterleaving
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::S5PhysicalIsolationInterleaving
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        match basis.scenario_family() {
            PhysicalSimulationScenarioFamily::S5CompactionInterlock
            | PhysicalSimulationScenarioFamily::S5ReclaimReachability => {
                require_compaction_interlock(basis, self.family_kind(), self.oracle_kind())
            }
            PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock
            | PhysicalSimulationScenarioFamily::S5RestartDuringCutover => {
                require_checkpoint_interlock(basis, self.family_kind(), self.oracle_kind())
            }
            PhysicalSimulationScenarioFamily::S5TierMovementStability
            | PhysicalSimulationScenarioFamily::S5FutureChunkStability => {
                require_independent_verifier(basis, self.family_kind(), self.oracle_kind())
            }
            _ => Err(OracleDenial::OracleFamilyNotRequired {
                family: self.family_kind(),
            }),
        }
    }
}

fn require_compaction_interlock(
    basis: OracleVerdictBasis,
    family: OracleFamilyKind,
    oracle: PhysicalProofOracleKind,
) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
    if basis.compaction_interlock_present() {
        Ok(PhysicalProofOracleVerdict::satisfied(
            family,
            oracle,
            basis,
            [],
        ))
    } else {
        Err(OracleDenial::MissingCompactionInterlockObservation)
    }
}

fn require_checkpoint_interlock(
    basis: OracleVerdictBasis,
    family: OracleFamilyKind,
    oracle: PhysicalProofOracleKind,
) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
    if basis.checkpoint_interlock_present() {
        Ok(PhysicalProofOracleVerdict::satisfied(
            family,
            oracle,
            basis,
            [],
        ))
    } else {
        Err(OracleDenial::MissingCheckpointInterlockObservation)
    }
}

fn require_independent_verifier(
    basis: OracleVerdictBasis,
    family: OracleFamilyKind,
    oracle: PhysicalProofOracleKind,
) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
    if basis.independent_verifier_present() {
        Ok(PhysicalProofOracleVerdict::satisfied(
            family,
            oracle,
            basis,
            [],
        ))
    } else {
        Err(OracleDenial::MissingIndependentVerifierObservation)
    }
}
