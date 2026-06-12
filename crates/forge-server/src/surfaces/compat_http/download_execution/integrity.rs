use crate::{
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode, ForgeServerReadValidator,
};

use super::super::binary_digest::stable_byte_digest;
use super::{ForgeServerBinaryDownloadRequest, ForgeServerBinaryEgressSession};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerBinaryIntegrityDigest {
    full_representation_digest: String,
    selected_representation_digest: String,
    validator_entity_tag: String,
    selected_start: usize,
    selected_end_exclusive: usize,
    total_bytes: usize,
    head_only: bool,
    canonical_digest: String,
}

impl ForgeServerBinaryIntegrityDigest {
    pub(crate) fn project(session: &ForgeServerBinaryEgressSession) -> Self {
        Self::project_for_validation(
            session.download_request(),
            session.validator(),
            session.selected_start(),
            session.selected_end_exclusive(),
            session.head_only(),
        )
    }

    pub(crate) fn project_for_validation(
        download: &ForgeServerBinaryDownloadRequest,
        validator: &ForgeServerReadValidator,
        selected_start: usize,
        selected_end_exclusive: usize,
        head_only: bool,
    ) -> Self {
        let full_representation_digest = stable_byte_digest(download.body_bytes());
        let selected_representation_digest =
            stable_byte_digest(&download.body_bytes()[selected_start..selected_end_exclusive]);
        let validator_entity_tag = validator.entity_tag().to_string();
        let canonical_digest = format!(
            "compat-http-binary-integrity-v1|full={}|selected={}|validator={}|span={}-{}|total={}|head_only={}",
            full_representation_digest,
            selected_representation_digest,
            validator_entity_tag,
            selected_start,
            selected_end_exclusive,
            download.body_bytes().len(),
            head_only,
        );
        Self {
            full_representation_digest,
            selected_representation_digest,
            validator_entity_tag,
            selected_start,
            selected_end_exclusive,
            total_bytes: download.body_bytes().len(),
            head_only,
            canonical_digest,
        }
    }

    pub(crate) fn verify_resume_expectation(
        &self,
        expected: &Self,
        diagnostics_profile: forge_foundational::DiagnosticRichnessProfile,
    ) -> Result<(), ForgeServerQueryHandoffDenial> {
        if self.full_representation_digest != expected.full_representation_digest {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
                diagnostics_profile,
                format!(
                    "resume integrity digest mismatch: expected full digest `{}` but observed `{}`",
                    expected.full_representation_digest, self.full_representation_digest
                ),
            ));
        }
        if self.validator_entity_tag != expected.validator_entity_tag {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
                diagnostics_profile,
                format!(
                    "resume validator mismatch: expected `{}` but observed `{}`",
                    expected.validator_entity_tag, self.validator_entity_tag
                ),
            ));
        }
        Ok(())
    }

    pub fn full_representation_digest(&self) -> &str {
        &self.full_representation_digest
    }

    pub fn selected_representation_digest(&self) -> &str {
        &self.selected_representation_digest
    }

    pub fn validator_entity_tag(&self) -> &str {
        &self.validator_entity_tag
    }

    pub fn selected_start(&self) -> usize {
        self.selected_start
    }

    pub fn selected_end_exclusive(&self) -> usize {
        self.selected_end_exclusive
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn head_only(&self) -> bool {
        self.head_only
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
