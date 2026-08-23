P6_REQUIREMENTS = (
    "P6-PREDECESSOR-01",
    "P6-INPUT-AFFINITY-01",
    "P6-IME-01",
    "P6-POINTER-TIME-01",
    "P6-PROFILE-ORDER-01",
    "P6-READINESS-01",
    "P6-SETTLEMENT-01",
    "P6-PROTOCOL-WORLD-01",
    "P6-WINDOWS-WORLD-01",
    "P6-CLOSE-01",
)

P6_MUTATIONS = {
    "P6-PREDECESSOR-01": ("stale-predecessor", "stale-phase-five-source"),
    "P6-INPUT-AFFINITY-01": ("input-affinity", "current-coordinate-retargeting"),
    "P6-IME-01": ("ime-semantic-phase", "preedit-as-text-input"),
    "P6-POINTER-TIME-01": ("pointer-time", "post-delivery-cursor-proxy"),
    "P6-PROFILE-ORDER-01": ("profile-order", "synthetic-event-time"),
    "P6-READINESS-01": ("readiness-delivery", "silent-level-wake"),
    "P6-SETTLEMENT-01": (
        "typed-settlement",
        "generic-error-for-typed-settlement",
    ),
    "P6-PROTOCOL-WORLD-01": ("oracle", "oracle-substitution"),
    "P6-WINDOWS-WORLD-01": (
        "pointer-source",
        "get-cursor-pos-production-proxy",
    ),
    "P6-CLOSE-01": ("ledger", "open-requirement"),
}

P6_COUNTERS = {
    "P6-PREDECESSOR-01": ("requirements", 80),
    "P6-INPUT-AFFINITY-01": ("observations", 2),
    "P6-IME-01": ("ime-phases", 3),
    "P6-POINTER-TIME-01": ("pointer-witnesses", 1),
    "P6-PROFILE-ORDER-01": ("profile-transitions", 1),
    "P6-READINESS-01": ("readiness-generations", 2),
    "P6-SETTLEMENT-01": ("settlement-outcomes", 1),
    "P6-PROTOCOL-WORLD-01": ("protocol-schedules", 177),
    "P6-WINDOWS-WORLD-01": ("pointer-witnesses", 1),
    "P6-CLOSE-01": ("requirements", 10),
}

P6_FAULT_BOUNDARIES = {
    "P6-PREDECESSOR-01": "predecessor-handoff-source-binding",
    "P6-INPUT-AFFINITY-01": "input-admission-presentation-affinity",
    "P6-IME-01": "ime-phase-classification",
    "P6-POINTER-TIME-01": "pointer-event-time-witness",
    "P6-PROFILE-ORDER-01": "profile-transition-admission",
    "P6-READINESS-01": "readiness-commit-signal-consume",
    "P6-SETTLEMENT-01": "typed-settlement-outcome-mapping",
    "P6-PROTOCOL-WORLD-01": "protocol-production-oracle-comparison",
    "P6-WINDOWS-WORLD-01": "windows-message-position-witness",
    "P6-CLOSE-01": "phase-six-closure-source-prefix",
}


def p6_construction_cost(requirement: str) -> str:
    if requirement == "P6-PREDECESSOR-01":
        return (
            "main-tests=55;hostile-controls=57;product-processes=54;"
            "compile-sessions=2;courtroom-worlds=66"
        )
    if requirement == "P6-WINDOWS-WORLD-01":
        return (
            "main-tests=1;hostile-controls=1;product-processes=1;"
            "compile-sessions=0;courtroom-worlds=1"
        )
    return (
        "main-tests=1;hostile-controls=1;product-processes=0;"
        "compile-sessions=0;courtroom-worlds=0"
    )


def p6_execution_cost(requirement: str) -> str:
    if requirement == "P6-PREDECESSOR-01":
        return "executed-tests=114;presentations=207"
    if requirement == "P6-WINDOWS-WORLD-01":
        return "executed-tests=2;presentations=1"
    return "executed-tests=2;presentations=0"
