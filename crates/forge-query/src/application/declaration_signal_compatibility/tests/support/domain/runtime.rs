use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.signal-compatibility"
    }

    fn display_name(&self) -> &'static str {
        "GeometrySignalCompatibilityDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryWorld(pub &'static str);

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("signal.compatibility.{}", self.0)
    }
}

pub fn handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld(regime))
        .validate()
        .expect("signal compatibility world should validate")
        .admit()
        .expect("signal compatibility world should admit")
}

pub(crate) fn signal_aspect_contract() -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_face", "signal.dependency.runtime_inputs"],
        &["selection.neighborhood.local_topology"],
        &["signal.preview.surface"],
        &["signal.private_authority"],
        &["signal.conflicting_dependency"],
    )
}

pub(crate) fn signal_aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
    ForgeQueryDeclarationAspectCoverage::from_slices(
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

pub(crate) fn signal_dependency_aspects() -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_face", "signal.dependency.runtime_inputs"],
        &["selection.neighborhood.local_topology"],
        &[],
        &["signal.private_authority"],
        &["signal.conflicting_dependency"],
    )
}

pub(crate) fn signal_produced_aspects() -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::from_slices(
        &["signal.produced.derived_face_preview"],
        &["signal.produced.material_projection"],
        &["signal.produced.preview.surface"],
        &[],
        &[],
    )
}
