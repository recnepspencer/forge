use crate::ForgeQueryEvidenceIdentity;

use super::evidence::consumer_kit_docs_family_row_identity;
use super::family::ForgeQueryConsumerKitFamilyName;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerKitDocsFamilyRow {
    family: ForgeQueryConsumerKitFamilyName,
    ai_readme_present: bool,
    test_requirements_present: bool,
    closeout_present: bool,
    ordinary_path_present: bool,
    family_obligation_present: bool,
    row_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryConsumerKitDocsFamilyRow {
    pub(super) fn derive(
        family: ForgeQueryConsumerKitFamilyName,
        ai_readme: &str,
        test_requirements: &str,
        closeout: &str,
    ) -> Self {
        let ai_readme_present = ai_readme.contains(family.as_str());
        let test_requirements_present = test_requirements.contains(family.as_str());
        let closeout_present = closeout.contains(family.as_str());
        let ordinary_path_present = [ai_readme, test_requirements, closeout]
            .iter()
            .all(|document| document.contains("ordinary downstream path"));
        let family_obligation_present =
            family_obligation_present(family, ai_readme, test_requirements, closeout);
        let row_identity = consumer_kit_docs_family_row_identity(
            family,
            ai_readme_present,
            test_requirements_present,
            closeout_present,
            ordinary_path_present,
            family_obligation_present,
        );
        Self {
            family,
            ai_readme_present,
            test_requirements_present,
            closeout_present,
            ordinary_path_present,
            family_obligation_present,
            row_identity,
        }
    }

    pub fn family(&self) -> ForgeQueryConsumerKitFamilyName {
        self.family
    }

    pub fn ai_readme_present(&self) -> bool {
        self.ai_readme_present
    }

    pub fn test_requirements_present(&self) -> bool {
        self.test_requirements_present
    }

    pub fn closeout_present(&self) -> bool {
        self.closeout_present
    }

    pub fn ordinary_path_present(&self) -> bool {
        self.ordinary_path_present
    }

    pub fn family_obligation_present(&self) -> bool {
        self.family_obligation_present
    }

    pub fn agrees(&self) -> bool {
        self.ai_readme_present
            && self.test_requirements_present
            && self.closeout_present
            && self.ordinary_path_present
            && self.family_obligation_present
    }

    pub fn row_digest(&self) -> &str {
        self.row_identity.as_str()
    }
}

fn family_obligation_present(
    family: ForgeQueryConsumerKitFamilyName,
    ai_readme: &str,
    test_requirements: &str,
    closeout: &str,
) -> bool {
    if family != ForgeQueryConsumerKitFamilyName::ConsumerResidueAudit {
        return true;
    }
    let combined = [ai_readme, test_requirements, closeout].join("\n");
    let direct_requirements_present = [
        "query_consumer_residue_audit",
        "without local residue classes",
        "local scanners",
        "not optional ergonomics",
    ]
    .iter()
    .all(|required| combined.contains(required));
    direct_requirements_present
        && combined.contains("Milestone `9.9`")
        && combined.contains("graph-obligation local ceremony audit")
}
