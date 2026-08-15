pub(super) fn scenario_delta(requirement: &str) -> Option<&'static str> {
    Some(match requirement {
        "P5-PREDECESSOR-01" => "stale-phase-four-source",
        "P5-GLYPH-RASTER-01" => "consumer-reshape-or-system-font",
        "P5-COLOR-EMOJI-01" => "emoji-tint-or-split",
        "P5-ATLAS-01" => "host-atlas-escape",
        "P5-ATLAS-PINNING-01" => "live-layout-unpin",
        "P5-TEXT-DPI-01" => "stale-dpi-raster",
        "P5-TEXT-SPAN-PAINT-01" => "single-color-or-visual-order-or-layout-regen",
        "P5-TEXT-PIXELS-01" => "transcript-pixel-mismatch",
        "P5-TEXT-RECONSTRUCTION-01" => "stale-raster-reuse",
        "P5-TEXT-COST-01" => "complete-document-rescan",
        "P5-CLOSE-01" => "open-requirement",
        _ => return None,
    })
}

pub(super) fn construction_cost(requirement: &str) -> Option<&'static str> {
    if !requirement.starts_with("P5-") {
        return None;
    }
    Some(if requirement == "P5-PREDECESSOR-01" {
        "main-tests=47;hostile-controls=48;product-processes=3;compile-sessions=2;courtroom-worlds=6"
    } else {
        "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0"
    })
}

pub(super) fn execution_cost(requirement: &str) -> Option<&'static str> {
    if !requirement.starts_with("P5-") {
        return None;
    }
    Some(if requirement == "P5-PREDECESSOR-01" {
        "executed-tests=97;presentations=28"
    } else {
        "executed-tests=2;presentations=0"
    })
}
