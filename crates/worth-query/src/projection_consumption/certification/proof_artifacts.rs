use crate::projection_consumption::identity::{
    compose_proof_artifact_bundle_digest, compose_proof_artifact_entry_digest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionGoldenTranscript {
    CommonRead,
    CommonWrite,
    CommonQueryContext,
    AdvancedPath,
    SerializedAuthorityContract,
}

impl ProjectionConsumptionGoldenTranscript {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CommonRead => "projection_consumption_common_read_golden_path_compiles",
            Self::CommonWrite => "projection_consumption_common_write_golden_path_compiles",
            Self::CommonQueryContext => {
                "projection_consumption_common_query_context_golden_path_compiles"
            }
            Self::AdvancedPath => "projection_consumption_advanced_path_golden_transcript_compiles",
            Self::SerializedAuthorityContract => {
                "projection_authority_serialized_contract_compiles"
            }
        }
    }

    fn source(&self) -> &'static str {
        match self {
            Self::CommonRead => include_str!(
                "../../../tests/ui/projection_consumption/golden/projection_consumption_common_read_golden_path_compiles.rs"
            ),
            Self::CommonWrite => include_str!(
                "../../../tests/ui/projection_consumption/golden/projection_consumption_common_write_golden_path_compiles.rs"
            ),
            Self::CommonQueryContext => include_str!(
                "../../../tests/ui/projection_consumption/golden/projection_consumption_common_query_context_golden_path_compiles.rs"
            ),
            Self::AdvancedPath => include_str!(
                "../../../tests/ui/projection_consumption/golden/projection_consumption_advanced_path_golden_transcript_compiles.rs"
            ),
            Self::SerializedAuthorityContract => include_str!(
                "../../../tests/ui/projection_consumption/golden/projection_authority_serialized_contract_compiles.rs"
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionCompileFailProof {
    AuthoringSurfaceConstructorPrivate,
    ContractConstructorPrivate,
    ContractHasNoGenericExtract,
    CertificationBundleConstructorPrivate,
    DeclarationConstructorPrivate,
    EligibilityArtifactsConstructorPrivate,
    EnvelopeConstructorPrivate,
    FactSetConstructorPrivate,
    NonAdmittedCannotBindContract,
    RawSourceHasNoConsumedFactAccessors,
    ReceiptConstructorPrivate,
    SupportArtifactsConstructorPrivate,
    ConsumedAuthorityConstructorPrivate,
    ConsumedAuthorityNotCloneable,
    ConsumedAuthorityCertificationBundleConstructorPrivate,
    LegacyCompletedConsumptionNotInFacade,
    LegacyConsumptionAttemptNotInFacade,
}

impl ProjectionConsumptionCompileFailProof {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoringSurfaceConstructorPrivate => {
                "projection_consumption_authoring_surface_constructor_private"
            }
            Self::ContractConstructorPrivate => {
                "projection_consumption_contract_constructor_private"
            }
            Self::ContractHasNoGenericExtract => {
                "projection_consumption_contract_has_no_generic_extract"
            }
            Self::CertificationBundleConstructorPrivate => {
                "projection_consumption_certification_bundle_constructor_private"
            }
            Self::DeclarationConstructorPrivate => {
                "projection_consumption_declaration_constructor_private"
            }
            Self::EligibilityArtifactsConstructorPrivate => {
                "projection_consumption_eligibility_artifacts_constructor_private"
            }
            Self::EnvelopeConstructorPrivate => {
                "projection_consumption_envelope_constructor_private"
            }
            Self::FactSetConstructorPrivate => {
                "projection_consumption_fact_set_constructor_private"
            }
            Self::NonAdmittedCannotBindContract => {
                "projection_consumption_non_admitted_cannot_bind_contract"
            }
            Self::RawSourceHasNoConsumedFactAccessors => {
                "projection_consumption_raw_source_has_no_consumed_fact_accessors"
            }
            Self::ReceiptConstructorPrivate => "projection_consumption_receipt_constructor_private",
            Self::SupportArtifactsConstructorPrivate => {
                "projection_consumption_support_artifacts_constructor_private"
            }
            Self::ConsumedAuthorityConstructorPrivate => {
                "consumed_projection_authority_constructor_private"
            }
            Self::ConsumedAuthorityNotCloneable => "consumed_projection_authority_not_cloneable",
            Self::ConsumedAuthorityCertificationBundleConstructorPrivate => {
                "consumed_projection_authority_certification_bundle_constructor_private"
            }
            Self::LegacyCompletedConsumptionNotInFacade => {
                "legacy_completed_consumption_not_in_facade"
            }
            Self::LegacyConsumptionAttemptNotInFacade => "legacy_consumption_attempt_not_in_facade",
        }
    }

    fn source(&self) -> &'static str {
        match self {
            Self::AuthoringSurfaceConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_authoring_surface_constructor_private.rs"
            ),
            Self::ContractConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_contract_constructor_private.rs"
            ),
            Self::ContractHasNoGenericExtract => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/projection_consumption_contract_has_no_generic_extract.rs"
            ),
            Self::CertificationBundleConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_certification_bundle_constructor_private.rs"
            ),
            Self::DeclarationConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_declaration_constructor_private.rs"
            ),
            Self::EligibilityArtifactsConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_eligibility_artifacts_constructor_private.rs"
            ),
            Self::EnvelopeConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_envelope_constructor_private.rs"
            ),
            Self::FactSetConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_fact_set_constructor_private.rs"
            ),
            Self::NonAdmittedCannotBindContract => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/projection_consumption_non_admitted_cannot_bind_contract.rs"
            ),
            Self::RawSourceHasNoConsumedFactAccessors => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/projection_consumption_raw_source_has_no_consumed_fact_accessors.rs"
            ),
            Self::ReceiptConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_receipt_constructor_private.rs"
            ),
            Self::SupportArtifactsConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_support_artifacts_constructor_private.rs"
            ),
            Self::ConsumedAuthorityConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/consumed_projection_authority_constructor_private.rs"
            ),
            Self::ConsumedAuthorityNotCloneable => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/consumed_projection_authority_not_cloneable.rs"
            ),
            Self::ConsumedAuthorityCertificationBundleConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/consumed_projection_authority_certification_bundle_constructor_private.rs"
            ),
            Self::LegacyCompletedConsumptionNotInFacade => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/legacy_completed_consumption_not_in_facade.rs"
            ),
            Self::LegacyConsumptionAttemptNotInFacade => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/legacy_consumption_attempt_not_in_facade.rs"
            ),
        }
    }

    fn stderr(&self) -> &'static str {
        match self {
            Self::AuthoringSurfaceConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_authoring_surface_constructor_private.stderr"
            ),
            Self::ContractConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_contract_constructor_private.stderr"
            ),
            Self::ContractHasNoGenericExtract => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/projection_consumption_contract_has_no_generic_extract.stderr"
            ),
            Self::CertificationBundleConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_certification_bundle_constructor_private.stderr"
            ),
            Self::DeclarationConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_declaration_constructor_private.stderr"
            ),
            Self::EligibilityArtifactsConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_eligibility_artifacts_constructor_private.stderr"
            ),
            Self::EnvelopeConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_envelope_constructor_private.stderr"
            ),
            Self::FactSetConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_fact_set_constructor_private.stderr"
            ),
            Self::NonAdmittedCannotBindContract => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/projection_consumption_non_admitted_cannot_bind_contract.stderr"
            ),
            Self::RawSourceHasNoConsumedFactAccessors => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/projection_consumption_raw_source_has_no_consumed_fact_accessors.stderr"
            ),
            Self::ReceiptConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_receipt_constructor_private.stderr"
            ),
            Self::SupportArtifactsConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/projection_consumption_support_artifacts_constructor_private.stderr"
            ),
            Self::ConsumedAuthorityConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/consumed_projection_authority_constructor_private.stderr"
            ),
            Self::ConsumedAuthorityNotCloneable => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/consumed_projection_authority_not_cloneable.stderr"
            ),
            Self::ConsumedAuthorityCertificationBundleConstructorPrivate => include_str!(
                "../../../tests/ui/projection_consumption/construction/consumed_projection_authority_certification_bundle_constructor_private.stderr"
            ),
            Self::LegacyCompletedConsumptionNotInFacade => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/legacy_completed_consumption_not_in_facade.stderr"
            ),
            Self::LegacyConsumptionAttemptNotInFacade => include_str!(
                "../../../tests/ui/projection_consumption/boundaries/legacy_consumption_attempt_not_in_facade.stderr"
            ),
        }
    }
}

