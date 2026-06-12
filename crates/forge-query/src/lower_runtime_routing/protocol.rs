use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeSeamKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeSubjectIdentity {
    evidence_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeSubjectIdentity {
    pub(crate) fn compose(
        subject_family: impl AsRef<str>,
    ) -> ForgeQueryLowerRuntimeSubjectIdentityEncoder {
        ForgeQueryLowerRuntimeSubjectIdentityEncoder {
            encoder: ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::LowerRuntimeCapabilitySubject,
            )
            .field_shape(ForgeQueryEvidenceTag::new("subject_family"), subject_family),
        }
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.evidence_identity
    }

    pub fn as_str(&self) -> &str {
        self.evidence_identity.as_ref()
    }
}

impl AsRef<str> for ForgeQueryLowerRuntimeSubjectIdentity {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[must_use]
pub struct ForgeQueryLowerRuntimeSubjectIdentityEncoder {
    encoder: crate::evidence_identity::ForgeQueryEvidenceIdentityEncoder,
}

impl ForgeQueryLowerRuntimeSubjectIdentityEncoder {
    pub(crate) fn field_identity(
        mut self,
        tag: ForgeQueryEvidenceTag,
        value: impl AsRef<str>,
    ) -> Self {
        self.encoder = self.encoder.field_identity(tag, value);
        self
    }

    pub(crate) fn field_evidence_identity(
        mut self,
        tag: ForgeQueryEvidenceTag,
        value: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        self.encoder = self.encoder.field_evidence_identity(tag, value);
        self
    }

    pub(crate) fn field_shape(
        mut self,
        tag: ForgeQueryEvidenceTag,
        value: impl AsRef<str>,
    ) -> Self {
        self.encoder = self.encoder.field_shape(tag, value);
        self
    }

    pub(crate) fn field_usize(mut self, tag: ForgeQueryEvidenceTag, value: usize) -> Self {
        self.encoder = self.encoder.field_usize(tag, value);
        self
    }

    pub(crate) fn seal(self) -> ForgeQueryLowerRuntimeSubjectIdentity {
        ForgeQueryLowerRuntimeSubjectIdentity {
            evidence_identity: self.encoder.seal(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeCapabilityRequest {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    route_kind: ForgeQueryLowerRuntimeRouteKind,
    authority_owner: ForgeQueryLowerRuntimeAuthorityOwner,
    capability_label: String,
    subject_identity: ForgeQueryLowerRuntimeSubjectIdentity,
    request_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeCapabilityRequest {
    pub(crate) fn new(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        route_kind: ForgeQueryLowerRuntimeRouteKind,
        authority_owner: ForgeQueryLowerRuntimeAuthorityOwner,
        capability_label: impl Into<String>,
        subject_identity: ForgeQueryLowerRuntimeSubjectIdentity,
    ) -> Self {
        let capability_label = capability_label.into();
        let request_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeCapabilityRequest)
                .field_shape(ForgeQueryEvidenceTag::new("seam"), seam_key.as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("route_kind"),
                    route_kind.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("owner"),
                    authority_owner.as_str(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("capability"), &capability_label)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("subject"),
                    subject_identity.evidence_identity(),
                )
                .seal();
        Self {
            seam_key,
            route_kind,
            authority_owner,
            capability_label,
            subject_identity,
            request_identity,
        }
    }

    pub fn seam_key(&self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn route_kind(&self) -> ForgeQueryLowerRuntimeRouteKind {
        self.route_kind
    }

    pub fn authority_owner(&self) -> ForgeQueryLowerRuntimeAuthorityOwner {
        self.authority_owner
    }

    pub fn capability_label(&self) -> &str {
        &self.capability_label
    }

    pub fn subject_digest(&self) -> &str {
        self.subject_identity.as_str()
    }

    pub fn subject_identity(&self) -> &ForgeQueryLowerRuntimeSubjectIdentity {
        &self.subject_identity
    }

    pub fn request_digest(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn request_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn drift_from_contract(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        route_kind: ForgeQueryLowerRuntimeRouteKind,
        authority_owner: ForgeQueryLowerRuntimeAuthorityOwner,
        capability_label: &str,
        subject_identity: &ForgeQueryLowerRuntimeSubjectIdentity,
    ) -> Option<String> {
        if self.seam_key != seam_key {
            return Some("lower-runtime capability request seam key drifted".to_string());
        }
        if self.route_kind != route_kind {
            return Some("lower-runtime capability request route kind drifted".to_string());
        }
        if self.authority_owner != authority_owner {
            return Some("lower-runtime capability request authority owner drifted".to_string());
        }
        if self.capability_label != capability_label {
            return Some("lower-runtime capability request capability label drifted".to_string());
        }
        if &self.subject_identity != subject_identity {
            return Some("lower-runtime capability request subject identity drifted".to_string());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_digest_binds_seam_route_owner_and_subject() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "write-authority",
            ForgeQueryLowerRuntimeSubjectIdentity::compose("test-subject")
                .field_identity(ForgeQueryEvidenceTag::new("test_subject"), "subject-1")
                .seal(),
        );

        assert_eq!(
            request.seam_key(),
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution
        );
        assert_eq!(
            request.route_kind(),
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning
        );
        assert_eq!(
            request.authority_owner(),
            ForgeQueryLowerRuntimeAuthorityOwner::Query
        );
        assert_eq!(request.capability_label(), "write-authority");
        assert_eq!(
            request.subject_identity().evidence_identity().scope(),
            ForgeQueryEvidenceScope::LowerRuntimeCapabilitySubject
        );
        assert!(!request.request_digest().is_empty());
    }
}
