use worth_store::physical_runtime::{
    PhysicalWorkBackendEvidenceClass, PhysicalWorkBackendProfileEvidence,
    PhysicalWorkCounterStageEvidence, PhysicalWorkCourtroomFinding, PhysicalWorkEffectFateEvidence,
    PhysicalWorkFamilyEvidence, PhysicalWorkPressureEvidence, PhysicalWorkRecoveryEvidence,
    PhysicalWorkSignalSettlementEvidence,
};

pub(super) const fn backend_profile(value: PhysicalWorkBackendProfileEvidence) -> &'static str {
    match value {
        PhysicalWorkBackendProfileEvidence::SimulatedStrictDurable => "simulated-strict-durable",
        PhysicalWorkBackendProfileEvidence::PosixFileFsyncDirSync => "posix-file-fsync-dir-sync",
        PhysicalWorkBackendProfileEvidence::WindowsFlushFileBuffers => "windows-flush-file-buffers",
        PhysicalWorkBackendProfileEvidence::MmapFlushNotDurabilityCertified => {
            "mmap-flush-not-durability-certified"
        }
        PhysicalWorkBackendProfileEvidence::AdversarialLostFlush => "adversarial-lost-flush",
        PhysicalWorkBackendProfileEvidence::AdversarialReorderedFlush => {
            "adversarial-reordered-flush"
        }
    }
}

pub(super) const fn evidence_class(value: PhysicalWorkBackendEvidenceClass) -> &'static str {
    match value {
        PhysicalWorkBackendEvidenceClass::DeclaredByConfig => "declared-by-config",
        PhysicalWorkBackendEvidenceClass::ObservedByProbe => "observed-by-probe",
        PhysicalWorkBackendEvidenceClass::EstablishedByFilesystemAdmission => {
            "established-by-filesystem-admission"
        }
        PhysicalWorkBackendEvidenceClass::ExternallyGuaranteed => "externally-guaranteed",
        PhysicalWorkBackendEvidenceClass::UnverifiableAssumption => "unverifiable-assumption",
        PhysicalWorkBackendEvidenceClass::CertifiedBackendProfile => "certified-backend-profile",
    }
}

pub(super) const fn family(value: PhysicalWorkFamilyEvidence) -> &'static str {
    match value {
        PhysicalWorkFamilyEvidence::ArtifactMetadataRead => "artifact-metadata-read",
        PhysicalWorkFamilyEvidence::ArtifactRangeRead => "artifact-range-read",
        PhysicalWorkFamilyEvidence::ArtifactRangeWrite => "artifact-range-write",
        PhysicalWorkFamilyEvidence::ArtifactPublication => "artifact-publication",
    }
}

pub(super) const fn pressure(value: PhysicalWorkPressureEvidence) -> &'static str {
    match value {
        PhysicalWorkPressureEvidence::Unscheduled => "unscheduled",
        PhysicalWorkPressureEvidence::ForegroundPointRead => "foreground-point-read",
        PhysicalWorkPressureEvidence::ForegroundRangeRead => "foreground-range-read",
        PhysicalWorkPressureEvidence::ForegroundInteractiveRead => "foreground-interactive-read",
        PhysicalWorkPressureEvidence::ForegroundInternalRead => "foreground-internal-read",
        PhysicalWorkPressureEvidence::ForegroundMutation => "foreground-mutation",
    }
}

pub(super) const fn counter_stage(value: PhysicalWorkCounterStageEvidence) -> &'static str {
    match value {
        PhysicalWorkCounterStageEvidence::Declared => "declared",
        PhysicalWorkCounterStageEvidence::Blocked => "blocked",
        PhysicalWorkCounterStageEvidence::Ready => "ready",
        PhysicalWorkCounterStageEvidence::Queued => "queued",
        PhysicalWorkCounterStageEvidence::Dispatched => "dispatched",
        PhysicalWorkCounterStageEvidence::Settling => "settling",
        PhysicalWorkCounterStageEvidence::Terminal => "terminal",
    }
}

pub(super) const fn effect_fate(value: PhysicalWorkEffectFateEvidence) -> &'static str {
    match value {
        PhysicalWorkEffectFateEvidence::ProvenNoEffect => "proven-no-effect",
        PhysicalWorkEffectFateEvidence::ReadCompleted => "read-completed",
        PhysicalWorkEffectFateEvidence::ReadIncomplete => "read-incomplete",
        PhysicalWorkEffectFateEvidence::WriteCompleted => "write-completed",
        PhysicalWorkEffectFateEvidence::PublicationCompleted => "publication-completed",
        PhysicalWorkEffectFateEvidence::CheckpointCompleted => "checkpoint-completed",
        PhysicalWorkEffectFateEvidence::WalReclamationCompleted => "wal-reclamation-completed",
        PhysicalWorkEffectFateEvidence::WrittenButSchedulerRejected => {
            "written-but-scheduler-rejected"
        }
        PhysicalWorkEffectFateEvidence::Indeterminate => "indeterminate",
        PhysicalWorkEffectFateEvidence::StaleOrForeignOutcome => "stale-or-foreign-outcome",
    }
}

