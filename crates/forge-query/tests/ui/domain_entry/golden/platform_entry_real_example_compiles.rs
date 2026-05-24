use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryChecked,
    ForgeQueryDomainEntryMarker,
};

const SPATIAL_ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::QueryComposition,
    ForgeQueryCapabilityFamily::QueryContext,
    ForgeQueryCapabilityFamily::IdentityEvolution,
    ForgeQueryCapabilityFamily::PreviewSession,
    ForgeQueryCapabilityFamily::WorkflowOrchestration,
    ForgeQueryCapabilityFamily::HistoricalEvaluation,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SpatialDomain;

impl ForgeQueryDomainEntryMarker for SpatialDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial"
    }

    fn display_name(&self) -> &'static str {
        "SpatialDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        SPATIAL_ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let support = query.domain_entry_support_snapshot();

    match query.domain_checked(SpatialDomain) {
        ForgeQueryDomainEntryChecked::Admitted(root) => {
            let _ = root.domain_key();
            let _ = root.support_snapshot().snapshot_digest();
            let _ = support.section_postures();
        }
        ForgeQueryDomainEntryChecked::Deferred(deferred) => {
            let _ = deferred.blocking_capability_families();
        }
        ForgeQueryDomainEntryChecked::Unsupported(unsupported) => {
            let _ = unsupported.blocking_capability_families();
        }
    }
}
