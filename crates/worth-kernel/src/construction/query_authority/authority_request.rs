use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQueryAuthorityRequest {
    subject: String,
    purpose: String,
    authority_basis_digest: String,
    request_digest: String,
}

impl PrimitiveConstructionQueryAuthorityRequest {
    pub(crate) fn projection_consumption_surface(
        subject: impl Into<String>,
        authority_basis_digest: impl Into<String>,
    ) -> Self {
        Self::new(
            subject,
            "projection-consumption-surface",
            authority_basis_digest,
        )
    }

    #[cfg(test)]
    pub(crate) fn authority_probe(subject: impl Into<String>) -> Self {
        Self::new(subject, "authority-probe", "authority-probe-no-basis")
    }

    fn new(
        subject: impl Into<String>,
        purpose: impl Into<String>,
        authority_basis_digest: impl Into<String>,
    ) -> Self {
        let subject = subject.into();
        let purpose = purpose.into();
        let authority_basis_digest = authority_basis_digest.into();
        let request_digest = digest_owned_parts(&[
            "primitive-construction-query-authority-request".to_string(),
            format!("subject:{subject}"),
            format!("purpose:{purpose}"),
            format!("authority-basis:{authority_basis_digest}"),
        ]);
        Self {
            subject,
            purpose,
            authority_basis_digest,
            request_digest,
        }
    }

    pub(crate) fn subject(&self) -> &str {
        &self.subject
    }

    pub(crate) fn purpose(&self) -> &str {
        &self.purpose
    }

    pub(crate) fn authority_basis_digest(&self) -> &str {
        &self.authority_basis_digest
    }

    pub(crate) fn request_digest(&self) -> &str {
        &self.request_digest
    }
}
