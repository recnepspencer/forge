use core::marker::PhantomData;

use crate::capability::{
    UiIntent, UiIntentPayload, UiIntentProductOutcome, UiSemanticInteractionFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentDeclarationConstructionError {
    InvalidIdentity,
    InteractionNotAccepted,
}

pub struct UiIntentDeclaration<I: UiIntent> {
    identity: Box<str>,
    interaction: UiSemanticInteractionFamily,
    intent: PhantomData<fn() -> I>,
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

    pub fn into_dsl_spec(self) -> worth_ui_dsl::WorthUiIntentDeclarationSpec {
        let payload = I::Payload::SCHEMA;
        let outcome = I::ProductOutcome::SCHEMA;
        worth_ui_dsl::WorthUiIntentDeclarationSpec::new(
            self.identity.to_string(),
            I::ID.as_str(),
            dsl_family(self.interaction),
        )
        .with_expected_schemas(
            payload.stable_identity(),
            payload.version(),
            outcome.stable_identity(),
            outcome.version(),
        )
    }

    fn for_interaction(
        identity: impl Into<Box<str>>,
        interaction: UiSemanticInteractionFamily,
    ) -> Result<Self, UiIntentDeclarationConstructionError> {
        let identity = identity.into();
        if !valid_identity(&identity) {
            return Err(UiIntentDeclarationConstructionError::InvalidIdentity);
        }
        if !I::ACCEPTED_INTERACTIONS.as_slice().contains(&interaction) {
            return Err(UiIntentDeclarationConstructionError::InteractionNotAccepted);
        }
        Ok(Self {
            identity,
            interaction,
            intent: PhantomData,
        })
    }
}

fn valid_identity(identity: &str) -> bool {
    !identity.is_empty()
        && identity
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
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
