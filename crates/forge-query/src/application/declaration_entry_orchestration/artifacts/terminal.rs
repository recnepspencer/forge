use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::outcome::{
    ForgeQueryDeclarationEntryOrchestrationDeferred, ForgeQueryDeclarationEntryOrchestrationDenied,
    ForgeQueryDeclarationEntryOrchestrationFailed, ForgeQueryDeclarationEntryOrchestrationOutcome,
    ForgeQueryDeclarationEntryOrchestrationRebindRequired,
    ForgeQueryDeclarationEntryOrchestrationStale,
};
use super::refusal::ForgeQueryDeclarationEntryOrchestrationRefusal;

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

pub(crate) fn terminal_error_from_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    outcome: ForgeQueryDeclarationEntryOrchestrationOutcome<D, I>,
) -> Result<
    crate::application::ForgeQueryDeclarationEnvelope<D, I>,
    ForgeQueryDeclarationEntryOrchestrationTerminalError<D, I>,
> {
    match outcome {
        ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope) => Ok(envelope),
        ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::Deferred(outcome))
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::Denied(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::Denied(outcome))
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::Stale(outcome) => Err(
            ForgeQueryDeclarationEntryOrchestrationTerminalError::Stale(outcome),
        ),
        ForgeQueryDeclarationEntryOrchestrationOutcome::RebindRequired(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::RebindRequired(outcome))
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::Failed(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::Failed(outcome))
        }
        ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(outcome) => {
            Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::Refused(outcome))
        }
    }
}
