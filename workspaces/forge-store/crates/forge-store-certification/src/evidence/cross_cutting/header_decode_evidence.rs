use crate::PhysicalSubstrateLane;
use forge_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use forge_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial,
    CanonicalSingleSequenceDigestAlgorithmSlot,
};
use forge_proof::TransitionOutcome;
use forge_store_contracts::StableArtifactId;
use forge_store_physical_format::{
    PhysicalHeaderDecodeCounterSnapshot, PhysicalHeaderDecodeReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalHeaderDecodeEvidenceRow {
    HeaderDecodeWitnessSealed,
    PayloadViewRequiresWitness,
    UnknownHeaderKindRejectedBeforePayload,
    UnsupportedHeaderVersionRejectedBeforePayload,
    HeaderLengthMismatchRejectedBeforePayload,
    HeaderReservedFieldMisuseRejected,
}

impl PhysicalHeaderDecodeEvidenceRow {
    pub const fn physical_format_required() -> [Self; 6] {
        [
            Self::HeaderDecodeWitnessSealed,
            Self::PayloadViewRequiresWitness,
            Self::UnknownHeaderKindRejectedBeforePayload,
            Self::UnsupportedHeaderVersionRejectedBeforePayload,
            Self::HeaderLengthMismatchRejectedBeforePayload,
            Self::HeaderReservedFieldMisuseRejected,
        ]
    }

    pub const fn physical_substrate_lane(self) -> PhysicalSubstrateLane {
        match self {
            Self::HeaderDecodeWitnessSealed | Self::PayloadViewRequiresWitness => {
                PhysicalSubstrateLane::HappyAuthority
            }
            Self::UnknownHeaderKindRejectedBeforePayload
            | Self::UnsupportedHeaderVersionRejectedBeforePayload
            | Self::HeaderLengthMismatchRejectedBeforePayload
            | Self::HeaderReservedFieldMisuseRejected => PhysicalSubstrateLane::HostileFormat,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalHeaderDecodeEvidenceReport {
    row: PhysicalHeaderDecodeEvidenceRow,
    lane: PhysicalSubstrateLane,
    artifact_id: StableArtifactId,
    foundational_basis: StableArtifactId,
    canonical_digest: CanonicalDerivedDigest,
    counters: PhysicalHeaderDecodeCounterSnapshot,
}

impl PhysicalHeaderDecodeEvidenceReport {
    pub fn from_decode_report(
        row: PhysicalHeaderDecodeEvidenceRow,
        report: PhysicalHeaderDecodeReport,
    ) -> Result<Self, PhysicalHeaderDecodeEvidenceDenial> {
        let counters = report.counters();
        require_header_decode_attempt(counters)?;
        let canonical_digest = derive_header_report_digest(report)?;
        Ok(Self {
            row,
            lane: row.physical_substrate_lane(),
            artifact_id: StableArtifactId::new("forge_store.header_decode.for_canonical_physical_format")
                .expect("static artifact id"),
            foundational_basis: StableArtifactId::new("forge_foundational.canonical_bytes")
                .expect("static artifact id"),
            canonical_digest,
            counters,
        })
    }

    pub const fn row(&self) -> PhysicalHeaderDecodeEvidenceRow {
        self.row
    }

    pub const fn lane(&self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn artifact_id(&self) -> &StableArtifactId {
        &self.artifact_id
    }

    pub const fn foundational_basis(&self) -> &StableArtifactId {
        &self.foundational_basis
    }

    pub const fn canonical_digest(&self) -> &CanonicalDerivedDigest {
        &self.canonical_digest
    }

    pub const fn counters(&self) -> PhysicalHeaderDecodeCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalHeaderDecodeEvidenceDenial {
    MissingHeaderDecodeAttempt,
    CanonicalBasisDenied(CanonicalBasisConstructionDenial),
    CanonicalDigestDenied(CanonicalDigestDerivationDenial),
}

fn require_header_decode_attempt(
    counters: PhysicalHeaderDecodeCounterSnapshot,
) -> Result<(), PhysicalHeaderDecodeEvidenceDenial> {
    if counters.header_decode_attempt_count() != 1 {
        return Err(PhysicalHeaderDecodeEvidenceDenial::MissingHeaderDecodeAttempt);
    }
    Ok(())
}

fn derive_header_report_digest(
    report: PhysicalHeaderDecodeReport,
) -> Result<CanonicalDerivedDigest, PhysicalHeaderDecodeEvidenceDenial> {
    let witness = report.witness();
    let counters = report.counters();
    let entries = [
        basis_entry(
            0,
            witness.kind().tag() as u128,
            CanonicalIntegerWidth::Bits8,
        ),
        basis_entry(
            1,
            witness.payload_length() as u128,
            CanonicalIntegerWidth::Bits32,
        ),
        basis_entry(
            2,
            witness.publication().code() as u128,
            CanonicalIntegerWidth::Bits8,
        ),
        basis_entry(
            3,
            counters.header_decode_attempt_count() as u128,
            CanonicalIntegerWidth::Bits32,
        ),
    ];
    let version = header_decode_rule_version();
    let domain = header_decode_domain();
    let sequence = match prepare_canonical_basis_sequence(version.clone(), domain, entries) {
        TransitionOutcome::Success(sequence) => sequence,
        TransitionOutcome::Denied(denial) => {
            return Err(PhysicalHeaderDecodeEvidenceDenial::CanonicalBasisDenied(
                denial,
            ));
        }
        _ => unreachable!("canonical basis preparation only returns success or denial"),
    };
    let digest_slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
        CanonicalDigestAlgorithmId::test_stable_fixture(),
        domain,
        version,
    );
    let digest_ready = match admit_canonical_sequence_digest_derivation(sequence, digest_slot) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            return Err(PhysicalHeaderDecodeEvidenceDenial::CanonicalDigestDenied(
                denial,
            ));
        }
        _ => unreachable!("canonical digest admission only returns success or denial"),
    };
    Ok(derive_canonical_digest(digest_ready))
}

fn basis_entry(ordinal: u32, value: u128, width: CanonicalIntegerWidth) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        header_decode_domain(),
        CanonicalBasisLocus::EntryOrdinal(ordinal),
        CanonicalBasisEntryKind::Future("physical-header-decode"),
        CanonicalBasisValue::UnsignedInteger { width, value },
    )
}

fn header_decode_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("forge-store.header-decode.for_canonical_physical_format.v1")
        .expect("static header decode canonicalization rule version")
}

