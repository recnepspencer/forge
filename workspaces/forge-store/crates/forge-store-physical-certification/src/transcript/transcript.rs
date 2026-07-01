use crate::{
    ObservedPhysicalTrace, PersistedStoreFixtureManifest, PhysicalCounterEvidenceReceipt,
    PhysicalFaultEvent, PhysicalInterleavingSchedule, PhysicalProofOracleVerdict,
    PhysicalSimulationPlan,
};

use super::{
    ExecutedTranscriptParts, PhysicalSimulationTranscriptIdentity, TranscriptReplayDenial,
    TranscriptReplayEvidenceIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationTranscript {
    identity: PhysicalSimulationTranscriptIdentity,
    replay_basis_identity: TranscriptReplayEvidenceIdentity,
    plan: PhysicalSimulationPlan,
    schedule: PhysicalInterleavingSchedule,
    fixture_manifest: PersistedStoreFixtureManifest,
    trace: ObservedPhysicalTrace,
    counter_receipt: PhysicalCounterEvidenceReceipt,
    fault_events: Vec<PhysicalFaultEvent>,
    oracle_verdicts: Vec<PhysicalProofOracleVerdict>,
}

pub type PhysicalStoryTranscript = PhysicalSimulationTranscript;

impl PhysicalSimulationTranscript {
    pub fn from_executed_parts(
        parts: ExecutedTranscriptParts,
    ) -> Result<Self, TranscriptReplayDenial> {
        let replay_basis_identity = TranscriptReplayEvidenceIdentity::from_parts(&parts)?;
        parts.require_complete(&replay_basis_identity)?;
        let identity = PhysicalSimulationTranscriptIdentity::from_parts(&parts)?;
        let (
            plan,
            schedule,
            fixture_manifest,
            trace,
            counter_receipt,
            fault_events,
            oracle_verdicts,
        ) = parts.into_members();
        Ok(Self {
            identity,
            replay_basis_identity,
            plan,
            schedule,
            fixture_manifest,
            trace,
            counter_receipt,
            fault_events,
            oracle_verdicts,
        })
    }

    pub const fn identity(&self) -> &PhysicalSimulationTranscriptIdentity {
        &self.identity
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
