use worth_query::facade::domain::{
    WorthQueryDeclarationFamilyMarker, WorthQueryDomainDeclarationFamilyDefinition,
};

use crate::query_entry::HadwigerResearchDomainEntry;

use super::*;

fn family<F>() -> WorthQueryDomainDeclarationFamilyDefinition
where
    F: WorthQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>,
{
    WorthQueryDomainDeclarationFamilyDefinition::from_marker::<HadwigerResearchDomainEntry, F>(1)
        .expect("typed Hadwiger declaration family keys must be valid package identities")
}

pub(crate) fn hadwiger_declaration_family_definitions(
) -> Vec<WorthQueryDomainDeclarationFamilyDefinition> {
    vec![
        family::<AdvisoryNoteDeclarationFamily>(),
        family::<AutocorrelationZeroScreeningDeclarationFamily>(),
        family::<BackgroundTheoremDeclarationFamily>(),
        family::<BoundaryOwnershipScreeningDeclarationFamily>(),
        family::<CandidateGraphDeclarationFamily>(),
        family::<CandidateNoveltyScreeningDeclarationFamily>(),
        family::<ColorabilityDeclarationFamily>(),
        family::<ConflictGraphExtractionDeclarationFamily>(),
        family::<CoreExtractionDeclarationFamily>(),
        family::<DensityCapScreeningDeclarationFamily>(),
        family::<EmbeddingDeclarationFamily>(),
        family::<ExactArithmeticIntervalScreeningDeclarationFamily>(),
        family::<ExactConflictGraphScreeningDeclarationFamily>(),
        family::<ExactUnitDistanceConflictScreeningDeclarationFamily>(),
        family::<ExhaustiveLocalNeighborhoodScreeningDeclarationFamily>(),
        family::<FinitePatchBoundaryExtensionScreeningDeclarationFamily>(),
        family::<ForbiddenDisplacementScreeningDeclarationFamily>(),
        family::<FractionalChromaticScreeningDeclarationFamily>(),
        family::<GeneratedPatternClosureDeclarationFamily>(),
        family::<KnownObstructionContainmentScreeningDeclarationFamily>(),
        family::<LocalDensityWindowScreeningDeclarationFamily>(),
        family::<LovaszThetaScreeningDeclarationFamily>(),
        family::<LowerBoundTilingIterationDeclarationFamily>(),
        family::<LowerBoundWitnessDeclarationFamily>(),
        family::<MinkowskiDifferenceScreeningDeclarationFamily>(),
        family::<MonodromyColorHolonomyScreeningDeclarationFamily>(),
        family::<MotifSeedDeclarationFamily>(),
        family::<NumericalMarginScreeningDeclarationFamily>(),
        family::<PartialAdmissionExplanationDeclarationFamily>(),
        family::<PeriodicQuotientCellDeclarationFamily>(),
        family::<PeriodicQuotientGraphScreeningDeclarationFamily>(),
        family::<PlaneExactValueClaimDeclarationFamily>(),
        family::<PlaneLowerBoundClaimDeclarationFamily>(),
        family::<PlaneUpperBoundClaimDeclarationFamily>(),
        family::<RejectionExplanationDeclarationFamily>(),
        family::<RigidityRealizationScreeningDeclarationFamily>(),
        family::<SameColorSeparationScreeningDeclarationFamily>(),
        family::<SubstitutionConsistencyScreeningDeclarationFamily>(),
        family::<SymmetryOrbitReductionScreeningDeclarationFamily>(),
        family::<TerminalForcingStudyDeclarationFamily>(),
        family::<TileContactWitnessDeclarationFamily>(),
        family::<TileDiameterScreeningDeclarationFamily>(),
        family::<TilingEquivalenceClassificationDeclarationFamily>(),
        family::<TilingReactivationDeclarationFamily>(),
        family::<TilingSuppressionDeclarationFamily>(),
        family::<TranslationRotationClosureScreeningDeclarationFamily>(),
        family::<UnitDistanceEmbeddabilityScreeningDeclarationFamily>(),
        family::<UnitDistanceVerificationDeclarationFamily>(),
        family::<UpperBoundTilingIterationDeclarationFamily>(),
        family::<WholePlaneColoringConstructionDeclarationFamily>(),
    ]
}
