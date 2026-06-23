use crate::ForgeQueryEvidenceIdentity;

use super::docs_report::ForgeQueryConsumerKitDocsFamilyRow;
use super::evidence::consumer_kit_docs_agreement_identity;
use super::family::ForgeQueryConsumerKitFamilyName;

const AI_README: &str = include_str!("../../../../docs/AI_README.md");
const TEST_REQUIREMENTS: &str =
    include_str!("../../../../../../_docs/forge-query/test-requirements.md");
const MILESTONE_CLOSEOUT: &str =
    include_str!("../../../../../../_docs/forge-query/milestone-9.8-closeout.md");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerKitDocsAgreement {
    support_families: Vec<ForgeQueryConsumerKitFamilyName>,
    documented_families: Vec<ForgeQueryConsumerKitFamilyName>,
    family_rows: Vec<ForgeQueryConsumerKitDocsFamilyRow>,
    ordinary_path_language_present: bool,
    agreement_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryConsumerKitDocsAgreement {
    #[cfg(test)]
    pub(crate) fn new(
        support_families: Vec<ForgeQueryConsumerKitFamilyName>,
        documented_families: Vec<ForgeQueryConsumerKitFamilyName>,
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
                ForgeQueryConsumerKitDocsFamilyRow::derive(
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
            .map(ForgeQueryConsumerKitDocsFamilyRow::family)
            .collect::<Vec<_>>();
        let ordinary_path_language_present = family_rows
            .iter()
            .all(ForgeQueryConsumerKitDocsFamilyRow::ordinary_path_present);
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

    pub fn support_families(&self) -> &[ForgeQueryConsumerKitFamilyName] {
        &self.support_families
    }

    pub fn documented_families(&self) -> &[ForgeQueryConsumerKitFamilyName] {
        &self.documented_families
    }

    pub fn family_rows(&self) -> &[ForgeQueryConsumerKitDocsFamilyRow] {
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
                .all(ForgeQueryConsumerKitDocsFamilyRow::agrees)
    }

    pub fn agreement_digest(&self) -> &str {
        self.agreement_identity.as_str()
    }

    pub fn agreement_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.agreement_identity
    }
}
