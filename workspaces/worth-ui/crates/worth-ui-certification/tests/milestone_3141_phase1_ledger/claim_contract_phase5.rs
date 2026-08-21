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
        "P5-TEXT-ASYNC-PRESENTATION-01" => "bypass-query-or-stale-presentation-completion",
        "P5-CLOSE-01" => "open-requirement",
        _ => return None,
    })
}

pub(super) fn construction_cost(requirement: &str) -> Option<&'static str> {
    if !requirement.starts_with("P5-") {
        return None;
    }
    Some(match requirement {
        "P5-PREDECESSOR-01" => {
            "main-tests=46;hostile-controls=48;product-processes=6;compile-sessions=2;courtroom-worlds=12"
        }
        "P5-ATLAS-PINNING-01" => {
            "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;courtroom-worlds=1"
        }
        "P5-TEXT-PIXELS-01" => {
            "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;courtroom-worlds=1"
        }
        "P5-TEXT-ASYNC-PRESENTATION-01" => {
            "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=2;courtroom-worlds=1"
        }
        "P5-TEXT-RECONSTRUCTION-01" => {
            "main-tests=1;hostile-controls=1;product-processes=7;compile-sessions=0;courtroom-worlds=7"
        }
        "P5-TEXT-COST-01" => {
            "main-tests=1;hostile-controls=1;product-processes=32;compile-sessions=0;courtroom-worlds=32"
        }
        _ => {
            "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0"
        }
    })
}

pub(super) fn execution_cost(requirement: &str) -> Option<&'static str> {
    if !requirement.starts_with("P5-") {
        return None;
    }
    Some(match requirement {
        "P5-PREDECESSOR-01" => "executed-tests=96;presentations=56",
        "P5-ATLAS-PINNING-01" => "executed-tests=2;presentations=4;atlas-transactions=4",
        "P5-TEXT-PIXELS-01" | "P5-TEXT-ASYNC-PRESENTATION-01" => "executed-tests=2;presentations=3",
        "P5-TEXT-RECONSTRUCTION-01" => "executed-tests=2;presentations=21",
        "P5-TEXT-COST-01" => "executed-tests=2;presentations=64",
        _ => "executed-tests=2;presentations=0",
    })
}
