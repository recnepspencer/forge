use crate::{
    AdmittedDriverContractSet, CertifiedPhysicalScenario, PhysicalCounterEvidenceReceipt,
    PhysicalInterleavingSchedule, PhysicalProofOracleVerdict, PhysicalProofOracleVerdictKind,
    PhysicalScenarioCanonicalIdentity, PhysicalSimulationPlan, SimulationReplayBundle,
};

use super::{
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, GeneratedCoverageMatrix,
    MutationValidationPosture, PhysicalCoverageMatrixRow, PhysicalMutationCoverageEvidence,
    Roadmap2HarnessSequence, Roadmap2PhysicalCoverageMatrix,
};

#[derive(Debug, Clone)]
pub struct Roadmap2CoverageRegistry {
    sequence: Roadmap2HarnessSequence,
    scenario_identity: Option<PhysicalScenarioCanonicalIdentity>,
    plan: Option<PhysicalSimulationPlan>,
    rows: Vec<PhysicalCoverageMatrixRow>,
}

impl Roadmap2CoverageRegistry {
    pub fn for_sequence(sequence: Roadmap2HarnessSequence) -> Self {
        Self {
            sequence,
            scenario_identity: None,
            plan: None,
            rows: Vec::new(),
        }
    }

    pub fn register_scenario(
        mut self,
        scenario: &CertifiedPhysicalScenario,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Scenario)?;
        if let Some(plan) = self.plan.as_ref() {
            if scenario.identity() != plan.scenario_identity() {
                return Err(CoverageGapDenial::PlanScenarioIdentityMismatch);
            }
        }
        self.scenario_identity = Some(scenario.identity().clone());
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Scenario,
            *scenario.identity().digest_bytes(),
            [
                CoverageRowDimension::ProductionBoundaryYieldpoint(
                    scenario
                        .definition()
                        .schedule()
                        .production_boundary_yieldpoint()
                        .to_owned(),
                ),
                CoverageRowDimension::FaultPhase(scenario.definition().fault().kind()),
            ],
        ));
        Ok(self)
    }

    pub fn register_plan(
        mut self,
        plan: &PhysicalSimulationPlan,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Plan)?;
        if let Some(scenario_identity) = self.scenario_identity.as_ref() {
            if scenario_identity != plan.scenario_identity() {
                return Err(CoverageGapDenial::PlanScenarioIdentityMismatch);
            }
        }
        let dimensions = plan_dimensions(plan);
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Plan,
            *plan.identity().digest_bytes(),
            dimensions,
        ));
        self.plan = Some(plan.clone());
        Ok(self)
    }

    pub fn register_schedule(
        mut self,
        schedule: &PhysicalInterleavingSchedule,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::YieldpointSchedule)?;
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::YieldpointSchedule,
                })?;
        if !schedule.replay_identity_matches_plan(plan) {
            return Err(CoverageGapDenial::PlanScheduleIdentityMismatch);
        }
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::YieldpointSchedule,
            *schedule.identity().digest_bytes(),
            [CoverageRowDimension::ProductionBoundaryYieldpoint(
                plan.yieldpoint_binding().scheduled_yieldpoint().to_owned(),
            )],
        ));
        Ok(self)
    }

    pub fn register_actor_set(mut self) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Actor)?;
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Actor,
                })?;
        if plan.actors().len() == 0 {
            return Err(CoverageGapDenial::EmptyActorRegistration);
        }
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Actor,
            *plan.identity().digest_bytes(),
            plan.actors()
                .iter()
                .map(|actor| CoverageRowDimension::ActorRole(actor.role())),
        ));
        Ok(self)
    }

    pub fn register_driver_contracts(
        mut self,
        contracts: &AdmittedDriverContractSet,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Driver)?;
        if contracts.iter().next().is_none() {
            return Err(CoverageGapDenial::EmptyDriverRegistration);
        }
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Driver,
                })?;
        if contracts != plan.driver_contracts() {
            return Err(CoverageGapDenial::DriverContractPlanMismatch);
        }
        let identity = *plan.identity().digest_bytes();
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Driver,
            identity,
            contracts
                .iter()
                .map(|driver| CoverageRowDimension::BackgroundInterference(driver.kind())),
        ));
        Ok(self)
    }

    pub fn register_oracle_verdicts(
        mut self,
        verdicts: &[PhysicalProofOracleVerdict],
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Oracle)?;
        if verdicts.is_empty() {
            return Err(CoverageGapDenial::EmptyOracleVerdictRegistration);
        }
        if verdicts.iter().any(|verdict| {
            verdict.kind() != PhysicalProofOracleVerdictKind::Satisfied
                || !self
                    .plan
                    .as_ref()
                    .is_some_and(|plan| plan.oracle_families().contains(verdict.family()))
        }) {
            return Err(CoverageGapDenial::UnsatisfiedOracleVerdict);
        }
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Oracle,
                })?;
        for required_family in plan.oracle_families().iter() {
            if !verdicts.iter().any(|verdict| {
                verdict.family() == required_family
                    && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
            }) {
                return Err(CoverageGapDenial::MissingRequiredOracleVerdict);
            }
        }
        let identity = *plan.identity().digest_bytes();
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Oracle,
            identity,
            verdicts.iter().flat_map(|verdict| {
                [
                    CoverageRowDimension::AuthorityFamily(verdict.family()),
                    CoverageRowDimension::Oracle(verdict.oracle()),
                ]
            }),
        ));
        Ok(self)
    }

    pub fn register_counter_receipt(
        mut self,
        receipt: &PhysicalCounterEvidenceReceipt,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Counter)?;
        if receipt.rows().is_empty() {
            return Err(CoverageGapDenial::EmptyCounterReceiptRegistration);
        }
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Counter,
                })?;
        if receipt.plan_identity() != plan.identity() {
            return Err(CoverageGapDenial::CounterReceiptPlanMismatch);
        }
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Counter,
            *plan.identity().digest_bytes(),
            plan.counter_contracts()
                .iter()
                .map(|contract| CoverageRowDimension::CounterContract(contract.kind())),
        ));
        Ok(self)
    }

    pub fn register_transcript(
        mut self,
        replay: &SimulationReplayBundle,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::Transcript)?;
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::Transcript,
                })?;
        if replay.plan().identity() != plan.identity() {
            return Err(CoverageGapDenial::TranscriptPlanMismatch);
        }
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::Transcript,
            *replay.transcript_identity().digest_bytes(),
            [
                CoverageRowDimension::TranscriptOutput,
                CoverageRowDimension::OfflineVerifier(replay.trace().observer()),
            ],
        ));
        Ok(self)
    }

    pub fn register_mutation_result(
        mut self,
        mutation: &PhysicalMutationCoverageEvidence,
    ) -> Result<Self, CoverageGapDenial> {
        self.require_surface_not_registered(CoverageSurfaceKind::MutationResult)?;
        if mutation.sequence() != self.sequence {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
        let plan =
            self.plan
                .as_ref()
                .ok_or(CoverageGapDenial::MissingPlanBeforeDependentSurface {
                    surface: CoverageSurfaceKind::MutationResult,
                })?;
        if mutation.plan_identity() != plan.identity().digest_bytes() {
            return Err(CoverageGapDenial::MutationPlanIdentityMismatch);
        }
        let identity = mutation_identity(mutation.posture());
        let mut dimensions = vec![CoverageRowDimension::MutationValidationPosture(
            mutation.posture(),
        )];
        dimensions.extend(
            mutation
                .compaction_mutations()
                .iter()
                .map(|row| CoverageRowDimension::CompactionMutation(row.kind())),
        );
        dimensions.extend(
            mutation
                .s5_physical_isolation_mutations()
                .iter()
                .copied()
                .map(CoverageRowDimension::S5PhysicalIsolationMutation),
        );
        self.rows.push(PhysicalCoverageMatrixRow::generated(
            self.sequence,
            CoverageSurfaceKind::MutationResult,
            identity,
            dimensions,
        ));
        Ok(self)
    }

    pub fn generate_matrix(self) -> Result<GeneratedCoverageMatrix, CoverageGapDenial> {
        let matrix = Roadmap2PhysicalCoverageMatrix::generated(self.sequence, self.rows)?;
        Ok(GeneratedCoverageMatrix::from_matrix(matrix))
    }

    fn require_surface_not_registered(
        &self,
        surface: CoverageSurfaceKind,
    ) -> Result<(), CoverageGapDenial> {
        if self.rows.iter().any(|row| row.surface() == surface) {
            Err(CoverageGapDenial::DuplicateRegistrationEvidence { surface })
        } else {
            Ok(())
        }
    }
}

fn plan_dimensions(plan: &PhysicalSimulationPlan) -> Vec<CoverageRowDimension> {
    let mut dimensions = vec![CoverageRowDimension::ResourceEnvelopeProfile(
        plan.profile(),
    )];
    dimensions.extend(
        plan.fixture_classes()
            .iter()
            .map(CoverageRowDimension::ArtifactClass),
    );
    dimensions.extend(
        plan.observers()
            .iter()
            .map(CoverageRowDimension::OfflineVerifier),
    );
    dimensions
}

fn mutation_identity(posture: MutationValidationPosture) -> [u8; 32] {
    let mut identity = [0_u8; 32];
    let token = match posture {
        MutationValidationPosture::ExpectedFailureObserved => b"expected-failure-observed",
    };
    for (slot, byte) in identity.iter_mut().zip(token.iter().copied()) {
        *slot = byte;
    }
    identity
}