pub(super) const fn recovery(value: PhysicalWorkRecoveryEvidence) -> &'static str {
    match value {
        PhysicalWorkRecoveryEvidence::NoEffect => "no-effect",
        PhysicalWorkRecoveryEvidence::RetryExact => "retry-exact",
        PhysicalWorkRecoveryEvidence::ContinueSettlement => "continue-settlement",
        PhysicalWorkRecoveryEvidence::InspectionRequired => "inspection-required",
    }
}

pub(super) const fn signal_settlement(value: PhysicalWorkSignalSettlementEvidence) -> &'static str {
    match value {
        PhysicalWorkSignalSettlementEvidence::Committed => "committed",
        PhysicalWorkSignalSettlementEvidence::ReconciledFromPhysicalTruth => {
            "reconciled-from-physical-truth"
        }
        PhysicalWorkSignalSettlementEvidence::DerivedStateUnavailable => {
            "derived-state-unavailable"
        }
    }
}

pub(super) const fn finding(value: PhysicalWorkCourtroomFinding) -> &'static str {
    match value {
        PhysicalWorkCourtroomFinding::MissingCausalRecord => "missing-causal-record",
        PhysicalWorkCourtroomFinding::CausalEvidenceOverflow => "causal-evidence-overflow",
        PhysicalWorkCourtroomFinding::ForeignStoreIdentity => "foreign-store-identity",
        PhysicalWorkCourtroomFinding::ForeignRuntimeIdentity => "foreign-runtime-identity",
        PhysicalWorkCourtroomFinding::ForeignLifecycleGeneration => "foreign-lifecycle-generation",
        PhysicalWorkCourtroomFinding::DuplicateOperationIdentity => "duplicate-operation-identity",
        PhysicalWorkCourtroomFinding::DuplicateSignalAttemptIdentity => {
            "duplicate-signal-attempt-identity"
        }
        PhysicalWorkCourtroomFinding::DuplicateBackendOperationIdentity => {
            "duplicate-backend-operation-identity"
        }
        PhysicalWorkCourtroomFinding::InvalidRetryCausalChain => "invalid-retry-causal-chain",
        PhysicalWorkCourtroomFinding::MixedBackendProfile => "mixed-backend-profile",
        PhysicalWorkCourtroomFinding::ShutdownResidual => "shutdown-residual",
        PhysicalWorkCourtroomFinding::ShutdownOvercount => "shutdown-overcount",
        PhysicalWorkCourtroomFinding::DrainEvidenceOverflow => "drain-evidence-overflow",
        PhysicalWorkCourtroomFinding::DrainResidual => "drain-residual",
        PhysicalWorkCourtroomFinding::MissingArtifactManifest => "missing-artifact-manifest",
        PhysicalWorkCourtroomFinding::DuplicateArtifactPath => "duplicate-artifact-path",
        PhysicalWorkCourtroomFinding::OracleRejected => "oracle-rejected",
        PhysicalWorkCourtroomFinding::MissingMutantLocalization => "missing-mutant-localization",
        PhysicalWorkCourtroomFinding::MutantSurvived => "mutant-survived",
    }
}

#[cfg(test)]
mod tests {
    use super::{effect_fate, PhysicalWorkEffectFateEvidence};

    #[test]
    fn incomplete_read_fate_has_its_own_terminal_label() {
        let incomplete = effect_fate(PhysicalWorkEffectFateEvidence::ReadIncomplete);

        assert_eq!(incomplete, "read-incomplete");
        assert_ne!(
            incomplete,
            effect_fate(PhysicalWorkEffectFateEvidence::ReadCompleted)
        );
    }

    #[test]
    fn checkpoint_completion_has_its_own_terminal_label() {
        let checkpoint = effect_fate(PhysicalWorkEffectFateEvidence::CheckpointCompleted);

        assert_eq!(checkpoint, "checkpoint-completed");
        assert_ne!(
            checkpoint,
            effect_fate(PhysicalWorkEffectFateEvidence::PublicationCompleted)
        );
    }

    #[test]
    fn wal_reclamation_completion_has_its_own_terminal_label() {
        let reclamation = effect_fate(PhysicalWorkEffectFateEvidence::WalReclamationCompleted);

        assert_eq!(reclamation, "wal-reclamation-completed");
        assert_ne!(
            reclamation,
            effect_fate(PhysicalWorkEffectFateEvidence::CheckpointCompleted)
        );
    }
}
