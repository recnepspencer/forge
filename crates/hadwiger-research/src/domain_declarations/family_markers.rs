use worth_query::facade::foundation::{
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRelationalTruthContract, WorthQueryDeclarationRouteContract,
    WorthQueryDescriptiveOnlyAuthority, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalNotCompatiblePosture,
    WorthQuerySingleOnlyGrouping,
};

use crate::query_entry::HadwigerResearchDomainEntry;

macro_rules! relational_family {
    ($name:ident, $key:literal) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;

        impl WorthQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry> for $name {
            type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
            type SignalCompatibility = WorthQuerySignalNotCompatiblePosture;
            type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                $key
            }

            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> WorthQueryDeclarationRouteContract {
                WorthQueryDeclarationRouteContract::relational_only()
            }

            fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
                Some(WorthQueryDeclarationRelationalTruthContract::authoritative_current_truth())
            }
        }
    };
}

macro_rules! descriptive_family {
    ($name:ident, $key:literal) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;

        impl WorthQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry> for $name {
            type PrimaryAuthority = WorthQueryDescriptiveOnlyAuthority;
            type SignalCompatibility = WorthQuerySignalNotCompatiblePosture;
            type GroupedPosture = WorthQuerySingleOnlyGrouping;

            fn semantic_family_key() -> &'static str {
                $key
            }

            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::descriptive_deferred_support()
            }

            fn route_contract() -> WorthQueryDeclarationRouteContract {
                WorthQueryDeclarationRouteContract::deferred_auto()
            }
        }
    };
}

