use super::super::{execution_contract, requirement_contract::RequirementContract, Row};

pub(crate) fn validate_open_claim(row: &Row, contract: &RequirementContract) -> Result<(), String> {
    if !matches!(row["phase"].as_str(), "3" | "4" | "5") {
        return Ok(());
    }
    let scenario = scenario_delta(&row["requirement"])
        .ok_or_else(|| "future requirement omits its mutation case".to_owned())?;
    let mutation = format!("family={};case={scenario}", contract.mutation_family);
    let expected_fault = execution_contract::fault_boundary(&row["requirement"])
        .ok_or_else(|| "future requirement omits its exact fault boundary".to_owned())?;
    if row["scenario_delta"] != scenario
        || row["mutation_control"] != mutation
        || row["fault_injection_boundary"] != expected_fault
    {
        return Err(format!("future claim drifted: {}", row["requirement"]));
    }
    let open_counter = format!("{}=open", contract.counter_family);
    if row["result"] == "OPEN" && row["structural_counters"] != open_counter {
        validate_prepared_open_claim(row, contract)?;
    }
    Ok(())
}

fn validate_prepared_open_claim(row: &Row, contract: &RequirementContract) -> Result<(), String> {
    let expected = execution_contract::counter_amount(&row["requirement"])
        .ok_or_else(|| "prepared future claim lacks an execution counter".to_owned())?;
    if row["structural_counters"] != format!("{}={expected}", contract.counter_family)
        || row["production_entry"] == "not-bound"
        || row["independent_oracle"] == "not-bound"
        || row["exact_command"] == "not-bound"
        || row["source_identity"] == "not-bound"
    {
        return Err("prepared future requirement is not exactly bound".to_owned());
    }
    Ok(())
}

fn scenario_delta(requirement: &str) -> Option<&'static str> {
    Some(match requirement {
        "P3-BASELINE-REPLAY-01" => "opaque-baseline-clear",
        "P3-CLIPPED-DELTA-01" => "zero-paint-as-indeterminate",
        "P3-CLOSE-01" => "open-requirement",
        "P3-DAMAGE-INDEX-01" => "full-retained-scan",
        "P3-DAMAGE-REPLAY-01" => "omitted-vacated-replay",
        "P3-DELTA-SOURCE-01" => "successor-rediscovery",
        "P3-DRAW-LIST-01" => "complete-map-clone",
        "P3-HEADLESS-COST-01" => "complete-transcript-clone",
        "P3-HP02-WORLD-01" => "synthetic-successor",
        "P3-PHYSICAL-AMPLIFICATION-01" => "hidden-full-surface-copy",
        "P3-PREDECESSOR-01" => "stale-phase-two-source",
        "P3-PRODUCER-SLOPE-01" => "complete-successor-scan",
        "P3-RECONSTRUCTION-01" => "stale-derived-state",
        "P3-STALE-DELTA-01" => "stale-affinity-acceptance",
        "P3-TOTAL-ORDER-01" => "identity-ordering",
        "P3-TRANSACTION-01" => "commit-before-handoff",
        "P3-UNCHANGED-01" => "fresh-unchanged-epoch",
        "P4-BIDI-01" => "logical-order-rendering",
        "P4-CAPACITY-01" => "shape-before-capacity-denial",
        "P4-EMOJI-SEQUENCE-01" => "variation-or-zwj-decomposition",
        "P4-FALLBACK-01" => "emoji-or-indic-split",
        "P4-FONT-COLLECTION-01" => {
            "ambient-or-single-family-or-stale-generation-or-registration-order-substitution"
        }
        "P4-LINE-LAYOUT-01" => "mid-cluster-wrap",
        "P4-MEASUREMENT-IDENTITY-01" => "independent-measurement-pass",
        "P4-ORIGINAL-RANGE-01" => "normalized-offset-substitution",
        "P4-SHAPING-01" => "one-run-latin",
        "P4-TEXT-COST-01" => "complete-document-rescan",
        "P4-TEXT-PROFILE-01" => "font-or-unicode-digest-drift",
        "P4-TEXT-RECONSTRUCTION-01" => "stale-layout-reuse",
        "P4-UNICODE-SEGMENTATION-01" => "zwj-or-flag-split",
        "P4-UNCHANGED-01" => "unchanged-paragraph-rescan",
        "P4-PREDECESSOR-01" => "stale-phase-three-source",
        "P4-BIDI-INTERACTION-01" => "swapped-bidi-caret-affinity",
        "P4-TEXT-CONTENT-LOCALITY-01" => "content-only-global-rescan",
        "P4-TEXT-WIDTH-LOCALITY-01" => "paragraph-width-global-rescan",
        "P4-ACCESSIBILITY-GEOMETRY-01" => "accessibility-reshape",
        "P4-COLOR-FONT-ADMISSION-01" => "unsupported-svg-or-layer-drop",
        "P4-CLOSE-01" => "open-requirement",
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
