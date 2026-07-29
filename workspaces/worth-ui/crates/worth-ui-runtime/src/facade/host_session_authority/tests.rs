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
