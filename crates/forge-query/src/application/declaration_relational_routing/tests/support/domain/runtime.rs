use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.relational-routing"
    }

    fn display_name(&self) -> &'static str {
        "GeometryRelationalRoutingDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
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

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.relational-routing.{}", self.regime)
    }
}

pub(crate) fn admitted_handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld::named(regime))
        .validate()
        .expect("relational-routing world should validate")
        .admit()
        .expect("relational-routing world should admit")
}

pub(crate) fn relational_aspect_contract() -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::from_slices(
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

pub(crate) fn relational_aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
    ForgeQueryDeclarationAspectCoverage::from_slices(
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