pub fn projection_consumption_golden_transcripts(
) -> &'static [ProjectionConsumptionGoldenTranscript] {
    &[
        ProjectionConsumptionGoldenTranscript::CommonRead,
        ProjectionConsumptionGoldenTranscript::CommonWrite,
        ProjectionConsumptionGoldenTranscript::CommonQueryContext,
        ProjectionConsumptionGoldenTranscript::AdvancedPath,
        ProjectionConsumptionGoldenTranscript::SerializedAuthorityContract,
    ]
}

pub fn projection_consumption_compile_fail_proofs(
) -> &'static [ProjectionConsumptionCompileFailProof] {
    &[
        ProjectionConsumptionCompileFailProof::AuthoringSurfaceConstructorPrivate,
        ProjectionConsumptionCompileFailProof::ContractConstructorPrivate,
        ProjectionConsumptionCompileFailProof::ContractHasNoGenericExtract,
        ProjectionConsumptionCompileFailProof::CertificationBundleConstructorPrivate,
        ProjectionConsumptionCompileFailProof::DeclarationConstructorPrivate,
        ProjectionConsumptionCompileFailProof::EligibilityArtifactsConstructorPrivate,
        ProjectionConsumptionCompileFailProof::EnvelopeConstructorPrivate,
        ProjectionConsumptionCompileFailProof::FactSetConstructorPrivate,
        ProjectionConsumptionCompileFailProof::NonAdmittedCannotBindContract,
        ProjectionConsumptionCompileFailProof::RawSourceHasNoConsumedFactAccessors,
        ProjectionConsumptionCompileFailProof::ReceiptConstructorPrivate,
        ProjectionConsumptionCompileFailProof::SupportArtifactsConstructorPrivate,
        ProjectionConsumptionCompileFailProof::ConsumedAuthorityConstructorPrivate,
        ProjectionConsumptionCompileFailProof::ConsumedAuthorityNotCloneable,
        ProjectionConsumptionCompileFailProof::ConsumedAuthorityCertificationBundleConstructorPrivate,
        ProjectionConsumptionCompileFailProof::LegacyCompletedConsumptionNotInFacade,
        ProjectionConsumptionCompileFailProof::LegacyConsumptionAttemptNotInFacade,
    ]
}

pub fn golden_transcript_bundle_digest() -> String {
    compose_proof_artifact_bundle_digest(
        "projection_consumption_golden_transcript_bundle_v1",
        projection_consumption_golden_transcripts()
            .iter()
            .map(|transcript| {
                compose_proof_artifact_entry_digest(
                    "projection_consumption_golden_transcript_entry_v1",
                    [transcript.as_str(), transcript.source()],
                )
            }),
    )
}

pub fn compile_fail_boundary_bundle_digest() -> String {
    compose_proof_artifact_bundle_digest(
        "projection_consumption_compile_fail_boundary_bundle_v1",
        projection_consumption_compile_fail_proofs()
            .iter()
            .map(|proof| {
                compose_proof_artifact_entry_digest(
                    "projection_consumption_compile_fail_proof_entry_v1",
                    [proof.as_str(), proof.source(), proof.stderr()],
                )
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_artifact_digests_bind_to_real_fixture_contents() {
        assert_eq!(projection_consumption_golden_transcripts().len(), 5);
        assert_eq!(projection_consumption_compile_fail_proofs().len(), 17);
        assert!(!golden_transcript_bundle_digest().is_empty());
        assert!(!compile_fail_boundary_bundle_digest().is_empty());
    }
}
