use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::checked::{
    ForgeQueryDeclarationEntryOrchestrationDeferred, ForgeQueryDeclarationEntryOrchestrationDenied,
    ForgeQueryDeclarationEntryOrchestrationFailed,
    ForgeQueryDeclarationEntryOrchestrationRebindRequired,
    ForgeQueryDeclarationEntryOrchestrationStale,
};
use super::proof::ForgeQueryDeclarationEntryOrchestrationStage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationRefusalClass {
    UnsupportedAutomation,
    ExplicitIntentRequired,
    StrongerProofRequired,
    AuthorityTransitionRequired,
    ExpensiveWorkNotAdmittedByDefault,
    PreparedButNotExecutedContinuation,
}

impl ForgeQueryDeclarationEntryOrchestrationRefusalClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedAutomation => "unsupported_automation",
            Self::ExplicitIntentRequired => "explicit_intent_required",
            Self::StrongerProofRequired => "stronger_proof_required",
            Self::AuthorityTransitionRequired => "authority_transition_required",
            Self::ExpensiveWorkNotAdmittedByDefault => "expensive_work_not_admitted_by_default",
            Self::PreparedButNotExecutedContinuation => "prepared_but_not_executed_continuation",
        }
    }
}

pub struct ForgeQueryDeclarationEntryOrchestrationRefusal<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    refusal_class: ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    reason: &'static str,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryOrchestrationRefusal<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        refusal_class: ForgeQueryDeclarationEntryOrchestrationRefusalClass,
        stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        reason: &'static str,
    ) -> Self {
        Self {
            declaration_family_key,
            refusal_class,
            stop_stage,
            reason,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn refusal_class(&self) -> ForgeQueryDeclarationEntryOrchestrationRefusalClass {
        self.refusal_class
    }

    pub fn stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.stop_stage
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

pub enum ForgeQueryDeclarationEntryOrchestrationTerminalError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Deferred(ForgeQueryDeclarationEntryOrchestrationDeferred<D, I>),
    Denied(ForgeQueryDeclarationEntryOrchestrationDenied<D, I>),
    Stale(ForgeQueryDeclarationEntryOrchestrationStale<D, I>),
    RebindRequired(ForgeQueryDeclarationEntryOrchestrationRebindRequired<D, I>),
    Failed(ForgeQueryDeclarationEntryOrchestrationFailed<D, I>),
    Refused(ForgeQueryDeclarationEntryOrchestrationRefusal<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryOrchestrationTerminalError<D, I>
{
    pub fn reason(&self) -> &'static str {
        match self {
            Self::Deferred(outcome) => outcome.reason(),
            Self::Denied(outcome) => outcome.reason(),
            Self::Stale(outcome) => outcome.reason(),
            Self::RebindRequired(outcome) => outcome.reason(),
            Self::Failed(outcome) => outcome.reason(),
            Self::Refused(outcome) => outcome.reason(),
        }
    }
}
