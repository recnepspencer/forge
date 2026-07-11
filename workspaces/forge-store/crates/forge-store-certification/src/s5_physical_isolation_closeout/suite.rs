use super::S5CloseoutReservationSet;
use crate::s6::verify_executed_closeout_handoff_admissible;
use crate::{
    s5_physical_isolation_required_mutation_rows, S5ExecutedIsolationEvidenceBundle,
    S5PhysicalIsolationMutationEvidence, S6IoQosReadinessHandoffMaterializationDenial,
};
use forge_store_physical_certification::{
    CertifiedPhysicalScenario, CoverageSurfaceKind, GeneratedCoverageMatrix,
    MutationValidationPosture, OracleFamilyKind, PhysicalCertificationEvidenceBundle,
    PhysicalProofOracleKind, PhysicalProofOracleVerdictKind, PhysicalSimulationPlan,
    PhysicalSimulationScenarioFamily, Roadmap2HarnessSequence, S5HarnessReadinessReceipt,
};
use forge_store_physical_isolation::{
    ExecutedIsolationEvidence, ProjectionArtifactKind, StorePhysicalAuthoritySurface,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalIsolationCloseoutDenial {
    MissingLane(PhysicalSimulationScenarioFamily),
    DuplicateLane(PhysicalSimulationScenarioFamily),
    WrongHarnessSequence,
    ScenarioPlanMismatch,
    MissingCoverageSurface(CoverageSurfaceKind),
    CoverageIdentityMismatch(CoverageSurfaceKind),
    NonCiCertificationProfile,
    MissingMutationValidation,
    MutationReplayBasisMismatch,
    MutationRowsDoNotMatchFamily,
    MissingS5Oracle,
    MissingProofProjection,
    ExecutedEvidenceReplayBasisMismatch,
    ProjectionCouldMintAuthority,
    S6(S6IoQosReadinessHandoffMaterializationDenial),
}

#[derive(Debug, Clone)]
pub struct PhysicalIsolationCloseoutLaneEvidence {
    scenario: CertifiedPhysicalScenario,
    plan: PhysicalSimulationPlan,
    coverage: GeneratedCoverageMatrix,
    certification: PhysicalCertificationEvidenceBundle,
    mutation: S5PhysicalIsolationMutationEvidence,
    executed: S5ExecutedIsolationEvidenceBundle,
}

#[derive(Debug, Clone)]
pub struct PhysicalIsolationCloseoutSuite {
    s45_readiness: S5HarnessReadinessReceipt,
    lanes: Vec<PhysicalIsolationCloseoutLaneEvidence>,
    reservations: S5CloseoutReservationSet,
}

/// Certification-only handoff evidence sealing an executed S5 closeout for production admission.
#[derive(Debug, Clone)]
pub struct PhysicalIsolationCloseoutHandoffEvidence {
    suite: PhysicalIsolationCloseoutSuite,
    executed_closeout: ExecutedIsolationEvidence,
}

impl PhysicalIsolationCloseoutLaneEvidence {
    pub fn from_executed_lane(
        scenario: CertifiedPhysicalScenario,
        plan: PhysicalSimulationPlan,
        coverage: GeneratedCoverageMatrix,
        certification: PhysicalCertificationEvidenceBundle,
        mutation: S5PhysicalIsolationMutationEvidence,
        executed: S5ExecutedIsolationEvidenceBundle,
    ) -> Result<Self, PhysicalIsolationCloseoutDenial> {
        let row = Self {
            scenario,
            plan,
            coverage,
            certification,
            mutation,
            executed,
        };
        row.require_complete()?;
        Ok(row)
    }

    pub const fn scenario(&self) -> &CertifiedPhysicalScenario {
        &self.scenario
    }

    pub const fn plan(&self) -> &PhysicalSimulationPlan {
        &self.plan
    }

    pub const fn coverage(&self) -> &GeneratedCoverageMatrix {
        &self.coverage
    }

    pub const fn certification(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.certification
    }

    pub const fn mutation(&self) -> &S5PhysicalIsolationMutationEvidence {
        &self.mutation
    }

    pub const fn executed(&self) -> &S5ExecutedIsolationEvidenceBundle {
        &self.executed
    }

    pub const fn family(&self) -> PhysicalSimulationScenarioFamily {
        self.plan.scenario_family()
    }

    fn require_complete(&self) -> Result<(), PhysicalIsolationCloseoutDenial> {
        if self.coverage.sequence() != Roadmap2HarnessSequence::S45 {
            return Err(PhysicalIsolationCloseoutDenial::WrongHarnessSequence);
        }
        if self.plan.profile()
            != forge_store_physical_certification::PhysicalSimulationProfile::CiCertification
            || self.certification.replay().schedule().profile()
                != forge_store_physical_certification::PhysicalSimulationProfile::CiCertification
        {
            return Err(PhysicalIsolationCloseoutDenial::NonCiCertificationProfile);
        }
        if self.scenario.definition().family() != self.plan.scenario_family()
            || self.certification.replay().plan().identity() != self.plan.identity()
        {
            return Err(PhysicalIsolationCloseoutDenial::ScenarioPlanMismatch);
        }
        self.require_coverage_identity()?;
        self.require_mutation_identity()?;
        if !self
            .certification
            .replay()
            .oracle_verdicts()
            .iter()
            .any(|verdict| {
                verdict.family() == OracleFamilyKind::S5PhysicalIsolationInterleaving
                    && verdict.oracle() == PhysicalProofOracleKind::S5PhysicalIsolationInterleaving
                    && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
            })
        {
            return Err(PhysicalIsolationCloseoutDenial::MissingS5Oracle);
        }
        if !self
            .executed
            .proof()
            .is_checked_from_executed_store_isolation()
        {
            return Err(PhysicalIsolationCloseoutDenial::MissingProofProjection);
        }
        self.require_executed_evidence_identity()?;
        self.require_projection_denials()
    }

    fn require_coverage_identity(&self) -> Result<(), PhysicalIsolationCloseoutDenial> {
        let primary = self.certification.primary();
        require_surface_identity(
            &self.coverage,
            CoverageSurfaceKind::Scenario,
            primary.scenario_digest(),
        )?;
        require_surface_identity(
            &self.coverage,
            CoverageSurfaceKind::Plan,
            primary.plan_digest(),
        )?;
        require_surface_identity(
            &self.coverage,
            CoverageSurfaceKind::YieldpointSchedule,
            self.certification
                .replay()
                .schedule()
                .identity()
                .digest_bytes(),
        )?;
        require_surface_identity(
            &self.coverage,
            CoverageSurfaceKind::Actor,
            primary.plan_digest(),
        )?;
        require_surface_identity(
            &self.coverage,
            CoverageSurfaceKind::Driver,
            primary.plan_digest(),
        )?;
        require_surface_identity(
            &self.coverage,
            CoverageSurfaceKind::Oracle,
            primary.plan_digest(),
        )?;
        require_surface_identity(
            &self.coverage,
            CoverageSurfaceKind::Counter,
            primary.plan_digest(),
        )?;
        require_surface_identity(
            &self.coverage,
            CoverageSurfaceKind::Transcript,
            primary.transcript_digest(),
        )?;
        require_surface_identity(
            &self.coverage,
            CoverageSurfaceKind::MutationResult,
            &mutation_result_identity(self.mutation.physical().posture()),
        )
    }

    fn require_mutation_identity(&self) -> Result<(), PhysicalIsolationCloseoutDenial> {
        let replay = self.certification.replay();
        if self.mutation.required_rows().is_empty() {
            return Err(PhysicalIsolationCloseoutDenial::MissingMutationValidation);
        }
        if self.mutation.plan_identity() != replay.plan().identity().digest_bytes()
            || self.mutation.schedule_identity() != replay.schedule().identity().digest_bytes()
            || self.mutation.transcript_identity() != replay.transcript_identity().digest_bytes()
            || self.mutation.replay_basis_identity()
                != replay.replay_basis_identity().digest_bytes()
        {
            return Err(PhysicalIsolationCloseoutDenial::MutationReplayBasisMismatch);
        }
        if self.mutation.required_rows()
            != s5_physical_isolation_required_mutation_rows(replay.plan().scenario_family())
        {
            return Err(PhysicalIsolationCloseoutDenial::MutationRowsDoNotMatchFamily);
        }
        Ok(())
    }

    fn require_executed_evidence_identity(&self) -> Result<(), PhysicalIsolationCloseoutDenial> {
        let replay = self.certification.replay();
        let basis = self.executed.source_finding().basis();
        if basis.plan_digest() != replay.plan().identity().digest_bytes()
            || basis.schedule_digest() != replay.schedule().identity().digest_bytes()
            || basis.transcript_digest() != replay.transcript_identity().digest_bytes()
            || basis.replay_basis_digest() != replay.replay_basis_identity().digest_bytes()
        {
            return Err(PhysicalIsolationCloseoutDenial::ExecutedEvidenceReplayBasisMismatch);
        }
        Ok(())
    }

    fn require_projection_denials(&self) -> Result<(), PhysicalIsolationCloseoutDenial> {
        for surface in authority_surfaces() {
            let denial = self.executed.reject_projection_as_store_authority(
                ProjectionArtifactKind::FoundationalPerformanceReceipt,
                surface,
            );
            if denial.is_ok() {
                return Err(PhysicalIsolationCloseoutDenial::ProjectionCouldMintAuthority);
            }
            let denial = self.executed.reject_projection_as_store_authority(
                ProjectionArtifactKind::ProofProgressionTrace,
                surface,
            );
            if denial.is_ok() {
                return Err(PhysicalIsolationCloseoutDenial::ProjectionCouldMintAuthority);
            }
        }
        Ok(())
    }
}

impl PhysicalIsolationCloseoutSuite {
    pub fn from_s45_readiness(
        s45_readiness: S5HarnessReadinessReceipt,
        lanes: Vec<PhysicalIsolationCloseoutLaneEvidence>,
    ) -> Result<Self, PhysicalIsolationCloseoutDenial> {
        let suite = Self {
            s45_readiness,
            lanes,
            reservations: S5CloseoutReservationSet::s5_closeout_reservations(),
        };
        suite.require_complete()?;
        Ok(suite)
    }

    pub fn seal_executed_closeout_handoff(
        self,
        closeout: ExecutedIsolationEvidence,
    ) -> Result<PhysicalIsolationCloseoutHandoffEvidence, PhysicalIsolationCloseoutDenial> {
        verify_executed_closeout_handoff_admissible(closeout.clone())
            .map_err(PhysicalIsolationCloseoutDenial::S6)?;
        Ok(PhysicalIsolationCloseoutHandoffEvidence {
            suite: self,
            executed_closeout: closeout,
        })
    }

    pub fn lanes(&self) -> &[PhysicalIsolationCloseoutLaneEvidence] {
        &self.lanes
    }

    pub const fn s45_readiness(&self) -> &S5HarnessReadinessReceipt {
        &self.s45_readiness
    }

    pub const fn reservations(&self) -> &S5CloseoutReservationSet {
        &self.reservations
    }

    pub fn require_complete(&self) -> Result<(), PhysicalIsolationCloseoutDenial> {
        for family in required_families() {
            let count = self
                .lanes
                .iter()
                .filter(|lane| lane.family() == family)
                .count();
            match count {
                0 => return Err(PhysicalIsolationCloseoutDenial::MissingLane(family)),
                1 => {}
                _ => return Err(PhysicalIsolationCloseoutDenial::DuplicateLane(family)),
            }
        }
        if !self.reservations.reserves_only_future_work() {
            return Err(PhysicalIsolationCloseoutDenial::WrongHarnessSequence);
        }
        Ok(())
    }
}

impl PhysicalIsolationCloseoutHandoffEvidence {
    pub const fn suite(&self) -> &PhysicalIsolationCloseoutSuite {
        &self.suite
    }

    pub const fn executed_closeout(&self) -> &ExecutedIsolationEvidence {
        &self.executed_closeout
    }
}

fn require_surface_identity(
    coverage: &GeneratedCoverageMatrix,
    surface: CoverageSurfaceKind,
    expected: &[u8; 32],
) -> Result<(), PhysicalIsolationCloseoutDenial> {
    if coverage
        .rows()
        .iter()
        .any(|row| row.surface() == surface && row.source_identity() == expected)
    {
        Ok(())
    } else if coverage.rows().iter().any(|row| row.surface() == surface) {
        Err(PhysicalIsolationCloseoutDenial::CoverageIdentityMismatch(
            surface,
        ))
    } else {
        Err(PhysicalIsolationCloseoutDenial::MissingCoverageSurface(
            surface,
        ))
    }
}

fn mutation_result_identity(posture: MutationValidationPosture) -> [u8; 32] {
    let mut identity = [0_u8; 32];
    let token = match posture {
        MutationValidationPosture::ExpectedFailureObserved => b"expected-failure-observed",
    };
    for (slot, byte) in identity.iter_mut().zip(token.iter().copied()) {
        *slot = byte;
    }
    identity
}

fn required_families() -> [PhysicalSimulationScenarioFamily; 6] {
    [
        PhysicalSimulationScenarioFamily::S5CompactionInterlock,
        PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock,
        PhysicalSimulationScenarioFamily::S5ReclaimReachability,
        PhysicalSimulationScenarioFamily::S5TierMovementStability,
        PhysicalSimulationScenarioFamily::S5FutureChunkStability,
        PhysicalSimulationScenarioFamily::S5RestartDuringCutover,
    ]
}

fn authority_surfaces() -> [StorePhysicalAuthoritySurface; 4] {
    [
        StorePhysicalAuthoritySurface::StablePhysicalReadPlan,
        StorePhysicalAuthoritySurface::LatchOrderProof,
        StorePhysicalAuthoritySurface::PhysicalEpochBasis,
        StorePhysicalAuthoritySurface::ReclaimEligibilityProof,
    ]
}
