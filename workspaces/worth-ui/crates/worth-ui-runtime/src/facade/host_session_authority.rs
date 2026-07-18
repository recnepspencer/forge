use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity,
    WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
    WorthUiOperationalHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::host::{
    UiHostMeasurementCollectionInput, UiHostMeasurementNeed, UiHostMeasurementNormalizationContext,
};

static NEXT_HOST_SESSION_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHostSessionIdentity {
    value: u64,
}

#[derive(Clone)]
pub struct WorthUiHostMeasurementCapability {
    session_identity: WorthUiHostSessionIdentity,
    capability_report: WorthUiHostCapabilityReport,
    adapter: Rc<dyn WorthUiOperationalHostAdapter>,
}

pub struct WorthUiHostMeasurementSessionInput {
    identity: UiMeasurementRequestIdentity,
    evidence_family: UiMeasurementEvidenceFamily,
    need: UiHostMeasurementNeed,
    evidence_generation: UiEvidenceAuthorityGeneration,
    normalization_context: UiHostMeasurementNormalizationContext,
}

pub(crate) struct WorthUiHostSessionAuthority {
    identity: WorthUiHostSessionIdentity,
    measurement_capability: WorthUiHostMeasurementCapability,
}

impl WorthUiHostSessionAuthority {
    pub(crate) fn activate(
        plan: &crate::facade::prepared_application_authority::WorthUiHostSessionPlan,
    ) -> Self {
        let value = NEXT_HOST_SESSION_IDENTITY.fetch_add(1, Ordering::Relaxed);
        let identity = WorthUiHostSessionIdentity { value };
        let capability_report = plan
            .capability_report()
            .clone()
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(value));
        Self {
            identity,
            measurement_capability: WorthUiHostMeasurementCapability {
                session_identity: identity,
                capability_report,
                adapter: plan.adapter(),
            },
        }
    }

    pub(crate) fn identity(&self) -> WorthUiHostSessionIdentity {
        self.identity
    }

    pub(crate) fn observation_generation(&self) -> WorthUiHostCapabilityObservationGeneration {
        self.measurement_capability.observation_generation()
    }

    pub(crate) fn measurement_capability(&self) -> WorthUiHostMeasurementCapability {
        self.measurement_capability.clone()
    }
}

impl WorthUiHostSessionIdentity {
    pub fn as_u64(self) -> u64 {
        self.value
    }
}

impl WorthUiHostMeasurementCapability {
    pub fn session_identity(&self) -> WorthUiHostSessionIdentity {
        self.session_identity
    }

    pub fn observation_generation(&self) -> WorthUiHostCapabilityObservationGeneration {
        self.capability_report.observation_generation()
    }

    pub(crate) fn adapter(&self) -> &dyn WorthUiOperationalHostAdapter {
        self.adapter.as_ref()
    }

    pub fn capability_report(&self) -> &WorthUiHostCapabilityReport {
        &self.capability_report
    }
}

impl WorthUiHostMeasurementSessionInput {
    pub fn new(
        identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        need: UiHostMeasurementNeed,
        evidence_generation: UiEvidenceAuthorityGeneration,
        normalization_context: UiHostMeasurementNormalizationContext,
    ) -> Self {
        Self {
            identity,
            evidence_family,
            need,
            evidence_generation,
            normalization_context,
        }
    }

    pub(crate) fn bind_report(
        self,
        capability_report: &WorthUiHostCapabilityReport,
    ) -> UiHostMeasurementCollectionInput<'_> {
        UiHostMeasurementCollectionInput {
            identity: self.identity,
            evidence_family: self.evidence_family,
            need: self.need,
            capability_report,
            evidence_generation: self.evidence_generation,
            normalization_context: self.normalization_context,
        }
    }
}
