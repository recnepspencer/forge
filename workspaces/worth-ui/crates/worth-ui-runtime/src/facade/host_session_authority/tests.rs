use worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration;

use super::{
    WorthUiHostPlanBinding, WorthUiHostSessionAuthority, WorthUiHostSessionIdentity,
    WorthUiHostSessionReleaseRecovery,
};

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
        canvas_spatial_execution_supported: true,
        realtime_overlay_execution_supported: true,
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
                canvas_spatial_execution_supported: false,
                ..baseline
            },
            false,
        ),
        (
            "realtime capability",
            WorthUiHostPlanBinding {
                realtime_overlay_execution_supported: false,
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

#[test]
fn indeterminate_launch_cleanup_retains_the_exact_host_authority_for_retry() {
    let attempts = std::rc::Rc::new(std::cell::Cell::new(0));
    let plan = crate::facade::prepared_application_authority::WorthUiHostSessionPlan::prepare(
        RetryReleaseAdapter {
            attempts: std::rc::Rc::clone(&attempts),
        },
    );
    let mut session = WorthUiHostSessionAuthority::activate(&plan).expect("host session activates");
    let identity = session.identity();
    assert!(matches!(
        session.release_adapter_session(),
        worth_ui_host_contract::UiHostSessionReleaseOutcome::ReleaseIndeterminate(_)
    ));
    let recovery = WorthUiHostSessionReleaseRecovery::retain(session);
    assert_eq!(recovery.host_session_identity(), identity);
    let recovery = recovery
        .retry()
        .expect_err("second indeterminate release retains authority");
    let receipt = recovery.retry().expect("third release settles");
    assert_eq!(receipt.host_session_identity(), identity.as_u64());
    assert_eq!(attempts.get(), 3);
}

struct RetryReleaseAdapter {
    attempts: std::rc::Rc<std::cell::Cell<u8>>,
}

impl worth_ui_host_contract::WorthUiMeasurementHostAdapter for RetryReleaseAdapter {
    fn observe_measurement(
        &self,
        _request: &worth_ui_host_contract::UiHostMeasurementRequest,
    ) -> worth_ui_host_contract::UiHostMeasurementObservationValue {
        unreachable!("cleanup authority proof performs no measurement")
    }
}

impl crate::host::adapter::WorthUiOperationalHostAdapter for RetryReleaseAdapter {
    fn operational_host_contract(&self) -> worth_ui_host_contract::WorthUiHostContract {
        worth_ui_host_contract::WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> worth_ui_host_contract::WorthUiHostCapabilityReport {
        worth_ui_host_contract::WorthUiHostCapabilityReport::available(vec![])
    }

    fn release_host_session(
        &self,
        authority: &crate::host::adapter::UiHostAdapterSessionAuthority,
    ) -> crate::host::adapter::UiHostSessionReleaseOutcome {
        let attempt = self.attempts.get() + 1;
        self.attempts.set(attempt);
        if attempt <= 2 {
            crate::host::adapter::UiHostSessionReleaseOutcome::ReleaseIndeterminate(
                crate::host::adapter::UiHostSessionReleaseIndeterminate::after_effects_may_have_begun(
                    authority.host_session_identity(),
                ),
            )
        } else {
            crate::host::adapter::UiHostSessionReleaseOutcome::Released(
                crate::host::adapter::UiHostSessionReleaseReceipt::released(
                    authority.host_session_identity(),
                    0,
                ),
            )
        }
    }
}
