use std::marker::PhantomData;

use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::proof::ForgeQueryDeclarationEntryOrchestrationStage;
use super::refusal::ForgeQueryDeclarationEntryOrchestrationRefusal;

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

pub enum ForgeQueryDeclarationEntryOrchestrationChecked<
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
    ForgeQueryDeclarationEntryOrchestrationChecked<D, I>
{
    pub(crate) fn outcome_identity(&self) -> String {
        match self {
            Self::Enveloped(envelope) => {
                format!(
                    "enveloped:{}",
                    canonical_digest_token(envelope.envelope_digest())
                )
            }
            Self::Deferred(outcome) => {
                format!(
                    "deferred:{}:{}",
                    outcome.declaration_family_key(),
                    outcome.reason()
                )
            }
            Self::Denied(outcome) => {
                format!(
                    "denied:{}:{}",
                    outcome.declaration_family_key(),
                    outcome.reason()
                )
            }
            Self::Stale(outcome) => {
                format!(
                    "stale:{}:{}",
                    outcome.declaration_family_key(),
                    outcome.reason()
                )
            }
            Self::RebindRequired(outcome) => format!(
                "rebind_required:{}:{}",
                outcome.declaration_family_key(),
                outcome.reason()
            ),
            Self::Failed(outcome) => {
                format!(
                    "failed:{}:{}",
                    outcome.declaration_family_key(),
                    outcome.reason()
                )
            }
            Self::Refused(outcome) => format!(
                "refused:{}:{}:{}",
                outcome.declaration_family_key(),
                outcome.refusal_class().as_str(),
                outcome.reason()
            ),
        }
    }
}

pub(crate) fn forge_query_checked_declaration_entry_orchestration_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
) -> ForgeQueryDeclarationEntryOrchestrationChecked<D, I> {
    super::lower::forge_query_lower_declaration_entry_orchestration_on_handle(handle, input).checked
}

fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
