use std::marker::PhantomData;

use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};
use crate::identity::hash_parts;

use super::refusal::ForgeQueryDeclarationEntryOrchestrationRefusal;
use super::step_record::ForgeQueryDeclarationEntryOrchestrationStage;

macro_rules! define_terminal_outcome {
    ($name:ident) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            declaration_family_key: &'static str,
            stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
            reason: &'static str,
            retained_digest: Option<String>,
            _marker: PhantomData<(D, I)>,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            pub(crate) fn new(
                declaration_family_key: &'static str,
                stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
                reason: &'static str,
                retained_digest: Option<String>,
            ) -> Self {
                Self {
                    declaration_family_key,
                    stop_stage,
                    reason,
                    retained_digest,
                    _marker: PhantomData,
                }
            }

            pub fn declaration_family_key(&self) -> &'static str {
                self.declaration_family_key
            }

            pub fn stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
                self.stop_stage
            }

            pub fn reason(&self) -> &'static str {
                self.reason
            }

            pub fn retained_digest(&self) -> Option<&str> {
                self.retained_digest.as_deref()
            }
        }
    };
}

define_terminal_outcome!(ForgeQueryDeclarationEntryOrchestrationDeferred);
define_terminal_outcome!(ForgeQueryDeclarationEntryOrchestrationDenied);
define_terminal_outcome!(ForgeQueryDeclarationEntryOrchestrationStale);
define_terminal_outcome!(ForgeQueryDeclarationEntryOrchestrationRebindRequired);
define_terminal_outcome!(ForgeQueryDeclarationEntryOrchestrationFailed);

pub enum ForgeQueryDeclarationEntryOrchestrationOutcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Enveloped(ForgeQueryDeclarationEnvelope<D, I>),
    Deferred(ForgeQueryDeclarationEntryOrchestrationDeferred<D, I>),
    Denied(ForgeQueryDeclarationEntryOrchestrationDenied<D, I>),
    Stale(ForgeQueryDeclarationEntryOrchestrationStale<D, I>),
    RebindRequired(ForgeQueryDeclarationEntryOrchestrationRebindRequired<D, I>),
    Failed(ForgeQueryDeclarationEntryOrchestrationFailed<D, I>),
    Refused(ForgeQueryDeclarationEntryOrchestrationRefusal<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>
{
    pub fn stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        match self {
            Self::Enveloped(_) => ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            Self::Deferred(outcome) => outcome.stop_stage(),
            Self::Denied(outcome) => outcome.stop_stage(),
            Self::Stale(outcome) => outcome.stop_stage(),
            Self::RebindRequired(outcome) => outcome.stop_stage(),
            Self::Failed(outcome) => outcome.stop_stage(),
            Self::Refused(outcome) => outcome.stop_stage(),
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        match self {
            Self::Enveloped(envelope) => envelope.declaration_family_key(),
            Self::Deferred(outcome) => outcome.declaration_family_key(),
            Self::Denied(outcome) => outcome.declaration_family_key(),
            Self::Stale(outcome) => outcome.declaration_family_key(),
            Self::RebindRequired(outcome) => outcome.declaration_family_key(),
            Self::Failed(outcome) => outcome.declaration_family_key(),
            Self::Refused(outcome) => outcome.declaration_family_key(),
        }
    }

    pub fn retained_digest(&self) -> Option<&str> {
        match self {
            Self::Enveloped(_) => None,
            Self::Deferred(outcome) => outcome.retained_digest(),
            Self::Denied(outcome) => outcome.retained_digest(),
            Self::Stale(outcome) => outcome.retained_digest(),
            Self::RebindRequired(outcome) => outcome.retained_digest(),
            Self::Failed(outcome) => outcome.retained_digest(),
            Self::Refused(outcome) => outcome.retained_digest(),
        }
    }

    pub fn outcome_identity_digest(&self) -> String {
        match self {
            Self::Enveloped(envelope) => hash_parts(&[
                "kind:enveloped".to_string(),
                format!("family:{}", envelope.declaration_family_key()),
                format!(
                    "digest:{}",
                    canonical_digest_token(envelope.envelope_digest())
                ),
            ]),
            Self::Deferred(outcome) => terminal_identity("deferred", outcome),
            Self::Denied(outcome) => terminal_identity("denied", outcome),
            Self::Stale(outcome) => terminal_identity("stale", outcome),
            Self::RebindRequired(outcome) => terminal_identity("rebind_required", outcome),
            Self::Failed(outcome) => terminal_identity("failed", outcome),
            Self::Refused(outcome) => hash_parts(&[
                "kind:refused".to_string(),
                format!("family:{}", outcome.declaration_family_key()),
                format!("stage:{}", outcome.stop_stage().as_str()),
                format!("refusal_class:{}", outcome.refusal_class().as_str()),
                format!(
                    "automation_refusal_class:{}",
                    outcome.automation_refusal_class().as_str()
                ),
                format!(
                    "automation_boundary:{}",
                    outcome.automation_boundary().as_str()
                ),
                format!("reason:{}", outcome.reason()),
                format!("digest:{}", outcome.retained_digest().unwrap_or("none")),
            ]),
        }
    }

    pub fn is_automation_refusal(&self) -> bool {
        matches!(self, Self::Refused(_))
    }

    pub fn is_expensive_work_refusal(&self) -> bool {
        matches!(
            self,
            Self::Refused(outcome)
                if outcome.refusal_class()
                    == super::refusal::ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault
        )
    }
}

fn terminal_identity(kind: &str, outcome: &dyn TerminalOutcomeView) -> String {
    hash_parts(&[
        format!("kind:{kind}"),
        format!("family:{}", outcome.declaration_family_key()),
        format!("stage:{}", outcome.stop_stage().as_str()),
        format!("reason:{}", outcome.reason()),
        format!("digest:{}", outcome.retained_digest().unwrap_or("none")),
    ])
}

trait TerminalOutcomeView {
    fn declaration_family_key(&self) -> &'static str;
    fn stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage;
    fn reason(&self) -> &'static str;
    fn retained_digest(&self) -> Option<&str>;
}

macro_rules! impl_terminal_view {
    ($($name:ident),+ $(,)?) => {$(
        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> TerminalOutcomeView
            for $name<D, I>
        {
            fn declaration_family_key(&self) -> &'static str { self.declaration_family_key() }
            fn stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage { self.stop_stage() }
            fn reason(&self) -> &'static str { self.reason() }
            fn retained_digest(&self) -> Option<&str> { self.retained_digest() }
        }
    )+};
}

impl_terminal_view!(
    ForgeQueryDeclarationEntryOrchestrationDeferred,
    ForgeQueryDeclarationEntryOrchestrationDenied,
    ForgeQueryDeclarationEntryOrchestrationStale,
    ForgeQueryDeclarationEntryOrchestrationRebindRequired,
    ForgeQueryDeclarationEntryOrchestrationFailed,
);

pub(crate) fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}

pub type ForgeQueryDeclarationEntryOrchestrationChecked<D, I> =
    ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>;
