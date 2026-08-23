BASIC_PLATFORM_VERSIONS = "protocol=4"
PROFILE_PLATFORM_VERSIONS = (
    "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;"
    "wgpu-features=std+parking_lot+dx12+wgsl;rustybuzz=0.20.1;"
    "swash=0.2.10;protocol=4"
)
NATIVE_PLATFORM_VERSIONS = (
    "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;"
    "wgpu-features=std+parking_lot+dx12+wgsl;xcap=0.9.7;xcap-features=wgc;"
    "winsafe=0.0.28;winsafe-features=dwm+kernel+user;uiautomation=0.25.0;"
    "uiautomation-features=control+input+screenshot;win32job=2.0.3;protocol=4"
)
NATIVE_PHASE6_PLATFORM_VERSIONS = (
    "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;"
    "wgpu-features=std+parking_lot+dx12+wgsl;xcap=0.9.7;xcap-features=wgc;"
    "winsafe=0.0.28;winsafe-host-features=user;"
    "winsafe-pulse-features=dwm+kernel+user;uiautomation=0.25.0;"
    "uiautomation-features=control+input+screenshot;win32job=2.0.3;"
    "native-pointer=GetMessagePos+event-ordered-client-origin+low16-wrapping;protocol=4"
)
TEXT_PLATFORM_VERSIONS = (
    "harfrust=0.12.0;harfrust-features=std;read-fonts=0.41.0;"
    "read-fonts-features=std+experimental_traverse;icu-segmenter=2.2.0;"
    "skrifa=0.44.0;skrifa-features=std;"
    "kurbo=0.13.1;kurbo-features=default+serde+std;linesweeper=0.4.0;"
    "linesweeper-features=none;"
    "icu-segmenter-features=compiled_data+auto;unicode-bidi=0.3.18;"
    "unicode-bidi-features=std;unicode-segmentation=1.13.3;protocol=5;"
    "text-profile=worth-ui-global-text-v2;qualification=closed"
)


MOUNTED_BASELINE_REQUIREMENTS = {
    "P1-AFFINITY-01", "P1-BASELINE-01", "P1-CONSUMERS-01", "P1-DAMAGE-01",
    "P1-HEADLESS-01", "P1-HEADLESS-COST-01", "P1-ORDER-01",
    "P1-ORDER-SOURCE-01", "P1-PRESENTATION-AUTHORITY-01", "P1-PRODUCER-01",
    "P1-PRODUCER-COST-01", "P1-PROTOCOL-01", "P1-WORLDS-01",
}
P3_NATIVE_REQUIREMENTS = {
    "P3-BASELINE-REPLAY-01", "P3-DAMAGE-INDEX-01", "P3-DAMAGE-REPLAY-01",
    "P3-CLIPPED-DELTA-01", "P3-DRAW-LIST-01", "P3-HP02-WORLD-01",
    "P3-PHYSICAL-AMPLIFICATION-01", "P3-TOTAL-ORDER-01", "P3-TRANSACTION-01",
    "P3-UNCHANGED-01",
}


def baseline_path(requirement: str) -> str | None:
    if (
        requirement.startswith(("P2-", "P6-"))
        or requirement in P3_NATIVE_REQUIREMENTS
        or "PROFILE" in requirement
        or "BACKEND" in requirement
    ):
        return (
            "workspaces/worth-ui/crates/worth-ui-host-native/profiles/"
            "worth-ui-windows-dx12-v1.toml"
        )
    if requirement in MOUNTED_BASELINE_REQUIREMENTS:
        return (
            "workspaces/worth-ui/crates/worth-ui-certification/tests/"
            "application_contracts/host_platform/control_points.toml"
        )
    if requirement in {
        "P3-DELTA-SOURCE-01", "P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01",
        "P3-RECONSTRUCTION-01", "P3-STALE-DELTA-01",
    }:
        return (
            "workspaces/worth-ui/crates/worth-ui-certification/tests/"
            "application_contracts/host_platform/control_points.toml"
        )
    return None
