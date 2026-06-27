use super::catalog::{CoveredEvidenceLookupSurface, EvidenceLookupCatalogDiscoveryExpectation};
use super::row::{
    EvidenceLookupAuthorityKind as Kind, EvidenceLookupCertificationPosture as Cert,
    EvidenceLookupCostPosture as Cost, EvidenceLookupDisposition as Disposition,
    EvidenceLookupInventoryRowScope as Scope, EvidenceLookupOwner as Owner,
    EvidenceLookupQuerySurface as Query, EvidenceLookupReplacementPhase as Phase,
};

pub(crate) fn fixture_surface(
    source_path: &'static str,
    row_scope: Scope,
) -> CoveredEvidenceLookupSurface {
    CoveredEvidenceLookupSurface {
        source_path,
        surface_name: "fixture evidence lookup surface",
        owner: Owner::WorthSpatial,
        current_caller: "fixture scanner",
        authority_kind: Kind::StageLocalNearbyLookup,
        disposition: Disposition::Migrate,
        replacement_phase: Phase::PhaseTwoFamilyCatalog,
        blocker: "fixture lookup source must be classified by catalog",
        removal_trigger: "fixture catalog coverage",
        certification_posture: Cert::OrdinaryProductionReachable,
        cost_posture: Cost::LocalTypedLookup,
        query_surface: Query::NotQuery,
        row_scope,
        discovery_expectation: match row_scope {
            Scope::ConcreteSource => EvidenceLookupCatalogDiscoveryExpectation::DiscoveryRequired,
            Scope::FamilySummary => EvidenceLookupCatalogDiscoveryExpectation::CatalogOnly,
        },
    }
}
