use worth_query::facade::foundation::{WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryDomainEntryChecked, WorthQueryDomainEntryMarker};

const SPATIAL_ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::QueryComposition,
    WorthQueryCapabilityFamily::QueryContext,
    WorthQueryCapabilityFamily::IdentityEvolution,
    WorthQueryCapabilityFamily::PreviewSession,
    WorthQueryCapabilityFamily::WorkflowOrchestration,
    WorthQueryCapabilityFamily::HistoricalEvaluation,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SpatialDomain;

impl WorthQueryDomainEntryMarker for SpatialDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial"
    }

    fn display_name(&self) -> &'static str {
        "SpatialDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        SPATIAL_ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let support = query.domain_entry_support_snapshot();

    match query.domain_checked(SpatialDomain) {
        WorthQueryDomainEntryChecked::Admitted(root) => {
            let _ = root.domain_key();
            let _ = root.support_snapshot().snapshot_digest();
            let _ = support.section_postures();
        }
        WorthQueryDomainEntryChecked::Deferred(deferred) => {
            let _ = deferred.blocking_capability_families();
        }
        WorthQueryDomainEntryChecked::Unsupported(unsupported) => {
            let _ = unsupported.blocking_capability_families();
        }
    }
}
