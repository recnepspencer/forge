from __future__ import annotations

TURN_TRANSITIONS: dict[str, tuple[str, ...]] = {
    "plan": ("implement",),
    "implement": ("implement", "review"),
    "review": ("repair", "test_review"),
    "repair": ("review",),
    "test_review": ("test_repair_plan", "code_quality_review"),
    "test_repair_plan": ("test_repair_implement",),
    "test_repair_implement": ("test_review", "code_quality_review"),
    "code_quality_review": (),
}


def next_turn_allowed(current_turn: str, requested_turn: str | None) -> bool:
    if requested_turn is None:
        return current_turn == "code_quality_review"
    return requested_turn in TURN_TRANSITIONS.get(current_turn, ())
