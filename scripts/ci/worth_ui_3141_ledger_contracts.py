from worth_ui_3141_fault_boundaries import fault_boundaries
from worth_ui_3141_p5_contracts import (
    P5_COUNTERS,
    P5_MUTATIONS,
    p5_construction_cost,
    p5_execution_cost,
)

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
    "P3-PREDECESSOR-01": ("stale-predecessor", "stale-phase-two-source"),
    "P3-BASELINE-REPLAY-01": ("baseline-substitution", "opaque-baseline-clear"),
    "P3-CLIPPED-DELTA-01": ("effect-posture", "zero-paint-as-indeterminate"),
    "P3-CLOSE-01": ("ledger", "open-requirement"),
    "P3-DAMAGE-INDEX-01": ("retained-scan", "full-retained-scan"),
    "P3-DAMAGE-REPLAY-01": ("vacated-replay", "omitted-vacated-replay"),
    "P3-DELTA-SOURCE-01": ("delta-rediscovery", "successor-rediscovery"),
    "P3-DRAW-LIST-01": ("retained-clone", "complete-map-clone"),
    "P3-HEADLESS-COST-01": ("retained-clone", "complete-transcript-clone"),
    "P3-HP02-WORLD-01": ("world-substitution", "synthetic-successor"),
    "P3-PHYSICAL-AMPLIFICATION-01": ("local-present-relabel", "hidden-full-surface-copy"),
    "P3-PRODUCER-SLOPE-01": ("retained-scan", "complete-successor-scan"),
    "P3-RECONSTRUCTION-01": ("derived-state-reuse", "stale-derived-state"),
    "P3-STALE-DELTA-01": ("stale-delta", "stale-affinity-acceptance"),
    "P3-TOTAL-ORDER-01": ("identity-tie-break", "identity-ordering"),
    "P3-TRANSACTION-01": ("premature-commit", "commit-before-handoff"),
    "P3-UNCHANGED-01": ("epoch-mint", "fresh-unchanged-epoch"),
    "P4-FONT-COLLECTION-01": (
        "font-resolution-authority",
        "ambient-or-single-family-or-stale-generation-or-registration-order-substitution",
    ),
    "P4-PREDECESSOR-01": ("stale-predecessor", "stale-phase-three-source"),
    "P4-TEXT-PROFILE-01": ("profile-drift", "font-or-unicode-digest-drift"),
    "P4-COLOR-FONT-ADMISSION-01": ("unsupported-color-table", "unsupported-svg-or-layer-drop"),
    "P4-UNICODE-SEGMENTATION-01": ("emoji-sequence-split", "zwj-or-flag-split"),
    "P4-EMOJI-SEQUENCE-01": ("emoji-sequence-decomposition", "variation-or-zwj-decomposition"),
    "P4-BIDI-01": ("visual-source-order", "logical-order-rendering"),
    "P4-FALLBACK-01": ("cluster-split", "emoji-or-indic-split"),
    "P4-SHAPING-01": ("glyph-substitution", "one-run-latin"),
    "P4-LINE-LAYOUT-01": ("cluster-break", "mid-cluster-wrap"),
    "P4-CAPACITY-01": ("post-admission-overflow", "shape-before-capacity-denial"),
    "P4-MEASUREMENT-IDENTITY-01": ("duplicate-shaper", "independent-measurement-pass"),
    "P4-ORIGINAL-RANGE-01": ("range-normalization", "normalized-offset-substitution"),
    "P4-BIDI-INTERACTION-01": ("caret-affinity-swap", "swapped-bidi-caret-affinity"),
    "P4-ACCESSIBILITY-GEOMETRY-01": ("accessibility-reshape", "accessibility-reshape"),
    "P4-TEXT-CONTENT-LOCALITY-01": ("content-global-rescan", "content-only-global-rescan"),
    "P4-TEXT-WIDTH-LOCALITY-01": ("paragraph-width-global-rescan", "paragraph-width-global-rescan"),
    "P4-TEXT-RECONSTRUCTION-01": ("derived-state-reuse", "stale-layout-reuse"),
    "P4-UNCHANGED-01": ("unchanged-analysis", "unchanged-paragraph-rescan"),
    "P4-TEXT-COST-01": ("paragraph-rescan", "complete-document-rescan"),
    "P4-CLOSE-01": ("ledger", "open-requirement"),
}
MUTATIONS.update(P5_MUTATIONS)

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
    "P1-TOPOLOGY-01": ("inventory", 27),
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
    "P3-PREDECESSOR-01": ("requirements", 30),
    "P3-BASELINE-REPLAY-01": ("baseline-clears", 1),
    "P3-CLIPPED-DELTA-01": ("effect-free-successors", 1),
    "P3-CLOSE-01": ("requirements", 17),
    "P3-DAMAGE-INDEX-01": ("damage-probes", 2048),
    "P3-DAMAGE-REPLAY-01": ("replayed-commands", 2048),
    "P3-DELTA-SOURCE-01": ("source-rows", 1),
    "P3-DRAW-LIST-01": ("draw-list-mutations", 2048),
    "P3-HEADLESS-COST-01": ("retained-scans", 0),
    "P3-HP02-WORLD-01": ("worlds", 2),
    "P3-PHYSICAL-AMPLIFICATION-01": ("amplification-boundary", 1),
    "P3-PRODUCER-SLOPE-01": ("retained-scans", 0),
    "P3-RECONSTRUCTION-01": ("reconstructed-commands", 2),
    "P3-STALE-DELTA-01": ("stale-denials", 2),
    "P3-TOTAL-ORDER-01": ("order-mutations", 2),
    "P3-TRANSACTION-01": ("transactions", 1),
    "P3-UNCHANGED-01": ("unchanged-work", 0),
    "P4-FONT-COLLECTION-01": ("font-resolution-cases", 16),
    "P4-PREDECESSOR-01": ("requirements", 47),
    "P4-TEXT-PROFILE-01": ("qualified-assets", 34),
    "P4-COLOR-FONT-ADMISSION-01": ("color-formats", 4),
    "P4-UNICODE-SEGMENTATION-01": ("conformance-cases", 22048),
    "P4-EMOJI-SEQUENCE-01": ("rgi-sequences", 3953),
    "P4-BIDI-01": ("bidi-runs", 582553),
    "P4-FALLBACK-01": ("fallback-probes", 3953),
    "P4-SHAPING-01": ("shaped-glyphs", 15),
    "P4-LINE-LAYOUT-01": ("lines", 3),
    "P4-CAPACITY-01": ("denied-before-analysis", 3),
    "P4-MEASUREMENT-IDENTITY-01": ("layout-identities", 1),
    "P4-ORIGINAL-RANGE-01": ("original-ranges", 8),
    "P4-BIDI-INTERACTION-01": ("interaction-records", 29),
    "P4-ACCESSIBILITY-GEOMETRY-01": ("layout-identities", 1),
    "P4-TEXT-CONTENT-LOCALITY-01": ("analyzed-bytes", 13),
    "P4-TEXT-WIDTH-LOCALITY-01": ("relayout-paragraphs", 1),
    "P4-TEXT-RECONSTRUCTION-01": ("reconstructed-layouts", 1),
    "P4-UNCHANGED-01": ("unchanged-analysis", 0),
    "P4-TEXT-COST-01": ("retained-scans", 0),
    "P4-CLOSE-01": ("requirements", 21),
}
COUNTERS.update(P5_COUNTERS)

