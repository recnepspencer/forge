from __future__ import annotations


POSITIVE_CASES = {
    "P6-PROTOCOL-WORLD-01": (
        "every-input-family-before-first-presentation",
        "successor-in-flight-affinity",
        "profile-transition-stale-input",
        "ime-preedit-commit-cancel",
        "pointer-event-time-or-unavailable",
        "readiness-and-close-drain",
        "mixed-ordering-capacity-and-wake",
        "retention-ordering-and-bounded-capacity",
        "post-close-input-is-noop",
        "closing-pending-and-drain-request",
        "stale-profile-after-completion",
        "empty-close-before-first-presentation",
        "resize-dpi-zero-sized-profile-around-input",
        "over-capacity-text-stops-before-retention",
        "unprovable-ime-range-stops-before-retention",
    ),
}

HOSTILE_CASES = {
    "P6-PREDECESSOR-01": ("stale-phase-five-source",),
    "P6-INPUT-AFFINITY-01": ("current-coordinate-retargeting",),
    "P6-IME-01": ("preedit-as-text-input",),
    "P6-POINTER-TIME-01": ("post-delivery-cursor-proxy",),
    "P6-PROFILE-ORDER-01": ("synthetic-event-time",),
    "P6-READINESS-01": ("silent-level-wake",),
    "P6-SETTLEMENT-01": ("generic-error-for-typed-settlement",),
    "P6-PROTOCOL-WORLD-01": ("oracle-substitution",),
    "P6-WINDOWS-WORLD-01": ("get-cursor-pos-production-proxy",),
    "P6-CLOSE-01": ("open-requirement",),
}


def positive_cases(requirement: str) -> tuple[str, ...] | None:
    return POSITIVE_CASES.get(requirement)


def hostile_cases(requirement: str) -> tuple[str, ...] | None:
    return HOSTILE_CASES.get(requirement)
