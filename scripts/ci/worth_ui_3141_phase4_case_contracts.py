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
}


def positive_cases(requirement: str) -> tuple[str, ...] | None:
    return POSITIVE_CASES.get(requirement)


def hostile_cases(requirement: str) -> tuple[str, ...] | None:
    return HOSTILE_CASES.get(requirement)
