use forge_store_contracts::StableDigest;
use forge_store_physical_format::{
    physical_format_required_covered_header_fields, ChecksumCompatibilityFieldPosture,
    ChecksumCoverageAuthoritySource, ChecksumCoverageEncoding, ChecksumCoverageMap,
    ChecksumCoverageMapDenial, ChecksumCoverageRegion, ChecksumFieldHandling,
    ChecksumGenerationFieldPosture, ChecksumHeaderField, ChecksumLengthFieldPosture,
    ChecksumPaddingPosture, ChecksumPayloadRegion, ChecksumReservedFieldPosture,
    ChecksumUnknownFieldPosture, PhysicalFormatDeclaration, PhysicalFormatVersion,
};
use forge_store_physical_integrity::{
    ChecksumAlgorithmClaim, ChecksumAlgorithmId, ChecksumAlgorithmMismatchDenial,
    ChecksumAuthenticityPosture, ChecksumAuthorizationPosture, ChecksumCollisionPosture,
    ChecksumCompatibilityPosture, ChecksumCorruptionClass, ChecksumScopeDeclaration,
};

#[test]
fn equivalent_checksum_declarations_share_basis_and_foundational_identity() {
    let first = declared_crc32c_with(ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap());
    let second = declared_crc32c_with(explicit_physical_format_coverage().unwrap());

    assert_eq!(first.coverage_basis(), second.coverage_basis());
    assert_eq!(
        first.foundational_evidence_identity(),
        second.foundational_evidence_identity()
    );
}

#[test]
fn unsupported_and_substitute_algorithm_claims_deny_before_inspection() {
    assert_eq!(
        ChecksumAlgorithmId::admit_claim(ChecksumAlgorithmClaim::declared_text("sha256")),
        Err(ChecksumAlgorithmMismatchDenial::UnknownAlgorithm)
    );
    assert_eq!(
        ChecksumAlgorithmId::crc64_nvme().require_matches(ChecksumAlgorithmId::crc32c()),
        Err(ChecksumAlgorithmMismatchDenial::AlgorithmIdMismatch)
    );
    let artifact_digest = StableDigest::new("sha256:not-a-physical-checksum").unwrap();
    assert_eq!(
        ChecksumAlgorithmId::admit_claim(ChecksumAlgorithmClaim::artifact_digest_substitution(
            &artifact_digest,
        )),
        Err(ChecksumAlgorithmMismatchDenial::DigestAsChecksumSubstitution)
    );
    assert_eq!(
        ChecksumAlgorithmId::admit_claim(ChecksumAlgorithmClaim::checksum_as_authenticity_claim()),
        Err(ChecksumAlgorithmMismatchDenial::ChecksumAsAuthenticityClaim)
    );
}

#[test]
fn missing_coverage_and_private_layout_coverage_deny() {
    let covered_without_generation = physical_format_required_covered_header_fields()
        .into_iter()
        .filter(|field| *field != ChecksumHeaderField::Generation);
    let missing_generation = explicit_physical_format_builder()
        .covered_header_fields(covered_without_generation)
        .excluded_header_fields([ChecksumHeaderField::ChecksumField])
        .define();

    assert_eq!(
        missing_generation,
        Err(ChecksumCoverageMapDenial::MissingRequiredHeaderField(
            ChecksumHeaderField::Generation,
        ))
    );

    assert_eq!(
        explicit_physical_format_builder()
            .authority_source(ChecksumCoverageAuthoritySource::SerdeMapOrder)
            .define(),
        Err(ChecksumCoverageMapDenial::SerializerOrderRejected)
    );
    assert_eq!(
        explicit_physical_format_builder()
            .authority_source(ChecksumCoverageAuthoritySource::RustStructLayout)
            .define(),
        Err(ChecksumCoverageMapDenial::RustLayoutRejected)
    );
}

