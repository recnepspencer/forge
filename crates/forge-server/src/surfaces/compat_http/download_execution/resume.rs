use super::{ForgeServerBinaryIntegrityDigest, ForgeServerBinarySessionResume};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerBinaryResumeRequest {
    session_resume: ForgeServerBinarySessionResume,
    expected_integrity: Option<ForgeServerBinaryIntegrityDigest>,
    require_restart_stable: bool,
    canonical_digest: String,
}

impl ForgeServerBinaryResumeRequest {
    pub fn resume_from(session_resume: ForgeServerBinarySessionResume) -> Self {
        let mut request = Self {
            session_resume,
            expected_integrity: None,
            require_restart_stable: false,
            canonical_digest: String::new(),
        };
        request.rebuild_digest();
        request
    }

    pub fn with_expected_integrity(
        mut self,
        expected_integrity: ForgeServerBinaryIntegrityDigest,
    ) -> Self {
        self.expected_integrity = Some(expected_integrity);
        self.rebuild_digest();
        self
    }

    pub fn require_restart_stable(mut self) -> Self {
        self.require_restart_stable = true;
        self.rebuild_digest();
        self
    }

    pub fn session_resume(&self) -> &ForgeServerBinarySessionResume {
        &self.session_resume
    }

    pub fn expected_integrity(&self) -> Option<&ForgeServerBinaryIntegrityDigest> {
        self.expected_integrity.as_ref()
    }

    pub fn require_restart_stable_claim(&self) -> bool {
        self.require_restart_stable
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    fn rebuild_digest(&mut self) {
        self.canonical_digest = format!(
            "compat-http-binary-resume-request-v1|session={}|expected_integrity={}|require_restart_stable={}",
            self.session_resume.canonical_digest(),
            self.expected_integrity
                .as_ref()
                .map_or("none", ForgeServerBinaryIntegrityDigest::canonical_digest),
            self.require_restart_stable,
        );
    }
}