EXPECTED_IGNORED = {
    requirement: (
        requirement in {"P1-CLOSE-01", "P1-HEADLESS-COST-01", "P1-WORLDS-01"}
        or requirement.startswith(("P2-", "P3-", "P4-"))
    )
    for requirement in COUNTERS
}
for _requirement in COUNTERS:
    if _requirement.startswith("P3-"):
        EXPECTED_IGNORED[_requirement] = _requirement in {
            "P3-BASELINE-REPLAY-01", "P3-CLOSE-01", "P3-DAMAGE-REPLAY-01",
            "P3-DRAW-LIST-01",
            "P3-HP02-WORLD-01", "P3-PHYSICAL-AMPLIFICATION-01",
            "P3-PREDECESSOR-01", "P3-TRANSACTION-01",
            "P3-UNCHANGED-01", "P3-DELTA-SOURCE-01",
            "P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01",
        }
for _requirement in COUNTERS:
    if _requirement.startswith("P4-"):
        EXPECTED_IGNORED[_requirement] = _requirement in {
            "P4-PREDECESSOR-01", "P4-TEXT-PROFILE-01", "P4-FONT-COLLECTION-01",
            "P4-COLOR-FONT-ADMISSION-01", "P4-UNICODE-SEGMENTATION-01",
            "P4-EMOJI-SEQUENCE-01", "P4-BIDI-01", "P4-FALLBACK-01",
            "P4-CLOSE-01",
        }
    if _requirement.startswith("P5-"):
        EXPECTED_IGNORED[_requirement] = _requirement in {
            "P5-PREDECESSOR-01", "P5-ATLAS-01", "P5-ATLAS-PINNING-01",
            "P5-TEXT-PIXELS-01", "P5-TEXT-RECONSTRUCTION-01",
            "P5-TEXT-COST-01", "P5-TEXT-ASYNC-PRESENTATION-01",
            "P5-CLOSE-01",
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
    "P3-CLIPPED-DELTA-01",
    "P3-DRAW-LIST-01", "P3-HP02-WORLD-01", "P3-PHYSICAL-AMPLIFICATION-01",
    "P3-TOTAL-ORDER-01", "P3-TRANSACTION-01",
    "P3-UNCHANGED-01",
}


