from __future__ import annotations


POSITIVE_CASES = {
    "P5-GLYPH-RASTER-01": (
        "exact-demand-identity",
        "fractional-origin",
        "variable-outline",
        "last-resort-outline",
        "cross-layout-raster-reuse",
        "qualified-alpha-color-batches",
    ),
    "P5-COLOR-EMOJI-01": (
        "colrv0-cpal",
        "colrv1-cpal",
        "cbdt-cblc",
        "sbix-png-dupe",
        "selector-lane",
        "exhaustive-rgi",
        "gradient-composite",
        "nonseparable-composite",
        "bitmap-composite",
    ),
    "P5-ATLAS-01": (
        "exact-signal-basis",
        "independent-model",
        "real-dx12-alpha-color",
        "bounded-capacity",
        "temporal-recovery",
        "retry-correlation",
        "retained-content-extent",
        "production-supersession",
        "terminal-census",
    ),
    "P5-ATLAS-PINNING-01": (
        "shared-layout-pins",
        "runtime-transaction-owner",
        "native-signal-settlement",
        "alpha-color-event-loop-progression",
        "last-owner-release",
        "preclose-pin-transition",
        "terminal-census",
    ),
}

HOSTILE_CASES = {
    "P5-GLYPH-RASTER-01": ("consumer-reshape", "ambient-system-font"),
    "P5-COLOR-EMOJI-01": (
        "foreground-tint",
        "cluster-split",
        "source-substitution",
        "malformed-graph",
        "unsupported-bitmap",
        "unbounded-current-color",
    ),
    "P5-ATLAS-01": (
        "callback-before-effects",
        "partial-upload-indeterminate",
        "replayed-completion",
        "capacity-before-raster",
        "cancellation-recovery",
        "equal-epoch-registration-order",
        "alpha-color-owner-merger",
    ),
    "P5-ATLAS-PINNING-01": ("shared-owner-preservation", "last-owner-release"),
}


def positive_cases(requirement: str) -> tuple[str, ...] | None:
    return POSITIVE_CASES.get(requirement)


def hostile_cases(requirement: str) -> tuple[str, ...] | None:
    return HOSTILE_CASES.get(requirement)
