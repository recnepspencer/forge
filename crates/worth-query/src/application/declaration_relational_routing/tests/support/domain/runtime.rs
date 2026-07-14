use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.relational-routing"
    }

    fn display_name(&self) -> &'static str {
        "GeometryRelationalRoutingDomain"
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
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.relational-routing.{}", self.regime)
    }
}

pub(crate) fn admitted_handle(
    regime: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld> {
    WorthQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld::named(regime))
        .validate()
        .expect("relational-routing world should validate")
        .admit()
        .expect("relational-routing world should admit")
}

pub(crate) fn relational_aspect_contract() -> WorthQueryDeclarationAspectContract {
    WorthQueryDeclarationAspectContract::from_slices(
        &[
            "selection.active_face",
            "selection.neighborhood.local_topology",
        ],
        &["selection.material_edit"],
        &["selection.preview.surface"],
        &["selection.private_authority"],
        &["selection.conflicting_preview"],
    )
}

pub(crate) fn relational_aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
    WorthQueryDeclarationAspectCoverage::from_slices(
        &[
            "selection.active_face",
            "selection.neighborhood.local_topology",
            "selection.material_edit",
            "selection.preview.surface",
        ],
        &[],
        &[],
    )
}
