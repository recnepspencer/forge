use worth_proof::TransitionOutcome;

use crate::{
    StoreCurrentAuthorityWitness, StoreExternalAuthorityToken,
    StoreExternalAuthorityTokenFreshness, StoreRetainedAuthorityEvidence,
};

pub type StoreAuthorityReadmissionOutcome =
    TransitionOutcome<StoreCurrentAuthorityWitness, StoreAuthorityReadmissionDenial>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreLowerAuthoritySource {
    TerminalProjectionText,
    DigestText,
    Filename,
    ExternalToken,
    DerivedEvidence,
    RetainedEvidence,
    SemanticTransactionVisibility,
    SemanticBranchVisibility,
    SemanticSnapshotVisibility,
    SemanticProjectionVisibility,
    SemanticCurrentBasisExport,
    SemanticCommitVisibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreAuthorityReadmissionDenial {
    EmptyExternalToken,
    ExternalTokenMismatch {
        external_token_text: String,
        current_identity_text: String,
    },
    StaleExternalToken {
        external_token_text: String,
    },
    RetainedEvidenceIdentityMismatch {
        retained_identity_text: String,
        current_identity_text: String,
    },
    RetainedEvidencePhysicalWitnessMismatch,
    LowerAuthoritySourceRequiresOwnerReadmission {
        source: StoreLowerAuthoritySource,
    },
    UnsupportedAuthoritySource {
        source: StoreLowerAuthoritySource,
    },
}

pub fn readmit_external_store_authority_token(
    external_token: StoreExternalAuthorityToken,
    current_authority: &StoreCurrentAuthorityWitness,
) -> StoreAuthorityReadmissionOutcome {
    let token_text = external_token.external_token_text().trim();
    if token_text.is_empty() {
        return TransitionOutcome::denied(StoreAuthorityReadmissionDenial::EmptyExternalToken);
    }

    if external_token.freshness() == StoreExternalAuthorityTokenFreshness::StaleRetained {
        return TransitionOutcome::denied(StoreAuthorityReadmissionDenial::StaleExternalToken {
            external_token_text: token_text.to_owned(),
        });
    }

    let current_identity_text = current_authority.identity().aspect_key().as_str();
    if token_text != current_identity_text {
        return TransitionOutcome::denied(StoreAuthorityReadmissionDenial::ExternalTokenMismatch {
            external_token_text: token_text.to_owned(),
            current_identity_text: current_identity_text.to_owned(),
        });
    }

    TransitionOutcome::success(current_authority.clone())
}

pub fn readmit_retained_store_authority_evidence(
    retained_evidence: StoreRetainedAuthorityEvidence,
    current_authority: &StoreCurrentAuthorityWitness,
) -> StoreAuthorityReadmissionOutcome {
    let retained_identity_text = retained_evidence.identity().aspect_key().as_str();
    let current_identity_text = current_authority.identity().aspect_key().as_str();
    if retained_identity_text != current_identity_text {
        return TransitionOutcome::denied(
            StoreAuthorityReadmissionDenial::RetainedEvidenceIdentityMismatch {
                retained_identity_text: retained_identity_text.to_owned(),
                current_identity_text: current_identity_text.to_owned(),
            },
        );
    }

    if retained_evidence.physical_witness() != current_authority.physical_witness() {
        return TransitionOutcome::denied(
            StoreAuthorityReadmissionDenial::RetainedEvidencePhysicalWitnessMismatch,
        );
    }

    TransitionOutcome::success(current_authority.clone())
}

pub fn deny_lower_authority_source_as_current_authority(
    source: StoreLowerAuthoritySource,
) -> StoreAuthorityReadmissionDenial {
    StoreAuthorityReadmissionDenial::LowerAuthoritySourceRequiresOwnerReadmission { source }
}

pub fn deny_lower_authority_source_readmission_as_current_authority(
    source: StoreLowerAuthoritySource,
) -> StoreAuthorityReadmissionOutcome {
    TransitionOutcome::denied(deny_lower_authority_source_as_current_authority(source))
}

pub fn deny_unsupported_authority_source_as_current_authority(
    source: StoreLowerAuthoritySource,
) -> StoreAuthorityReadmissionDenial {
    StoreAuthorityReadmissionDenial::UnsupportedAuthoritySource { source }
}

pub fn deny_unsupported_authority_source_readmission_as_current_authority(
    source: StoreLowerAuthoritySource,
) -> StoreAuthorityReadmissionOutcome {
    TransitionOutcome::denied(deny_unsupported_authority_source_as_current_authority(
        source,
    ))
}
