use crate::application::WorthQueryMilestoneClosureStatus;
use crate::WorthQueryEvidenceIdentity;

use super::certification::WorthQueryConsumerKitHostileCertification;
use super::certification_gate::{
    certification_source_paths_for_family, consumer_kit_family_certification_gate_certified,
    consumer_kit_family_evidence_digest,
};
use super::docs_agreement::WorthQueryConsumerKitDocsAgreement;
use super::evidence::{consumer_kit_closure_identity, required_consumer_kit_families};
use super::family::{WorthQueryConsumerKitFamilyClosureRow, WorthQueryConsumerKitFamilyName};
use super::residue::WorthQueryConsumerKitReferenceResidue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerKitClosure {
    status: WorthQueryMilestoneClosureStatus,
    family_rows: Vec<WorthQueryConsumerKitFamilyClosureRow>,
    docs_agreement: WorthQueryConsumerKitDocsAgreement,
    reference_residue: WorthQueryConsumerKitReferenceResidue,
    hostile_certification: WorthQueryConsumerKitHostileCertification,
    defended_exclusions: Vec<String>,
    closure_identity: WorthQueryEvidenceIdentity,
}

pub fn milestone_nine_eight_consumer_kit_closure() -> WorthQueryConsumerKitClosure {
    WorthQueryConsumerKitClosure::derive_from_parts(
        closed_consumer_kit_family_rows(),
        WorthQueryConsumerKitDocsAgreement::current(),
        WorthQueryConsumerKitReferenceResidue::current(),
        ["durable persisted kit archives remain Milestone 10/11 scope"],
    )
}

impl WorthQueryConsumerKitClosure {
    pub(crate) fn derive_from_parts(
        family_rows: impl IntoIterator<Item = WorthQueryConsumerKitFamilyClosureRow>,
        docs_agreement: WorthQueryConsumerKitDocsAgreement,
        reference_residue: WorthQueryConsumerKitReferenceResidue,
        defended_exclusions: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let mut family_rows = family_rows.into_iter().collect::<Vec<_>>();
        family_rows.sort_by_key(WorthQueryConsumerKitFamilyClosureRow::family_name);
        let defended_exclusions = defended_exclusions
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let hostile_certification = WorthQueryConsumerKitHostileCertification::derive(
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

    pub fn status(&self) -> WorthQueryMilestoneClosureStatus {
        self.status
    }

    pub fn kit_families(&self) -> &[WorthQueryConsumerKitFamilyClosureRow] {
        &self.family_rows
    }

    pub fn docs_agreement(&self) -> &WorthQueryConsumerKitDocsAgreement {
        &self.docs_agreement
    }

    pub fn docs_agree_with_support_profile(&self) -> bool {
        self.docs_agreement.agrees()
    }

    pub fn reference_consumer_residue(&self) -> &WorthQueryConsumerKitReferenceResidue {
        &self.reference_residue
    }

    pub fn hostile_certification(&self) -> &WorthQueryConsumerKitHostileCertification {
        &self.hostile_certification
    }

    pub fn defended_exclusions(&self) -> &[String] {
        &self.defended_exclusions
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_identity.as_str()
    }

    pub fn closure_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.closure_identity
    }

    pub fn required_families() -> &'static [WorthQueryConsumerKitFamilyName] {
        required_consumer_kit_families()
    }
}

fn closed_consumer_kit_family_rows() -> Vec<WorthQueryConsumerKitFamilyClosureRow> {
    [
        (
            WorthQueryConsumerKitFamilyName::EvidenceReportKit,
            "Consumer Evidence Report Kit Parity Test",
        ),
        (
            WorthQueryConsumerKitFamilyName::HardProhibitionRegistry,
            "Prohibition Registry And Seam Visibility Test",
        ),
        (
            WorthQueryConsumerKitFamilyName::BoundaryAudit,
            "Shipped Bypass Audit Artifact Boundary Test",
        ),
        (
            WorthQueryConsumerKitFamilyName::SupportSnapshot,
            "Support Snapshot Projection Test",
        ),
        (
            WorthQueryConsumerKitFamilyName::SupportPinning,
            "Worth-Kernel Support Pinning Drift Test",
        ),
        (
            WorthQueryConsumerKitFamilyName::InMemoryTestBackend,
            "Shipped Test Backend Honesty Test",
        ),
        (
            WorthQueryConsumerKitFamilyName::ConsumerResidueAudit,
            "Typed Consumer Residue Audit For Query Proof Folklore",
        ),
    ]
    .into_iter()
    .map(|(family, label)| {
        let source_paths = certification_source_paths_for_family(family);
        let evidence_digest = consumer_kit_family_evidence_digest(family, label);
        if consumer_kit_family_certification_gate_certified(family) {
            WorthQueryConsumerKitFamilyClosureRow::closed(
                family,
                label,
                evidence_digest,
                source_paths,
            )
        } else {
            WorthQueryConsumerKitFamilyClosureRow::new(
                family,
                WorthQueryMilestoneClosureStatus::Open,
                label,
                evidence_digest,
                source_paths,
            )
        }
    })
    .collect()
}

fn derive_consumer_kit_closure_status(
    family_rows: &[WorthQueryConsumerKitFamilyClosureRow],
    docs_agreement: &WorthQueryConsumerKitDocsAgreement,
    reference_residue: &WorthQueryConsumerKitReferenceResidue,
    hostile_certification: &WorthQueryConsumerKitHostileCertification,
) -> WorthQueryMilestoneClosureStatus {
    let all_required_closed = required_consumer_kit_families()
        .iter()
        .all(|required_family| {
            family_rows.iter().any(|row| {
                row.family_name() == *required_family
                    && row.status() == WorthQueryMilestoneClosureStatus::Closed
                    && !row.evidence_digest().is_empty()
                    && !row.evidence_source_paths().is_empty()
            })
        });
    if all_required_closed
        && docs_agreement.agrees()
        && reference_residue.is_query_owned_clean()
        && hostile_certification.status() == WorthQueryMilestoneClosureStatus::Closed
    {
        return WorthQueryMilestoneClosureStatus::Closed;
    }
    if family_rows
        .iter()
        .any(|row| row.status() != WorthQueryMilestoneClosureStatus::Open)
        || docs_agreement.agrees()
        || reference_residue.is_query_owned_clean()
        || hostile_certification.status() != WorthQueryMilestoneClosureStatus::Open
    {
        return WorthQueryMilestoneClosureStatus::Partial;
    }
    WorthQueryMilestoneClosureStatus::Open
}