relational_family!(CandidateGraphDeclarationFamily, "hadwiger.candidate_graph");
relational_family!(EmbeddingDeclarationFamily, "hadwiger.embedding");
relational_family!(ColorabilityDeclarationFamily, "hadwiger.colorability");
relational_family!(
    LowerBoundWitnessDeclarationFamily,
    "hadwiger.lower_bound_witness"
);
relational_family!(
    UnitDistanceVerificationDeclarationFamily,
    "hadwiger.unit_distance_verification"
);
relational_family!(
    WholePlaneColoringConstructionDeclarationFamily,
    "hadwiger.whole_plane_coloring_construction"
);
relational_family!(MotifSeedDeclarationFamily, "hadwiger.tiling.motif_seed");
relational_family!(
    TerminalForcingStudyDeclarationFamily,
    "hadwiger.tiling.terminal_forcing_study"
);
relational_family!(
    PeriodicQuotientCellDeclarationFamily,
    "hadwiger.tiling.periodic_quotient_cell"
);
relational_family!(
    GeneratedPatternClosureDeclarationFamily,
    "hadwiger.tiling.generated_pattern_closure"
);
relational_family!(
    TileContactWitnessDeclarationFamily,
    "hadwiger.tiling.tile_contact_witness"
);
relational_family!(
    ConflictGraphExtractionDeclarationFamily,
    "hadwiger.tiling.conflict_graph_extraction"
);
relational_family!(
    CoreExtractionDeclarationFamily,
    "hadwiger.tiling.core_extraction"
);
relational_family!(
    TilingEquivalenceClassificationDeclarationFamily,
    "hadwiger.tiling.equivalence_classification"
);
relational_family!(
    TilingSuppressionDeclarationFamily,
    "hadwiger.tiling.suppression"
);
relational_family!(
    TilingReactivationDeclarationFamily,
    "hadwiger.tiling.reactivation"
);
relational_family!(
    LowerBoundTilingIterationDeclarationFamily,
    "hadwiger.tiling.iteration.lower_bound_obstruction"
);
relational_family!(
    UpperBoundTilingIterationDeclarationFamily,
    "hadwiger.tiling.iteration.upper_bound_periodic_quotient"
);
relational_family!(
    FractionalChromaticScreeningDeclarationFamily,
    "hadwiger.screening.fractional_chromatic"
);
relational_family!(
    LovaszThetaScreeningDeclarationFamily,
    "hadwiger.screening.lovasz_theta"
);
relational_family!(
    AutocorrelationZeroScreeningDeclarationFamily,
    "hadwiger.screening.autocorrelation_zero"
);
relational_family!(
    DensityCapScreeningDeclarationFamily,
    "hadwiger.screening.density_cap"
);
relational_family!(
    LocalDensityWindowScreeningDeclarationFamily,
    "hadwiger.screening.local_density_window"
);
relational_family!(
    ExactUnitDistanceConflictScreeningDeclarationFamily,
    "hadwiger.screening.exact_unit_distance_conflict"
);
relational_family!(
    SameColorSeparationScreeningDeclarationFamily,
    "hadwiger.screening.same_color_separation"
);
relational_family!(
    TileDiameterScreeningDeclarationFamily,
    "hadwiger.screening.tile_diameter"
);
relational_family!(
    ExactConflictGraphScreeningDeclarationFamily,
    "hadwiger.screening.exact_conflict_graph"
);
relational_family!(
    NumericalMarginScreeningDeclarationFamily,
    "hadwiger.screening.numerical_margin"
);
relational_family!(
    MinkowskiDifferenceScreeningDeclarationFamily,
    "hadwiger.screening.minkowski_difference"
);
relational_family!(
    ForbiddenDisplacementScreeningDeclarationFamily,
    "hadwiger.screening.forbidden_displacement"
);
relational_family!(
    PeriodicQuotientGraphScreeningDeclarationFamily,
    "hadwiger.screening.periodic_quotient_graph"
);
relational_family!(
    UnitDistanceEmbeddabilityScreeningDeclarationFamily,
    "hadwiger.screening.unit_distance_embeddability"
);
relational_family!(
    RigidityRealizationScreeningDeclarationFamily,
    "hadwiger.screening.rigidity_realization"
);
relational_family!(
    ExactArithmeticIntervalScreeningDeclarationFamily,
    "hadwiger.screening.exact_arithmetic_interval"
);
relational_family!(
    SymmetryOrbitReductionScreeningDeclarationFamily,
    "hadwiger.screening.symmetry_orbit_reduction"
);
relational_family!(
    ExhaustiveLocalNeighborhoodScreeningDeclarationFamily,
    "hadwiger.screening.exhaustive_local_neighborhood"
);
relational_family!(
    KnownObstructionContainmentScreeningDeclarationFamily,
    "hadwiger.screening.known_obstruction_containment"
);
relational_family!(
    CandidateNoveltyScreeningDeclarationFamily,
    "hadwiger.screening.candidate_novelty"
);
relational_family!(
    BoundaryOwnershipScreeningDeclarationFamily,
    "hadwiger.screening.boundary_ownership"
);
relational_family!(
    MonodromyColorHolonomyScreeningDeclarationFamily,
    "hadwiger.screening.monodromy_color_holonomy"
);
relational_family!(
    TranslationRotationClosureScreeningDeclarationFamily,
    "hadwiger.screening.translation_rotation_closure"
);
relational_family!(
    SubstitutionConsistencyScreeningDeclarationFamily,
    "hadwiger.screening.substitution_consistency"
);
relational_family!(
    FinitePatchBoundaryExtensionScreeningDeclarationFamily,
    "hadwiger.screening.finite_patch_boundary_extension"
);
relational_family!(
    PlaneLowerBoundClaimDeclarationFamily,
    "hadwiger.plane_lower_bound_claim"
);
relational_family!(
    PlaneUpperBoundClaimDeclarationFamily,
    "hadwiger.plane_upper_bound_claim"
);
relational_family!(
    PlaneExactValueClaimDeclarationFamily,
    "hadwiger.plane_exact_value_claim"
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdvisoryNoteDeclarationFamily;

impl WorthQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>
    for AdvisoryNoteDeclarationFamily
{
    type PrimaryAuthority = WorthQueryDescriptiveOnlyAuthority;
    type SignalCompatibility = WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = WorthQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "hadwiger.advisory_note"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::descriptive_deferred_support()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

descriptive_family!(
    RejectionExplanationDeclarationFamily,
    "hadwiger.rejection_explanation"
);
descriptive_family!(
    PartialAdmissionExplanationDeclarationFamily,
    "hadwiger.partial_admission_explanation"
);
descriptive_family!(
    BackgroundTheoremDeclarationFamily,
    "hadwiger.background_theorem"
);
