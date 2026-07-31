use crate::{ChecksumAlgorithmMismatchDenial, ChecksumCoverageBasis};
use worth_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalIntegerWidth,
    CanonicalizationRuleVersion,
};
use worth_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalSingleSequenceDigestAlgorithmSlot,
};
use worth_proof::TransitionOutcome;
use worth_store_physical_format::{
    ChecksumCompatibilityFieldPosture, ChecksumCoverageEncoding, ChecksumFieldHandling,
    ChecksumGenerationFieldPosture, ChecksumHeaderField, ChecksumLengthFieldPosture,
    ChecksumPaddingPosture, ChecksumPayloadRegion, ChecksumReservedFieldPosture,
    ChecksumUnknownFieldPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalChecksumEvidenceIdentity {
    digest: CanonicalDerivedDigest,
}

impl FoundationalChecksumEvidenceIdentity {
    pub const fn digest(&self) -> &CanonicalDerivedDigest {
        &self.digest
    }
}

pub(crate) fn foundational_identity_for_checksum_basis(
    basis: &ChecksumCoverageBasis,
) -> Result<FoundationalChecksumEvidenceIdentity, ChecksumAlgorithmMismatchDenial> {
    let version = checksum_basis_rule_version()?;
    let domain = CanonicalBasisDomain::Future("store.new.checksum.declaration");
    let ready =
        match prepare_canonical_basis_sequence(version.clone(), domain, basis_entries(basis)) {
            TransitionOutcome::Success(ready) => ready,
            _ => return Err(ChecksumAlgorithmMismatchDenial::FoundationalEvidenceDenied),
        };
    let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
        CanonicalDigestAlgorithmId::sha256(),
        domain,
        version,
    );
    let digest_ready = match admit_canonical_sequence_digest_derivation(ready, slot) {
        TransitionOutcome::Success(ready) => ready,
        _ => return Err(ChecksumAlgorithmMismatchDenial::FoundationalEvidenceDenied),
    };
    Ok(FoundationalChecksumEvidenceIdentity {
        digest: derive_canonical_digest(digest_ready),
    })
}

fn checksum_basis_rule_version(
) -> Result<CanonicalizationRuleVersion, ChecksumAlgorithmMismatchDenial> {
    CanonicalizationRuleVersion::new("store.new.checksum.declaration.v1")
        .ok_or(ChecksumAlgorithmMismatchDenial::FoundationalEvidenceDenied)
}

fn basis_entries(basis: &ChecksumCoverageBasis) -> Vec<CanonicalBasisEntry> {
    let map = basis.coverage_map();
    vec![
        text_entry("algorithm_id", basis.algorithm_id().as_str()),
        u16_entry("format_version", basis.physical_format_version().value()),
        text_entry(
            "coverage_encoding",
            coverage_encoding_token(map.coverage_encoding()),
        ),
        text_entry(
            "covered_header_fields",
            &header_field_tokens(map.covered_header_fields()),
        ),
        text_entry(
            "excluded_header_fields",
            &header_field_tokens(map.excluded_header_fields()),
        ),
        text_entry(
            "checksum_field_handling",
            checksum_field_handling_token(map.checksum_field_handling()),
        ),
        text_entry(
            "mutable_publication_fields",
            &header_field_tokens(map.mutable_publication_fields()),
        ),
        text_entry(
            "reserved_fields",
            reserved_fields_token(map.reserved_fields()),
        ),
        text_entry(
            "generation_fields",
            generation_fields_token(map.generation_fields()),
        ),
        text_entry("length_fields", length_fields_token(map.length_fields())),
        text_entry("payload_region", payload_region_token(map.payload_region())),
        text_entry("padding_bytes", padding_bytes_token(map.padding_bytes())),
        text_entry(
            "compatibility_fields",
            compatibility_fields_token(map.compatibility_fields()),
        ),
        text_entry(
            "unknown_field_posture",
            unknown_field_posture_token(map.unknown_field_posture()),
        ),
        text_entry("corruption_class", "accidental-physical-byte-corruption"),
        text_entry("collision_posture", "non-cryptographic-collision-possible"),
        text_entry("authenticity_posture", "does-not-prove-authenticity"),
        text_entry("authorization_posture", "does-not-prove-authorization"),
    ]
}

