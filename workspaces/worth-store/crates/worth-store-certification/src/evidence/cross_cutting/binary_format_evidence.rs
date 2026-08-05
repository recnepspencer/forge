use worth_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use worth_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial,
    CanonicalSingleSequenceDigestAlgorithmSlot,
};
use worth_proof::TransitionOutcome;
use worth_store_contracts::StableArtifactId;
use worth_store_physical_format::{PhysicalBinaryEncodingWitness, PhysicalFormatIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryPhysicalFormatEvidence {
    artifact_id: StableArtifactId,
    foundational_basis: StableArtifactId,
    canonical_digest: CanonicalDerivedDigest,
    format_identity: PhysicalFormatIdentity,
}

impl BinaryPhysicalFormatEvidence {
    pub fn from_witness(
        witness: &PhysicalBinaryEncodingWitness,
    ) -> Result<Self, BinaryPhysicalFormatEvidenceDenial> {
        let canonical_digest = derive_witness_canonical_digest(witness)?;
        Ok(Self {
            artifact_id: StableArtifactId::new(
                "worth_store.binary_format.for_canonical_physical_format",
            )
            .expect("static artifact id"),
            foundational_basis: StableArtifactId::new("worth_foundational.canonical_bytes")
                .expect("static artifact id"),
            canonical_digest,
            format_identity: witness.format_identity(),
        })
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

    pub const fn format_identity(&self) -> PhysicalFormatIdentity {
        self.format_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryPhysicalFormatEvidenceDenial {
    CanonicalBasisDenied(CanonicalBasisConstructionDenial),
    CanonicalDigestDenied(CanonicalDigestDerivationDenial),
}

fn derive_witness_canonical_digest(
    witness: &PhysicalBinaryEncodingWitness,
) -> Result<CanonicalDerivedDigest, BinaryPhysicalFormatEvidenceDenial> {
    let version = binary_format_rule_version();
    let domain = binary_format_domain();
    let entries = witness
        .encode_golden_format_header()
        .into_iter()
        .enumerate()
        .map(binary_format_byte_entry);

    let sequence = match prepare_canonical_basis_sequence(version.clone(), domain, entries) {
        TransitionOutcome::Success(sequence) => sequence,
        TransitionOutcome::Denied(denial) => {
            return Err(BinaryPhysicalFormatEvidenceDenial::CanonicalBasisDenied(
                denial,
            ));
        }
        _ => unreachable!("canonical basis preparation only returns success or denial"),
    };

    let digest_slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
        CanonicalDigestAlgorithmId::sha256(),
        domain,
        version,
    );
    let digest_ready = match admit_canonical_sequence_digest_derivation(sequence, digest_slot) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            return Err(BinaryPhysicalFormatEvidenceDenial::CanonicalDigestDenied(
                denial,
            ));
        }
        _ => unreachable!("canonical digest admission only returns success or denial"),
    };

    Ok(derive_canonical_digest(digest_ready))
}

fn binary_format_byte_entry((offset, byte): (usize, u8)) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        binary_format_domain(),
        CanonicalBasisLocus::EntryOrdinal(offset as u32),
        CanonicalBasisEntryKind::Future("binary-format-golden-byte"),
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits8,
            value: byte as u128,
        },
    )
}

fn binary_format_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("worth-store.binary-format.for_canonical_physical_format.v1")
        .expect("static binary format canonicalization rule version")
}

const fn binary_format_domain() -> CanonicalBasisDomain {
    CanonicalBasisDomain::Future("worth-store.binary-format.for_canonical_physical_format")
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_store_physical_format::{
        PhysicalAlignmentClass, PhysicalBinaryEncodingWitness, PhysicalByteOrder,
        PhysicalFieldWidth, PhysicalFormatDeclaration, PhysicalFormatMagic, PhysicalFormatVersion,
        PhysicalForwardCompatibilityPolicy, PhysicalGoldenFormatHeaderFixture,
        PhysicalPageSizeClass, PhysicalReservedFieldPolicy,
    };

    #[test]
    fn certification_records_canonical_basis_only_at_evidence_boundary() {
        let witness = PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap();
        let evidence = BinaryPhysicalFormatEvidence::from_witness(&witness).unwrap();

        assert_eq!(
            evidence.artifact_id(),
            &StableArtifactId::new("worth_store.binary_format.for_canonical_physical_format")
                .unwrap()
        );
        assert_eq!(
            evidence.foundational_basis(),
            &StableArtifactId::new("worth_foundational.canonical_bytes").unwrap()
        );
        assert_eq!(
            evidence.format_identity().page_size(),
            PhysicalPageSizeClass::KiB16
        );
        assert_eq!(
            evidence.canonical_digest().metadata().entry_count(),
            PhysicalGoldenFormatHeaderFixture::physical_format_canonical().len() as u32
        );
    }

    #[test]
    fn certification_digest_changes_when_admitted_format_bytes_change() {
        let kib16_witness = PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap();
        let kib32_witness = witness_for_page_size(PhysicalPageSizeClass::KiB32);

        let kib16_evidence = BinaryPhysicalFormatEvidence::from_witness(&kib16_witness).unwrap();
        let kib32_evidence = BinaryPhysicalFormatEvidence::from_witness(&kib32_witness).unwrap();

        assert_ne!(
            kib16_evidence.canonical_digest().value().bytes(),
            kib32_evidence.canonical_digest().value().bytes()
        );
    }

    fn witness_for_page_size(page_size: PhysicalPageSizeClass) -> PhysicalBinaryEncodingWitness {
        let declaration = PhysicalFormatDeclaration::builder()
            .magic(PhysicalFormatMagic::store_format_magic())
            .version(PhysicalFormatVersion::initial_format_version())
            .byte_order(PhysicalByteOrder::LittleEndian)
            .field_width(PhysicalFieldWidth::segment_id_u64())
            .field_width(PhysicalFieldWidth::page_id_u64())
            .field_width(PhysicalFieldWidth::generation_u64())
            .field_width(PhysicalFieldWidth::header_length_u16())
            .field_width(PhysicalFieldWidth::payload_length_u32())
            .page_size(page_size)
            .alignment(PhysicalAlignmentClass::page_start_4k())
            .alignment(PhysicalAlignmentClass::frame_start_8())
            .alignment(PhysicalAlignmentClass::slot_directory_offset_8())
            .alignment(PhysicalAlignmentClass::extent_start_4k())
            .alignment(PhysicalAlignmentClass::manifest_record_8())
            .reserved_field_policy(PhysicalReservedFieldPolicy::zeroed_and_preserved())
            .forward_compatibility(PhysicalForwardCompatibilityPolicy::reject_unknown_kind())
            .define()
            .unwrap();
        PhysicalBinaryEncodingWitness::admit(declaration).unwrap()
    }
}