const fn header_decode_domain() -> CanonicalBasisDomain {
    CanonicalBasisDomain::Future("forge-store.header-decode.for_canonical_physical_format")
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_store_physical_format::{
        PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
        PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId,
        PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority,
        PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
    };

    #[test]
    fn every_header_decode_evidence_row_maps_to_physical_substrate() {
        for row in PhysicalHeaderDecodeEvidenceRow::physical_format_required() {
            assert_eq!(
                row.physical_substrate_lane().family().as_str(),
                "physical_substrate"
            );
        }
    }

    #[test]
    fn header_decode_evidence_exports_real_report_at_foundational_boundary() {
        let report = decoded_frame_header_report();
        let evidence = PhysicalHeaderDecodeEvidenceReport::from_decode_report(
            PhysicalHeaderDecodeEvidenceRow::HeaderDecodeWitnessSealed,
            report,
        )
        .unwrap();

        assert_eq!(
            evidence.artifact_id(),
            &StableArtifactId::new("forge_store.header_decode.for_canonical_physical_format").unwrap()
        );
        assert_eq!(
            evidence.foundational_basis(),
            &StableArtifactId::new("forge_foundational.canonical_bytes").unwrap()
        );
        assert_eq!(evidence.counters().header_decode_attempt_count(), 1);
        assert_eq!(evidence.lane(), PhysicalSubstrateLane::HappyAuthority);
    }

    fn decoded_frame_header_report() -> forge_store_physical_format::PhysicalHeaderDecodeReport {
        let authority =
            PhysicalHeaderAuthority::for_canonical_physical_format(PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap());
        authority
            .decode_frame_header(
                validated_slot_reference(),
                &header_bytes(PhysicalFrameKind::RecordFrame.tag(), 3, b"abc"),
                PhysicalFrameKind::RecordFrame,
            )
            .unwrap()
    }

    fn validated_slot_reference() -> forge_store_physical_format::PhysicalReferenceValidationWitness
    {
        let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
        let references = PhysicalReferenceAuthority::for_canonical_physical_format();
        let cell = generations
            .slot_cell(segment(1), page(2), slot(3))
            .with_slot_generation(generation(3));
        references
            .validate_page_slot(references.admit_page_slot(cell), cell)
            .unwrap()
    }

    fn header_bytes(kind_tag: u8, generation_value: u64, payload: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
        bytes.push(kind_tag);
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&generation_value.to_le_bytes());
        bytes.push(PhysicalPublicationState::Published.code());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u64.to_le_bytes());
        bytes.extend_from_slice(payload);
        bytes
    }

    fn segment(value: u64) -> PhysicalSegmentId {
        PhysicalSegmentId::from_raw(value).unwrap()
    }

    fn page(value: u64) -> PhysicalPageId {
        PhysicalPageId::from_raw(value).unwrap()
    }

    fn slot(value: u16) -> PhysicalRecordSlot {
        PhysicalRecordSlot::from_raw(value).unwrap()
    }

    fn generation(value: u64) -> PhysicalGeneration {
        PhysicalGeneration::from_raw(value).unwrap()
    }
}
