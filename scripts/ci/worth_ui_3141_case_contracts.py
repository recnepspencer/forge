from __future__ import annotations

import worth_ui_3141_p5_case_contracts as phase_five
import worth_ui_3141_phase4_case_contracts as phase_four


def positive_cases(requirement: str) -> tuple[str, ...] | None:
    return phase_four.positive_cases(requirement) or phase_five.positive_cases(requirement)


def hostile_cases(requirement: str) -> tuple[str, ...] | None:
    return phase_four.hostile_cases(requirement) or phase_five.hostile_cases(requirement)
