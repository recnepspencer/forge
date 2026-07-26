#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactOwnershipContract {
    NotDeclared,
    DomainPayload {
        payload_owner: String,
        provider_family: String,
    },
}

impl WorthQueryArtifactOwnershipContract {
    pub fn domain_payload(
        payload_owner: impl Into<String>,
        provider_family: impl Into<String>,
    ) -> Self {
        Self::DomainPayload {
            payload_owner: payload_owner.into(),
            provider_family: provider_family.into(),
        }
    }

    pub fn payload_owner(&self) -> Option<&str> {
        match self {
            Self::NotDeclared => None,
            Self::DomainPayload { payload_owner, .. } => Some(payload_owner),
        }
    }

    pub fn provider_family(&self) -> Option<&str> {
        match self {
            Self::NotDeclared => None,
            Self::DomainPayload {
                provider_family, ..
            } => Some(provider_family),
        }
    }
}
