use forge_query::facade::{
    ForgeQueryDeclarationEntryCrossingInventory, ForgeQueryDeclarationEntryReadinessReport,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDeclaredFamilyChecked,
    ForgeQueryOrdinaryOutcome,
};

use super::proof_claim_request_types::{
    BackgroundTheoremDeclaration, PlaneExactValueClaimDeclaration, PlaneLowerBoundClaimDeclaration,
    PlaneUpperBoundClaimDeclaration,
};
use super::request_types::{
    AdvisoryNoteDeclaration, CandidateGraphDeclaration, ColorabilityDeclaration,
    EmbeddingDeclaration, FractionalChromaticScreeningDeclaration, LovaszThetaScreeningDeclaration,
    LowerBoundWitnessDeclaration, PartialAdmissionExplanationDeclaration,
    RejectionExplanationDeclaration, UnitDistanceVerificationDeclaration,
    WholePlaneColoringConstructionDeclaration,
};
use crate::query_entry::{HadwigerResearchDomainEntry, HadwigerResearchHandle};

pub trait HadwigerResearchDeclarationInput:
    ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> + sealed::HadwigerResearchRequestSeal
{
}

macro_rules! hadwiger_request_input {
    ($($type:ty),+ $(,)?) => {
        $(
            impl sealed::HadwigerResearchRequestSeal for $type {}
            impl HadwigerResearchDeclarationInput for $type {}
        )+
    };
}

hadwiger_request_input!(
    CandidateGraphDeclaration,
    EmbeddingDeclaration,
    ColorabilityDeclaration,
    LowerBoundWitnessDeclaration,
    AdvisoryNoteDeclaration,
    RejectionExplanationDeclaration,
    PartialAdmissionExplanationDeclaration,
    UnitDistanceVerificationDeclaration,
    WholePlaneColoringConstructionDeclaration,
    FractionalChromaticScreeningDeclaration,
    LovaszThetaScreeningDeclaration,
    PlaneLowerBoundClaimDeclaration,
    PlaneUpperBoundClaimDeclaration,
    PlaneExactValueClaimDeclaration,
    BackgroundTheoremDeclaration,
);

pub fn declare_research_request_checked<I>(
    handle: &HadwigerResearchHandle,
    input: I,
) -> ForgeQueryDeclaredFamilyChecked<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.declare_checked(input)
}

pub fn orchestrate_research_request_entry<I>(
    handle: &HadwigerResearchHandle,
    input: I,
) -> ForgeQueryOrdinaryOutcome<ForgeQueryDeclarationEnvelope<HadwigerResearchDomainEntry, I>>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.orchestrate_declaration_entry_outcome(input)
}

pub fn research_declaration_entry_inventory<I>(
    handle: &HadwigerResearchHandle,
) -> ForgeQueryDeclarationEntryCrossingInventory<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.declaration_entry_crossing_inventory::<I>()
}

pub fn research_declaration_entry_readiness<I>(
    handle: &HadwigerResearchHandle,
) -> ForgeQueryDeclarationEntryReadinessReport<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.declaration_entry_readiness::<I>()
}

mod sealed {
    pub trait HadwigerResearchRequestSeal {}
}
