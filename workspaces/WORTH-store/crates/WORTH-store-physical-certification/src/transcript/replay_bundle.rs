use crate::{
    ObservedPhysicalTrace, PersistedStoreFixtureManifest, PhysicalCounterEvidenceReceipt,
    PhysicalFaultEvent, PhysicalInterleavingSchedule, PhysicalProofOracleVerdict,
    PhysicalSimulationPlan,
};

use super::{
    ExecutedTranscriptParts, PhysicalSimulationTranscript, PhysicalSimulationTranscriptIdentity,
    TranscriptReplayDenial, TranscriptReplayEvidenceIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetachedSimulationReplayParts {
    transcript_identity: PhysicalSimulationTranscriptIdentity,
    replay_basis_identity: TranscriptReplayEvidenceIdentity,
    plan: PhysicalSimulationPlan,
    schedule: PhysicalInterleavingSchedule,
    fixture_manifest: PersistedStoreFixtureManifest,
    trace: ObservedPhysicalTrace,
    counter_receipt: PhysicalCounterEvidenceReceipt,
    fault_events: Vec<PhysicalFaultEvent>,
    oracle_verdicts: Vec<PhysicalProofOracleVerdict>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationReplayBundle {
    transcript_identity: PhysicalSimulationTranscriptIdentity,
    replay_basis_identity: TranscriptReplayEvidenceIdentity,
    plan: PhysicalSimulationPlan,
    schedule: PhysicalInterleavingSchedule,
    fixture_manifest: PersistedStoreFixtureManifest,
    trace: ObservedPhysicalTrace,
    counter_receipt: PhysicalCounterEvidenceReceipt,
    fault_events: Vec<PhysicalFaultEvent>,
    oracle_verdicts: Vec<PhysicalProofOracleVerdict>,
}

impl DetachedSimulationReplayParts {
    pub fn from_transcript(transcript: &PhysicalSimulationTranscript) -> Self {
        Self {
            transcript_identity: transcript.identity().clone(),
            replay_basis_identity: transcript.replay_basis_identity().clone(),
            plan: transcript.plan().clone(),
            schedule: transcript.schedule().clone(),
            fixture_manifest: transcript.fixture_manifest().clone(),
            trace: transcript.trace().clone(),
            counter_receipt: transcript.counter_receipt().clone(),
            fault_events: transcript.fault_events().to_vec(),
            oracle_verdicts: transcript.oracle_verdicts().to_vec(),
        }
    }

    pub fn with_replayed_schedule_candidate(
        mut self,
        schedule: PhysicalInterleavingSchedule,
    ) -> Self {
        self.schedule = schedule;
        self
    }

    pub fn admit_replay_bundle(self) -> Result<SimulationReplayBundle, TranscriptReplayDenial> {
        let expected_transcript_identity = self.transcript_identity;
        let expected_replay_basis_identity = self.replay_basis_identity;
        let parts = ExecutedTranscriptParts::from_detached_members(
            self.plan,
            self.schedule,
            self.fixture_manifest,
            self.trace,
            self.counter_receipt,
            self.fault_events,
            self.oracle_verdicts,
        )?;
        let transcript = PhysicalSimulationTranscript::from_executed_parts(parts)?;
        if transcript.replay_basis_identity() != &expected_replay_basis_identity
            || transcript.identity() != &expected_transcript_identity
        {
            return Err(TranscriptReplayDenial::CopiedTranscriptFieldsDenied);
        }
        Ok(SimulationReplayBundle::from_transcript(transcript))
    }
}

impl SimulationReplayBundle {
    pub fn from_transcript(transcript: PhysicalSimulationTranscript) -> Self {
        Self {
            transcript_identity: transcript.identity().clone(),
            replay_basis_identity: transcript.replay_basis_identity().clone(),
            plan: transcript.plan().clone(),
            schedule: transcript.schedule().clone(),
            fixture_manifest: transcript.fixture_manifest().clone(),
            trace: transcript.trace().clone(),
            counter_receipt: transcript.counter_receipt().clone(),
            fault_events: transcript.fault_events().to_vec(),
            oracle_verdicts: transcript.oracle_verdicts().to_vec(),
        }
    }

    pub const fn transcript_identity(&self) -> &PhysicalSimulationTranscriptIdentity {
        &self.transcript_identity
    }

    pub const fn replay_basis_identity(&self) -> &TranscriptReplayEvidenceIdentity {
        &self.replay_basis_identity
    }

    pub const fn plan(&self) -> &PhysicalSimulationPlan {
        &self.plan
    }

    pub const fn schedule(&self) -> &PhysicalInterleavingSchedule {
        &self.schedule
    }

    pub const fn fixture_manifest(&self) -> &PersistedStoreFixtureManifest {
        &self.fixture_manifest
    }

    pub const fn trace(&self) -> &ObservedPhysicalTrace {
        &self.trace
    }

    pub const fn counter_receipt(&self) -> &PhysicalCounterEvidenceReceipt {
        &self.counter_receipt
    }

    pub fn fault_events(&self) -> &[PhysicalFaultEvent] {
        &self.fault_events
    }

    pub fn oracle_verdicts(&self) -> &[PhysicalProofOracleVerdict] {
        &self.oracle_verdicts
    }
}
