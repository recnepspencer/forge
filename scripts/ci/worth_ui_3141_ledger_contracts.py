MUTATIONS = {
    "P1-AFFINITY-01": ("affinity", "stale-predecessor"),
    "P1-AUTHORITY-01": ("construction", "public-construction"),
    "P1-BACKEND-FEATURES-01": ("backend-feature", "vulkan-default"),
    "P1-BASELINE-01": ("baseline", "forged-known-empty"),
    "P1-CLOSE-01": ("ledger", "open-requirement"),
    "P1-CONSUMERS-01": ("validated-agreement", "agreement-validation-bypass"),
    "P1-DAMAGE-01": ("damage", "widened-damage"),
    "P1-HEADLESS-01": ("mechanics-substitution", "performed-external-effect"),
    "P1-HEADLESS-COST-01": ("carrier-inflation", "unchanged-carriage"),
    "P1-ORDER-01": ("paint-order", "identity-tie-break"),
    "P1-ORDER-SOURCE-01": ("identity-perturbation", "public-ordering"),
    "P1-PLATFORM-AUTHORITY-01": ("grant-forgery", "downstream-bind"),
    "P1-PREPARATION-LIFECYCLE-01": ("premature-runtime-effect", "host-during-prepare"),
    "P1-PRESENTATION-AUTHORITY-01": ("work-forgery", "external-work-issue"),
    "P1-PRODUCER-01": ("delta-carriage", "dropped-removal"),
    "P1-PRODUCER-COST-01": ("carrier-inflation", "unchanged-payload"),
    "P1-PROFILE-01": ("manifest-field", "qualified-capacity-drift"),
    "P1-PROTOCOL-01": ("protocol-revision", "mixed-revision"),
    "P1-TOPOLOGY-01": ("hidden-edge", "target-dependency-alias"),
    "P1-WORLDS-01": ("oracle-substitution", "damage-and-order-mutants"),
    "P2-APPLICATION-01": ("driver-substitution", "fake-client"),
    "P2-CLOSE-01": ("resource-leak", "held-readback"),
    "P2-EVENT-LOOP-01": ("thread-substitution", "off-thread-run"),
    "P2-GRAPHICS-01": ("backend-substitution", "vulkan-or-small-limit"),
    "P2-PIXELS-01": ("expected-pixel-substitution", "wrong-client-pixel"),
    "P2-PORTS-01": ("scripted-port-substitution", "indeterminate-as-before-effects"),
    "P2-PRESENT-01": ("geometry-color-substitution", "geometry-or-color-drift"),
    "P2-READINESS-01": ("wake-drop-duplicate", "duplicate-generation"),
    "P2-WINDOW-01": ("window-substitution", "dpi-basis-drift"),
    "P2-WORLD-01": ("world-substitution", "os-backend-client-or-close"),
}

COUNTERS = {
    "P1-AFFINITY-01": ("work", 3),
    "P1-AUTHORITY-01": ("preparation", 2),
    "P1-BACKEND-FEATURES-01": ("resolved-feature", 1),
    "P1-BASELINE-01": ("baseline", 1),
    "P1-CLOSE-01": ("requirements", 20),
    "P1-CONSUMERS-01": ("consumer", 2),
    "P1-DAMAGE-01": ("damage", 2),
    "P1-HEADLESS-01": ("headless", 1),
    "P1-HEADLESS-COST-01": ("carrier-cost", 0),
    "P1-ORDER-01": ("order", 2),
    "P1-ORDER-SOURCE-01": ("order-source", 2),
    "P1-PLATFORM-AUTHORITY-01": ("grant", 2),
    "P1-PREPARATION-LIFECYCLE-01": ("effect-surface", 0),
    "P1-PRESENTATION-AUTHORITY-01": ("authority", 2),
    "P1-PRODUCER-01": ("producer", 2),
    "P1-PRODUCER-COST-01": ("carrier-cost", 0),
    "P1-PROFILE-01": ("profile", 2),
    "P1-PROTOCOL-01": ("protocol", 4),
    "P1-TOPOLOGY-01": ("inventory", 25),
    "P1-WORLDS-01": ("world", 2048),
    "P2-APPLICATION-01": ("application", 1),
    "P2-CLOSE-01": ("resource-census", 0),
    "P2-EVENT-LOOP-01": ("event-loop", 1),
    "P2-GRAPHICS-01": ("graphics", 1),
    "P2-PIXELS-01": ("pixels", 3),
    "P2-PORTS-01": ("ports", 4),
    "P2-PRESENT-01": ("presentation", 1),
    "P2-READINESS-01": ("readiness", 1),
    "P2-WINDOW-01": ("window", 1),
    "P2-WORLD-01": ("world", 1),
}

