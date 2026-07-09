use crate::application::WorthQueryMilestoneClosureStatus;
use crate::WorthQueryEvidenceIdentity;

use super::certification_case::{
    required_consumer_kit_certification_cases, WorthQueryConsumerKitCertificationCase,
    WorthQueryConsumerKitCertificationTier,
};
use super::docs_agreement::WorthQueryConsumerKitDocsAgreement;
use super::evidence::consumer_kit_certification_identity;
use super::family::WorthQueryConsumerKitFamilyClosureRow;
use super::residue::WorthQueryConsumerKitReferenceResidue;

pub const MILESTONE_NINE_EIGHT_CERTIFICATION_SUITE: &str =
    "Milestone 9.8 Consumer Kit Hostile Certification Matrix";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerKitHostileCertification {
    suite_name: &'static str,
    status: WorthQueryMilestoneClosureStatus,
    case_rows: Vec<WorthQueryConsumerKitCertificationCaseRow>,
    missing_case_ids: Vec<&'static str>,
    certification_identity: WorthQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerKitCertificationCaseRow {
    family: super::family::WorthQueryConsumerKitFamilyName,
    case_id: &'static str,
    tier: WorthQueryConsumerKitCertificationTier,
    requirement: &'static str,
    required_signal: &'static str,
    evidence_source_paths: Vec<&'static str>,
    satisfied: bool,
    case_digest: String,
}

impl From<WorthQueryConsumerKitCertificationCase> for WorthQueryConsumerKitCertificationCaseRow {
    fn from(case: WorthQueryConsumerKitCertificationCase) -> Self {
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

impl WorthQueryConsumerKitCertificationCaseRow {
    pub fn family(&self) -> super::family::WorthQueryConsumerKitFamilyName {
        self.family
    }

    pub fn case_id(&self) -> &'static str {
        self.case_id
    }

    pub fn tier(&self) -> WorthQueryConsumerKitCertificationTier {
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

impl WorthQueryConsumerKitHostileCertification {
    pub(crate) fn derive(
        rows: &[WorthQueryConsumerKitFamilyClosureRow],
        docs_agreement: &WorthQueryConsumerKitDocsAgreement,
        residue: &WorthQueryConsumerKitReferenceResidue,
    ) -> Self {
        let case_rows = required_consumer_kit_certification_cases()
            .into_iter()
            .map(WorthQueryConsumerKitCertificationCaseRow::from)
            .collect::<Vec<_>>();
        let missing_case_ids = case_rows
            .iter()
            .filter(|case| !case.satisfied())
            .map(WorthQueryConsumerKitCertificationCaseRow::case_id)
            .collect::<Vec<_>>();
        let status = if rows.iter().all(|row| {
            row.status() == WorthQueryMilestoneClosureStatus::Closed
                && !row.evidence_digest().is_empty()
        }) && missing_case_ids.is_empty()
            && docs_agreement.agrees()
            && residue.is_query_owned_clean()
        {
            WorthQueryMilestoneClosureStatus::Closed
        } else if rows
            .iter()
            .any(|row| row.status() != WorthQueryMilestoneClosureStatus::Open)
            || docs_agreement.agrees()
            || residue.is_query_owned_clean()
        {
            WorthQueryMilestoneClosureStatus::Partial
        } else {
            WorthQueryMilestoneClosureStatus::Open
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

    pub fn status(&self) -> WorthQueryMilestoneClosureStatus {
        self.status
    }

    pub fn case_rows(&self) -> &[WorthQueryConsumerKitCertificationCaseRow] {
        &self.case_rows
    }

    pub fn missing_case_ids(&self) -> &[&'static str] {
        &self.missing_case_ids
    }

    pub fn certification_digest(&self) -> &str {
        self.certification_identity.as_str()
    }

    pub fn certification_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.certification_identity
    }
}
