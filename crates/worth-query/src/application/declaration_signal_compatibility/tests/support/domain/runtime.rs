use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.signal-compatibility"
    }

    fn display_name(&self) -> &'static str {
        "GeometrySignalCompatibilityDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryWorld(pub &'static str);

impl WorthQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::HistoricalEvaluation,
            WorthQueryCapabilityFamily::WorkflowOrchestration,
            WorthQueryCapabilityFamily::PreviewSession,
            WorthQueryCapabilityFamily::QueryComposition,
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
        format!("signal.compatibility.{}", self.0)
    }
}

pub fn handle(
    regime: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld> {
    WorthQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld(regime))
        .validate()
        .expect("signal compatibility world should validate")
        .admit()
        .expect("signal compatibility world should admit")
}

pub(crate) fn signal_aspect_contract() -> WorthQueryDeclarationAspectContract {
    WorthQueryDeclarationAspectContract::from_slices(
        &["selection.active_face", "signal.dependency.runtime_inputs"],
        &["selection.neighborhood.local_topology"],
        &["signal.preview.surface"],
        &["signal.private_authority"],
        &["signal.conflicting_dependency"],
    )
}

pub(crate) fn signal_aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
    WorthQueryDeclarationAspectCoverage::from_slices(
        &[
            "selection.active_face",
            "signal.dependency.runtime_inputs",
            "selection.neighborhood.local_topology",
            "signal.preview.surface",
        ],
        &[],
        &[],
    )
}

pub(crate) fn signal_dependency_aspects() -> WorthQueryDeclarationAspectContract {
    WorthQueryDeclarationAspectContract::from_slices(
        &["selection.active_face", "signal.dependency.runtime_inputs"],
        &["selection.neighborhood.local_topology"],
        &[],
        &["signal.private_authority"],
        &["signal.conflicting_dependency"],
    )
}

pub(crate) fn signal_produced_aspects() -> WorthQueryDeclarationAspectContract {
    WorthQueryDeclarationAspectContract::from_slices(
        &["signal.produced.derived_face_preview"],
        &["signal.produced.material_projection"],
        &["signal.produced.preview.surface"],
        &[],
        &[],
    )
}
