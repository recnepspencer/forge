use worth_query::facade::foundation::{
    WorthQueryDeclarationEntryCrossingInventory, WorthQueryDeclarationEntryReadinessReport,
    WorthQueryDeclarationInput, WorthQueryDeclaredFamilyChecked,
};

use super::geometry_screening_request_types::{
    BoundaryOwnershipScreeningDeclaration, CandidateNoveltyScreeningDeclaration,
    ExactArithmeticIntervalScreeningDeclaration, ExactConflictGraphScreeningDeclaration,
    ExactUnitDistanceConflictScreeningDeclaration, ExhaustiveLocalNeighborhoodScreeningDeclaration,
    FinitePatchBoundaryExtensionScreeningDeclaration, ForbiddenDisplacementScreeningDeclaration,
    KnownObstructionContainmentScreeningDeclaration, MinkowskiDifferenceScreeningDeclaration,
    MonodromyColorHolonomyScreeningDeclaration, NumericalMarginScreeningDeclaration,
    PeriodicQuotientGraphScreeningDeclaration, RigidityRealizationScreeningDeclaration,
    SameColorSeparationScreeningDeclaration, SubstitutionConsistencyScreeningDeclaration,
    SymmetryOrbitReductionScreeningDeclaration, TileDiameterScreeningDeclaration,
    TranslationRotationClosureScreeningDeclaration, UnitDistanceEmbeddabilityScreeningDeclaration,
};
use super::proof_claim_request_types::{
    BackgroundTheoremDeclaration, PlaneExactValueClaimDeclaration, PlaneLowerBoundClaimDeclaration,
    PlaneUpperBoundClaimDeclaration,
};
use super::request_types::{
    AdvisoryNoteDeclaration, CandidateGraphDeclaration, ColorabilityDeclaration,
    EmbeddingDeclaration, LowerBoundWitnessDeclaration, PartialAdmissionExplanationDeclaration,
    RejectionExplanationDeclaration, UnitDistanceVerificationDeclaration,
    WholePlaneColoringConstructionDeclaration,
};
use super::screening_request_types::{
    AutocorrelationZeroScreeningDeclaration, DensityCapScreeningDeclaration,
    FractionalChromaticScreeningDeclaration, LocalDensityWindowScreeningDeclaration,
    LovaszThetaScreeningDeclaration,
};
use super::tiling_request_types::{
    ConflictGraphExtractionDeclaration, CoreExtractionDeclaration,
    GeneratedPatternClosureDeclaration, LowerBoundTilingIterationDeclaration, MotifSeedDeclaration,
    PeriodicQuotientCellDeclaration, TerminalForcingStudyDeclaration,
    TileContactWitnessDeclaration, TilingEquivalenceClassificationDeclaration,
    TilingReactivationDeclaration, TilingSuppressionDeclaration,
    UpperBoundTilingIterationDeclaration,
};
use crate::query_entry::{HadwigerResearchDomainEntry, HadwigerResearchHandle};

pub trait HadwigerResearchDeclarationInput:
    WorthQueryDeclarationInput<HadwigerResearchDomainEntry> + sealed::HadwigerResearchRequestSeal
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
    MotifSeedDeclaration,
    TerminalForcingStudyDeclaration,
    PeriodicQuotientCellDeclaration,
    GeneratedPatternClosureDeclaration,
    TileContactWitnessDeclaration,
    ConflictGraphExtractionDeclaration,
    CoreExtractionDeclaration,
    TilingEquivalenceClassificationDeclaration,
    TilingSuppressionDeclaration,
    TilingReactivationDeclaration,
    LowerBoundTilingIterationDeclaration,
    UpperBoundTilingIterationDeclaration,
    FractionalChromaticScreeningDeclaration,
    LovaszThetaScreeningDeclaration,
    AutocorrelationZeroScreeningDeclaration,
    DensityCapScreeningDeclaration,
    LocalDensityWindowScreeningDeclaration,
    ExactUnitDistanceConflictScreeningDeclaration,
    SameColorSeparationScreeningDeclaration,
    TileDiameterScreeningDeclaration,
    ExactConflictGraphScreeningDeclaration,
    NumericalMarginScreeningDeclaration,
    MinkowskiDifferenceScreeningDeclaration,
    ForbiddenDisplacementScreeningDeclaration,
    PeriodicQuotientGraphScreeningDeclaration,
    UnitDistanceEmbeddabilityScreeningDeclaration,
    RigidityRealizationScreeningDeclaration,
    ExactArithmeticIntervalScreeningDeclaration,
    SymmetryOrbitReductionScreeningDeclaration,
    ExhaustiveLocalNeighborhoodScreeningDeclaration,
    KnownObstructionContainmentScreeningDeclaration,
    CandidateNoveltyScreeningDeclaration,
    BoundaryOwnershipScreeningDeclaration,
    MonodromyColorHolonomyScreeningDeclaration,
    TranslationRotationClosureScreeningDeclaration,
    SubstitutionConsistencyScreeningDeclaration,
    FinitePatchBoundaryExtensionScreeningDeclaration,
    PlaneLowerBoundClaimDeclaration,
    PlaneUpperBoundClaimDeclaration,
    PlaneExactValueClaimDeclaration,
    BackgroundTheoremDeclaration,
);

pub fn declare_research_request_checked<I>(
    handle: &HadwigerResearchHandle,
    input: I,
) -> WorthQueryDeclaredFamilyChecked<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.declare_checked(input)
}

pub fn research_declaration_entry_inventory<I>(
    handle: &HadwigerResearchHandle,
) -> WorthQueryDeclarationEntryCrossingInventory<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.declaration_entry_crossing_inventory::<I>()
}

pub fn research_declaration_entry_readiness<I>(
    handle: &HadwigerResearchHandle,
) -> WorthQueryDeclarationEntryReadinessReport<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.declaration_entry_readiness::<I>()
}

mod sealed {
    pub trait HadwigerResearchRequestSeal {}
}
