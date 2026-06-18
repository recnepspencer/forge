use crate::application::ForgeQueryMilestoneClosureStatus;
use crate::ForgeQueryEvidenceIdentity;

use super::certification_case::{
    required_consumer_kit_certification_cases, ForgeQueryConsumerKitCertificationCase,
    ForgeQueryConsumerKitCertificationTier,
};
use super::docs_agreement::ForgeQueryConsumerKitDocsAgreement;
use super::evidence::consumer_kit_certification_identity;
use super::family::ForgeQueryConsumerKitFamilyClosureRow;
use super::residue::ForgeQueryConsumerKitReferenceResidue;

pub const MILESTONE_NINE_EIGHT_CERTIFICATION_SUITE: &str =
    "Milestone 9.8 Consumer Kit Hostile Certification Matrix";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerKitHostileCertification {
    suite_name: &'static str,
    status: ForgeQueryMilestoneClosureStatus,
    case_rows: Vec<ForgeQueryConsumerKitCertificationCaseRow>,
    missing_case_ids: Vec<&'static str>,
    certification_identity: ForgeQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerKitCertificationCaseRow {
    family: super::family::ForgeQueryConsumerKitFamilyName,
    case_id: &'static str,
    tier: ForgeQueryConsumerKitCertificationTier,
    requirement: &'static str,
    required_signal: &'static str,
    evidence_source_paths: Vec<&'static str>,
    satisfied: bool,
    case_digest: String,
}

impl From<ForgeQueryConsumerKitCertificationCase> for ForgeQueryConsumerKitCertificationCaseRow {
    fn from(case: ForgeQueryConsumerKitCertificationCase) -> Self {
        Self {
            family: case.family(),
            case_id: case.case_id(),
            tier: case.tier(),
            requirement: case.requirement(),
            required_signal: case.required_signal(),
            evidence_source_paths: case.evidence_source_paths().to_vec(),
            satisfied: case.satisfied(),
            case_digest: case.case_digest().to_owned(),
        }
    }
}

impl ForgeQueryConsumerKitCertificationCaseRow {
    pub fn family(&self) -> super::family::ForgeQueryConsumerKitFamilyName {
        self.family
    }

    pub fn case_id(&self) -> &'static str {
        self.case_id
    }

    pub fn tier(&self) -> ForgeQueryConsumerKitCertificationTier {
        self.tier
    }

    pub fn requirement(&self) -> &'static str {
        self.requirement
    }

    pub fn required_signal(&self) -> &'static str {
        self.required_signal
    }

    pub fn evidence_source_paths(&self) -> &[&'static str] {
        &self.evidence_source_paths
    }

    pub fn satisfied(&self) -> bool {
        self.satisfied
    }

    pub fn case_digest(&self) -> &str {
        &self.case_digest
    }
}

impl ForgeQueryConsumerKitHostileCertification {
    pub(crate) fn derive(
        rows: &[ForgeQueryConsumerKitFamilyClosureRow],
        docs_agreement: &ForgeQueryConsumerKitDocsAgreement,
        residue: &ForgeQueryConsumerKitReferenceResidue,
    ) -> Self {
        let case_rows = required_consumer_kit_certification_cases()
            .into_iter()
            .map(ForgeQueryConsumerKitCertificationCaseRow::from)
            .collect::<Vec<_>>();
        let missing_case_ids = case_rows
            .iter()
            .filter(|case| !case.satisfied())
            .map(ForgeQueryConsumerKitCertificationCaseRow::case_id)
            .collect::<Vec<_>>();
        let status = if rows.iter().all(|row| {
            row.status() == ForgeQueryMilestoneClosureStatus::Closed
                && !row.evidence_digest().is_empty()
        }) && missing_case_ids.is_empty()
            && docs_agreement.agrees()
            && residue.is_query_owned_clean()
        {
            ForgeQueryMilestoneClosureStatus::Closed
        } else if rows
            .iter()
            .any(|row| row.status() != ForgeQueryMilestoneClosureStatus::Open)
            || docs_agreement.agrees()
            || residue.is_query_owned_clean()
        {
            ForgeQueryMilestoneClosureStatus::Partial
        } else {
            ForgeQueryMilestoneClosureStatus::Open
        };
        let certification_identity = consumer_kit_certification_identity(
            MILESTONE_NINE_EIGHT_CERTIFICATION_SUITE,
            rows,
            &case_rows,
            docs_agreement.agreement_digest(),
            residue.residue_digest(),
        );
        Self {
            suite_name: MILESTONE_NINE_EIGHT_CERTIFICATION_SUITE,
            status,
            case_rows,
            missing_case_ids,
            certification_identity,
        }
    }

    pub fn suite_name(&self) -> &'static str {
        self.suite_name
    }

    pub fn status(&self) -> ForgeQueryMilestoneClosureStatus {
        self.status
    }

    pub fn case_rows(&self) -> &[ForgeQueryConsumerKitCertificationCaseRow] {
        &self.case_rows
    }

    pub fn missing_case_ids(&self) -> &[&'static str] {
        &self.missing_case_ids
    }

    pub fn certification_digest(&self) -> &str {
        self.certification_identity.as_str()
    }

    pub fn certification_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.certification_identity
    }
}
