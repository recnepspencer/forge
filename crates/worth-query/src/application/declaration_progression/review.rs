use worth_proof::{
    AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    CheckedLoweredRecipeDxExt, CheckedResolvedRecipeDxExt, CheckedUnresolvedRecipeDxExt,
    ProofOutcomeKind, TransitionOutcome,
};

use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::admitted::WorthQueryAdmittedDeclarationProgression;
use super::checked::WorthQueryDeclarationProgressionChecked;
use super::denial::WorthQueryDeclarationProgressionFailed;
use super::denial::{
    WorthQueryDeclarationProgressionDeferred, WorthQueryDeclarationProgressionDenied,
};
use super::payload::WorthQueryDeclarationProgressionPayload;
use super::rebind::WorthQueryDeclarationProgressionRebindRequired;
use super::recipe::WorthQueryDeclarationProgressionRecipe;
use super::stale::WorthQueryDeclarationProgressionStale;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationProgressionContractClass {
    Admitted,
    Deferred,
    Denied,
    Stale,
    RebindRequired,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationProgressionContract {
    class: WorthQueryDeclarationProgressionContractClass,
    reason: &'static str,
}

impl WorthQueryDeclarationProgressionContract {
    pub fn admitted_current() -> Self {
        Self {
            class: WorthQueryDeclarationProgressionContractClass::Admitted,
            reason: "declaration progression is admitted for this legality-cleared declaration",
        }
    }

    pub fn deferred_support() -> Self {
        Self {
            class: WorthQueryDeclarationProgressionContractClass::Deferred,
            reason: "declaration progression remains explicitly deferred",
        }
    }

    pub fn denied_boundary() -> Self {
        Self {
            class: WorthQueryDeclarationProgressionContractClass::Denied,
            reason: "declaration progression is denied at the progression boundary",
        }
    }

    pub fn stale_readable() -> Self {
        Self {
            class: WorthQueryDeclarationProgressionContractClass::Stale,
            reason: "declaration progression requires stale-readable review before admission",
        }
    }

    pub fn rebind_required() -> Self {
        Self {
            class: WorthQueryDeclarationProgressionContractClass::RebindRequired,
            reason: "declaration progression requires explicit rebind before lowering",
        }
    }

    pub fn failed_transition() -> Self {
        Self {
            class: WorthQueryDeclarationProgressionContractClass::Failed,
            reason: "declaration progression failed during proof transition composition",
        }
    }

    pub fn class(&self) -> WorthQueryDeclarationProgressionContractClass {
        self.class
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationProgressionOutcomeView {
    kind: ProofOutcomeKind,
}

impl WorthQueryDeclarationProgressionOutcomeView {
    pub(crate) fn new(kind: ProofOutcomeKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> ProofOutcomeKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationProgressionBasis {
    handle_identity_digest: String,
    declaration_digest: String,
    support_digest: String,
    legality_digest: String,
}

impl WorthQueryDeclarationProgressionBasis {
    fn from_payload<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
        payload: &WorthQueryDeclarationProgressionPayload<D, I>,
    ) -> Self {
        Self {
            handle_identity_digest: payload
                .world_basis()
                .handle_identity_for_reporting()
                .to_string(),
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

pub(crate) fn worth_query_checked_declaration_progression<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    recipe: WorthQueryDeclarationProgressionRecipe<D, I>,
) -> WorthQueryDeclarationProgressionChecked<D, I> {
    let raw = recipe.into_raw();
    let contract = raw.payload().progression_contract();
    let basis = WorthQueryDeclarationProgressionBasis::from_payload(raw.payload());

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
        WorthQueryDeclarationProgressionContractClass::Admitted => {
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
                            WorthQueryDeclarationProgressionChecked::Admitted(
                                WorthQueryAdmittedDeclarationProgression::new(recipe),
                            )
                        }
                        TransitionOutcome::Stale(recipe) => {
                            WorthQueryDeclarationProgressionChecked::Stale(
                                WorthQueryDeclarationProgressionStale::new(recipe),
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
                    WorthQueryDeclarationProgressionChecked::RebindRequired(
                        WorthQueryDeclarationProgressionRebindRequired::new(recipe),
                    )
                }
                TransitionOutcome::Failed(_) => unreachable!("ready lowering may not fail"),
                TransitionOutcome::Stale(_) => unreachable!("lower transition may not stale"),
            }
        }
        WorthQueryDeclarationProgressionContractClass::Deferred => {
            WorthQueryDeclarationProgressionChecked::Deferred(
                WorthQueryDeclarationProgressionDeferred::new(resolved.into_parts().0),
            )
        }
        WorthQueryDeclarationProgressionContractClass::Denied => {
            WorthQueryDeclarationProgressionChecked::Denied(
                WorthQueryDeclarationProgressionDenied::new(resolved.into_parts().0),
            )
        }
        WorthQueryDeclarationProgressionContractClass::Stale => {
            let lowered = resolved.try_lower_ready(CapabilityWitness::from_capability_marker(
                ProgressionLoweringCapability,
            ));
            match lowered.into_raw() {
                TransitionOutcome::Success(lowered) => {
                    WorthQueryDeclarationProgressionChecked::Stale(
                        WorthQueryDeclarationProgressionStale::new(
                            lowered.downgrade_to_stale_readable(),
                        ),
                    )
                }
                TransitionOutcome::Denied(_) => unreachable!("ready lowering may not deny"),
                TransitionOutcome::Deferred(_) => unreachable!("ready lowering may not defer"),
                TransitionOutcome::RebindRequired(recipe) => {
                    WorthQueryDeclarationProgressionChecked::RebindRequired(
                        WorthQueryDeclarationProgressionRebindRequired::new(recipe),
                    )
                }
                TransitionOutcome::Failed(_) => unreachable!("ready lowering may not fail"),
                TransitionOutcome::Stale(_) => unreachable!("lower transition may not stale"),
            }
        }
        WorthQueryDeclarationProgressionContractClass::RebindRequired => {
            WorthQueryDeclarationProgressionChecked::RebindRequired(
                WorthQueryDeclarationProgressionRebindRequired::new(
                    resolved.downgrade_to_rebind_required(),
                ),
            )
        }
        WorthQueryDeclarationProgressionContractClass::Failed => {
            WorthQueryDeclarationProgressionChecked::Failed(
                WorthQueryDeclarationProgressionFailed::new(resolved.into_parts().0),
            )
        }
    }
}
