use crate::delta::ComplexityStatus;
use serde::Serialize;

use super::{
    certification::SupportDurabilityCertificationSummary,
    contracts::{
        Milestone7AccessStructureClaim, Milestone7AccessStructureContract,
        Milestone7AccessStructureVerification, Milestone7AccessStructureVerificationPath,
        Milestone7CounterContract,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7ComplexitySurface {
    pub schema_boundary_fetch: Milestone7ComplexityPathStatus,
    pub lineage_lookup: Milestone7ComplexityPathStatus,
    pub cursor_resume: Milestone7ComplexityPathStatus,
    pub embedded_checkpoint_fetch: Milestone7ComplexityPathStatus,
    pub commit_coupled_support_publication: Milestone7ComplexityPathStatus,
    pub cursor_identity_admission: Milestone7ComplexityPathStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7ComplexityPathStatus {
    pub status: ComplexityStatus,
    pub proof_basis: Option<String>,
    pub debt_reason: Option<String>,
}

impl Milestone7ComplexityPathStatus {
    pub(crate) fn verified(proof_basis: impl Into<String>) -> Self {
        Self {
            status: ComplexityStatus::Verified,
            proof_basis: Some(proof_basis.into()),
            debt_reason: None,
        }
    }
    pub(crate) fn debt(debt_reason: impl Into<String>) -> Self {
        Self {
            status: ComplexityStatus::Debt,
            proof_basis: None,
            debt_reason: Some(debt_reason.into()),
        }
    }
}

impl Milestone7ComplexitySurface {
    pub(crate) fn derive(
        certification_summary: &SupportDurabilityCertificationSummary,
        counter_contract: &Milestone7CounterContract,
        access_structure_contract: &Milestone7AccessStructureContract,
        access_structure_verification: &Milestone7AccessStructureVerification,
    ) -> Self {
        Self {
            schema_boundary_fetch: derive_verified_path(
                &access_structure_contract.schema_boundary_fetch,
                &access_structure_verification.schema_boundary_fetch,
            ),
            lineage_lookup: derive_verified_path(
                &access_structure_contract.lineage_lookup,
                &access_structure_verification.lineage_lookup,
            ),
            cursor_resume: if !access_structure_verification.cursor_resume.verified_at_open {
                Milestone7ComplexityPathStatus::debt(
                    access_structure_verification
                        .cursor_resume
                        .verification_gap
                        .clone()
                        .unwrap_or_else(|| {
                            "cursor resume access structure was not verified at open".to_string()
                        }),
                )
            } else if counter_contract.cursor_identity_lookup_count
                >= counter_contract.cursor_resume_count
            {
                Milestone7ComplexityPathStatus::verified(format!(
                    "{}; {}; {}",
                    access_structure_contract.cursor_resume.access_structure,
                    access_structure_contract.cursor_resume.guarantee,
                    access_structure_verification
                        .cursor_resume
                        .verification_basis
                        .as_deref()
                        .unwrap_or_default()
                ))
            } else {
                Milestone7ComplexityPathStatus::debt("cursor resume exceeds observed cursor identity lookup coverage; missing exact cursor identity admission evidence")
            },
            embedded_checkpoint_fetch: derive_verified_path(
                &access_structure_contract.embedded_checkpoint_fetch,
                &access_structure_verification.embedded_checkpoint_fetch,
            ),
            commit_coupled_support_publication: if !access_structure_verification
                .commit_coupled_support_publication
                .verified_at_open
            {
                Milestone7ComplexityPathStatus::debt(access_structure_verification.commit_coupled_support_publication.verification_gap.clone().unwrap_or_else(|| "commit-coupled support publication access structure was not verified at open".to_string()))
            } else if certification_summary.exactly_once_support_publication
                && counter_contract.commit_support_summary_build_count
                    >= counter_contract.commit_support_publication_count
                && counter_contract.commit_support_publication_gap_count == 0
            {
                Milestone7ComplexityPathStatus::verified(format!(
                    "{}; {}; {}",
                    access_structure_contract
                        .commit_coupled_support_publication
                        .access_structure,
                    access_structure_contract
                        .commit_coupled_support_publication
                        .guarantee,
                    access_structure_verification
                        .commit_coupled_support_publication
                        .verification_basis
                        .as_deref()
                        .unwrap_or_default()
                ))
            } else {
                Milestone7ComplexityPathStatus::debt("missing exactly-once commit support publication proof or publication-gap-free summary coupling")
            },
            cursor_identity_admission: if !access_structure_verification
                .cursor_identity_admission
                .verified_at_open
            {
                Milestone7ComplexityPathStatus::debt(
                    access_structure_verification
                        .cursor_identity_admission
                        .verification_gap
                        .clone()
                        .unwrap_or_else(|| {
                            "cursor identity admission access structure was not verified at open"
                                .to_string()
                        }),
                )
            } else if counter_contract.subscriber_checkpoint_write_count
                <= counter_contract.cursor_ack_count
            {
                Milestone7ComplexityPathStatus::verified(format!(
                    "{}; {}; {}",
                    access_structure_contract
                        .cursor_identity_admission
                        .access_structure,
                    access_structure_contract
                        .cursor_identity_admission
                        .guarantee,
                    access_structure_verification
                        .cursor_identity_admission
                        .verification_basis
                        .as_deref()
                        .unwrap_or_default()
                ))
            } else {
                Milestone7ComplexityPathStatus::debt("subscriber checkpoints outpaced acknowledged cursor admissions; missing exact cursor identity admission proof")
            },
        }
    }
}

fn derive_verified_path(
    contract: &Milestone7AccessStructureClaim,
    verification: &Milestone7AccessStructureVerificationPath,
) -> Milestone7ComplexityPathStatus {
    if verification.verified_at_open {
        Milestone7ComplexityPathStatus::verified(format!(
            "{}; {}; {}",
            contract.access_structure,
            contract.guarantee,
            verification
                .verification_basis
                .as_deref()
                .unwrap_or_default()
        ))
    } else {
        Milestone7ComplexityPathStatus::debt(
            verification.verification_gap.clone().unwrap_or_else(|| {
                "required access structure was not verified at open".to_string()
            }),
        )
    }
}
