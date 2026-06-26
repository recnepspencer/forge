use crate::PhysicalSubstrateLane;
use forge_store_physical_format::PhysicalReferenceValidationCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIdentityEvidenceRow {
    SlotGenerationOwnership,
    ExtentGenerationOwnership,
    FreeSpaceReuseGenerationOwnership,
    RootPublicationGenerationOwnership,
    PageGenerationVocabulary,
    SegmentGenerationVocabulary,
    StaleSlotReferenceDeniedBeforeDecode,
    StaleExtentReferenceDeniedBeforeDecode,
    StaleFreeSpaceReferenceDeniedBeforeDecode,
    StaleRootPublicationReferenceDeniedBeforeDecode,
    ForgedReferenceConstructionRejected,
    ForgedAdmissionWitnessRejected,
}

impl PhysicalIdentityEvidenceRow {
    pub const fn s1_required() -> [Self; 12] {
        [
            Self::SlotGenerationOwnership,
            Self::ExtentGenerationOwnership,
            Self::FreeSpaceReuseGenerationOwnership,
            Self::RootPublicationGenerationOwnership,
            Self::PageGenerationVocabulary,
            Self::SegmentGenerationVocabulary,
            Self::StaleSlotReferenceDeniedBeforeDecode,
            Self::StaleExtentReferenceDeniedBeforeDecode,
            Self::StaleFreeSpaceReferenceDeniedBeforeDecode,
            Self::StaleRootPublicationReferenceDeniedBeforeDecode,
            Self::ForgedReferenceConstructionRejected,
            Self::ForgedAdmissionWitnessRejected,
        ]
    }

