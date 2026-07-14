use worth_store_buffer_pool::{
    EvictionCounterSnapshot, EvictionPlan, EvictionReceipt, ResidentFrameDenial,
    ResidentFrameDenialKind, ResidentFrameTable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvictionProtectionEvidenceReport {
    row: EvictionProtectionEvidenceRow,
    counters: EvictionCounterSnapshot,
}

impl EvictionProtectionEvidenceReport {
    pub fn from_plan(
        row: EvictionProtectionEvidenceRow,
        plan: &EvictionPlan,
    ) -> Result<Self, EvictionProtectionEvidenceDenial> {
        row.prove_plan(plan)?;
        Ok(Self {
            row,
            counters: plan.counters(),
        })
    }

    pub fn from_scan_bound(
        table: &ResidentFrameTable,
        plan: &EvictionPlan,
    ) -> Result<Self, EvictionProtectionEvidenceDenial> {
        let scanned = plan.candidate_set().resident_frames_scanned();
        if scanned == 0 || scanned > table.resident_frame_count() {
            return Err(EvictionProtectionEvidenceDenial::UnprovenEvictionRow);
        }
        Ok(Self {
            row: EvictionProtectionEvidenceRow::CandidateScanBoundedByResidentFrameCount,
            counters: plan.counters(),
        })
    }

    pub fn from_denial(
        row: EvictionProtectionEvidenceRow,
        denial: ResidentFrameDenial,
    ) -> Result<Self, EvictionProtectionEvidenceDenial> {
        row.prove_denial(denial)?;
        let protected = denial
            .protected_frame_denial()
            .ok_or(EvictionProtectionEvidenceDenial::UnprovenEvictionRow)?;
        Ok(Self {
            row,
            counters: protected.counters(),
        })
    }

    pub fn from_receipt(
        receipt: EvictionReceipt,
    ) -> Result<Self, EvictionProtectionEvidenceDenial> {
        if receipt.proves_durability() {
            return Err(EvictionProtectionEvidenceDenial::DurabilityClaimRejected);
        }
        if receipt.evicted_frame_count() == 0
            || receipt.released_resident_bytes().as_bytes() == 0
            || receipt.counters().receipt_count() == 0
        {
            return Err(EvictionProtectionEvidenceDenial::UnprovenEvictionRow);
        }
        Ok(Self {
            row: EvictionProtectionEvidenceRow::EvictionReceiptObserved,
            counters: receipt.counters(),
        })
    }

    pub const fn row(self) -> EvictionProtectionEvidenceRow {
        self.row
    }

    pub const fn counters(self) -> EvictionCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionProtectionEvidenceRow {
    CandidateSetFromResidentFrameTable,
    ProtectedFramesExcludedBeforePolicyRanking,
    AllProtectedResidentSetDeniedWithReasons,
    CandidateScanBoundedByResidentFrameCount,
    EvictionReceiptObserved,
}

impl EvictionProtectionEvidenceRow {
    fn prove_plan(self, plan: &EvictionPlan) -> Result<(), EvictionProtectionEvidenceDenial> {
        let candidates = plan.candidate_set();
        match self {
            Self::CandidateSetFromResidentFrameTable
                if candidates.resident_frames_scanned() > 0
                    && candidates.candidate_count() > 0
                    && !candidates.includes_protected_frames() =>
            {
                Ok(())
            }
            Self::ProtectedFramesExcludedBeforePolicyRanking
                if candidates.protected_exclusions().total_protected_reasons() > 0
                    && candidates.policy_rank_count() == 1
                    && !candidates.includes_protected_frames() =>
            {
                Ok(())
            }
            Self::CandidateSetFromResidentFrameTable
            | Self::ProtectedFramesExcludedBeforePolicyRanking => {
                Err(EvictionProtectionEvidenceDenial::UnprovenEvictionRow)
            }
            Self::AllProtectedResidentSetDeniedWithReasons
            | Self::CandidateScanBoundedByResidentFrameCount
            | Self::EvictionReceiptObserved => Err(EvictionProtectionEvidenceDenial::WrongRow),
        }
    }

    fn prove_denial(
        self,
        denial: ResidentFrameDenial,
    ) -> Result<(), EvictionProtectionEvidenceDenial> {
        match self {
            Self::AllProtectedResidentSetDeniedWithReasons
                if denial.kind() == ResidentFrameDenialKind::AllEvictionCandidatesProtected =>
            {
                let protected = denial
                    .protected_frame_denial()
                    .ok_or(EvictionProtectionEvidenceDenial::UnprovenEvictionRow)?;
                if protected.reasons().total_protected_reasons() == 0
                    || protected.counters().all_protected_denial_count() == 0
                    || protected.counters().policy_rank_count() != 0
                {
                    return Err(EvictionProtectionEvidenceDenial::UnprovenEvictionRow);
                }
                Ok(())
            }
            Self::AllProtectedResidentSetDeniedWithReasons => {
                Err(EvictionProtectionEvidenceDenial::DenialMismatch)
            }
            Self::CandidateSetFromResidentFrameTable
            | Self::ProtectedFramesExcludedBeforePolicyRanking
            | Self::CandidateScanBoundedByResidentFrameCount
            | Self::EvictionReceiptObserved => Err(EvictionProtectionEvidenceDenial::WrongRow),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionProtectionEvidenceDenial {
    WrongRow,
    DenialMismatch,
    DurabilityClaimRejected,
    UnprovenEvictionRow,
}
