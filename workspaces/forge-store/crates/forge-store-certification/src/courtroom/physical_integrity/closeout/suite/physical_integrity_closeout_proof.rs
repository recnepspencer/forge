use crate::{
    CorruptionLocalizationBoundary, IntegrityCloseoutDenialBoundary,
    PhysicalIntegrityCloseoutDenial,
};
use forge_store_physical_integrity::{
    ChecksumAlgorithmMismatchDenial, ChunkIntegrityDenial, ChunkIntegrityDenialKind,
    IndexPageIntegrityDenial, IndexPageIntegrityDenialKind, IntegrityEntryDenial,
    IntegrityEntryDenialKind, ManifestIntegrityDenial, PhysicalContainerIntegrityDenial,
    PhysicalContainerIntegrityDenialKind, PreDecodePhysicalDenial, PreDecodePhysicalDenialKind,
    QuarantineSealDenial, QuarantineSealDenialKind, ScrubPlanDenial, ScrubPlanDenialKind,
    WalFrameDamageDenial, WalFrameDamageDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutedCorruptionLocalizationEvidence {
    boundary: CorruptionLocalizationBoundary,
}

impl ExecutedCorruptionLocalizationEvidence {
    pub fn from_pre_decode_byte_flip(
        denial: &PreDecodePhysicalDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial.kind() == PreDecodePhysicalDenialKind::ChecksumMismatch
            && denial.counters().skipped_logical_decode().skipped_count() > 0
            && denial
                .counters()
                .semantic_decoder_invocations()
                .invocation_count()
                == 0
        {
            Ok(Self::new(CorruptionLocalizationBoundary::ByteFlip))
        } else {
            Err(
                PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
                    CorruptionLocalizationBoundary::ByteFlip,
                ),
            )
        }
    }

    pub fn from_torn_frame_denial(
        denial: &PhysicalContainerIntegrityDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial.kind() == PhysicalContainerIntegrityDenialKind::TornFrame
            && denial.torn_frame().is_some()
            && denial.counters().skipped_record_view_constructions() > 0
        {
            Ok(Self::new(CorruptionLocalizationBoundary::TornFrame))
        } else {
            Err(
                PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
                    CorruptionLocalizationBoundary::TornFrame,
                ),
            )
        }
    }

    pub fn from_pre_decode_stale_generation(
        denial: &PreDecodePhysicalDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial.kind() == PreDecodePhysicalDenialKind::StaleGeneration
            && denial.locality().is_some()
            && denial.counters().skipped_logical_decode().skipped_count() > 0
            && denial
                .counters()
                .semantic_decoder_invocations()
                .invocation_count()
                == 0
        {
            Ok(Self::new(CorruptionLocalizationBoundary::StaleGeneration))
        } else {
            Err(
                PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
                    CorruptionLocalizationBoundary::StaleGeneration,
                ),
            )
        }
    }

    pub fn from_manifest_denial(
        denial: &ManifestIntegrityDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial.locality().is_some() || !denial.posture().admits_scope() {
            Ok(Self::new(
                CorruptionLocalizationBoundary::ManifestCorruption,
            ))
        } else {
            Err(
                PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
                    CorruptionLocalizationBoundary::ManifestCorruption,
                ),
            )
        }
    }

    pub fn from_index_page_denial(
        denial: &IndexPageIntegrityDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if matches!(
            denial.kind(),
            IndexPageIntegrityDenialKind::MissingAuthorityBasis
                | IndexPageIntegrityDenialKind::DamagedAuthority
                | IndexPageIntegrityDenialKind::StaleIndexGeneration
                | IndexPageIntegrityDenialKind::MissingGenerationLink
                | IndexPageIntegrityDenialKind::MismatchedAuthorityRoot
        ) && denial.counters().skipped_semantic_index_lookups() > 0
        {
            Ok(Self::new(
                CorruptionLocalizationBoundary::IndexPageCorruption,
            ))
        } else {
            Err(
                PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
                    CorruptionLocalizationBoundary::IndexPageCorruption,
                ),
            )
        }
    }

    pub fn from_wal_frame_denial(
        denial: &WalFrameDamageDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if !matches!(
            denial.kind(),
            WalFrameDamageDenialKind::WrongPhysicalFamily
                | WalFrameDamageDenialKind::RecoveryPrecedenceRequired
        ) && denial.counters().skipped_replay_attempts() > 0
        {
            Ok(Self::new(
                CorruptionLocalizationBoundary::WalFrameCorruption,
            ))
        } else {
            Err(
                PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
                    CorruptionLocalizationBoundary::WalFrameCorruption,
                ),
            )
        }
    }

    pub fn from_extent_damage_denial(
        denial: &ChunkIntegrityDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial.kind() == ChunkIntegrityDenialKind::ExtentBoundaryDamage
            && denial.damage_locality().is_some()
            && denial.counters().extent_boundary_checks() > 0
        {
            Ok(Self::new(CorruptionLocalizationBoundary::ExtentDamage))
        } else {
            Err(
                PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
                    CorruptionLocalizationBoundary::ExtentDamage,
                ),
            )
        }
    }

    pub fn from_chunk_damage_denial(
        denial: &ChunkIntegrityDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if matches!(
            denial.kind(),
            ChunkIntegrityDenialKind::ChunkHeaderDamage
                | ChunkIntegrityDenialKind::ChunkPayloadDamage
                | ChunkIntegrityDenialKind::ChunkBoundaryDamage
                | ChunkIntegrityDenialKind::UnknownChunkIntegrity
        ) && denial.damage_locality().is_some()
            && denial.counters().skipped_whole_object_reads() > 0
        {
            Ok(Self::new(CorruptionLocalizationBoundary::ChunkDamage))
        } else {
            Err(
                PhysicalIntegrityCloseoutDenial::UnexecutedCorruptionLocalization(
                    CorruptionLocalizationBoundary::ChunkDamage,
                ),
            )
        }
    }

    pub const fn boundary(self) -> CorruptionLocalizationBoundary {
        self.boundary
    }

    const fn new(boundary: CorruptionLocalizationBoundary) -> Self {
        Self { boundary }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutedIntegrityBoundaryDenialEvidence {
    boundary: IntegrityCloseoutDenialBoundary,
}

impl ExecutedIntegrityBoundaryDenialEvidence {
    pub fn from_forged_checksum_denial(
        denial: &WalFrameDamageDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial.kind() == WalFrameDamageDenialKind::ChecksumFailure
            && denial.counters().checksum_posture_checks() > 0
        {
            Ok(Self::new(IntegrityCloseoutDenialBoundary::ForgedChecksum))
        } else {
            Err(PhysicalIntegrityCloseoutDenial::UnexecutedBoundaryDenial(
                IntegrityCloseoutDenialBoundary::ForgedChecksum,
            ))
        }
    }

    pub fn from_digest_as_checksum_denial(
        denial: ChecksumAlgorithmMismatchDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial == ChecksumAlgorithmMismatchDenial::DigestAsChecksumSubstitution {
            Ok(Self::new(IntegrityCloseoutDenialBoundary::DigestAsChecksum))
        } else {
            Err(PhysicalIntegrityCloseoutDenial::UnexecutedBoundaryDenial(
                IntegrityCloseoutDenialBoundary::DigestAsChecksum,
            ))
        }
    }

    pub fn from_checksum_authenticity_denial(
        denial: ChecksumAlgorithmMismatchDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial == ChecksumAlgorithmMismatchDenial::ChecksumAsAuthenticityClaim {
            Ok(Self::new(
                IntegrityCloseoutDenialBoundary::ChecksumAsAuthenticity,
            ))
        } else {
            Err(PhysicalIntegrityCloseoutDenial::UnexecutedBoundaryDenial(
                IntegrityCloseoutDenialBoundary::ChecksumAsAuthenticity,
            ))
        }
    }

    pub fn from_raw_byte_entry_denial(
        denial: IntegrityEntryDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial.kind() == IntegrityEntryDenialKind::MissingProtectedPhysicalByteView {
            Ok(Self::new(IntegrityCloseoutDenialBoundary::RawByteEntry))
        } else {
            Err(PhysicalIntegrityCloseoutDenial::UnexecutedBoundaryDenial(
                IntegrityCloseoutDenialBoundary::RawByteEntry,
            ))
        }
    }

    pub fn from_copied_quarantine_record_denial(
        denial: QuarantineSealDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if denial.kind() == QuarantineSealDenialKind::LaterLifecycleOwnerRequired {
            Ok(Self::new(
                IntegrityCloseoutDenialBoundary::CopiedQuarantineRecord,
            ))
        } else {
            Err(PhysicalIntegrityCloseoutDenial::UnexecutedBoundaryDenial(
                IntegrityCloseoutDenialBoundary::CopiedQuarantineRecord,
            ))
        }
    }

    pub fn from_over_budget_scrub_plan_denial(
        denial: ScrubPlanDenial,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        if matches!(
            denial.kind(),
            ScrubPlanDenialKind::ResidentMemoryLimitExceeded { .. }
                | ScrubPlanDenialKind::PinPageLimitExceeded { .. }
                | ScrubPlanDenialKind::AllocationLimitExceeded { .. }
                | ScrubPlanDenialKind::StreamingWindowLimitExceeded { .. }
                | ScrubPlanDenialKind::ProtectedReadLimitExceeded { .. }
        ) {
            Ok(Self::new(
                IntegrityCloseoutDenialBoundary::OverBudgetScrubPlan,
            ))
        } else {
            Err(PhysicalIntegrityCloseoutDenial::UnexecutedBoundaryDenial(
                IntegrityCloseoutDenialBoundary::OverBudgetScrubPlan,
            ))
        }
    }

    pub const fn boundary(self) -> IntegrityCloseoutDenialBoundary {
        self.boundary
    }

    const fn new(boundary: IntegrityCloseoutDenialBoundary) -> Self {
        Self { boundary }
    }
}
