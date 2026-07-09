use crate::{
    ObservedPhysicalTrace, OracleDenial, PersistedStoreFixtureManifest,
    PhysicalCounterEvidenceReceipt, PhysicalFaultEvent, PhysicalInterleavingSchedule,
    PhysicalProofOracleVerdict, PhysicalSimulationPlan, ProductionBackedPhysicalFixture,
    ReusablePhysicalOracleFamily, TranscriptReplayOracle,
};

use super::{
    require_plan_bound_oracle_verdicts_for_replay_basis, TranscriptReplayDenial,
    TranscriptReplayEvidenceIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedTranscriptParts {
    plan: PhysicalSimulationPlan,
    schedule: PhysicalInterleavingSchedule,
    fixture_manifest: PersistedStoreFixtureManifest,
    trace: ObservedPhysicalTrace,
    counter_receipt: PhysicalCounterEvidenceReceipt,
    fault_events: Vec<PhysicalFaultEvent>,
    oracle_verdicts: Vec<PhysicalProofOracleVerdict>,
}

impl ExecutedTranscriptParts {
    pub fn new(
        plan: &PhysicalSimulationPlan,
        schedule: PhysicalInterleavingSchedule,
        fixture: &ProductionBackedPhysicalFixture,
        trace: ObservedPhysicalTrace,
        counter_receipt: PhysicalCounterEvidenceReceipt,
    ) -> Result<Self, TranscriptReplayDenial> {
        require_executed_inputs_match_plan(plan, &schedule, &trace)?;
        require_counter_receipt_matches_plan(plan, &counter_receipt)?;
        require_driver_profiles(plan)?;
        Ok(Self {
            plan: plan.clone(),
            schedule,
            fixture_manifest: fixture.manifest().clone(),
            trace,
            counter_receipt,
            fault_events: Vec::new(),
            oracle_verdicts: Vec::new(),
        })
    }

    pub fn from_optional_schedule(
        plan: &PhysicalSimulationPlan,
        schedule: Option<PhysicalInterleavingSchedule>,
        fixture: &ProductionBackedPhysicalFixture,
        trace: ObservedPhysicalTrace,
        counter_receipt: PhysicalCounterEvidenceReceipt,
    ) -> Result<Self, TranscriptReplayDenial> {
        let schedule = schedule.ok_or(TranscriptReplayDenial::MissingSeed)?;
        Self::new(plan, schedule, fixture, trace, counter_receipt)
    }

    pub(crate) fn from_detached_members(
        plan: PhysicalSimulationPlan,
        schedule: PhysicalInterleavingSchedule,
        fixture_manifest: PersistedStoreFixtureManifest,
        trace: ObservedPhysicalTrace,
        counter_receipt: PhysicalCounterEvidenceReceipt,
        fault_events: Vec<PhysicalFaultEvent>,
        oracle_verdicts: Vec<PhysicalProofOracleVerdict>,
    ) -> Result<Self, TranscriptReplayDenial> {
        require_executed_inputs_match_plan(&plan, &schedule, &trace)?;
        require_counter_receipt_matches_plan(&plan, &counter_receipt)?;
        require_driver_profiles(&plan)?;
        Ok(Self {
            plan,
            schedule,
            fixture_manifest,
            trace,
            counter_receipt,
            fault_events,
            oracle_verdicts,
        })
    }

    pub fn with_faults(mut self, faults: impl IntoIterator<Item = PhysicalFaultEvent>) -> Self {
        self.fault_events = faults.into_iter().collect();
        self
    }

    pub fn with_oracle_verdict(mut self, verdict: PhysicalProofOracleVerdict) -> Self {
        self.oracle_verdicts.push(verdict);
        self
    }

    pub fn with_transcript_replay_verdict(mut self) -> Result<Self, TranscriptReplayDenial> {
        let replay_basis = TranscriptReplayEvidenceIdentity::from_parts(&self)?;
        let verdict = ReusablePhysicalOracleFamily::transcript_replay_evidence()
            .oracle(TranscriptReplayOracle)
            .judge(&self.plan, &self.trace)
            .and_then(|verdict| verdict.with_transcript_replay_basis(*replay_basis.digest_bytes()))
            .map_err(transcript_replay_denial_from_oracle)?;
        self.oracle_verdicts.push(verdict);
        Ok(self)
    }

    pub fn require_complete(
        &self,
        replay_basis: &TranscriptReplayEvidenceIdentity,
    ) -> Result<(), TranscriptReplayDenial> {
        require_plan_bound_oracle_verdicts_for_replay_basis(
            &self.plan,
            &self.oracle_verdicts,
            replay_basis,
        )
    }

    pub(crate) const fn plan(&self) -> &PhysicalSimulationPlan {
        &self.plan
    }

    pub(crate) const fn schedule(&self) -> &PhysicalInterleavingSchedule {
        &self.schedule
    }

    pub(crate) const fn fixture_manifest(&self) -> &PersistedStoreFixtureManifest {
        &self.fixture_manifest
    }

    pub(crate) const fn trace(&self) -> &ObservedPhysicalTrace {
        &self.trace
    }

    pub(crate) const fn counter_receipt(&self) -> &PhysicalCounterEvidenceReceipt {
        &self.counter_receipt
    }

    pub(crate) fn fault_events(&self) -> &[PhysicalFaultEvent] {
        &self.fault_events
    }

    pub(crate) fn oracle_verdicts(&self) -> &[PhysicalProofOracleVerdict] {
        &self.oracle_verdicts
    }

    pub(crate) fn into_members(
        self,
    ) -> (
        PhysicalSimulationPlan,
        PhysicalInterleavingSchedule,
        PersistedStoreFixtureManifest,
        ObservedPhysicalTrace,
        PhysicalCounterEvidenceReceipt,
        Vec<PhysicalFaultEvent>,
        Vec<PhysicalProofOracleVerdict>,
    ) {
        (
            self.plan,
            self.schedule,
            self.fixture_manifest,
            self.trace,
            self.counter_receipt,
            self.fault_events,
            self.oracle_verdicts,
        )
    }

    pub(crate) fn driver_profile_rows(&self) -> Vec<String> {
        self.plan
            .driver_contracts()
            .iter()
            .enumerate()
            .map(|(index, driver)| {
                format!(
                    "{index:04}:{:?}:{:?}:{:?}:{}",
                    driver.kind(),
                    driver.profile().boundary(),
                    driver.profile().evidence_surfaces(),
                    driver
                        .backend_profile()
                        .map(|profile| format!("{profile:?}"))
                        .unwrap_or_else(|| "none".to_owned())
                )
            })
            .collect()
    }
}

fn require_executed_inputs_match_plan(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
    trace: &ObservedPhysicalTrace,
) -> Result<(), TranscriptReplayDenial> {
    if !schedule.replay_identity_matches_plan(plan) {
        return Err(TranscriptReplayDenial::PlanScheduleIdentityMismatch);
    }
    if trace.scenario_identity() != plan.scenario_identity()
        || trace.plan_identity() != plan.identity()
    {
        return Err(TranscriptReplayDenial::PlanTraceIdentityMismatch);
    }
    Ok(())
}

fn require_counter_receipt_matches_plan(
    plan: &PhysicalSimulationPlan,
    counter_receipt: &PhysicalCounterEvidenceReceipt,
) -> Result<(), TranscriptReplayDenial> {
    if counter_receipt.plan_identity() == plan.identity() {
        Ok(())
    } else {
        Err(TranscriptReplayDenial::CounterReceiptPlanMismatch)
    }
}

fn require_driver_profiles(plan: &PhysicalSimulationPlan) -> Result<(), TranscriptReplayDenial> {
    let has_driver_profile = plan
        .driver_contracts()
        .iter()
        .any(|driver| !driver.profile().evidence_surfaces().is_empty());
    if has_driver_profile {
        Ok(())
    } else {
        Err(TranscriptReplayDenial::MissingDriverProfile)
    }
}

fn transcript_replay_denial_from_oracle(denial: OracleDenial) -> TranscriptReplayDenial {
    match denial {
        OracleDenial::OracleFamilyNotRequired { .. } => {
            TranscriptReplayDenial::OracleFamilyNotRequired
        }
        OracleDenial::OracleFamilyMismatch { .. } => TranscriptReplayDenial::OracleFamilyMismatch,
        OracleDenial::PlanTraceIdentityMismatch => {
            TranscriptReplayDenial::PlanTraceIdentityMismatch
        }
        _ => TranscriptReplayDenial::OracleVerdictPlanMismatch,
    }
}