#[test]
fn detection_model_names_required_checksum_scope_categories() {
    let declaration =
        declared_crc32c_with(ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap());
    let basis = declaration.coverage_basis();
    let map = basis.coverage_map();
    let model = basis.detection_model();

    assert_eq!(
        model.corruption_class(),
        ChecksumCorruptionClass::AccidentalPhysicalByteCorruption
    );
    assert_eq!(
        model.collision_posture(),
        ChecksumCollisionPosture::NonCryptographicCollisionPossible
    );
    assert_eq!(
        model.authenticity_posture(),
        ChecksumAuthenticityPosture::DoesNotProveAuthenticity
    );
    assert_eq!(
        model.authorization_posture(),
        ChecksumAuthorizationPosture::DoesNotProveAuthorization
    );
    assert!(map
        .covered_header_fields()
        .contains(&ChecksumHeaderField::Generation));
    assert!(map
        .covered_header_fields()
        .contains(&ChecksumHeaderField::PayloadLength));
    assert_eq!(
        map.excluded_header_fields(),
        &[ChecksumHeaderField::ChecksumField]
    );
    assert_eq!(
        map.checksum_field_handling(),
        ChecksumFieldHandling::ExcludedDuringComputation
    );
    assert_eq!(
        map.mutable_publication_fields(),
        &[ChecksumHeaderField::PublicationState]
    );
    assert_eq!(
        map.reserved_fields(),
        ChecksumReservedFieldPosture::CoveredAsZeroedAndPreserved
    );
    assert_eq!(
        map.generation_fields(),
        ChecksumGenerationFieldPosture::CoveredAsPhysicalGeneration
    );
    assert_eq!(
        map.length_fields(),
        ChecksumLengthFieldPosture::CoveredAsSerializedPayloadLength
    );
    assert_eq!(
        map.payload_region(),
        ChecksumPayloadRegion::SerializedPayloadBytes
    );
    assert_eq!(
        map.padding_bytes(),
        ChecksumPaddingPosture::ExcludedAndMustRemainZeroed
    );
    assert_eq!(
        map.compatibility_fields(),
        ChecksumCompatibilityFieldPosture::CoveredAndDenyUnknown
    );
    assert_eq!(
        map.unknown_field_posture(),
        ChecksumUnknownFieldPosture::DenyUntilReadmitted
    );
    assert_eq!(
        map.coverage_encoding(),
        ChecksumCoverageEncoding::SerializedBytes
    );
}

#[test]
fn coverage_map_answers_all_region_dispositions() {
    let map = ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap();

    assert_eq!(
        map.disposition_for_region(ChecksumCoverageRegion::HeaderField(
            ChecksumHeaderField::Generation,
        )),
        forge_store_physical_format::ChecksumCoverageDisposition::Covered
    );
    assert_eq!(
        map.disposition_for_region(ChecksumCoverageRegion::HeaderField(
            ChecksumHeaderField::ChecksumField,
        )),
        forge_store_physical_format::ChecksumCoverageDisposition::Excluded
    );
    assert_eq!(
        map.disposition_for_region(ChecksumCoverageRegion::HeaderField(
            ChecksumHeaderField::PublicationState,
        )),
        forge_store_physical_format::ChecksumCoverageDisposition::Preserved
    );
    assert_eq!(
        map.disposition_for_region(ChecksumCoverageRegion::PayloadRegion),
        forge_store_physical_format::ChecksumCoverageDisposition::Covered
    );
    assert_eq!(
        map.disposition_for_region(ChecksumCoverageRegion::CompatibilityFields),
        forge_store_physical_format::ChecksumCoverageDisposition::Covered
    );
    assert_eq!(
        map.disposition_for_region(ChecksumCoverageRegion::LaterPhysicalFamily),
        forge_store_physical_format::ChecksumCoverageDisposition::Skipped
    );
    assert_eq!(
        map.disposition_for_region(ChecksumCoverageRegion::UnknownFutureField),
        forge_store_physical_format::ChecksumCoverageDisposition::Denied
    );
}

#[test]
fn stronger_declared_algorithm_uses_its_own_detection_model_path() {
    let coverage = ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap();
    let declaration = declared_algorithm_with(ChecksumAlgorithmId::crc64_nvme(), coverage);

    assert_eq!(
        declaration.coverage_basis().algorithm_id(),
        ChecksumAlgorithmId::crc64_nvme()
    );
    assert_eq!(
        declaration.coverage_basis().detection_model(),
        ChecksumAlgorithmId::crc64_nvme().detection_model()
    );
}

#[test]
fn coverage_changes_require_explicit_readmission_instead_of_silent_reuse() {
    let declaration =
        declared_crc32c_with(ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap());
    let future_coverage = explicit_future_coverage().unwrap();
    let same_version_changed_encoding = explicit_physical_format_builder()
        .coverage_encoding(ChecksumCoverageEncoding::CanonicalizedFields)
        .define()
        .unwrap();

    assert_eq!(
        declaration.compatibility_posture_for_coverage(&future_coverage),
        ChecksumCompatibilityPosture::ExplicitReadmissionRequired
    );
    assert_eq!(
        declaration.compatibility_with_coverage(&future_coverage),
        Err(ChecksumAlgorithmMismatchDenial::CompatibilityReadmissionRequired)
    );
    assert_eq!(
        declaration.compatibility_with_coverage(declaration.coverage_basis().coverage_map()),
        Ok(ChecksumCompatibilityPosture::SameCoverageReused)
    );
    assert_eq!(
        declaration.compatibility_with_coverage(&same_version_changed_encoding),
        Err(ChecksumAlgorithmMismatchDenial::CompatibilityReadmissionRequired)
    );
}

