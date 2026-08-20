from __future__ import annotations


P5_FEATURE_ROWS = (
    "P5-GLYPH-RASTER-01",
    "P5-COLOR-EMOJI-01",
    "P5-ATLAS-01",
    "P5-ATLAS-PINNING-01",
    "P5-TEXT-DPI-01",
    "P5-TEXT-SPAN-PAINT-01",
    "P5-TEXT-PIXELS-01",
    "P5-TEXT-RECONSTRUCTION-01",
    "P5-TEXT-COST-01",
    "P5-TEXT-ASYNC-PRESENTATION-01",
)

P5_REQUIREMENTS = ("P5-PREDECESSOR-01", *P5_FEATURE_ROWS, "P5-CLOSE-01")

P5_MUTATIONS = {
    "P5-PREDECESSOR-01": ("stale-predecessor", "stale-phase-four-source"),
    "P5-GLYPH-RASTER-01": ("raster-authority", "consumer-reshape-or-system-font"),
    "P5-COLOR-EMOJI-01": ("color-layer-drop", "emoji-tint-or-split"),
    "P5-ATLAS-01": ("atlas-lifecycle", "host-atlas-escape"),
    "P5-ATLAS-PINNING-01": ("pin-eviction", "live-layout-unpin"),
    "P5-TEXT-DPI-01": ("dpi-reuse", "stale-dpi-raster"),
    "P5-TEXT-SPAN-PAINT-01": (
        "paint-span",
        "single-color-or-visual-order-or-layout-regen",
    ),
    "P5-TEXT-PIXELS-01": ("pixel-identity", "transcript-pixel-mismatch"),
    "P5-TEXT-RECONSTRUCTION-01": ("derived-state-reuse", "stale-raster-reuse"),
    "P5-TEXT-COST-01": ("retained-scan", "complete-document-rescan"),
    "P5-TEXT-ASYNC-PRESENTATION-01": (
        "query-presentation-authority",
        "bypass-query-or-stale-presentation-completion",
    ),
    "P5-CLOSE-01": ("ledger", "open-requirement"),
}

P5_COUNTERS = {
    "P5-PREDECESSOR-01": ("requirements", 68),
    "P5-GLYPH-RASTER-01": ("raster-batches", 2),
    "P5-COLOR-EMOJI-01": ("rgi-sequences", 3953),
    "P5-ATLAS-01": ("physical-signal-runtimes", 1),
    "P5-ATLAS-PINNING-01": ("pinned-layouts", 3),
    "P5-TEXT-DPI-01": ("dpi-replacements", 1),
    "P5-TEXT-SPAN-PAINT-01": ("paint-spans", 2),
    "P5-TEXT-PIXELS-01": ("pixel-observations", 2),
    "P5-TEXT-RECONSTRUCTION-01": ("reconstructed-derived-states", 7),
    "P5-TEXT-COST-01": ("ui-locality-worlds", 32),
    "P5-TEXT-ASYNC-PRESENTATION-01": ("presentation-transitions", 10),
    "P5-CLOSE-01": ("requirements", 12),
}

P5_FAULT_BOUNDARIES = {
    requirement: (
        "before-effects"
        if requirement
        in {
            "P5-GLYPH-RASTER-01",
            "P5-COLOR-EMOJI-01",
            "P5-ATLAS-01",
            "P5-ATLAS-PINNING-01",
            "P5-TEXT-DPI-01",
            "P5-TEXT-SPAN-PAINT-01",
        }
        else (
            "after-effects-may-have-begun"
            if requirement == "P5-TEXT-ASYNC-PRESENTATION-01"
            else "not-applicable"
        )
    )
    for requirement in P5_REQUIREMENTS
}


def p5_construction_cost(requirement: str) -> str:
    if requirement == "P5-PREDECESSOR-01":
        return (
            "main-tests=44;hostile-controls=45;product-processes=6;"
            "compile-sessions=2;courtroom-worlds=12"
        )
    if requirement == "P5-ATLAS-PINNING-01":
        return (
            "main-tests=1;hostile-controls=1;product-processes=1;"
            "compile-sessions=0;courtroom-worlds=1"
        )
    if requirement == "P5-TEXT-ASYNC-PRESENTATION-01":
        return (
            "main-tests=1;hostile-controls=1;product-processes=1;"
            "compile-sessions=2;courtroom-worlds=1"
        )
    if requirement == "P5-TEXT-PIXELS-01":
        return (
            "main-tests=1;hostile-controls=1;product-processes=1;"
            "compile-sessions=0;courtroom-worlds=1"
        )
    if requirement == "P5-TEXT-RECONSTRUCTION-01":
        return (
            "main-tests=1;hostile-controls=1;product-processes=7;"
            "compile-sessions=0;courtroom-worlds=7"
        )
    if requirement == "P5-TEXT-COST-01":
        return (
            "main-tests=1;hostile-controls=1;product-processes=32;"
            "compile-sessions=0;courtroom-worlds=32"
        )
    return (
        "main-tests=1;hostile-controls=1;product-processes=0;"
        "compile-sessions=0;courtroom-worlds=0"
    )


def p5_execution_cost(requirement: str) -> str:
    if requirement == "P5-PREDECESSOR-01":
        return "executed-tests=91;presentations=56"
    if requirement == "P5-ATLAS-PINNING-01":
        return "executed-tests=2;presentations=4;atlas-transactions=4"
    if requirement in {"P5-TEXT-PIXELS-01", "P5-TEXT-ASYNC-PRESENTATION-01"}:
        return "executed-tests=2;presentations=3"
    if requirement == "P5-TEXT-RECONSTRUCTION-01":
        return "executed-tests=2;presentations=21"
    if requirement == "P5-TEXT-COST-01":
        return "executed-tests=2;presentations=64"
    return "executed-tests=2;presentations=0"
