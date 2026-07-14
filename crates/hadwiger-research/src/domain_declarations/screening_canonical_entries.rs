use worth_query::facade::foundation::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue, WorthQueryDeclarationInput,
};

use super::family_markers::{
    AutocorrelationZeroScreeningDeclarationFamily, BoundaryOwnershipScreeningDeclarationFamily,
    CandidateNoveltyScreeningDeclarationFamily, DensityCapScreeningDeclarationFamily,
    ExactArithmeticIntervalScreeningDeclarationFamily,
    ExactConflictGraphScreeningDeclarationFamily,
    ExactUnitDistanceConflictScreeningDeclarationFamily,
    ExhaustiveLocalNeighborhoodScreeningDeclarationFamily,
    FinitePatchBoundaryExtensionScreeningDeclarationFamily,
    ForbiddenDisplacementScreeningDeclarationFamily, FractionalChromaticScreeningDeclarationFamily,
    KnownObstructionContainmentScreeningDeclarationFamily,
    LocalDensityWindowScreeningDeclarationFamily, LovaszThetaScreeningDeclarationFamily,
    MinkowskiDifferenceScreeningDeclarationFamily,
    MonodromyColorHolonomyScreeningDeclarationFamily, NumericalMarginScreeningDeclarationFamily,
    PeriodicQuotientGraphScreeningDeclarationFamily, RigidityRealizationScreeningDeclarationFamily,
    SameColorSeparationScreeningDeclarationFamily,
    SubstitutionConsistencyScreeningDeclarationFamily,
    SymmetryOrbitReductionScreeningDeclarationFamily, TileDiameterScreeningDeclarationFamily,
    TranslationRotationClosureScreeningDeclarationFamily,
    UnitDistanceEmbeddabilityScreeningDeclarationFamily,
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
use super::screening_request_types::{
    AutocorrelationZeroScreeningDeclaration, DensityCapScreeningDeclaration,
    FractionalChromaticScreeningDeclaration, LocalDensityWindowScreeningDeclaration,
    LovaszThetaScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchDomainEntry;

fn kind_entry(kind: &'static str) -> WorthQueryDeclarationCanonicalEntry {
    WorthQueryDeclarationCanonicalEntry::text("declaration_kind", kind)
}

fn unsigned_entry(locus: &'static str, value: u32) -> WorthQueryDeclarationCanonicalEntry {
    WorthQueryDeclarationCanonicalEntry::new(
        locus,
        WorthQueryDeclarationCanonicalEntryKind::Field,
        WorthQueryDeclarationCanonicalValue::UnsignedInteger(value as u128),
    )
}
impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for FractionalChromaticScreeningDeclaration
{
    type Family = FractionalChromaticScreeningDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("fractional_chromatic_screening"),
            WorthQueryDeclarationCanonicalEntry::text(
                "graph_version_reference",
                self.graph_version_reference(),
            ),
            unsigned_entry("color_limit", self.color_limit()),
            WorthQueryDeclarationCanonicalEntry::text("screening_basis", self.screening_basis()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for LovaszThetaScreeningDeclaration {
    type Family = LovaszThetaScreeningDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("lovasz_theta_screening"),
            WorthQueryDeclarationCanonicalEntry::text(
                "graph_version_reference",
                self.graph_version_reference(),
            ),
            unsigned_entry("color_limit", self.color_limit()),
            WorthQueryDeclarationCanonicalEntry::text("screening_basis", self.screening_basis()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for AutocorrelationZeroScreeningDeclaration
{
    type Family = AutocorrelationZeroScreeningDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("autocorrelation_zero_screening"),
            WorthQueryDeclarationCanonicalEntry::text(
                "subject_reference",
                self.subject_reference(),
            ),
            WorthQueryDeclarationCanonicalEntry::text("model_reference", self.model_reference()),
            WorthQueryDeclarationCanonicalEntry::text("screening_basis", self.screening_basis()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for DensityCapScreeningDeclaration {
    type Family = DensityCapScreeningDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("density_cap_screening"),
            WorthQueryDeclarationCanonicalEntry::text(
                "subject_reference",
                self.subject_reference(),
            ),
            WorthQueryDeclarationCanonicalEntry::text("model_reference", self.model_reference()),
            WorthQueryDeclarationCanonicalEntry::text("color_id", self.color_id()),
            WorthQueryDeclarationCanonicalEntry::text(
                "retained_cap_reference",
                self.retained_cap_reference(),
            ),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for LocalDensityWindowScreeningDeclaration
{
    type Family = LocalDensityWindowScreeningDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("local_density_window_screening"),
            WorthQueryDeclarationCanonicalEntry::text(
                "subject_reference",
                self.subject_reference(),
            ),
            WorthQueryDeclarationCanonicalEntry::text("model_reference", self.model_reference()),
            WorthQueryDeclarationCanonicalEntry::text("window_reference", self.window_reference()),
            WorthQueryDeclarationCanonicalEntry::text("color_id", self.color_id()),
            WorthQueryDeclarationCanonicalEntry::text(
                "retained_bound_reference",
                self.retained_bound_reference(),
            ),
        ]
    }
}

macro_rules! subject_certificate_screening_input {
    ($type:ty, $family:ty, $kind:literal) => {
        impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for $type {
            type Family = $family;

            fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                vec![
                    kind_entry($kind),
                    WorthQueryDeclarationCanonicalEntry::text(
                        "subject_reference",
                        self.subject_reference(),
                    ),
                    WorthQueryDeclarationCanonicalEntry::text(
                        "certificate_reference",
                        self.certificate_reference(),
                    ),
                ]
            }
        }
    };
}

subject_certificate_screening_input!(
    ExactUnitDistanceConflictScreeningDeclaration,
    ExactUnitDistanceConflictScreeningDeclarationFamily,
    "exact_unit_distance_conflict_screening"
);
subject_certificate_screening_input!(
    SameColorSeparationScreeningDeclaration,
    SameColorSeparationScreeningDeclarationFamily,
    "same_color_separation_screening"
);
subject_certificate_screening_input!(
    TileDiameterScreeningDeclaration,
    TileDiameterScreeningDeclarationFamily,
    "tile_diameter_screening"
);
subject_certificate_screening_input!(
    ExactConflictGraphScreeningDeclaration,
    ExactConflictGraphScreeningDeclarationFamily,
    "exact_conflict_graph_screening"
);
subject_certificate_screening_input!(
    NumericalMarginScreeningDeclaration,
    NumericalMarginScreeningDeclarationFamily,
    "numerical_margin_screening"
);
subject_certificate_screening_input!(
    UnitDistanceEmbeddabilityScreeningDeclaration,
    UnitDistanceEmbeddabilityScreeningDeclarationFamily,
    "unit_distance_embeddability_screening"
);
subject_certificate_screening_input!(
    RigidityRealizationScreeningDeclaration,
    RigidityRealizationScreeningDeclarationFamily,
    "rigidity_realization_screening"
);
subject_certificate_screening_input!(
    ExactArithmeticIntervalScreeningDeclaration,
    ExactArithmeticIntervalScreeningDeclarationFamily,
    "exact_arithmetic_interval_screening"
);
subject_certificate_screening_input!(
    SymmetryOrbitReductionScreeningDeclaration,
    SymmetryOrbitReductionScreeningDeclarationFamily,
    "symmetry_orbit_reduction_screening"
);
subject_certificate_screening_input!(
    ExhaustiveLocalNeighborhoodScreeningDeclaration,
    ExhaustiveLocalNeighborhoodScreeningDeclarationFamily,
    "exhaustive_local_neighborhood_screening"
);
subject_certificate_screening_input!(
    KnownObstructionContainmentScreeningDeclaration,
    KnownObstructionContainmentScreeningDeclarationFamily,
    "known_obstruction_containment_screening"
);
subject_certificate_screening_input!(
    CandidateNoveltyScreeningDeclaration,
    CandidateNoveltyScreeningDeclarationFamily,
    "candidate_novelty_screening"
);
subject_certificate_screening_input!(
    BoundaryOwnershipScreeningDeclaration,
    BoundaryOwnershipScreeningDeclarationFamily,
    "boundary_ownership_screening"
);
subject_certificate_screening_input!(
    MonodromyColorHolonomyScreeningDeclaration,
    MonodromyColorHolonomyScreeningDeclarationFamily,
    "monodromy_color_holonomy_screening"
);
subject_certificate_screening_input!(
    TranslationRotationClosureScreeningDeclaration,
    TranslationRotationClosureScreeningDeclarationFamily,
    "translation_rotation_closure_screening"
);
subject_certificate_screening_input!(
    SubstitutionConsistencyScreeningDeclaration,
    SubstitutionConsistencyScreeningDeclarationFamily,
    "substitution_consistency_screening"
);
subject_certificate_screening_input!(
    FinitePatchBoundaryExtensionScreeningDeclaration,
    FinitePatchBoundaryExtensionScreeningDeclarationFamily,
    "finite_patch_boundary_extension_screening"
);

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for MinkowskiDifferenceScreeningDeclaration
{
    type Family = MinkowskiDifferenceScreeningDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("minkowski_difference_screening"),
            WorthQueryDeclarationCanonicalEntry::text(
                "subject_reference",
                self.subject_reference(),
            ),
            WorthQueryDeclarationCanonicalEntry::text(
                "certificate_reference",
                self.certificate_reference(),
            ),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for ForbiddenDisplacementScreeningDeclaration
{
    type Family = ForbiddenDisplacementScreeningDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("forbidden_displacement_screening"),
            WorthQueryDeclarationCanonicalEntry::text(
                "subject_reference",
                self.subject_reference(),
            ),
            WorthQueryDeclarationCanonicalEntry::text(
                "certificate_reference",
                self.certificate_reference(),
            ),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for PeriodicQuotientGraphScreeningDeclaration
{
    type Family = PeriodicQuotientGraphScreeningDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("periodic_quotient_graph_screening"),
            WorthQueryDeclarationCanonicalEntry::text(
                "subject_reference",
                self.subject_reference(),
            ),
            WorthQueryDeclarationCanonicalEntry::text("model_reference", self.model_reference()),
            WorthQueryDeclarationCanonicalEntry::text(
                "certificate_reference",
                self.certificate_reference(),
            ),
        ]
    }
}
