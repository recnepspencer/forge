use crate::capability::{UiIntentBoolean, UiIntentPayloadFieldKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentConfirmationContractIdentityError {
    InvalidIdentity,
}

pub struct UiIntentConfirmationContract {
    policy_identity: Box<str>,
    source: UiAuthoredIntentConfirmationSource,
}

enum UiAuthoredIntentConfirmationSource {
    NotRequired,
    ApplicationBoolean(Box<str>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiResolvedIntentConfirmationContract {
    policy_identity: Box<str>,
    source: UiResolvedIntentConfirmationSource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiResolvedIntentConfirmationSource {
    NotRequired,
    ApplicationBoolean(super::UiIntentApplicationFactSlot),
}

impl UiIntentConfirmationContract {
    pub fn not_required(
        policy_identity: impl Into<Box<str>>,
    ) -> Result<Self, UiIntentConfirmationContractIdentityError> {
        Self::new(
            policy_identity,
            UiAuthoredIntentConfirmationSource::NotRequired,
        )
    }

    pub fn application_fact(
        policy_identity: impl Into<Box<str>>,
        fact: &super::UiIntentApplicationFact<UiIntentBoolean>,
    ) -> Result<Self, UiIntentConfirmationContractIdentityError> {
        Self::new(
            policy_identity,
            UiAuthoredIntentConfirmationSource::ApplicationBoolean(fact.identity().into()),
        )
    }

    fn new(
        policy_identity: impl Into<Box<str>>,
        source: UiAuthoredIntentConfirmationSource,
    ) -> Result<Self, UiIntentConfirmationContractIdentityError> {
        let policy_identity = policy_identity.into();
        if !super::valid_intent_identity(&policy_identity) {
            return Err(UiIntentConfirmationContractIdentityError::InvalidIdentity);
        }
        Ok(Self {
            policy_identity,
            source,
        })
    }

    pub fn policy_identity(&self) -> &str {
        &self.policy_identity
    }

    pub(crate) fn into_dsl(self) -> worth_ui_dsl::WorthUiIntentConfirmationContractSpec {
        match self.source {
            UiAuthoredIntentConfirmationSource::NotRequired => {
                worth_ui_dsl::WorthUiIntentConfirmationContractSpec::not_required(
                    self.policy_identity,
                )
            }
            UiAuthoredIntentConfirmationSource::ApplicationBoolean(fact) => {
                worth_ui_dsl::WorthUiIntentConfirmationContractSpec::application_boolean(
                    self.policy_identity,
                    fact,
                )
            }
        }
    }
}

pub(crate) fn resolve_confirmation_contract(
    declaration: &str,
    spec: &worth_ui_dsl::WorthUiIntentConfirmationContractSpec,
    application_facts: &super::UiIntentApplicationFactPlan,
) -> Result<UiResolvedIntentConfirmationContract, super::UiIntentCatalogPreparationDenial> {
    let source = match spec.source().application_fact() {
        None => UiResolvedIntentConfirmationSource::NotRequired,
        Some(identity) => {
            let fact = application_facts.get(identity).ok_or_else(|| {
                super::UiIntentCatalogPreparationDenial::UnknownConfirmationApplicationFact {
                    declaration: declaration.into(),
                    fact: identity.into(),
                }
            })?;
            if fact.kind() != UiIntentPayloadFieldKind::Boolean {
                return Err(
                    super::UiIntentCatalogPreparationDenial::
                        ConfirmationApplicationFactKindMismatch {
                            declaration: declaration.into(),
                            fact: identity.into(),
                            observed: fact.kind(),
                        },
                );
            }
            UiResolvedIntentConfirmationSource::ApplicationBoolean(fact.slot())
        }
    };
    Ok(UiResolvedIntentConfirmationContract {
        policy_identity: spec.policy_identity().into(),
        source,
    })
}

impl UiResolvedIntentConfirmationContract {
    pub(crate) fn policy_identity(&self) -> &str {
        &self.policy_identity
    }

    pub(crate) const fn source(&self) -> &UiResolvedIntentConfirmationSource {
        &self.source
    }
}
