use crate::declaration::stable_text_digest;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphSessionLabel(Box<str>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiPreviewSessionIdentity(Box<str>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphSessionIdentityError {
    Empty,
}

impl UiGraphSessionLabel {
    pub fn new(value: impl Into<String>) -> Result<Self, UiGraphSessionIdentityError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(UiGraphSessionIdentityError::Empty);
        }
        Ok(Self(value.into_boxed_str()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl UiPreviewSessionIdentity {
    pub fn new(value: impl Into<String>) -> Result<Self, UiGraphSessionIdentityError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(UiGraphSessionIdentityError::Empty);
        }
        Ok(Self(value.into_boxed_str()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphWorldProfile {
    Authoritative,
    PreviewSessionLabel {
        session_label: UiGraphSessionLabel,
    },
    PreviewSessionIdentity {
        preview_session_identity: UiPreviewSessionIdentity,
    },
    BranchSessionLabel {
        session_label: UiGraphSessionLabel,
    },
    HotReloadCandidate {
        session_label: UiGraphSessionLabel,
    },
    Diagnostic {
        session_label: UiGraphSessionLabel,
    },
    HostObservation {
        session_label: UiGraphSessionLabel,
    },
    TestCertification {
        session_label: UiGraphSessionLabel,
    },
    SettledQueryBinding {
        view_binding_id: crate::capability::ViewBindingId,
        binding_reference: worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
    },
}

impl UiGraphWorldProfile {
    pub const fn authoritative() -> Self {
        Self::Authoritative
    }
    pub fn preview_session_label(session_label: UiGraphSessionLabel) -> Self {
        Self::PreviewSessionLabel { session_label }
    }
    pub fn preview_session_identity(preview_session_identity: UiPreviewSessionIdentity) -> Self {
        Self::PreviewSessionIdentity {
            preview_session_identity,
        }
    }
    pub fn branch_session_label(session_label: UiGraphSessionLabel) -> Self {
        Self::BranchSessionLabel { session_label }
    }
    pub fn hot_reload_candidate(session_label: UiGraphSessionLabel) -> Self {
        Self::HotReloadCandidate { session_label }
    }
    pub fn diagnostic(session_label: UiGraphSessionLabel) -> Self {
        Self::Diagnostic { session_label }
    }
    pub fn host_observation(session_label: UiGraphSessionLabel) -> Self {
        Self::HostObservation { session_label }
    }
    pub fn test_certification(session_label: UiGraphSessionLabel) -> Self {
        Self::TestCertification { session_label }
    }
    pub fn settled_query_fact(
        view_binding_id: crate::capability::ViewBindingId,
        fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ) -> Self {
        Self::settled_query_binding(view_binding_id, fact.binding_reference())
    }

    pub fn settled_query_binding(
        view_binding_id: crate::capability::ViewBindingId,
        binding_reference: &worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
    ) -> Self {
        Self::SettledQueryBinding {
            view_binding_id,
            binding_reference: binding_reference.clone(),
        }
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        match self {
            Self::Authoritative => stable_text_digest("graph-world:authoritative"),
            Self::PreviewSessionLabel { session_label } => {
                session_digest("preview-label", session_label.as_str())
            }
            Self::PreviewSessionIdentity {
                preview_session_identity,
            } => session_digest("preview-session", preview_session_identity.as_str()),
            Self::BranchSessionLabel { session_label } => {
                session_digest("branch-label", session_label.as_str())
            }
            Self::HotReloadCandidate { session_label } => {
                session_digest("hot-reload-candidate", session_label.as_str())
            }
            Self::Diagnostic { session_label } => {
                session_digest("diagnostic", session_label.as_str())
            }
            Self::HostObservation { session_label } => {
                session_digest("host-observation", session_label.as_str())
            }
            Self::TestCertification { session_label } => {
                session_digest("test-certification", session_label.as_str())
            }
            Self::SettledQueryBinding {
                view_binding_id,
                binding_reference,
            } => {
                stable_text_digest(view_binding_id.as_str())
                    ^ opaque_reference_digest(binding_reference).rotate_left(29)
            }
        }
    }

    pub(crate) fn comparison_family(&self) -> u64 {
        match self {
            Self::Authoritative => stable_text_digest("graph-world-family:authoritative"),
            Self::PreviewSessionLabel { .. } => {
                stable_text_digest("graph-world-family:preview-label")
            }
            Self::PreviewSessionIdentity { .. } => {
                stable_text_digest("graph-world-family:preview-session")
            }
            Self::BranchSessionLabel { .. } => {
                stable_text_digest("graph-world-family:branch-label")
            }
            Self::HotReloadCandidate { .. } => {
                stable_text_digest("graph-world-family:hot-reload-candidate")
            }
            Self::Diagnostic { .. } => stable_text_digest("graph-world-family:diagnostic"),
            Self::HostObservation { .. } => {
                stable_text_digest("graph-world-family:host-observation")
            }
            Self::TestCertification { .. } => {
                stable_text_digest("graph-world-family:test-certification")
            }
            Self::SettledQueryBinding { .. } => {
                stable_text_digest("graph-world-family:settled-query-binding")
            }
        }
    }
}

fn opaque_reference_digest(
    reference: &worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    reference.hash(&mut hasher);
    hasher.finish()
}

fn session_digest(role: &str, identity: &str) -> u64 {
    stable_text_digest(role) ^ stable_text_digest(identity).rotate_left(17)
}
