use forge_query::facade::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRelationalTruthContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDescriptiveOnlyAuthority, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
};

use crate::query_entry::HadwigerResearchDomainEntry;

macro_rules! relational_family {
    ($name:ident, $key:literal) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;

        impl ForgeQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry> for $name {
            type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
            type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                $key
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                ForgeQueryDeclarationRouteContract::relational_only()
            }

            fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
                Some(ForgeQueryDeclarationRelationalTruthContract::authoritative_current_truth())
            }
        }
    };
}

macro_rules! descriptive_family {
    ($name:ident, $key:literal) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;

        impl ForgeQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry> for $name {
            type PrimaryAuthority = ForgeQueryDescriptiveOnlyAuthority;
            type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
            type GroupedPosture = ForgeQuerySingleOnlyGrouping;

            fn semantic_family_key() -> &'static str {
                $key
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::descriptive_deferred_support()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                ForgeQueryDeclarationRouteContract::deferred_auto()
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
relational_family!(
    FractionalChromaticScreeningDeclarationFamily,
    "hadwiger.screening.fractional_chromatic"
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

impl ForgeQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>
    for AdvisoryNoteDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryDescriptiveOnlyAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "hadwiger.advisory_note"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::descriptive_deferred_support()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
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
