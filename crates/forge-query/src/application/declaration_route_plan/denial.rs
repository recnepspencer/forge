use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationEntryProgressionError,
    ForgeQueryDeclarationFoundationalEvidence, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

use super::{
    contract::ForgeQueryDeclarationRouteContract, intent::ForgeQueryDeclarationRouteIntent,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRoutePlanDenialCause {
    WrongAdmittedWorld,
    EvidenceMismatch,
    IntentRequired,
    IntentForbidden,
    IntentConflictsWithRouteContract,
    NoAllowedRoutes,
    ForbiddenRouteCombination,
}

impl ForgeQueryDeclarationRoutePlanDenialCause {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WrongAdmittedWorld => "wrong_admitted_world",
            Self::EvidenceMismatch => "evidence_mismatch",
            Self::IntentRequired => "intent_required",
            Self::IntentForbidden => "intent_forbidden",
            Self::IntentConflictsWithRouteContract => "intent_conflicts_with_route_contract",
            Self::NoAllowedRoutes => "no_allowed_routes",
            Self::ForbiddenRouteCombination => "forbidden_route_combination",
        }
    }

    pub fn reason(&self) -> &'static str {
        match self {
            Self::WrongAdmittedWorld => {
                "route planning requires retained proof from the same admitted world"
            }
            Self::EvidenceMismatch => {
                "route planning requires matching admitted progression and foundational evidence"
            }
            Self::IntentRequired => {
                "the declaration route contract requires explicit caller route intent"
            }
            Self::IntentForbidden => {
                "the declaration route contract forbids caller-owned route narrowing"
            }
            Self::IntentConflictsWithRouteContract => {
                "the declaration route contract does not allow deferred routing here"
            }
            Self::NoAllowedRoutes => {
                "the declaration route contract admitted no concrete lower-authority routes"
            }
            Self::ForbiddenRouteCombination => {
                "the declaration route contract forbids plural lower-authority routing"
            }
        }
    }
}

macro_rules! define_route_terminal {
    ($name:ident) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
            evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
            route_intent: Option<ForgeQueryDeclarationRouteIntent>,
            route_contract: ForgeQueryDeclarationRouteContract,
            reason: &'static str,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            pub(crate) fn new(
                progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
                evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
                route_intent: Option<ForgeQueryDeclarationRouteIntent>,
                route_contract: ForgeQueryDeclarationRouteContract,
                reason: &'static str,
            ) -> Self {
                Self {
                    progressed,
                    evidence,
                    route_intent,
                    route_contract,
                    reason,
                }
            }

            pub fn reason(&self) -> &'static str {
                self.reason
            }

            pub fn route_intent(&self) -> Option<ForgeQueryDeclarationRouteIntent> {
                self.route_intent
            }

            pub fn route_contract(&self) -> ForgeQueryDeclarationRouteContract {
                self.route_contract
            }

            pub fn declaration_family_key(&self) -> &'static str {
                self.progressed.declaration_family_key()
            }

            pub fn progressed_declaration(
                &self,
            ) -> &ForgeQueryAdmittedDeclarationProgression<D, I> {
                &self.progressed
            }

            pub fn foundational_evidence(
                &self,
            ) -> &ForgeQueryDeclarationFoundationalEvidence<D, I> {
                &self.evidence
            }

            pub(crate) fn into_parts(
                self,
            ) -> (
                ForgeQueryAdmittedDeclarationProgression<D, I>,
                ForgeQueryDeclarationFoundationalEvidence<D, I>,
                Option<ForgeQueryDeclarationRouteIntent>,
                ForgeQueryDeclarationRouteContract,
                &'static str,
            ) {
                (
                    self.progressed,
                    self.evidence,
                    self.route_intent,
                    self.route_contract,
                    self.reason,
                )
            }
        }
    };
}

define_route_terminal!(ForgeQueryDeclarationRoutePlanDeferred);
define_route_terminal!(ForgeQueryDeclarationRoutePlanFailed);

pub struct ForgeQueryDeclarationRoutePlanDenied<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    route_contract: ForgeQueryDeclarationRouteContract,
    cause: ForgeQueryDeclarationRoutePlanDenialCause,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationRoutePlanDenied<D, I>
{
    pub(crate) fn new(
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
        route_intent: Option<ForgeQueryDeclarationRouteIntent>,
        route_contract: ForgeQueryDeclarationRouteContract,
        cause: ForgeQueryDeclarationRoutePlanDenialCause,
    ) -> Self {
        Self {
            progressed,
            evidence,
            route_intent,
            route_contract,
            cause,
        }
    }

    pub fn cause(&self) -> ForgeQueryDeclarationRoutePlanDenialCause {
        self.cause
    }

    pub fn reason(&self) -> &'static str {
        self.cause.reason()
    }

    pub fn route_intent(&self) -> Option<ForgeQueryDeclarationRouteIntent> {
        self.route_intent
    }

    pub fn route_contract(&self) -> ForgeQueryDeclarationRouteContract {
        self.route_contract
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.progressed.declaration_family_key()
    }

    pub fn progressed_declaration(&self) -> &ForgeQueryAdmittedDeclarationProgression<D, I> {
        &self.progressed
    }

    pub fn foundational_evidence(&self) -> &ForgeQueryDeclarationFoundationalEvidence<D, I> {
        &self.evidence
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationFoundationalEvidence<D, I>,
        Option<ForgeQueryDeclarationRouteIntent>,
        ForgeQueryDeclarationRouteContract,
        ForgeQueryDeclarationRoutePlanDenialCause,
    ) {
        (
            self.progressed,
            self.evidence,
            self.route_intent,
            self.route_contract,
            self.cause,
        )
    }
}

pub enum ForgeQueryDeclarationRoutePlanTerminalError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Deferred(ForgeQueryDeclarationRoutePlanDeferred<D, I>),
    Denied(ForgeQueryDeclarationRoutePlanDenied<D, I>),
    Failed(ForgeQueryDeclarationRoutePlanFailed<D, I>),
}

pub enum ForgeQueryDeclarationEntryRoutePlanError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Entry(ForgeQueryDeclarationEntryProgressionError<D, I>),
    RoutePlan(ForgeQueryDeclarationRoutePlanTerminalError<D, I>),
}