def baseline_path(requirement: str) -> str | None:
    if (requirement.startswith("P2-") or requirement in P3_NATIVE_REQUIREMENTS
            or "PROFILE" in requirement or "BACKEND" in requirement):
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


def construction_cost(requirement: str) -> str:
    if requirement == "P3-PREDECESSOR-01":
        return (
            "main-tests=21;hostile-controls=12;product-processes=1;compile-sessions=2;"
            "courtroom-worlds=2"
        )
    if requirement.startswith("P3-"):
        if requirement == "P3-HP02-WORLD-01":
            return (
                "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;"
                "courtroom-worlds=1;shared-mounted-worlds=1"
            )
        if requirement in {
            "P3-BASELINE-REPLAY-01", "P3-DAMAGE-REPLAY-01", "P3-DRAW-LIST-01",
            "P3-PHYSICAL-AMPLIFICATION-01",
            "P3-TRANSACTION-01", "P3-UNCHANGED-01",
        }:
            return (
                "main-tests=0;hostile-controls=1;product-processes=0;compile-sessions=0;"
                "courtroom-worlds=0;shared-native-worlds=1"
            )
        if requirement == "P3-CLIPPED-DELTA-01":
            return (
                "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;"
                "courtroom-worlds=0"
            )
        if requirement in {"P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01"}:
            return (
                "main-tests=0;hostile-controls=1;product-processes=0;compile-sessions=0;"
                "courtroom-worlds=0;shared-mounted-worlds=1"
            )
        native = requirement in {
            "P3-BASELINE-REPLAY-01", "P3-DAMAGE-REPLAY-01", "P3-DRAW-LIST-01",
            "P3-HP02-WORLD-01", "P3-PHYSICAL-AMPLIFICATION-01", "P3-TRANSACTION-01",
            "P3-UNCHANGED-01",
        }
        mixed = requirement in {
            "P3-DELTA-SOURCE-01", "P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01",
        }
        return (
            f"main-tests=1;hostile-controls=1;product-processes={int(native)};"
            f"compile-sessions=0;courtroom-worlds={int(native or mixed)}"
        )
    if requirement.startswith("P5-"):
        return p5_construction_cost(requirement)
    if requirement.startswith("P4-"):
        if requirement == "P4-PREDECESSOR-01":
            return (
                "main-tests=26;hostile-controls=27;product-processes=3;compile-sessions=2;"
                "courtroom-worlds=6"
            )
        compile_sessions = int(requirement == "P4-FONT-COLLECTION-01")
        return (
            "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;"
            "courtroom-worlds=0"
        ).replace("compile-sessions=0", f"compile-sessions={compile_sessions}")
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
    if requirement == "P3-PREDECESSOR-01":
        return "executed-tests=35;presentations=8"
    if requirement.startswith("P3-"):
        if requirement == "P3-CLIPPED-DELTA-01":
            return "executed-tests=2;presentations=0"
        if requirement in {
            "P3-BASELINE-REPLAY-01", "P3-DAMAGE-REPLAY-01", "P3-DRAW-LIST-01",
            "P3-PHYSICAL-AMPLIFICATION-01",
            "P3-TRANSACTION-01", "P3-UNCHANGED-01",
        }:
            return "executed-tests=1;presentations=0;shared-presentations=7"
        if requirement in {"P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01"}:
            return "executed-tests=1;presentations=0;shared-presentations=5"
        if requirement == "P3-HP02-WORLD-01":
            return "executed-tests=2;presentations=7;shared-presentations=5"
        presentations = (
            7 if requirement in {
                "P3-BASELINE-REPLAY-01", "P3-DAMAGE-REPLAY-01", "P3-DRAW-LIST-01",
                "P3-PHYSICAL-AMPLIFICATION-01",
                "P3-TRANSACTION-01", "P3-UNCHANGED-01",
            } else 5 if requirement in {
                "P3-DELTA-SOURCE-01", "P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01",
            } else 0
        )
        return f"executed-tests=2;presentations={presentations}"
    if requirement.startswith("P5-"):
        return p5_execution_cost(requirement)
    if requirement == "P4-PREDECESSOR-01":
        return "executed-tests=55;presentations=28"
    if requirement.startswith("P4-"):
        return "executed-tests=2;presentations=0"
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
    if requirement.startswith(("P4-", "P5-")):
        return TEXT_PLATFORM_VERSIONS
    if requirement.startswith("P2-") or requirement in P3_NATIVE_REQUIREMENTS:
        return NATIVE_PLATFORM_VERSIONS
    if requirement == "P1-PROFILE-01":
        return PROFILE_PLATFORM_VERSIONS
    return BASIC_PLATFORM_VERSIONS

FAULT_BOUNDARIES = fault_boundaries(COUNTERS)
