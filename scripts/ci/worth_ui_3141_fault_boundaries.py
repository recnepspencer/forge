from __future__ import annotations

from worth_ui_3141_p5_contracts import P5_FAULT_BOUNDARIES


def fault_boundaries(counters: dict[str, object]) -> dict[str, str]:
    boundaries = {
        requirement: "not-applicable"
        for requirement in counters
        if requirement.startswith("P1-")
    }
    boundaries.update(
        {
            "P2-APPLICATION-01": "before-effects",
            "P2-EVENT-LOOP-01": "before-effects",
            "P2-GRAPHICS-01": "before-effects",
            "P2-READINESS-01": "before-effects",
            "P2-WINDOW-01": "before-effects",
        }
    )
    boundaries["P3-PREDECESSOR-01"] = "not-applicable"
    after_effects = {
        "P3-BASELINE-REPLAY-01",
        "P3-DAMAGE-REPLAY-01",
        "P3-HP02-WORLD-01",
        "P3-PHYSICAL-AMPLIFICATION-01",
        "P3-TRANSACTION-01",
    }
    before_effects = {
        "P4-TEXT-PROFILE-01",
        "P4-FONT-COLLECTION-01",
        "P4-COLOR-FONT-ADMISSION-01",
        "P4-CAPACITY-01",
    }
    for requirement in counters:
        if requirement.startswith("P3-") and requirement not in boundaries:
            boundaries[requirement] = (
                "after-effects-may-have-begun"
                if requirement in after_effects
                else "not-applicable"
            )
        elif requirement.startswith("P2-") and requirement not in boundaries:
            boundaries[requirement] = "after-effects-may-have-begun"
        elif requirement.startswith("P4-"):
            boundaries[requirement] = (
                "before-effects" if requirement in before_effects else "not-applicable"
            )
    boundaries.update(P5_FAULT_BOUNDARIES)
    return boundaries
