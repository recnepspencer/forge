use forge_foundational::{
    AspectKey, AspectLocator, AspectValue, BoundaryArtifactField, BoundaryArtifactId,
    BoundaryArtifactLocator, BoundaryEpoch, BoundaryHandle, BoundaryMismatchLocator,
    BoundarySourceLocator, InternedString, LocatorAuthority,
};

use crate::{
    BoundedRecoveryReceipt, OfflineRecoveryVerificationReport, OfflineRecoveryVerifierConclusion,
    RecoveredPhysicalState, RecoveryCounterSnapshot,
};

use super::denial::RecoveryEvidenceDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryEvidencePayloadKind {
    FoundationalAspectValues,
    JsonShapedPayload,
    RawBytes,
    DebugString,
    DisplayName,
    ProducerPrivateName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryEvidenceConstructionSource {
    ExecutedRecoveryFindings,
    PlannedRecovery,
    CopiedReceiptFields,
    LogExcerpt,
    SameRunSelfComparison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreRecoveryEvidenceAuthority {
    handle: BoundaryHandle,
    epoch: BoundaryEpoch,
}

impl StoreRecoveryEvidenceAuthority {
    const fn from_executed_recovery(artifact_id: BoundaryArtifactId, epoch: BoundaryEpoch) -> Self {
        Self {
            handle: BoundaryHandle::new(artifact_id.get()),
            epoch,
        }
    }

    pub const fn handle(self) -> BoundaryHandle {
        self.handle
    }

    pub const fn epoch(self) -> BoundaryEpoch {
        self.epoch
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsEvidenceSource {
    recovered_state: RecoveredPhysicalState,
    counters: RecoveryCounterSnapshot,
    payload: Vec<AspectValue>,
    artifact_locator: BoundaryArtifactLocator,
    source_locator: BoundarySourceLocator,
    mismatch_locator: BoundaryMismatchLocator,
    verifier_conclusion: OfflineRecoveryVerifierConclusion,
    verifier_state_agrees: bool,
    verifier_counters_agree: bool,
    authority: StoreRecoveryEvidenceAuthority,
}

impl RecoveryPhysicsEvidenceSource {
    pub fn from_executed_recovery(
        receipt: &BoundedRecoveryReceipt,
        report: &OfflineRecoveryVerificationReport,
        artifact_id: BoundaryArtifactId,
        epoch: BoundaryEpoch,
    ) -> Result<Self, RecoveryEvidenceDenial> {
        let executed_state = receipt.execution().recovered_state().clone();
        let counters = receipt.counters();
        let verifier_state_agrees = Some(&executed_state) == report.recovered_state();
        let verifier_counters_agree = Some(counters) == report.counters();
        let artifact_locator =
            BoundaryArtifactLocator::new(artifact_id, BoundaryArtifactField::Payload);
        let aspect_key = AspectKey::new("store.recovery.s4.evidence")
            .ok_or(RecoveryEvidenceDenial::ProducerPrivateNameCannotMaterializeEvidence)?;
        let aspect_locator = AspectLocator::new(LocatorAuthority::ReceiptBearing, aspect_key);
        Ok(Self {
            payload: foundational_payload(&executed_state, counters),
            source_locator: BoundarySourceLocator::aspect(aspect_locator.clone()),
            mismatch_locator: BoundaryMismatchLocator::boundary_artifact(artifact_locator),
            artifact_locator,
            recovered_state: executed_state,
            counters,
            verifier_conclusion: report.conclusion(),
            verifier_state_agrees,
            verifier_counters_agree,
            authority: StoreRecoveryEvidenceAuthority::from_executed_recovery(artifact_id, epoch),
        })
    }

    pub fn deny_non_executed_source(
        source: RecoveryEvidenceConstructionSource,
    ) -> RecoveryEvidenceDenial {
        match source {
            RecoveryEvidenceConstructionSource::ExecutedRecoveryFindings => {
                RecoveryEvidenceDenial::MissingStoreRecoveryAuthority
            }
            RecoveryEvidenceConstructionSource::PlannedRecovery => {
                RecoveryEvidenceDenial::PlannedRecoveryCannotMaterializeEvidence
            }
            RecoveryEvidenceConstructionSource::CopiedReceiptFields => {
                RecoveryEvidenceDenial::CopiedReceiptFieldsCannotMaterializeEvidence
            }
            RecoveryEvidenceConstructionSource::LogExcerpt => {
                RecoveryEvidenceDenial::LogExcerptCannotMaterializeEvidence
            }
            RecoveryEvidenceConstructionSource::SameRunSelfComparison => {
                RecoveryEvidenceDenial::SameRunSelfComparisonCannotMaterializeEvidence
            }
        }
    }

    pub fn deny_payload_kind(kind: RecoveryEvidencePayloadKind) -> Option<RecoveryEvidenceDenial> {
        match kind {
            RecoveryEvidencePayloadKind::FoundationalAspectValues => None,
            RecoveryEvidencePayloadKind::JsonShapedPayload => {
                Some(RecoveryEvidenceDenial::JsonPayloadCannotMaterializeEvidence)
            }
            RecoveryEvidencePayloadKind::RawBytes => {
                Some(RecoveryEvidenceDenial::RawBytesCannotMaterializeEvidence)
            }
            RecoveryEvidencePayloadKind::DebugString => {
                Some(RecoveryEvidenceDenial::DebugStringCannotMaterializeEvidence)
            }
            RecoveryEvidencePayloadKind::DisplayName => {
                Some(RecoveryEvidenceDenial::DisplayNameCannotMaterializeEvidence)
            }
            RecoveryEvidencePayloadKind::ProducerPrivateName => {
                Some(RecoveryEvidenceDenial::ProducerPrivateNameCannotMaterializeEvidence)
            }
        }
    }

    pub const fn recovered_state(&self) -> &RecoveredPhysicalState {
        &self.recovered_state
    }

    pub const fn counters(&self) -> RecoveryCounterSnapshot {
        self.counters
    }

    pub fn payload(&self) -> &[AspectValue] {
        &self.payload
    }

    pub const fn artifact_locator(&self) -> BoundaryArtifactLocator {
        self.artifact_locator
    }

    pub const fn source_locator(&self) -> &BoundarySourceLocator {
        &self.source_locator
    }

    pub const fn mismatch_locator(&self) -> &BoundaryMismatchLocator {
        &self.mismatch_locator
    }

    pub const fn authority(&self) -> StoreRecoveryEvidenceAuthority {
        self.authority
    }

    pub const fn verifier_conclusion(&self) -> OfflineRecoveryVerifierConclusion {
        self.verifier_conclusion
    }

    pub const fn verifier_state_agrees(&self) -> bool {
        self.verifier_state_agrees
    }

    pub const fn verifier_counters_agree(&self) -> bool {
        self.verifier_counters_agree
    }
}

fn foundational_payload(
    state: &RecoveredPhysicalState,
    counters: RecoveryCounterSnapshot,
) -> Vec<AspectValue> {
    vec![
        AspectValue::String(InternedString::from(state.recovered_physical_root())),
        AspectValue::String(InternedString::from(state.source_decision_digest())),
        AspectValue::UInt64(counters.replayed_frames() as u64),
        AspectValue::UInt64(counters.skipped_frames() as u64),
        AspectValue::UInt64(counters.page_redos() as u64),
    ]
}