EXPECTED_IGNORED = {
    requirement: (
        requirement in {"P1-CLOSE-01", "P1-HEADLESS-COST-01", "P1-WORLDS-01"}
        or requirement.startswith("P2-")
    )
    for requirement in COUNTERS
}

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


MOUNTED_BASELINE_REQUIREMENTS = {
    "P1-AFFINITY-01", "P1-BASELINE-01", "P1-CONSUMERS-01", "P1-DAMAGE-01",
    "P1-HEADLESS-01", "P1-HEADLESS-COST-01", "P1-ORDER-01",
    "P1-ORDER-SOURCE-01", "P1-PRESENTATION-AUTHORITY-01", "P1-PRODUCER-01",
    "P1-PRODUCER-COST-01", "P1-PROTOCOL-01", "P1-WORLDS-01",
}


def baseline_path(requirement: str) -> str | None:
    if requirement.startswith("P2-") or "PROFILE" in requirement or "BACKEND" in requirement:
        return (
            "workspaces/worth-ui/crates/worth-ui-host-native/profiles/"
            "worth-ui-windows-dx12-v1.toml"
        )
    if requirement in MOUNTED_BASELINE_REQUIREMENTS:
        return (
        "workspaces/worth-ui/crates/worth-ui-certification/tests/"
        "application_contracts/host_platform/control_points.toml"
        )
    return None


def construction_cost(requirement: str) -> str:
    compile_sessions = 2 if requirement in {
        "P1-AUTHORITY-01", "P1-ORDER-SOURCE-01", "P1-PLATFORM-AUTHORITY-01",
        "P1-PRESENTATION-AUTHORITY-01", "P1-PROTOCOL-01",
    } else 0
    p2 = requirement.startswith("P2-")
    shared_p2 = p2 and requirement != "P2-WORLD-01"
    control = p2 or requirement == "P1-CONSUMERS-01"
    world = requirement == "P2-WORLD-01" or requirement in {
        "P1-HEADLESS-COST-01", "P1-WORLDS-01"
    }
    if shared_p2:
        return (
            "main-tests=0;hostile-controls=1;product-processes=0;compile-sessions=0;"
            "courtroom-worlds=0;shared-native-worlds=1"
        )
    if requirement == "P1-HEADLESS-COST-01":
        return (
            "main-tests=0;hostile-controls=0;product-processes=0;compile-sessions=0;"
            "courtroom-worlds=0;shared-mounted-worlds=1"
        )
    return (
        f"main-tests=1;hostile-controls={int(control)};product-processes={int(p2)};"
        f"compile-sessions={compile_sessions};courtroom-worlds={int(world)}"
    )


def execution_cost(requirement: str) -> str:
    if requirement == "P1-HEADLESS-COST-01":
        return "executed-tests=0;presentations=0;shared-presentations=7"
    if requirement == "P1-WORLDS-01":
        return "executed-tests=1;presentations=7"
    if requirement == "P1-CONSUMERS-01":
        return "executed-tests=2;presentations=0"
    if requirement == "P2-WORLD-01":
        return "executed-tests=2;presentations=1"
    if requirement.startswith("P2-"):
        return "executed-tests=1;presentations=0;shared-presentations=1"
    return "executed-tests=1;presentations=0"


def platform_versions(requirement: str) -> str:
    if requirement.startswith("P2-"):
        return NATIVE_PLATFORM_VERSIONS
    if requirement == "P1-PROFILE-01":
        return PROFILE_PLATFORM_VERSIONS
    return BASIC_PLATFORM_VERSIONS

FAULT_BOUNDARIES = {
    requirement: "not-applicable"
    for requirement in COUNTERS
    if requirement.startswith("P1-")
}
FAULT_BOUNDARIES.update({
    "P2-APPLICATION-01": "before-effects",
    "P2-EVENT-LOOP-01": "before-effects",
    "P2-GRAPHICS-01": "before-effects",
    "P2-READINESS-01": "before-effects",
    "P2-WINDOW-01": "before-effects",
})
for _requirement in COUNTERS:
    if _requirement.startswith("P2-") and _requirement not in FAULT_BOUNDARIES:
        FAULT_BOUNDARIES[_requirement] = "after-effects-may-have-begun"
