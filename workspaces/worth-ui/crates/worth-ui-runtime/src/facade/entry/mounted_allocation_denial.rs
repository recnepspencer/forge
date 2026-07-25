use worth_ui_host_contract::UiMeasurementEvidenceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiMountedAllocationRuntimeStage {
    CatalogPreparation,
}

#[derive(Debug)]
pub enum WorthUiMountedAllocationEstablishmentDenial {
    PresentationInFlight,
    NoAllocationPlanningNodes,
    MissingMountedInstance(crate::graph::UiGraphNodeIdentity),
    MountedIdentity(crate::mounting::UiMountedIdentityDenial),
    GraphMountEligibility(crate::graph::UiGraphMountEligibilityAdmissionDenial),
    StaleGraphSuccessor,
    MissingMeasurementRequest(UiMeasurementEvidenceFamily),
    MeasurementRequestIdentityExhausted,
    HostMeasurement(crate::host::UiHostMeasurementEvidenceDenial),
    MeasurementBasis {
        node: crate::graph::UiGraphNodeIdentity,
        denial: crate::evidence::UiMeasurementBasisDenial,
    },
    CatalogAdmission(crate::graph::UiAllocationCatalogBasisAdmissionDenial),
    Activation(crate::runtime::WorthUiAllocationCatalogActivationDenial),
    Runtime(WorthUiMountedAllocationRuntimeStage),
}

pub(super) fn map_initial_activation_denial(
    denial: crate::runtime::WorthUiInitialMountedAllocationActivationDenial,
) -> WorthUiMountedAllocationEstablishmentDenial {
    WorthUiMountedAllocationEstablishmentDenial::Activation(denial.into_public_denial())
}