#[test]
fn foundational_identity_changes_with_algorithm_or_coverage_basis() {
    let crc32c = declared_crc32c_with(ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap());
    let crc64 = declared_algorithm_with(
        ChecksumAlgorithmId::crc64_nvme(),
        ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap(),
    );
    let canonicalized_s1_coverage = explicit_physical_format_builder()
        .coverage_encoding(ChecksumCoverageEncoding::CanonicalizedFields)
        .define()
        .unwrap();
    let changed_coverage = declared_crc32c_with(canonicalized_s1_coverage);

    assert_ne!(
        crc32c.foundational_evidence_identity(),
        crc64.foundational_evidence_identity()
    );
    assert_ne!(
        crc32c.foundational_evidence_identity(),
        changed_coverage.foundational_evidence_identity()
    );
}

fn declared_crc32c_with(
    coverage: ChecksumCoverageMap,
) -> forge_store_physical_integrity::ChecksumAlgorithmDeclaration {
    declared_algorithm_with(ChecksumAlgorithmId::crc32c(), coverage)
}

fn declared_algorithm_with(
    algorithm: ChecksumAlgorithmId,
    coverage: ChecksumCoverageMap,
) -> forge_store_physical_integrity::ChecksumAlgorithmDeclaration {
    let format = PhysicalFormatDeclaration::physical_format_canonical().unwrap();
    let scope = ChecksumScopeDeclaration::for_physical_format(format.identity(), coverage).unwrap();
    algorithm
        .declare_for_scope(scope)
        .expect("for_canonical_physical_format crc32c declaration admits")
}

fn explicit_physical_format_coverage() -> Result<ChecksumCoverageMap, ChecksumCoverageMapDenial> {
    explicit_physical_format_builder().define()
}

fn explicit_future_coverage() -> Result<ChecksumCoverageMap, ChecksumCoverageMapDenial> {
    ChecksumCoverageMap::builder(PhysicalFormatVersion::reserved_future(2).unwrap())
        .covered_header_fields(physical_format_required_covered_header_fields())
        .excluded_header_fields([ChecksumHeaderField::ChecksumField])
        .checksum_field_handling(ChecksumFieldHandling::ExcludedDuringComputation)
        .mutable_publication_fields([ChecksumHeaderField::PublicationState])
        .reserved_fields(ChecksumReservedFieldPosture::CoveredAsZeroedAndPreserved)
        .generation_fields(ChecksumGenerationFieldPosture::CoveredAsPhysicalGeneration)
        .length_fields(ChecksumLengthFieldPosture::CoveredAsSerializedPayloadLength)
        .payload_region(ChecksumPayloadRegion::SerializedPayloadBytes)
        .padding_bytes(ChecksumPaddingPosture::ExcludedAndMustRemainZeroed)
        .compatibility_fields(ChecksumCompatibilityFieldPosture::CoveredAndDenyUnknown)
        .unknown_field_posture(ChecksumUnknownFieldPosture::DenyUntilReadmitted)
        .coverage_encoding(ChecksumCoverageEncoding::SerializedBytes)
        .define()
}

fn explicit_physical_format_builder() -> forge_store_physical_format::ChecksumCoverageMapBuilder {
    ChecksumCoverageMap::builder(PhysicalFormatVersion::initial_format_version())
        .covered_header_fields(physical_format_required_covered_header_fields())
        .excluded_header_fields([ChecksumHeaderField::ChecksumField])
        .checksum_field_handling(ChecksumFieldHandling::ExcludedDuringComputation)
        .mutable_publication_fields([ChecksumHeaderField::PublicationState])
        .reserved_fields(ChecksumReservedFieldPosture::CoveredAsZeroedAndPreserved)
        .generation_fields(ChecksumGenerationFieldPosture::CoveredAsPhysicalGeneration)
        .length_fields(ChecksumLengthFieldPosture::CoveredAsSerializedPayloadLength)
        .payload_region(ChecksumPayloadRegion::SerializedPayloadBytes)
        .padding_bytes(ChecksumPaddingPosture::ExcludedAndMustRemainZeroed)
        .compatibility_fields(ChecksumCompatibilityFieldPosture::CoveredAndDenyUnknown)
        .unknown_field_posture(ChecksumUnknownFieldPosture::DenyUntilReadmitted)
        .coverage_encoding(ChecksumCoverageEncoding::SerializedBytes)
}
