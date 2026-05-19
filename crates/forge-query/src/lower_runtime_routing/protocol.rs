use crate::identity::hash_parts;

use super::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeSeamKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeCapabilityRequest {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    route_kind: ForgeQueryLowerRuntimeRouteKind,
    authority_owner: ForgeQueryLowerRuntimeAuthorityOwner,
    capability_label: String,
    subject_digest: String,
    request_digest: String,
}

impl ForgeQueryLowerRuntimeCapabilityRequest {
    pub(crate) fn new(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        route_kind: ForgeQueryLowerRuntimeRouteKind,
        authority_owner: ForgeQueryLowerRuntimeAuthorityOwner,
        capability_label: impl Into<String>,
        subject_digest: impl Into<String>,
    ) -> Self {
        let capability_label = capability_label.into();
        let subject_digest = subject_digest.into();
        let request_digest = hash_parts(&[
            "lower_runtime_capability_request_v1".to_string(),
            format!("seam:{}", seam_key.as_str()),
            format!("route_kind:{}", route_kind.as_str()),
            format!("owner:{}", authority_owner.as_str()),
            format!("capability:{capability_label}"),
            format!("subject:{subject_digest}"),
        ]);
        Self {
            seam_key,
            route_kind,
            authority_owner,
            capability_label,
            subject_digest,
            request_digest,
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
        &self.subject_digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn drift_from_contract(
        &self,
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        route_kind: ForgeQueryLowerRuntimeRouteKind,
        authority_owner: ForgeQueryLowerRuntimeAuthorityOwner,
        capability_label: &str,
        subject_digest: &str,
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
        if self.subject_digest != subject_digest {
            return Some("lower-runtime capability request subject digest drifted".to_string());
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
            "subject-1",
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
        assert_eq!(request.subject_digest(), "subject-1");
        assert!(!request.request_digest().is_empty());
    }
}
