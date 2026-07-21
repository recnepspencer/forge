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

/// Narrow immutable host authority carried into plan construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiHostPlanBinding {
    session_identity: WorthUiHostSessionIdentity,
    observation_generation: WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    canvas_spatial_supported: bool,
    realtime_overlay_supported: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiHostSessionActivationDenial {
    IdentityExhausted,
}

impl WorthUiHostSessionAuthority {
    pub(crate) fn activate(
        plan: &crate::facade::prepared_application_authority::WorthUiHostSessionPlan,
    ) -> Result<Self, WorthUiHostSessionActivationDenial> {
        let value = NEXT_HOST_SESSION_IDENTITY
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                next_host_session_identity_value(current)
            })
            .map_err(|_| WorthUiHostSessionActivationDenial::IdentityExhausted)?;
        let identity = WorthUiHostSessionIdentity { value };
        let capability_report = plan
            .capability_report()
            .clone()
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(value));
        Ok(Self {
            identity,
            measurement_capability: WorthUiHostMeasurementCapability {
                session_identity: identity,
                capability_report,
                adapter: plan.adapter(),
            },
        })
    }

    pub(crate) fn identity(&self) -> WorthUiHostSessionIdentity {
        self.identity
    }

    pub(crate) fn measurement_capability(&self) -> WorthUiHostMeasurementCapability {
        self.measurement_capability.clone()
    }

    pub(crate) fn output_adapter(&self) -> &dyn WorthUiOperationalHostAdapter {
        self.measurement_capability.adapter()
    }

    pub(crate) fn plan_binding(&self) -> WorthUiHostPlanBinding {
        WorthUiHostPlanBinding::from_session(self)
    }
}

impl WorthUiHostPlanBinding {
    fn from_session(session: &WorthUiHostSessionAuthority) -> Self {
        let report = session.measurement_capability.capability_report();
        let required = [
            worth_ui_host_contract::WorthUiHostCapability::CanvasSpatialDraw,
            worth_ui_host_contract::WorthUiHostCapability::CanvasSpatialHitTest,
            worth_ui_host_contract::WorthUiHostCapability::CanvasSpatialOverlay,
            worth_ui_host_contract::WorthUiHostCapability::CanvasSpatialToolState,
            worth_ui_host_contract::WorthUiHostCapability::CanvasSpatialRenderResource,
        ];
        let realtime_required = [
            worth_ui_host_contract::WorthUiHostCapability::RealtimeOverlayDraw,
            worth_ui_host_contract::WorthUiHostCapability::RealtimeOverlaySurface,
            worth_ui_host_contract::WorthUiHostCapability::RealtimeOverlayHook,
        ];
        Self {
            session_identity: session.identity,
            observation_generation: report.observation_generation(),
            capability_profile_digest: report.profile_identity_digest(),
            canvas_spatial_supported: required
                .into_iter()
                .all(|capability| report.supports(capability)),
            realtime_overlay_supported: realtime_required
                .into_iter()
                .all(|capability| report.supports(capability)),
        }
    }

    pub(crate) fn session_identity(self) -> WorthUiHostSessionIdentity {
        self.session_identity
    }

    pub(crate) fn observation_generation(self) -> WorthUiHostCapabilityObservationGeneration {
        self.observation_generation
    }

    pub(crate) fn capability_profile_digest(self) -> u64 {
        self.capability_profile_digest
    }

    pub(crate) fn canvas_spatial_supported(self) -> bool {
        self.canvas_spatial_supported
    }

    pub(crate) fn realtime_overlay_supported(self) -> bool {
        self.realtime_overlay_supported
    }

    pub(crate) fn shares_session_with(self, other: Self) -> bool {
        self.session_identity == other.session_identity
    }

    pub(crate) fn executable_contract_matches(self, other: Self) -> bool {
        self.canvas_spatial_supported == other.canvas_spatial_supported
            && self.realtime_overlay_supported == other.realtime_overlay_supported
    }
}

fn next_host_session_identity_value(current: u64) -> Option<u64> {
    current.checked_add(1)
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

#[cfg(test)]
mod tests {
    use worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration;

    use super::{WorthUiHostPlanBinding, WorthUiHostSessionIdentity};

    #[test]
    fn host_session_identity_capacity_never_wraps() {
        assert_eq!(
            super::next_host_session_identity_value(u64::MAX - 1),
            Some(u64::MAX)
        );
        assert_eq!(super::next_host_session_identity_value(u64::MAX), None);
    }

    #[test]
    fn host_plan_equivalence_includes_capabilities_but_excludes_provenance() {
        let baseline = WorthUiHostPlanBinding {
            session_identity: WorthUiHostSessionIdentity { value: 1 },
            observation_generation: WorthUiHostCapabilityObservationGeneration::new(1),
            capability_profile_digest: 11,
            canvas_spatial_supported: true,
            realtime_overlay_supported: true,
        };
        let cases = [
            ("identical", baseline, true),
            (
                "session identity",
                WorthUiHostPlanBinding {
                    session_identity: WorthUiHostSessionIdentity { value: 2 },
                    ..baseline
                },
                true,
            ),
            (
                "observation generation",
                WorthUiHostPlanBinding {
                    observation_generation: WorthUiHostCapabilityObservationGeneration::new(2),
                    ..baseline
                },
                true,
            ),
            (
                "profile provenance digest",
                WorthUiHostPlanBinding {
                    capability_profile_digest: 12,
                    ..baseline
                },
                true,
            ),
            (
                "canvas capability",
                WorthUiHostPlanBinding {
                    canvas_spatial_supported: false,
                    ..baseline
                },
                false,
            ),
            (
                "realtime capability",
                WorthUiHostPlanBinding {
                    realtime_overlay_supported: false,
                    ..baseline
                },
                false,
            ),
        ];
        for (name, candidate, expected) in cases {
            assert_eq!(
                baseline.executable_contract_matches(candidate),
                expected,
                "host-binding matrix row `{name}` drifted"
            );
        }
    }
}
