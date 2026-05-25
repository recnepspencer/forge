use forge_proof::{
    AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    CheckedLoweredRecipeDxExt, CheckedResolvedRecipeDxExt, CheckedUnresolvedRecipeDxExt,
    ProofOutcomeKind, TransitionOutcome,
};

use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::admitted::ForgeQueryAdmittedDeclarationProgression;
use super::checked::ForgeQueryDeclarationProgressionChecked;
use super::denial::{
    ForgeQueryDeclarationProgressionDeferred, ForgeQueryDeclarationProgressionDenied,
};
use super::payload::ForgeQueryDeclarationProgressionPayload;
use super::rebind::ForgeQueryDeclarationProgressionRebindRequired;
use super::recipe::ForgeQueryDeclarationProgressionRecipe;
use super::denial::ForgeQueryDeclarationProgressionFailed;
use super::stale::ForgeQueryDeclarationProgressionStale;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationProgressionContractClass {
    Admitted,
    Deferred,
    Denied,
    Stale,
    RebindRequired,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationProgressionContract {
    class: ForgeQueryDeclarationProgressionContractClass,
    reason: &'static str,
}

impl ForgeQueryDeclarationProgressionContract {
    pub fn admitted_current() -> Self {
        Self {
            class: ForgeQueryDeclarationProgressionContractClass::Admitted,
            reason: "declaration progression is admitted for this legality-cleared declaration",
        }
    }

    pub fn deferred_support() -> Self {
        Self {
            class: ForgeQueryDeclarationProgressionContractClass::Deferred,
            reason: "declaration progression remains explicitly deferred",
        }
    }

    pub fn denied_boundary() -> Self {
        Self {
            class: ForgeQueryDeclarationProgressionContractClass::Denied,
            reason: "declaration progression is denied at the progression boundary",
        }
    }

    pub fn stale_readable() -> Self {
        Self {
            class: ForgeQueryDeclarationProgressionContractClass::Stale,
            reason: "declaration progression requires stale-readable review before admission",
        }
    }

    pub fn rebind_required() -> Self {
        Self {
            class: ForgeQueryDeclarationProgressionContractClass::RebindRequired,
            reason: "declaration progression requires explicit rebind before lowering",
        }
    }

    pub fn failed_transition() -> Self {
        Self {
            class: ForgeQueryDeclarationProgressionContractClass::Failed,
            reason: "declaration progression failed during proof transition composition",
        }
    }

    pub fn class(&self) -> ForgeQueryDeclarationProgressionContractClass {
        self.class
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationProgressionOutcomeView {
    kind: ProofOutcomeKind,
}

impl ForgeQueryDeclarationProgressionOutcomeView {
    pub(crate) fn new(kind: ProofOutcomeKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> ProofOutcomeKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationProgressionBasis {
    handle_identity_digest: String,
    declaration_digest: String,
    support_digest: String,
    legality_digest: String,
}

impl ForgeQueryDeclarationProgressionBasis {
    fn from_payload<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
        payload: &ForgeQueryDeclarationProgressionPayload<D, I>,
    ) -> Self {
        Self {
            handle_identity_digest: payload.handle_identity_digest().to_string(),
            declaration_digest: payload.declaration_digest().to_string(),
            support_digest: payload.support_digest().to_string(),
            legality_digest: payload.legality_digest().to_string(),
        }
    }
}

struct ProgressionResolutionAuthority;
impl AuthorityMarker for ProgressionResolutionAuthority {}

struct ProgressionLoweringCapability;
impl CapabilityMarker for ProgressionLoweringCapability {}

struct ProgressionAdmissionAuthority;
impl AuthorityMarker for ProgressionAdmissionAuthority {}

pub(crate) fn forge_query_checked_declaration_progression<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    recipe: ForgeQueryDeclarationProgressionRecipe<D, I>,
) -> ForgeQueryDeclarationProgressionChecked<D, I> {
    let raw = recipe.into_raw();
    let contract = raw.payload().progression_contract();
    let basis = ForgeQueryDeclarationProgressionBasis::from_payload(raw.payload());

    let resolved = raw.try_resolve_ready(
        basis,
        AuthorityWitness::from_authority_marker(ProgressionResolutionAuthority),
    );

    let resolved = match resolved.into_raw() {
        TransitionOutcome::Success(resolved) => resolved,
        TransitionOutcome::Denied(_) => unreachable!("ready resolution may not deny"),
        TransitionOutcome::Deferred(_) => unreachable!("ready resolution may not defer"),
        TransitionOutcome::Stale(_) => unreachable!("ready resolution may not stale"),
        TransitionOutcome::RebindRequired(_) => {
            unreachable!("ready resolution may not require rebind")
        }
        TransitionOutcome::Failed(_) => unreachable!("ready resolution may not fail"),
    };

    match contract.class() {
        ForgeQueryDeclarationProgressionContractClass::Admitted => {
            let lowered = resolved.try_lower_ready(CapabilityWitness::from_capability_marker(
                ProgressionLoweringCapability,
            ));
            match lowered.into_raw() {
                TransitionOutcome::Success(lowered) => {
                    let admitted = lowered.try_admit_ready(
                        AuthorityWitness::from_authority_marker(ProgressionAdmissionAuthority),
                    );
                    match admitted.into_raw() {
                        TransitionOutcome::Success(recipe) => {
                            ForgeQueryDeclarationProgressionChecked::Admitted(
                                ForgeQueryAdmittedDeclarationProgression::new(recipe),
                            )
                        }
                        TransitionOutcome::Stale(recipe) => {
                            ForgeQueryDeclarationProgressionChecked::Stale(
                                ForgeQueryDeclarationProgressionStale::new(recipe),
                            )
                        }
                        TransitionOutcome::Denied(_) => {
                            unreachable!("ready admission may not deny")
                        }
                        TransitionOutcome::Deferred(_) => {
                            unreachable!("ready admission may not defer")
                        }
                        TransitionOutcome::RebindRequired(_) => {
                            unreachable!("ready admission may not rebind")
                        }
                        TransitionOutcome::Failed(_) => {
                            unreachable!("ready admission may not fail")
                        }
                    }
                }
                TransitionOutcome::Denied(_) => unreachable!("ready lowering may not deny"),
                TransitionOutcome::Deferred(_) => unreachable!("ready lowering may not defer"),
                TransitionOutcome::RebindRequired(recipe) => {
                    ForgeQueryDeclarationProgressionChecked::RebindRequired(
                        ForgeQueryDeclarationProgressionRebindRequired::new(recipe),
                    )
                }
                TransitionOutcome::Failed(_) => unreachable!("ready lowering may not fail"),
                TransitionOutcome::Stale(_) => unreachable!("lower transition may not stale"),
            }
        }
        ForgeQueryDeclarationProgressionContractClass::Deferred => {
            ForgeQueryDeclarationProgressionChecked::Deferred(
                ForgeQueryDeclarationProgressionDeferred::new(resolved.into_parts().0),
            )
        }
        ForgeQueryDeclarationProgressionContractClass::Denied => {
            ForgeQueryDeclarationProgressionChecked::Denied(
                ForgeQueryDeclarationProgressionDenied::new(resolved.into_parts().0),
            )
        }
        ForgeQueryDeclarationProgressionContractClass::Stale => {
            let lowered = resolved.try_lower_ready(CapabilityWitness::from_capability_marker(
                ProgressionLoweringCapability,
            ));
            match lowered.into_raw() {
                TransitionOutcome::Success(lowered) => {
                    ForgeQueryDeclarationProgressionChecked::Stale(
                        ForgeQueryDeclarationProgressionStale::new(
                            lowered.downgrade_to_stale_readable(),
                        ),
                    )
                }
                TransitionOutcome::Denied(_) => unreachable!("ready lowering may not deny"),
                TransitionOutcome::Deferred(_) => unreachable!("ready lowering may not defer"),
                TransitionOutcome::RebindRequired(recipe) => {
                    ForgeQueryDeclarationProgressionChecked::RebindRequired(
                        ForgeQueryDeclarationProgressionRebindRequired::new(recipe),
                    )
                }
                TransitionOutcome::Failed(_) => unreachable!("ready lowering may not fail"),
                TransitionOutcome::Stale(_) => unreachable!("lower transition may not stale"),
            }
        }
        ForgeQueryDeclarationProgressionContractClass::RebindRequired => {
            ForgeQueryDeclarationProgressionChecked::RebindRequired(
                ForgeQueryDeclarationProgressionRebindRequired::new(
                    resolved.downgrade_to_rebind_required(),
                ),
            )
        }
        ForgeQueryDeclarationProgressionContractClass::Failed => {
            ForgeQueryDeclarationProgressionChecked::Failed(
                ForgeQueryDeclarationProgressionFailed::new(resolved.into_parts().0),
            )
        }
    }
}
