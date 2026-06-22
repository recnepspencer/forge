use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionRuntimeBackedSupport {
    Admitted,
    Denied,
}

impl QuerySubscriptionRuntimeBackedSupport {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionActiveLifecycleSupport {
    Admitted,
    Denied,
}

impl QuerySubscriptionActiveLifecycleSupport {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionLifecycleCloseoutSupport {
    Admitted,
    Denied,
}

impl QuerySubscriptionLifecycleCloseoutSupport {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionDurableSupport {
    ExplicitDebt,
}

impl QuerySubscriptionDurableSupport {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExplicitDebt => "explicit_debt",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportProfile {
    runtime_backed_support: QuerySubscriptionRuntimeBackedSupport,
    active_lifecycle_support: QuerySubscriptionActiveLifecycleSupport,
    lifecycle_closeout_support: QuerySubscriptionLifecycleCloseoutSupport,
    durable_support: QuerySubscriptionDurableSupport,
    source_identity: ForgeQueryEvidenceIdentity,
    profile_identity: ForgeQueryEvidenceIdentity,
}

impl QuerySubscriptionSupportProfile {
    pub(crate) fn admitted(source_identity: &ForgeQueryEvidenceIdentity) -> Self {
        Self::new(
            QuerySubscriptionRuntimeBackedSupport::Admitted,
            QuerySubscriptionActiveLifecycleSupport::Admitted,
            QuerySubscriptionLifecycleCloseoutSupport::Admitted,
            source_identity,
        )
    }

    pub(crate) fn denied(source_identity: &ForgeQueryEvidenceIdentity) -> Self {
        Self::new(
            QuerySubscriptionRuntimeBackedSupport::Denied,
            QuerySubscriptionActiveLifecycleSupport::Denied,
            QuerySubscriptionLifecycleCloseoutSupport::Denied,
            source_identity,
        )
    }

    pub(crate) fn active_runtime_admitted(source_identity: &ForgeQueryEvidenceIdentity) -> Self {
        Self::new(
            QuerySubscriptionRuntimeBackedSupport::Admitted,
            QuerySubscriptionActiveLifecycleSupport::Admitted,
            QuerySubscriptionLifecycleCloseoutSupport::Admitted,
            source_identity,
        )
    }

    fn new(
        runtime_backed_support: QuerySubscriptionRuntimeBackedSupport,
        active_lifecycle_support: QuerySubscriptionActiveLifecycleSupport,
        lifecycle_closeout_support: QuerySubscriptionLifecycleCloseoutSupport,
        source_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        let durable_support = QuerySubscriptionDurableSupport::ExplicitDebt;
        let profile_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_support_profile_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("runtime_backed"),
            runtime_backed_support.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("active_lifecycle"),
            active_lifecycle_support.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("lifecycle_closeout"),
            lifecycle_closeout_support.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("durable"),
            durable_support.as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("source"), source_identity)
        .seal();
        Self {
            runtime_backed_support,
            active_lifecycle_support,
            lifecycle_closeout_support,
            durable_support,
            source_identity: source_identity.clone(),
            profile_identity,
        }
    }

    pub fn runtime_backed_support(&self) -> &QuerySubscriptionRuntimeBackedSupport {
        &self.runtime_backed_support
    }

    pub fn active_lifecycle_support(&self) -> &QuerySubscriptionActiveLifecycleSupport {
        &self.active_lifecycle_support
    }

    pub fn lifecycle_closeout_support(&self) -> &QuerySubscriptionLifecycleCloseoutSupport {
        &self.lifecycle_closeout_support
    }

    pub fn durable_support(&self) -> &QuerySubscriptionDurableSupport {
        &self.durable_support
    }

    pub fn source_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.source_identity
    }

    pub fn profile_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.profile_identity
    }
}
