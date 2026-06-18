use crate::application::ForgeQueryMilestoneClosureStatus;
use crate::ForgeQueryEvidenceIdentity;

use super::certification::ForgeQueryConsumerKitHostileCertification;
use super::certification_gate::{
    certification_source_paths_for_family, consumer_kit_family_certification_gate_certified,
    consumer_kit_family_evidence_digest,
};
use super::docs_agreement::ForgeQueryConsumerKitDocsAgreement;
use super::evidence::{consumer_kit_closure_identity, required_consumer_kit_families};
use super::family::{ForgeQueryConsumerKitFamilyClosureRow, ForgeQueryConsumerKitFamilyName};
use super::residue::ForgeQueryConsumerKitReferenceResidue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerKitClosure {
    status: ForgeQueryMilestoneClosureStatus,
    family_rows: Vec<ForgeQueryConsumerKitFamilyClosureRow>,
    docs_agreement: ForgeQueryConsumerKitDocsAgreement,
    reference_residue: ForgeQueryConsumerKitReferenceResidue,
    hostile_certification: ForgeQueryConsumerKitHostileCertification,
    defended_exclusions: Vec<String>,
    closure_identity: ForgeQueryEvidenceIdentity,
}

pub fn milestone_nine_eight_consumer_kit_closure() -> ForgeQueryConsumerKitClosure {
    ForgeQueryConsumerKitClosure::derive_from_parts(
        closed_consumer_kit_family_rows(),
        ForgeQueryConsumerKitDocsAgreement::current(),
        ForgeQueryConsumerKitReferenceResidue::current(),
        ["durable persisted kit archives remain Milestone 10/11 scope"],
    )
}

impl ForgeQueryConsumerKitClosure {
    pub(crate) fn derive_from_parts(
        family_rows: impl IntoIterator<Item = ForgeQueryConsumerKitFamilyClosureRow>,
        docs_agreement: ForgeQueryConsumerKitDocsAgreement,
        reference_residue: ForgeQueryConsumerKitReferenceResidue,
        defended_exclusions: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let mut family_rows = family_rows.into_iter().collect::<Vec<_>>();
        family_rows.sort_by_key(ForgeQueryConsumerKitFamilyClosureRow::family_name);
        let defended_exclusions = defended_exclusions
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let hostile_certification = ForgeQueryConsumerKitHostileCertification::derive(
            &family_rows,
            &docs_agreement,
            &reference_residue,
        );
        let status = derive_consumer_kit_closure_status(
            &family_rows,
            &docs_agreement,
            &reference_residue,
            &hostile_certification,
        );
        let closure_identity = consumer_kit_closure_identity(
            status,
            &family_rows,
            hostile_certification.certification_digest(),
            docs_agreement.agreement_digest(),
            reference_residue.residue_digest(),
            &defended_exclusions,
        );
        Self {
            status,
            family_rows,
            docs_agreement,
            reference_residue,
            hostile_certification,
            defended_exclusions,
            closure_identity,
        }
    }

    pub fn status(&self) -> ForgeQueryMilestoneClosureStatus {
        self.status
    }

    pub fn kit_families(&self) -> &[ForgeQueryConsumerKitFamilyClosureRow] {
        &self.family_rows
    }

    pub fn docs_agreement(&self) -> &ForgeQueryConsumerKitDocsAgreement {
        &self.docs_agreement
    }

    pub fn docs_agree_with_support_profile(&self) -> bool {
        self.docs_agreement.agrees()
    }

    pub fn reference_consumer_residue(&self) -> &ForgeQueryConsumerKitReferenceResidue {
        &self.reference_residue
    }

    pub fn hostile_certification(&self) -> &ForgeQueryConsumerKitHostileCertification {
        &self.hostile_certification
    }

    pub fn defended_exclusions(&self) -> &[String] {
        &self.defended_exclusions
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_identity.as_str()
    }

    pub fn closure_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.closure_identity
    }

    pub fn required_families() -> &'static [ForgeQueryConsumerKitFamilyName] {
        required_consumer_kit_families()
    }
}

fn closed_consumer_kit_family_rows() -> Vec<ForgeQueryConsumerKitFamilyClosureRow> {
    [
        (
            ForgeQueryConsumerKitFamilyName::EvidenceReportKit,
            "Consumer Evidence Report Kit Parity Test",
        ),
        (
            ForgeQueryConsumerKitFamilyName::HardProhibitionRegistry,
            "Prohibition Registry And Seam Visibility Test",
        ),
        (
            ForgeQueryConsumerKitFamilyName::BoundaryAudit,
            "Shipped Bypass Audit Artifact Boundary Test",
        ),
        (
            ForgeQueryConsumerKitFamilyName::SupportSnapshot,
            "Support Snapshot Projection Test",
        ),
        (
            ForgeQueryConsumerKitFamilyName::SupportPinning,
            "Worth-Kernel Support Pinning Drift Test",
        ),
        (
            ForgeQueryConsumerKitFamilyName::InMemoryTestBackend,
            "Shipped Test Backend Honesty Test",
        ),
        (
            ForgeQueryConsumerKitFamilyName::ReferenceConsumerAdoption,
            "Reference Consumer Enforcement Adoption Test",
        ),
    ]
    .into_iter()
    .map(|(family, label)| {
        let source_paths = certification_source_paths_for_family(family);
        let evidence_digest = consumer_kit_family_evidence_digest(family, label);
        if consumer_kit_family_certification_gate_certified(family) {
            ForgeQueryConsumerKitFamilyClosureRow::closed(
                family,
                label,
                evidence_digest,
                source_paths,
            )
        } else {
            ForgeQueryConsumerKitFamilyClosureRow::new(
                family,
                ForgeQueryMilestoneClosureStatus::Open,
                label,
                evidence_digest,
                source_paths,
            )
        }
    })
    .collect()
}

fn derive_consumer_kit_closure_status(
    family_rows: &[ForgeQueryConsumerKitFamilyClosureRow],
    docs_agreement: &ForgeQueryConsumerKitDocsAgreement,
    reference_residue: &ForgeQueryConsumerKitReferenceResidue,
    hostile_certification: &ForgeQueryConsumerKitHostileCertification,
) -> ForgeQueryMilestoneClosureStatus {
    let all_required_closed = required_consumer_kit_families()
        .iter()
        .all(|required_family| {
            family_rows.iter().any(|row| {
                row.family_name() == *required_family
                    && row.status() == ForgeQueryMilestoneClosureStatus::Closed
                    && !row.evidence_digest().is_empty()
                    && !row.evidence_source_paths().is_empty()
            })
        });
    if all_required_closed
        && docs_agreement.agrees()
        && reference_residue.is_query_owned_clean()
        && hostile_certification.status() == ForgeQueryMilestoneClosureStatus::Closed
    {
        return ForgeQueryMilestoneClosureStatus::Closed;
    }
    if family_rows
        .iter()
        .any(|row| row.status() != ForgeQueryMilestoneClosureStatus::Open)
        || docs_agreement.agrees()
        || reference_residue.is_query_owned_clean()
        || hostile_certification.status() != ForgeQueryMilestoneClosureStatus::Open
    {
        return ForgeQueryMilestoneClosureStatus::Partial;
    }
    ForgeQueryMilestoneClosureStatus::Open
}
