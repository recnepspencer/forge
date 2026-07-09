use crate::WorthQueryEvidenceIdentity;

use super::docs_report::WorthQueryConsumerKitDocsFamilyRow;
use super::evidence::consumer_kit_docs_agreement_identity;
use super::family::WorthQueryConsumerKitFamilyName;

const AI_README: &str = include_str!("../../../../docs/AI_README.md");
const TEST_REQUIREMENTS: &str =
    include_str!("../../../../../../_docs/worth-query/test-requirements.md");
const MILESTONE_CLOSEOUT: &str =
    include_str!("../../../../../../_docs/worth-query/milestone-9.8-closeout.md");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerKitDocsAgreement {
    support_families: Vec<WorthQueryConsumerKitFamilyName>,
    documented_families: Vec<WorthQueryConsumerKitFamilyName>,
    family_rows: Vec<WorthQueryConsumerKitDocsFamilyRow>,
    ordinary_path_language_present: bool,
    agreement_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryConsumerKitDocsAgreement {
    #[cfg(test)]
    pub(crate) fn new(
        support_families: Vec<WorthQueryConsumerKitFamilyName>,
        documented_families: Vec<WorthQueryConsumerKitFamilyName>,
        ordinary_path_language_present: bool,
    ) -> Self {
        let family_rows = Vec::new();
        let agreement_identity = consumer_kit_docs_agreement_identity(
            &support_families,
            &documented_families,
            &family_rows,
            ordinary_path_language_present,
        );
        Self {
            support_families,
            documented_families,
            family_rows,
            ordinary_path_language_present,
            agreement_identity,
        }
    }

    pub(crate) fn current() -> Self {
        let families = super::evidence::required_consumer_kit_families().to_vec();
        let family_rows = families
            .iter()
            .copied()
            .map(|family| {
                WorthQueryConsumerKitDocsFamilyRow::derive(
                    family,
                    AI_README,
                    TEST_REQUIREMENTS,
                    MILESTONE_CLOSEOUT,
                )
            })
            .collect::<Vec<_>>();
        let documented_families = family_rows
            .iter()
            .filter(|row| {
                row.ai_readme_present() && row.test_requirements_present() && row.closeout_present()
            })
            .map(WorthQueryConsumerKitDocsFamilyRow::family)
            .collect::<Vec<_>>();
        let ordinary_path_language_present = family_rows
            .iter()
            .all(WorthQueryConsumerKitDocsFamilyRow::ordinary_path_present);
        let agreement_identity = consumer_kit_docs_agreement_identity(
            &families,
            &documented_families,
            &family_rows,
            ordinary_path_language_present,
        );
        Self {
            support_families: families,
            documented_families,
            family_rows,
            ordinary_path_language_present,
            agreement_identity,
        }
    }

    pub fn support_families(&self) -> &[WorthQueryConsumerKitFamilyName] {
        &self.support_families
    }

    pub fn documented_families(&self) -> &[WorthQueryConsumerKitFamilyName] {
        &self.documented_families
    }

    pub fn family_rows(&self) -> &[WorthQueryConsumerKitDocsFamilyRow] {
        &self.family_rows
    }

    pub fn ordinary_path_language_present(&self) -> bool {
        self.ordinary_path_language_present
    }

    pub fn agrees(&self) -> bool {
        self.ordinary_path_language_present
            && self.support_families == self.documented_families
            && self
                .family_rows
                .iter()
                .all(WorthQueryConsumerKitDocsFamilyRow::agrees)
    }

    pub fn agreement_digest(&self) -> &str {
        self.agreement_identity.as_str()
    }

    pub fn agreement_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.agreement_identity
    }
}