    pub const fn physical_substrate_lane(self) -> PhysicalSubstrateLane {
        match self {
            Self::SlotGenerationOwnership
            | Self::ExtentGenerationOwnership
            | Self::FreeSpaceReuseGenerationOwnership
            | Self::RootPublicationGenerationOwnership
            | Self::PageGenerationVocabulary
            | Self::SegmentGenerationVocabulary => PhysicalSubstrateLane::HappyAuthority,
            Self::StaleSlotReferenceDeniedBeforeDecode
            | Self::StaleExtentReferenceDeniedBeforeDecode
            | Self::StaleFreeSpaceReferenceDeniedBeforeDecode
            | Self::StaleRootPublicationReferenceDeniedBeforeDecode
            | Self::ForgedReferenceConstructionRejected
            | Self::ForgedAdmissionWitnessRejected => PhysicalSubstrateLane::HostileReference,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIdentityEvidenceReport {
    row: PhysicalIdentityEvidenceRow,
    lane: PhysicalSubstrateLane,
    counters: PhysicalReferenceValidationCounterSnapshot,
}

impl PhysicalIdentityEvidenceReport {
    pub fn from_reference_validation(
        row: PhysicalIdentityEvidenceRow,
        counters: PhysicalReferenceValidationCounterSnapshot,
    ) -> Result<Self, PhysicalIdentityEvidenceDenial> {
        require_reference_validation_attempt(counters)?;
        require_row_counter(row, counters)?;
        Ok(Self {
            row,
            lane: row.physical_substrate_lane(),
            counters,
        })
    }

    pub const fn row(&self) -> PhysicalIdentityEvidenceRow {
        self.row
    }

    pub const fn lane(&self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn counters(&self) -> PhysicalReferenceValidationCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIdentityEvidenceDenial {
    MissingReferenceValidationAttempt,
    MissingExpectedFamilyCounter,
    MissingStaleGenerationCounter,
}

fn require_reference_validation_attempt(
    counters: PhysicalReferenceValidationCounterSnapshot,
) -> Result<(), PhysicalIdentityEvidenceDenial> {
    if counters.validation_attempt_count() != 1 {
        return Err(PhysicalIdentityEvidenceDenial::MissingReferenceValidationAttempt);
    }
    Ok(())
}

fn require_row_counter(
    row: PhysicalIdentityEvidenceRow,
    counters: PhysicalReferenceValidationCounterSnapshot,
) -> Result<(), PhysicalIdentityEvidenceDenial> {
    let family_count = match row {
        PhysicalIdentityEvidenceRow::StaleSlotReferenceDeniedBeforeDecode => {
            counters.page_slot_validation_count()
        }
        PhysicalIdentityEvidenceRow::StaleExtentReferenceDeniedBeforeDecode => {
            counters.extent_validation_count()
        }
        PhysicalIdentityEvidenceRow::StaleFreeSpaceReferenceDeniedBeforeDecode => {
            counters.free_space_reuse_validation_count()
        }
        PhysicalIdentityEvidenceRow::StaleRootPublicationReferenceDeniedBeforeDecode => {
            counters.root_publication_validation_count()
        }
        _ => 1,
    };
    if family_count != 1 {
        return Err(PhysicalIdentityEvidenceDenial::MissingExpectedFamilyCounter);
    }
    if matches!(
        row,
        PhysicalIdentityEvidenceRow::StaleSlotReferenceDeniedBeforeDecode
            | PhysicalIdentityEvidenceRow::StaleExtentReferenceDeniedBeforeDecode
            | PhysicalIdentityEvidenceRow::StaleFreeSpaceReferenceDeniedBeforeDecode
            | PhysicalIdentityEvidenceRow::StaleRootPublicationReferenceDeniedBeforeDecode
    ) && counters.stale_generation_rejection_count() != 1
    {
        return Err(PhysicalIdentityEvidenceDenial::MissingStaleGenerationCounter);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        PhysicalIdentityEvidenceDenial, PhysicalIdentityEvidenceReport, PhysicalIdentityEvidenceRow,
    };
    use crate::PhysicalSubstrateLane;
    use forge_store_physical_format::{
        PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
        PhysicalReferenceAuthority, PhysicalSegmentId,
    };

    #[test]
    fn every_identity_generation_row_maps_to_physical_substrate_lane() {
        for row in PhysicalIdentityEvidenceRow::s1_required() {
            assert_eq!(
                row.physical_substrate_lane().family().as_str(),
                "physical_substrate"
            );
        }
    }

    #[test]
    fn stale_reference_rows_are_hostile_reference_evidence() {
        let rows = [
            PhysicalIdentityEvidenceRow::StaleSlotReferenceDeniedBeforeDecode,
            PhysicalIdentityEvidenceRow::StaleExtentReferenceDeniedBeforeDecode,
            PhysicalIdentityEvidenceRow::StaleFreeSpaceReferenceDeniedBeforeDecode,
            PhysicalIdentityEvidenceRow::StaleRootPublicationReferenceDeniedBeforeDecode,
        ];

        for row in rows {
            assert_eq!(
                row.physical_substrate_lane(),
                PhysicalSubstrateLane::HostileReference
            );
        }
    }

    #[test]
    fn stale_identity_evidence_requires_exact_counter_report() {
        let counters = stale_slot_counter_report();
        let report = PhysicalIdentityEvidenceReport::from_reference_validation(
            PhysicalIdentityEvidenceRow::StaleSlotReferenceDeniedBeforeDecode,
            counters,
        )
        .unwrap();

        assert_eq!(
            report.row(),
            PhysicalIdentityEvidenceRow::StaleSlotReferenceDeniedBeforeDecode
        );
        assert_eq!(report.lane(), PhysicalSubstrateLane::HostileReference);
        assert_eq!(report.counters().page_slot_validation_count(), 1);
        assert_eq!(report.counters().stale_generation_rejection_count(), 1);
    }

    #[test]
    fn stale_identity_evidence_rejects_declaration_without_stale_counter() {
        let counters = forge_store_physical_format::PhysicalReferenceValidationCounterSnapshot::for_page_slot_attempt()
            .with_generation_check();

        let denial = PhysicalIdentityEvidenceReport::from_reference_validation(
            PhysicalIdentityEvidenceRow::StaleSlotReferenceDeniedBeforeDecode,
            counters,
        )
        .unwrap_err();

        assert_eq!(
            denial,
            PhysicalIdentityEvidenceDenial::MissingStaleGenerationCounter
        );
    }

    fn stale_slot_counter_report(
    ) -> forge_store_physical_format::PhysicalReferenceValidationCounterSnapshot {
        let generations = PhysicalGenerationAuthority::s1();
        let references = PhysicalReferenceAuthority::s1();
        let admitted = references.admit_page_slot(
            generations
                .slot_cell(segment(7), page(11), slot(3))
                .with_slot_generation(generation(9)),
        );
        let reused_cell = generations
            .slot_cell(segment(7), page(11), slot(3))
            .with_slot_generation(generation(10));

        references
            .validate_page_slot(admitted, reused_cell)
            .unwrap_err()
            .counters()
    }

    fn segment(value: u64) -> PhysicalSegmentId {
        PhysicalSegmentId::from_raw(value).expect("test segment id is non-zero")
    }

    fn page(value: u64) -> PhysicalPageId {
        PhysicalPageId::from_raw(value).expect("test page id is non-zero")
    }

    fn slot(value: u16) -> PhysicalRecordSlot {
        PhysicalRecordSlot::from_raw(value).expect("test slot is non-zero")
    }

    fn generation(value: u64) -> PhysicalGeneration {
        PhysicalGeneration::from_raw(value).expect("test generation is non-zero")
    }
}
