from __future__ import annotations


POSITIVE_CASES = {
    "P4-FONT-COLLECTION-01": (
        "owned-ttf",
        "owned-otf",
        "owned-ttc-multi-index",
        "owned-otc-multi-index",
        "ordered-multi-family-stack",
        "static-regular-bold-italic-oblique",
        "pack-scoped-family-name-collision",
        "variable-weight",
        "variable-width",
        "variable-slant",
        "explicit-opentype-feature",
        "whole-cluster-default-emoji-last-resort-fallback",
        "whole-cluster-khmer-shaping-syllable",
        "independent-per-span-stack",
        "generation-replace-remove-pins-predecessor-bytes",
        "exact-generation-reconstruction",
    ),
    "P5-ATLAS-01": (
        "exact-signal-basis",
        "independent-model",
        "real-dx12-alpha-color",
        "bounded-capacity",
        "temporal-recovery",
        "terminal-census",
    ),
    "P5-ATLAS-PINNING-01": (
        "shared-layout-pins",
        "runtime-transaction-owner",
        "native-signal-settlement",
        "pressure-saturation",
        "deterministic-unpinned-replacement",
        "last-owner-release",
        "atlas-capacity-dependency",
        "terminal-census",
    ),
}


HOSTILE_CASES = {
    "P4-FONT-COLLECTION-01": (
        "unsupported-web-container",
        "unsupported-aat-shaping-table",
        "unsupported-explicit-feature",
        "registration-order-substitution",
        "malformed-localized-name",
        "malformed-ambiguous-unsupported-over-capacity-pack",
        "generation-exhaustion-alias",
        "same-number-different-lineage",
        "face-definition-order-substitution",
        "worse-face-skips-later-family",
        "variable-axis-range-substitution",
        "missing-unicode-coverage",
        "pack-family-boundary-alias",
    ),
    "P5-ATLAS-01": (
        "callback-before-effects",
        "partial-upload-indeterminate",
        "replayed-completion",
        "capacity-before-raster",
        "cancellation-recovery",
        "equal-epoch-registration-order",
    ),
    "P5-ATLAS-PINNING-01": (
        "shared-owner-preservation",
        "last-owner-release",
    ),
}


def positive_cases(requirement: str) -> tuple[str, ...] | None:
    return POSITIVE_CASES.get(requirement)


def hostile_cases(requirement: str) -> tuple[str, ...] | None:
    return HOSTILE_CASES.get(requirement)
