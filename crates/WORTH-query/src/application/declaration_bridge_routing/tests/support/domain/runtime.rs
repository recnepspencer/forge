use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryApplicationFacade,
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.bridge-routing"
    }

    fn display_name(&self) -> &'static str {
        "GeometryBridgeRoutingDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryWorld {
    regime: &'static str,
}

impl GeometryWorld {
    pub(crate) fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl WorthQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::HistoricalEvaluation,
            WorthQueryCapabilityFamily::WorkflowOrchestration,
            WorthQueryCapabilityFamily::PreviewSession,
            WorthQueryCapabilityFamily::LiveQuery,
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.bridge-routing.{}", self.regime)
    }
}

pub(crate) fn admitted_handle(
    regime: &'static str,
) -> WorthQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    WorthQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld::named(regime))
        .validate()
        .expect("bridge-routing world should validate")
        .admit()
        .expect("bridge-routing world should admit")
}

pub(crate) fn bridge_aspect_contract() -> WorthQueryDeclarationAspectContract {
    WorthQueryDeclarationAspectContract::from_slices(
        &["selection.active_face", "continuation.preview_ready"],
        &["selection.neighborhood.local_topology"],
        &["continuation.preview.surface"],
        &["continuation.private_branch"],
        &["continuation.conflicting_preview"],
    )
}

pub(crate) fn bridge_aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
    WorthQueryDeclarationAspectCoverage::from_slices(
        &[
            "selection.active_face",
            "continuation.preview_ready",
            "selection.neighborhood.local_topology",
            "continuation.preview.surface",
        ],
        &[],
        &[],
    )
}
