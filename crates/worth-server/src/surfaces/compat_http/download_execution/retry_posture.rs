#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerBinaryRetryPosture {
    OrdinaryFullTransfer {
        canonical_digest: String,
    },
    OrdinaryRangedTransfer {
        canonical_digest: String,
    },
    SessionResume {
        previous_session_digest: String,
        expected_next_start: usize,
        restart_stable: bool,
        canonical_digest: String,
    },
}

impl WorthServerBinaryRetryPosture {
    pub(crate) fn ordinary(range_honored: bool) -> Self {
        if range_honored {
            Self::OrdinaryRangedTransfer {
                canonical_digest: "compat-http-binary-retry-posture-v1|ordinary_range".to_string(),
            }
        } else {
            Self::OrdinaryFullTransfer {
                canonical_digest: "compat-http-binary-retry-posture-v1|ordinary_full".to_string(),
            }
        }
    }

    pub(crate) fn resumed(
        previous_session_digest: impl Into<String>,
        expected_next_start: usize,
        restart_stable: bool,
    ) -> Self {
        let previous_session_digest = previous_session_digest.into();
        let canonical_digest = format!(
            "compat-http-binary-retry-posture-v1|resume_from={previous_session_digest}|expected_next_start={expected_next_start}|restart_stable={restart_stable}"
        );
        Self::SessionResume {
            previous_session_digest,
            expected_next_start,
            restart_stable,
            canonical_digest,
        }
    }

    pub fn is_resume(&self) -> bool {
        matches!(self, Self::SessionResume { .. })
    }

    pub fn restart_stable(&self) -> bool {
        matches!(
            self,
            Self::SessionResume {
                restart_stable: true,
                ..
            }
        )
    }

    pub fn expected_next_start(&self) -> Option<usize> {
        match self {
            Self::SessionResume {
                expected_next_start,
                ..
            } => Some(*expected_next_start),
            Self::OrdinaryFullTransfer { .. } | Self::OrdinaryRangedTransfer { .. } => None,
        }
    }

    pub fn previous_session_digest(&self) -> Option<&str> {
        match self {
            Self::SessionResume {
                previous_session_digest,
                ..
            } => Some(previous_session_digest),
            Self::OrdinaryFullTransfer { .. } | Self::OrdinaryRangedTransfer { .. } => None,
        }
    }

    pub fn canonical_digest(&self) -> &str {
        match self {
            Self::OrdinaryFullTransfer { canonical_digest }
            | Self::OrdinaryRangedTransfer { canonical_digest }
            | Self::SessionResume {
                canonical_digest, ..
            } => canonical_digest,
        }
    }
}

pub(super) fn derive_retry_posture(
    read: &crate::WorthServerCompatibilityRead,
    download: &crate::WorthServerBinaryDownloadRequest,
    selected_start: usize,
    selected_end_exclusive: usize,
    head_only: bool,
    range_honored: bool,
    diagnostics_profile: worth_foundational::DiagnosticRichnessProfile,
) -> Result<WorthServerBinaryRetryPosture, crate::WorthServerQueryHandoffDenial> {
    let Some(resume_request) = download.resume_request() else {
        return Ok(WorthServerBinaryRetryPosture::ordinary(range_honored));
    };
    if head_only {
        return Err(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "HEAD binary egress does not admit resumed byte delivery claims",
        ));
    }
    if resume_request.require_restart_stable_claim()
        && matches!(
            read.support_posture().durable_resume_support_posture(),
            worth_query::facade::runtime::WorthQueryLowerRuntimeSupportPosture::Deferred
                | worth_query::facade::runtime::WorthQueryLowerRuntimeSupportPosture::Forbidden
        )
    {
        return Err(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            format!(
                "restart-stable resume is not admitted for this binary delivery posture: `{}`",
                read.support_posture()
                    .durable_resume_support_posture()
                    .as_str()
            ),
        ));
    }
    let session_resume = resume_request.session_resume();
    if selected_start != session_resume.expected_next_start() {
        return Err(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            format!(
                "resume request expected byte {} but selected byte {}",
                session_resume.expected_next_start(),
                selected_start
            ),
        ));
    }
    if download.content_type() != session_resume.content_type() {
        return Err(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "resume request changed the canonical binary download artifact or policy story",
        ));
    }
    if download.authorization().canonical_digest() != session_resume.authorization_digest() {
        return Err(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "resume request widened the admitted authorization window",
        ));
    }
    if download.payload_digest() != session_resume.full_representation_digest() {
        return Err(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            format!(
                "resume integrity digest mismatch: expected full digest `{}` but observed `{}`",
                session_resume.full_representation_digest(),
                download.payload_digest()
            ),
        ));
    }
    if read.validator().entity_tag() != session_resume.validator_entity_tag() {
        return Err(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            format!(
                "resume validator mismatch: expected `{}` but observed `{}`",
                session_resume.validator_entity_tag(),
                read.validator().entity_tag()
            ),
        ));
    }
    if read.direct_context().workspace_digest() != session_resume.workspace_digest() {
        return Err(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "resume request crossed into a different workspace delivery context",
        ));
    }
    if read.direct_context().branch_digest() != session_resume.branch_digest() {
        return Err(crate::WorthServerQueryHandoffDenial::new(
            crate::WorthServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
            diagnostics_profile,
            "resume request crossed into a different branch delivery context",
        ));
    }
    if let Some(expected_integrity) = resume_request.expected_integrity() {
        crate::WorthServerBinaryIntegrityDigest::project_for_validation(
            download,
            read.validator(),
            selected_start,
            selected_end_exclusive,
            head_only,
        )
        .verify_resume_expectation(expected_integrity, diagnostics_profile)?;
    }
    Ok(WorthServerBinaryRetryPosture::resumed(
        session_resume.previous_session_digest(),
        session_resume.expected_next_start(),
        resume_request.require_restart_stable_claim(),
    ))
}
