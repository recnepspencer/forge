from worth_ui_3141_p5_contracts import p5_construction_cost, p5_execution_cost
from worth_ui_3141_p6_contracts import p6_construction_cost, p6_execution_cost
from worth_ui_3141_platform_contracts import (
    BASIC_PLATFORM_VERSIONS,
    NATIVE_PHASE6_PLATFORM_VERSIONS,
    NATIVE_PLATFORM_VERSIONS,
    P3_NATIVE_REQUIREMENTS,
    PROFILE_PLATFORM_VERSIONS,
    TEXT_PLATFORM_VERSIONS,
)


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
            "P3-PHYSICAL-AMPLIFICATION-01", "P3-TRANSACTION-01", "P3-UNCHANGED-01",
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
    if requirement.startswith("P6-"):
        return p6_construction_cost(requirement)
    if requirement.startswith("P5-"):
        return p5_construction_cost(requirement)
    if requirement.startswith("P4-"):
        if requirement == "P4-PREDECESSOR-01":
            return (
                "main-tests=26;hostile-controls=28;product-processes=3;compile-sessions=2;"
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
        "P1-HEADLESS-COST-01", "P1-WORLDS-01",
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
            "P3-PHYSICAL-AMPLIFICATION-01", "P3-TRANSACTION-01", "P3-UNCHANGED-01",
        }:
            return "executed-tests=1;presentations=0;shared-presentations=7"
        if requirement in {"P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01"}:
            return "executed-tests=1;presentations=0;shared-presentations=5"
        if requirement == "P3-HP02-WORLD-01":
            return "executed-tests=2;presentations=7;shared-presentations=5"
        presentations = (
            7 if requirement in {
                "P3-BASELINE-REPLAY-01", "P3-DAMAGE-REPLAY-01", "P3-DRAW-LIST-01",
                "P3-PHYSICAL-AMPLIFICATION-01", "P3-TRANSACTION-01", "P3-UNCHANGED-01",
            } else 5 if requirement in {
                "P3-DELTA-SOURCE-01", "P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01",
            } else 0
        )
        return f"executed-tests=2;presentations={presentations}"
    if requirement.startswith("P6-"):
        return p6_execution_cost(requirement)
    if requirement.startswith("P5-"):
        return p5_execution_cost(requirement)
    if requirement == "P4-PREDECESSOR-01":
        return "executed-tests=56;presentations=28"
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
    if requirement.startswith("P6-"):
        return NATIVE_PHASE6_PLATFORM_VERSIONS
    if requirement.startswith("P2-") or requirement in P3_NATIVE_REQUIREMENTS:
        return NATIVE_PLATFORM_VERSIONS
    if requirement == "P1-PROFILE-01":
        return PROFILE_PLATFORM_VERSIONS
    return BASIC_PLATFORM_VERSIONS
