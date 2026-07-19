use crate::evidence_identity::WorthQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionSourceIdentity {
    identity: ProjectionSourceIdentityKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ProjectionSourceIdentityKind {
    Evidence(WorthQueryEvidenceIdentity),
    Artifact(String),
}

impl ProjectionSourceIdentity {
    pub fn from_evidence_identity(identity: WorthQueryEvidenceIdentity) -> Self {
        Self {
            identity: ProjectionSourceIdentityKind::Evidence(identity),
        }
    }

    pub fn artifact(identity: impl Into<String>) -> Self {
        Self {
            identity: ProjectionSourceIdentityKind::Artifact(identity.into()),
        }
    }

    pub fn as_str(&self) -> &str {
        match &self.identity {
            ProjectionSourceIdentityKind::Evidence(identity) => identity.as_str(),
            ProjectionSourceIdentityKind::Artifact(identity) => identity,
        }
    }

    pub fn evidence_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        match &self.identity {
            ProjectionSourceIdentityKind::Evidence(identity) => Some(identity),
            ProjectionSourceIdentityKind::Artifact(_) => None,
        }
    }
}

impl From<&str> for ProjectionSourceIdentity {
    fn from(value: &str) -> Self {
        Self::artifact(value)
    }
}

impl From<String> for ProjectionSourceIdentity {
    fn from(value: String) -> Self {
        Self::artifact(value)
    }
}

impl std::fmt::Display for ProjectionSourceIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionSourceReferenceIdentity {
    label: &'static str,
    identity: String,
}

impl ProjectionSourceReferenceIdentity {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn synthetic(label: &'static str, identity: impl Into<String>) -> Self {
        Self {
            label,
            identity: identity.into(),
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only(label: &'static str, identity: impl Into<String>) -> Self {
        Self::synthetic(label, identity)
    }
}
