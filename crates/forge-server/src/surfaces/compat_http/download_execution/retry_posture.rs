#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerBinaryRetryPosture {
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

impl ForgeServerBinaryRetryPosture {
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
