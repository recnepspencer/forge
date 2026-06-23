use crate::query_obligation_selection::public_facade::WorthQueryObligationSelectionMilestoneSixSeed;

use super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessInventorySeed {
    selected_obligation_count: usize,
    selected_registration_count: usize,
    execution_row_count: usize,
    authority_digests: Vec<String>,
    touch_descriptor_digests: Vec<String>,
    selected_registration_digests: Vec<String>,
    residue_manifest_digests: Vec<String>,
    execution_proof_digests: Vec<String>,
    adoption_manifest_digests: Vec<String>,
    selector_precision_report_digests: Vec<String>,
}

impl WorthGraphReadAccessInventorySeed {
    pub fn from_milestone_five_seed(
        seed: WorthQueryObligationSelectionMilestoneSixSeed,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        if seed.graph_read_access_planning_claimed() {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::GraphReadAccessPlanningAlreadyClaimed,
            ));
        }
        Self::from_validated_parts(WorthGraphReadAccessInventorySeedParts {
            selected_obligation_count: seed.selected_obligation_count(),
            selected_registration_count: seed.selected_registration_count(),
            execution_row_count: seed.execution_row_count(),
            authority_digests: seed.authority_digests().to_vec(),
            touch_descriptor_digests: seed.touch_descriptor_digests().to_vec(),
            selected_registration_digests: seed.selected_registration_digests().to_vec(),
            residue_manifest_digests: seed.residue_manifest_digests().to_vec(),
            execution_proof_digests: seed.execution_proof_digests().to_vec(),
            adoption_manifest_digests: seed.adoption_manifest_digests().to_vec(),
            selector_precision_report_digests: seed.selector_precision_report_digests().to_vec(),
        })
    }

    fn from_validated_parts(
        parts: WorthGraphReadAccessInventorySeedParts,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        validate_nonzero_count(
            parts.selected_obligation_count,
            WorthGraphReadAccessInventoryErrorKind::MissingSelectedObligations,
        )?;
        validate_nonzero_count(
            parts.selected_registration_count,
            WorthGraphReadAccessInventoryErrorKind::MissingSelectedRegistrations,
        )?;
        validate_nonzero_count(
            parts.execution_row_count,
            WorthGraphReadAccessInventoryErrorKind::MissingExecutionRows,
        )?;
        validate_digest_family(
            &parts.authority_digests,
            WorthGraphReadAccessInventoryErrorKind::MissingAuthorityDigest,
        )?;
        validate_digest_family(
            &parts.touch_descriptor_digests,
            WorthGraphReadAccessInventoryErrorKind::MissingTouchDescriptorDigest,
        )?;
        validate_digest_family(
            &parts.selected_registration_digests,
            WorthGraphReadAccessInventoryErrorKind::MissingSelectedRegistrationDigest,
        )?;
        validate_digest_family(
            &parts.residue_manifest_digests,
            WorthGraphReadAccessInventoryErrorKind::MissingResidueManifestDigest,
        )?;
        validate_digest_family(
            &parts.execution_proof_digests,
            WorthGraphReadAccessInventoryErrorKind::MissingExecutionProofDigest,
        )?;
        validate_digest_family(
            &parts.adoption_manifest_digests,
            WorthGraphReadAccessInventoryErrorKind::MissingAdoptionManifestDigest,
        )?;
        validate_digest_family(
            &parts.selector_precision_report_digests,
            WorthGraphReadAccessInventoryErrorKind::MissingSelectorPrecisionReportDigest,
        )?;
        validate_digest_count(
            parts.selected_obligation_count,
            &parts.authority_digests,
            WorthGraphReadAccessInventoryErrorKind::AuthorityDigestCountMismatch,
        )?;
        validate_digest_count(
            parts.selected_obligation_count,
            &parts.touch_descriptor_digests,
            WorthGraphReadAccessInventoryErrorKind::TouchDescriptorDigestCountMismatch,
        )?;
        validate_digest_count(
            parts.selected_registration_count,
            &parts.selected_registration_digests,
            WorthGraphReadAccessInventoryErrorKind::SelectedRegistrationDigestCountMismatch,
        )?;
        validate_digest_count(
            parts.selected_obligation_count,
            &parts.residue_manifest_digests,
            WorthGraphReadAccessInventoryErrorKind::ResidueManifestDigestCountMismatch,
        )?;
        validate_digest_count(
            parts.selected_obligation_count,
            &parts.execution_proof_digests,
            WorthGraphReadAccessInventoryErrorKind::ExecutionProofDigestCountMismatch,
        )?;
        validate_digest_count(
            parts.selected_obligation_count,
            &parts.adoption_manifest_digests,
            WorthGraphReadAccessInventoryErrorKind::AdoptionManifestDigestCountMismatch,
        )?;
        validate_digest_count(
            parts.selected_obligation_count,
            &parts.selector_precision_report_digests,
            WorthGraphReadAccessInventoryErrorKind::SelectorPrecisionReportDigestCountMismatch,
        )?;

        Ok(Self {
            selected_obligation_count: parts.selected_obligation_count,
            selected_registration_count: parts.selected_registration_count,
            execution_row_count: parts.execution_row_count,
            authority_digests: parts.authority_digests,
            touch_descriptor_digests: parts.touch_descriptor_digests,
            selected_registration_digests: parts.selected_registration_digests,
            residue_manifest_digests: parts.residue_manifest_digests,
            execution_proof_digests: parts.execution_proof_digests,
            adoption_manifest_digests: parts.adoption_manifest_digests,
            selector_precision_report_digests: parts.selector_precision_report_digests,
        })
    }

    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub const fn selected_registration_count(&self) -> usize {
        self.selected_registration_count
    }

    pub const fn execution_row_count(&self) -> usize {
        self.execution_row_count
    }

    pub fn authority_digests(&self) -> &[String] {
        &self.authority_digests
    }

    pub fn touch_descriptor_digests(&self) -> &[String] {
        &self.touch_descriptor_digests
    }

    pub fn selected_registration_digests(&self) -> &[String] {
        &self.selected_registration_digests
    }

    pub fn residue_manifest_digests(&self) -> &[String] {
        &self.residue_manifest_digests
    }

    pub fn execution_proof_digests(&self) -> &[String] {
        &self.execution_proof_digests
    }

    pub fn adoption_manifest_digests(&self) -> &[String] {
        &self.adoption_manifest_digests
    }

    pub fn selector_precision_report_digests(&self) -> &[String] {
        &self.selector_precision_report_digests
    }

    #[cfg(test)]
    pub(super) fn for_tests() -> Self {
        Self::from_validated_parts(WorthGraphReadAccessInventorySeedParts {
            selected_obligation_count: 2,
            selected_registration_count: 2,
            execution_row_count: 2,
            authority_digests: two_test_digests("authority"),
            touch_descriptor_digests: two_test_digests("touch"),
            selected_registration_digests: two_test_digests("registration"),
            residue_manifest_digests: two_test_digests("residue"),
            execution_proof_digests: two_test_digests("execution"),
            adoption_manifest_digests: two_test_digests("adoption"),
            selector_precision_report_digests: two_test_digests("precision"),
        })
        .expect("test seed parts must be valid")
    }

    #[cfg(test)]
    pub(super) fn from_parts_for_tests(
        parts: WorthGraphReadAccessInventorySeedParts,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::from_validated_parts(parts)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthGraphReadAccessInventorySeedParts {
    pub selected_obligation_count: usize,
    pub selected_registration_count: usize,
    pub execution_row_count: usize,
    pub authority_digests: Vec<String>,
    pub touch_descriptor_digests: Vec<String>,
    pub selected_registration_digests: Vec<String>,
    pub residue_manifest_digests: Vec<String>,
    pub execution_proof_digests: Vec<String>,
    pub adoption_manifest_digests: Vec<String>,
    pub selector_precision_report_digests: Vec<String>,
}

fn validate_nonzero_count(
    count: usize,
    error_kind: WorthGraphReadAccessInventoryErrorKind,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    if count == 0 {
        return Err(error(error_kind));
    }
    Ok(())
}

fn validate_digest_family(
    digests: &[String],
    error_kind: WorthGraphReadAccessInventoryErrorKind,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    if digests.is_empty() || digests.iter().any(|digest| digest.is_empty()) {
        return Err(error(error_kind));
    }
    Ok(())
}

fn validate_digest_count(
    expected_count: usize,
    digests: &[String],
    error_kind: WorthGraphReadAccessInventoryErrorKind,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    if digests.len() != expected_count {
        return Err(error(error_kind));
    }
    Ok(())
}

#[cfg(test)]
fn two_test_digests(prefix: &str) -> Vec<String> {
    vec![format!("{prefix}-a"), format!("{prefix}-b")]
}

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
