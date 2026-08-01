#[path = "operability_contract/resolution.rs"]
mod resolution;

pub(crate) use resolution::resolve_operability_contract;

use crate::capability::UiIntentBoolean;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentOperabilityDependencyAxis {
    Mutability,
    Readiness,
    Policy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentOperabilityContractIdentityError {
    InvalidIdentity,
}

pub struct UiIntentOperabilityContract {
    identity: Box<str>,
    mutability: UiIntentMutabilitySource,
    readiness: UiIntentReadinessSource,
    policy: UiIntentPolicySource,
}

pub struct UiIntentMutabilitySource {
    source: UiAuthoredIntentMutabilitySource,
}

enum UiAuthoredIntentMutabilitySource {
    ApplicationBoolean(Box<str>),
    ProjectionReadonly(Box<str>),
    CommittedDraft,
}

pub struct UiIntentReadinessSource {
    source: UiAuthoredIntentReadinessSource,
}

enum UiAuthoredIntentReadinessSource {
    ApplicationBoolean(Box<str>),
    Projection(Box<str>),
    CommittedDraft,
}

pub struct UiIntentPolicySource {
    fact: Box<str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiResolvedIntentOperabilityContract {
    identity: Box<str>,
    mutability: UiResolvedIntentMutabilitySource,
    readiness: UiResolvedIntentReadinessSource,
    policy: UiResolvedIntentPolicySource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiResolvedIntentMutabilitySource {
    ApplicationBoolean(super::UiIntentApplicationFactSlot),
    ProjectionReadonly {
        identity: worth_ui_query_binding::WorthUiQueryViewIdentity,
        slot: worth_ui_query_binding::UiProjectionInputSlot,
    },
    CommittedDraft,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiResolvedIntentReadinessSource {
    ApplicationBoolean(super::UiIntentApplicationFactSlot),
    Projection {
        identity: worth_ui_query_binding::WorthUiQueryViewIdentity,
        slot: worth_ui_query_binding::UiProjectionInputSlot,
    },
    CommittedDraft,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiResolvedIntentPolicySource {
    slot: super::UiIntentApplicationFactSlot,
}

impl UiIntentOperabilityContract {
    pub fn new(
        identity: impl Into<Box<str>>,
        mutability: UiIntentMutabilitySource,
        readiness: UiIntentReadinessSource,
        policy: UiIntentPolicySource,
    ) -> Result<Self, UiIntentOperabilityContractIdentityError> {
        let identity = identity.into();
        if !super::valid_intent_identity(&identity) {
            return Err(UiIntentOperabilityContractIdentityError::InvalidIdentity);
        }
        Ok(Self {
            identity,
            mutability,
            readiness,
            policy,
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn into_dsl(self) -> worth_ui_dsl::WorthUiIntentOperabilityContractSpec {
        worth_ui_dsl::WorthUiIntentOperabilityContractSpec::new(
            self.identity,
            self.mutability.into_dsl(),
            self.readiness.into_dsl(),
            self.policy.into_dsl(),
        )
    }
}

impl UiIntentMutabilitySource {
    pub fn application_fact(fact: &super::UiIntentApplicationFact<UiIntentBoolean>) -> Self {
        Self {
            source: UiAuthoredIntentMutabilitySource::ApplicationBoolean(fact.identity().into()),
        }
    }

    pub fn readonly_projection(
        projection: &worth_ui_query_binding::WorthUiQueryViewIdentity,
    ) -> Self {
        Self {
            source: UiAuthoredIntentMutabilitySource::ProjectionReadonly(
                projection.as_str().into(),
            ),
        }
    }

    pub const fn committed_draft() -> Self {
        Self {
            source: UiAuthoredIntentMutabilitySource::CommittedDraft,
        }
    }

    fn into_dsl(self) -> worth_ui_dsl::WorthUiIntentMutabilitySourceSpec {
        match self.source {
            UiAuthoredIntentMutabilitySource::ApplicationBoolean(fact) => {
                worth_ui_dsl::WorthUiIntentMutabilitySourceSpec::application_boolean(fact)
            }
            UiAuthoredIntentMutabilitySource::ProjectionReadonly(projection) => {
                worth_ui_dsl::WorthUiIntentMutabilitySourceSpec::projection_readonly(projection)
            }
            UiAuthoredIntentMutabilitySource::CommittedDraft => {
                worth_ui_dsl::WorthUiIntentMutabilitySourceSpec::committed_draft()
            }
        }
    }
}

impl UiIntentReadinessSource {
    pub fn application_fact(fact: &super::UiIntentApplicationFact<UiIntentBoolean>) -> Self {
        Self {
            source: UiAuthoredIntentReadinessSource::ApplicationBoolean(fact.identity().into()),
        }
    }

    pub fn projection(projection: &worth_ui_query_binding::WorthUiQueryViewIdentity) -> Self {
        Self {
            source: UiAuthoredIntentReadinessSource::Projection(projection.as_str().into()),
        }
    }

    pub const fn committed_draft() -> Self {
        Self {
            source: UiAuthoredIntentReadinessSource::CommittedDraft,
        }
    }

    fn into_dsl(self) -> worth_ui_dsl::WorthUiIntentReadinessSourceSpec {
        match self.source {
            UiAuthoredIntentReadinessSource::ApplicationBoolean(fact) => {
                worth_ui_dsl::WorthUiIntentReadinessSourceSpec::application_boolean(fact)
            }
            UiAuthoredIntentReadinessSource::Projection(projection) => {
                worth_ui_dsl::WorthUiIntentReadinessSourceSpec::projection(projection)
            }
            UiAuthoredIntentReadinessSource::CommittedDraft => {
                worth_ui_dsl::WorthUiIntentReadinessSourceSpec::committed_draft()
            }
        }
    }
}

impl UiIntentPolicySource {
    pub fn application_fact(fact: &super::UiIntentApplicationFact<UiIntentBoolean>) -> Self {
        Self {
            fact: fact.identity().into(),
        }
    }

    fn into_dsl(self) -> worth_ui_dsl::WorthUiIntentPolicySourceSpec {
        worth_ui_dsl::WorthUiIntentPolicySourceSpec::application_boolean(self.fact)
    }
}

impl UiResolvedIntentOperabilityContract {
    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) const fn mutability(&self) -> &UiResolvedIntentMutabilitySource {
        &self.mutability
    }

    pub(crate) const fn readiness(&self) -> &UiResolvedIntentReadinessSource {
        &self.readiness
    }

    pub(crate) const fn policy(&self) -> &UiResolvedIntentPolicySource {
        &self.policy
    }
}

impl UiResolvedIntentPolicySource {
    pub(crate) const fn slot(&self) -> super::UiIntentApplicationFactSlot {
        self.slot
    }
}
