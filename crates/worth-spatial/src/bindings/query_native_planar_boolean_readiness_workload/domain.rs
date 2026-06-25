use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanReadinessWorkloadQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarBooleanReadinessWorkloadQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_boolean_readiness_workload"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarBooleanReadinessWorkloadDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanReadinessWorkloadDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarBooleanReadinessWorkloadQueryDomain>
    for PlanarBooleanReadinessWorkloadDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarBooleanReadinessWorkload"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_aspect_contract::declaration_aspect_contract_from_slices(
            &[
                "geometry.boolean_readiness.declaration",
                "geometry.boolean_readiness.evidence_ledger",
                "geometry.boolean_readiness.topology",
                "geometry.boolean_readiness.binding",
                "geometry.boolean_readiness.surface_support",
                "geometry.boolean_readiness.projection",
                "geometry.boolean_readiness.transform",
                "geometry.boolean_readiness.retained_replay",
                "geometry.boolean_readiness.parity",
                "geometry.boolean_readiness.diagnostics",
                "geometry.boolean_readiness.user_response",
                "geometry.boolean_readiness.contract_bundle",
                "geometry.boolean_readiness.blocker",
                "geometry.boolean_readiness.no_boolean_execution",
            ],
            &[
                "geometry.boolean_readiness.workload_receipt",
                "geometry.boolean_readiness.m7_input_receipt",
                "geometry.boolean_readiness.counters",
                "geometry.boolean_readiness.digest",
            ],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}
