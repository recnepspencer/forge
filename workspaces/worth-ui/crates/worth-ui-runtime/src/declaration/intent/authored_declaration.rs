use core::marker::PhantomData;

use crate::capability::{
    UiIntent, UiIntentPayload, UiIntentProductOutcome, UiSemanticInteractionFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentDeclarationConstructionError {
    InvalidIdentity,
    InteractionNotAccepted,
}

pub struct UiIntentDeclaration<
    I: UiIntent,
    Operability = UiIntentDeclarationOperabilityMissing,
    Confirmation = UiIntentDeclarationConfirmationMissing,
    Concurrency = UiIntentDeclarationConcurrencyMissing,
    Consequences = UiIntentDeclarationConsequencesMissing,
> {
    identity: Box<str>,
    interaction: UiSemanticInteractionFamily,
    payload_sources: Vec<worth_ui_dsl::WorthUiIntentPayloadSourceSpec>,
    operability: Operability,
    confirmation: Confirmation,
    concurrency: Concurrency,
    consequences: Consequences,
    intent: PhantomData<fn() -> I>,
}

pub struct UiIntentDeclarationOperabilityMissing {
    _sealed: (),
}

pub struct UiIntentDeclarationConfirmationMissing {
    _sealed: (),
}

pub struct UiIntentDeclarationConcurrencyMissing {
    _sealed: (),
}

pub struct UiIntentDeclarationConsequencesMissing {
    _sealed: (),
}

pub struct UiIntentDeclarationOperabilityBound {
    contract: super::UiIntentOperabilityContract,
}

pub struct UiIntentDeclarationConfirmationBound {
    contract: super::UiIntentConfirmationContract,
}

pub struct UiIntentDeclarationConcurrencyBound {
    scope: super::UiIntentConcurrencyScope,
}

pub struct UiIntentDeclarationConsequencesBound {
    contract: super::UiIntentConsequenceContract,
}

impl<I: UiIntent> UiIntentDeclaration<I> {
    pub fn activate(
        identity: impl Into<Box<str>>,
    ) -> Result<Self, UiIntentDeclarationConstructionError> {
        Self::for_interaction(identity, UiSemanticInteractionFamily::Activate)
    }

    pub fn edit_commit(
        identity: impl Into<Box<str>>,
    ) -> Result<Self, UiIntentDeclarationConstructionError> {
        Self::for_interaction(identity, UiSemanticInteractionFamily::EditCommit)
    }

    pub fn selection_commit(
        identity: impl Into<Box<str>>,
    ) -> Result<Self, UiIntentDeclarationConstructionError> {
        Self::for_interaction(identity, UiSemanticInteractionFamily::SelectionCommit)
    }

    pub fn submit(
        identity: impl Into<Box<str>>,
    ) -> Result<Self, UiIntentDeclarationConstructionError> {
        Self::for_interaction(identity, UiSemanticInteractionFamily::Submit)
    }

    fn for_interaction(
        identity: impl Into<Box<str>>,
        interaction: UiSemanticInteractionFamily,
    ) -> Result<Self, UiIntentDeclarationConstructionError> {
        let identity = identity.into();
        if !super::valid_intent_identity(&identity) {
            return Err(UiIntentDeclarationConstructionError::InvalidIdentity);
        }
        if !I::ACCEPTED_INTERACTIONS.as_slice().contains(&interaction) {
            return Err(UiIntentDeclarationConstructionError::InteractionNotAccepted);
        }
        Ok(Self {
            identity,
            interaction,
            payload_sources: Vec::new(),
            operability: UiIntentDeclarationOperabilityMissing { _sealed: () },
            confirmation: UiIntentDeclarationConfirmationMissing { _sealed: () },
            concurrency: UiIntentDeclarationConcurrencyMissing { _sealed: () },
            consequences: UiIntentDeclarationConsequencesMissing { _sealed: () },
            intent: PhantomData,
        })
    }
}

impl<I, Confirmation, Concurrency, Consequences>
    UiIntentDeclaration<
        I,
        UiIntentDeclarationOperabilityMissing,
        Confirmation,
        Concurrency,
        Consequences,
    >
where
    I: UiIntent,
{
    pub fn operability_from(
        self,
        contract: super::UiIntentOperabilityContract,
    ) -> UiIntentDeclaration<
        I,
        UiIntentDeclarationOperabilityBound,
        Confirmation,
        Concurrency,
        Consequences,
    > {
        UiIntentDeclaration {
            identity: self.identity,
            interaction: self.interaction,
            payload_sources: self.payload_sources,
            operability: UiIntentDeclarationOperabilityBound { contract },
            confirmation: self.confirmation,
            concurrency: self.concurrency,
            consequences: self.consequences,
            intent: PhantomData,
        }
    }
}

impl<I, Operability, Concurrency, Consequences>
    UiIntentDeclaration<
        I,
        Operability,
        UiIntentDeclarationConfirmationMissing,
        Concurrency,
        Consequences,
    >
where
    I: UiIntent,
{
    pub fn confirmation(
        self,
        contract: super::UiIntentConfirmationContract,
    ) -> UiIntentDeclaration<
        I,
        Operability,
        UiIntentDeclarationConfirmationBound,
        Concurrency,
        Consequences,
    > {
        UiIntentDeclaration {
            identity: self.identity,
            interaction: self.interaction,
            payload_sources: self.payload_sources,
            operability: self.operability,
            confirmation: UiIntentDeclarationConfirmationBound { contract },
            concurrency: self.concurrency,
            consequences: self.consequences,
            intent: PhantomData,
        }
    }
}

impl<I, Operability, Confirmation, Consequences>
    UiIntentDeclaration<
        I,
        Operability,
        Confirmation,
        UiIntentDeclarationConcurrencyMissing,
        Consequences,
    >
where
    I: UiIntent,
{
    pub fn concurrency(
        self,
        scope: super::UiIntentConcurrencyScope,
    ) -> UiIntentDeclaration<
        I,
        Operability,
        Confirmation,
        UiIntentDeclarationConcurrencyBound,
        Consequences,
    > {
        UiIntentDeclaration {
            identity: self.identity,
            interaction: self.interaction,
            payload_sources: self.payload_sources,
            operability: self.operability,
            confirmation: self.confirmation,
            concurrency: UiIntentDeclarationConcurrencyBound { scope },
            consequences: self.consequences,
            intent: PhantomData,
        }
    }
}

impl<I, Operability, Confirmation, Concurrency, Consequences>
    UiIntentDeclaration<I, Operability, Confirmation, Concurrency, Consequences>
where
    I: UiIntent,
{
    pub fn bind_payload<K: crate::capability::UiIntentPayloadValueKind>(
        mut self,
        field: crate::capability::UiIntentPayloadField<I::Payload, K>,
        source: super::UiIntentPayloadSource<K>,
    ) -> Self {
        self.payload_sources
            .push(source.into_dsl(field.descriptor().stable_name()));
        self
    }
}

impl<I, Operability, Confirmation, Concurrency>
    UiIntentDeclaration<
        I,
        Operability,
        Confirmation,
        Concurrency,
        UiIntentDeclarationConsequencesMissing,
    >
where
    I: UiIntent,
{
    pub fn consequences(
        self,
        contract: super::UiIntentConsequenceContract,
    ) -> UiIntentDeclaration<
        I,
        Operability,
        Confirmation,
        Concurrency,
        UiIntentDeclarationConsequencesBound,
    > {
        UiIntentDeclaration {
            identity: self.identity,
            interaction: self.interaction,
            payload_sources: self.payload_sources,
            operability: self.operability,
            confirmation: self.confirmation,
            concurrency: self.concurrency,
            consequences: UiIntentDeclarationConsequencesBound { contract },
            intent: PhantomData,
        }
    }
}

impl<I: UiIntent>
    UiIntentDeclaration<
        I,
        UiIntentDeclarationOperabilityBound,
        UiIntentDeclarationConfirmationBound,
        UiIntentDeclarationConcurrencyBound,
        UiIntentDeclarationConsequencesBound,
    >
{
    pub fn into_dsl_spec(self) -> worth_ui_dsl::WorthUiIntentDeclarationSpec {
        let payload = I::Payload::SCHEMA;
        let outcome = I::ProductOutcome::SCHEMA;
        let declaration = worth_ui_dsl::WorthUiIntentDeclarationSpec::new(
            self.identity.to_string(),
            I::ID.as_str(),
            dsl_family(self.interaction),
            self.operability.contract.into_dsl(),
            self.confirmation.contract.into_dsl(),
            self.concurrency.scope.into_dsl(),
            self.consequences.contract.into_dsl(),
        )
        .with_expected_schemas(
            payload.stable_identity(),
            payload.version(),
            outcome.stable_identity(),
            outcome.version(),
        );
        self.payload_sources.into_iter().fold(
            declaration,
            worth_ui_dsl::WorthUiIntentDeclarationSpec::with_payload_source,
        )
    }
}

fn dsl_family(family: UiSemanticInteractionFamily) -> worth_ui_dsl::WorthUiIntentInteractionFamily {
    match family {
        UiSemanticInteractionFamily::Activate => {
            worth_ui_dsl::WorthUiIntentInteractionFamily::Activate
        }
        UiSemanticInteractionFamily::EditCommit => {
            worth_ui_dsl::WorthUiIntentInteractionFamily::EditCommit
        }
        UiSemanticInteractionFamily::SelectionCommit => {
            worth_ui_dsl::WorthUiIntentInteractionFamily::SelectionCommit
        }
        UiSemanticInteractionFamily::Submit => worth_ui_dsl::WorthUiIntentInteractionFamily::Submit,
    }
}
