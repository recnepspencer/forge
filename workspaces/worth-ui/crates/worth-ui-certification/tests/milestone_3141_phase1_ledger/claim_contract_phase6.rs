pub(super) const NATIVE_PHASE6_PLATFORM_VERSIONS: &str = "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;wgpu-features=std+parking_lot+dx12+wgsl;xcap=0.9.7;xcap-features=wgc;winsafe=0.0.28;winsafe-host-features=user;winsafe-pulse-features=dwm+kernel+user;uiautomation=0.25.0;uiautomation-features=control+input+screenshot;win32job=2.0.3;native-pointer=GetMessagePos+event-ordered-client-origin+low16-wrapping;protocol=4";

pub(super) fn scenario_delta(requirement: &str) -> Option<&'static str> {
    Some(match requirement {
        "P6-PREDECESSOR-01" => "stale-phase-five-source",
        "P6-INPUT-AFFINITY-01" => "current-coordinate-retargeting",
        "P6-IME-01" => "preedit-as-text-input",
        "P6-POINTER-TIME-01" => "post-delivery-cursor-proxy",
        "P6-PROFILE-ORDER-01" => "synthetic-event-time",
        "P6-READINESS-01" => "silent-level-wake",
        "P6-SETTLEMENT-01" => "generic-error-for-typed-settlement",
        "P6-PROTOCOL-WORLD-01" => "oracle-substitution",
        "P6-WINDOWS-WORLD-01" => "get-cursor-pos-production-proxy",
        "P6-CLOSE-01" => "open-requirement",
        _ => return None,
    })
}

pub(super) fn construction_cost(requirement: &str) -> Option<&'static str> {
    if !requirement.starts_with("P6-") {
        return None;
    }
    Some(match requirement {
        "P6-PREDECESSOR-01" => {
            "main-tests=55;hostile-controls=57;product-processes=54;compile-sessions=2;courtroom-worlds=66"
        }
        "P6-WINDOWS-WORLD-01" => {
            "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;courtroom-worlds=1"
        }
        _ => {
            "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0"
        }
    })
}

pub(super) fn execution_cost(requirement: &str) -> Option<&'static str> {
    if !requirement.starts_with("P6-") {
        return None;
    }
    Some(match requirement {
        "P6-PREDECESSOR-01" => "executed-tests=114;presentations=207",
        "P6-WINDOWS-WORLD-01" => "executed-tests=2;presentations=1",
        _ => "executed-tests=2;presentations=0",
    })
}