fn text_entry(locus: &'static str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Future("store.new.checksum.declaration"),
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Future("checksum-declaration-field"),
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

fn u16_entry(locus: &'static str, value: u16) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Future("store.new.checksum.declaration"),
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Future("checksum-declaration-field"),
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits16,
            value: u128::from(value),
        },
    )
}

fn header_field_tokens(fields: &[ChecksumHeaderField]) -> String {
    let mut tokens = fields
        .iter()
        .copied()
        .map(header_field_token)
        .collect::<Vec<_>>();
    tokens.sort_unstable();
    tokens.join(",")
}

fn header_field_token(field: ChecksumHeaderField) -> &'static str {
    match field {
        ChecksumHeaderField::Magic => "magic",
        ChecksumHeaderField::FormatVersion => "format-version",
        ChecksumHeaderField::HeaderLength => "header-length",
        ChecksumHeaderField::HeaderKind => "header-kind",
        ChecksumHeaderField::Generation => "generation",
        ChecksumHeaderField::PublicationState => "publication-state",
        ChecksumHeaderField::PayloadLength => "payload-length",
        ChecksumHeaderField::ReservedFields => "reserved-fields",
        ChecksumHeaderField::ChecksumField => "checksum-field",
        ChecksumHeaderField::CompatibilityFields => "compatibility-fields",
    }
}

fn checksum_field_handling_token(value: ChecksumFieldHandling) -> &'static str {
    match value {
        ChecksumFieldHandling::ExcludedDuringComputation => "excluded-during-computation",
    }
}

fn reserved_fields_token(value: ChecksumReservedFieldPosture) -> &'static str {
    match value {
        ChecksumReservedFieldPosture::CoveredAsZeroedAndPreserved => {
            "covered-as-zeroed-and-preserved"
        }
    }
}

fn generation_fields_token(value: ChecksumGenerationFieldPosture) -> &'static str {
    match value {
        ChecksumGenerationFieldPosture::CoveredAsPhysicalGeneration => {
            "covered-as-physical-generation"
        }
    }
}

fn length_fields_token(value: ChecksumLengthFieldPosture) -> &'static str {
    match value {
        ChecksumLengthFieldPosture::CoveredAsSerializedPayloadLength => {
            "covered-as-serialized-payload-length"
        }
    }
}

fn payload_region_token(value: ChecksumPayloadRegion) -> &'static str {
    match value {
        ChecksumPayloadRegion::SerializedPayloadBytes => "serialized-payload-bytes",
    }
}

fn padding_bytes_token(value: ChecksumPaddingPosture) -> &'static str {
    match value {
        ChecksumPaddingPosture::ExcludedAndMustRemainZeroed => "excluded-and-must-remain-zeroed",
    }
}

fn compatibility_fields_token(value: ChecksumCompatibilityFieldPosture) -> &'static str {
    match value {
        ChecksumCompatibilityFieldPosture::CoveredAndDenyUnknown => "covered-and-deny-unknown",
    }
}

fn unknown_field_posture_token(value: ChecksumUnknownFieldPosture) -> &'static str {
    match value {
        ChecksumUnknownFieldPosture::DenyUntilReadmitted => "deny-until-readmitted",
    }
}

fn coverage_encoding_token(value: ChecksumCoverageEncoding) -> &'static str {
    match value {
        ChecksumCoverageEncoding::SerializedBytes => "serialized-bytes",
        ChecksumCoverageEncoding::CanonicalizedFields => "canonicalized-fields",
    }
}
