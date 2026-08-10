use super::{requirement_contract::RequirementContract, Row};

pub(super) fn for_requirement(requirement: &str) -> Option<&'static RequirementContract> {
    CONTRACTS
        .iter()
        .find(|contract| contract.requirement == requirement)
}

macro_rules! contract {
    ($requirement:literal, $owner:literal, $boundary:literal, $world:literal,
     $proof:literal, $authority:literal, $mutation:literal, $counter:literal) => {
        RequirementContract {
            requirement: $requirement,
            owner: $owner,
            boundary: $boundary,
            world: $world,
            proof_kind: $proof,
            authority: $authority,
            mutation_family: $mutation,
            counter_family: $counter,
        }
    };
}

pub(super) const CONTRACTS: &[RequirementContract] = &[
    contract!(
        "P3-BASELINE-REPLAY-01",
        "worth-ui-host-native",
        "damage replay against the registered transparent baseline",
        "maximum-overlap-rectangle-world",
        "ordered-pixel",
        "worth_ui_host_native::presentation::damage",
        "baseline-substitution",
        "baseline-clears"
    ),
    contract!(
        "P3-CLOSE-01",
        "worth-ui-certification",
        "phase three final source closure",
        "phase-three-ledger-world",
        "ledger-closure",
        "worth_ui_certification::phase_three_ledger",
        "ledger",
        "requirements"
    ),
    contract!(
        "P3-DAMAGE-INDEX-01",
        "worth-ui-host-native",
        "derived damage intersection index",
        "maximum-overlap-rectangle-world",
        "runtime-model",
        "worth_ui_host_native::presentation::damage",
        "retained-scan",
        "damage-probes"
    ),
    contract!(
        "P3-DAMAGE-REPLAY-01",
        "worth-ui-host-native",
        "ordered damage replay",
        "maximum-overlap-rectangle-world",
        "ordered-pixel",
        "worth_ui_host_native::presentation::damage",
        "vacated-replay",
        "replayed-commands"
    ),
    contract!(
        "P3-DELTA-SOURCE-01",
        "worth-ui-runtime",
        "mounting issued exact successor scope",
        "mixed-carrier-world",
        "runtime-model",
        "worth_ui_runtime::mounting::projection",
        "delta-rediscovery",
        "source-rows"
    ),
    contract!(
        "P3-DRAW-LIST-01",
        "worth-ui-host-native",
        "receipt keyed retained draw list",
        "maximum-overlap-rectangle-world",
        "runtime-model",
        "worth_ui_host_native::presentation::draw_list",
        "retained-clone",
        "draw-list-mutations"
    ),
    contract!(
        "P3-HEADLESS-COST-01",
        "worth-ui-host-headless",
        "indexed headless delta consumption",
        "mixed-carrier-world",
        "slope-model",
        "worth_ui_host_headless::presentation",
        "retained-clone",
        "retained-scans"
    ),
    contract!(
        "P3-HP02-WORLD-01",
        "worth-ui-certification",
        "phase three dual courtroom",
        "maximum-overlap-phase-three-world",
        "integration-model",
        "worth_ui_certification::host_platform",
        "world-substitution",
        "worlds"
    ),
    contract!(
        "P3-PHYSICAL-AMPLIFICATION-01",
        "worth-ui-host-native",
        "logical damage versus physical presentation",
        "maximum-overlap-rectangle-world",
        "cost-model",
        "worth_ui_host_native::presentation::cost",
        "local-present-relabel",
        "physical-pixels"
    ),
    contract!(
        "P3-PREDECESSOR-01",
        "worth-ui-certification",
        "current source predecessor handoff",
        "phase-three-ledger-world",
        "operational-revalidation",
        "worth_ui_certification::phase_three_ledger",
        "stale-predecessor",
        "requirements"
    ),
    contract!(
        "P3-PRODUCER-SLOPE-01",
        "worth-ui-runtime",
        "incremental presentation work production",
        "mixed-carrier-world",
        "slope-model",
        "worth_ui_runtime::mounting::presentation",
        "retained-scan",
        "retained-scans"
    ),
    contract!(
        "P3-RECONSTRUCTION-01",
        "worth-ui-runtime",
        "cold reconstruction from mounted authority",
        "maximum-overlap-phase-three-world",
        "reconstruction",
        "worth_ui_runtime::mounting::presentation",
        "derived-state-reuse",
        "reconstructed-commands"
    ),
    contract!(
        "P3-STALE-DELTA-01",
        "worth-ui-runtime",
        "affine predecessor admission for mounted deltas",
        "mixed-carrier-world",
        "runtime-model",
        "worth_ui_runtime::mounting::presentation",
        "stale-delta",
        "stale-denials"
    ),
    contract!(
        "P3-TOTAL-ORDER-01",
        "worth-ui-runtime",
        "indexed total paint order mutation",
        "mixed-carrier-world",
        "runtime-model",
        "worth_ui_runtime::mounting::presentation",
        "identity-tie-break",
        "order-mutations"
    ),
    contract!(
        "P3-TRANSACTION-01",
        "worth-ui-host-native",
        "staged retained presentation transaction",
        "maximum-overlap-rectangle-world",
        "lifecycle-model",
        "worth_ui_host_native::presentation::transaction",
        "premature-commit",
        "transactions"
    ),
    contract!(
        "P3-UNCHANGED-01",
        "worth-ui-runtime",
        "retained equivalent unchanged progression",
        "mixed-carrier-world",
        "runtime-model",
        "worth_ui_runtime::mounting::presentation",
        "epoch-mint",
        "unchanged-work"
    ),
    contract!(
        "P4-BIDI-01",
        "worth-ui-text",
        "Unicode bidirectional analysis",
        "qualified-text-world",
        "unicode-conformance",
        "worth_ui_text::analysis::bidi",
        "visual-source-order",
        "bidi-runs"
    ),
    contract!(
        "P4-CAPACITY-01",
        "worth-ui-text",
        "text capacity admission before analysis or shaping",
        "qualified-text-world",
        "capacity-boundary",
        "worth_ui_text::profile::admission",
        "post-admission-overflow",
        "denied-before-analysis"
    ),
    contract!(
        "P4-EMOJI-SEQUENCE-01",
        "worth-ui-text",
        "Unicode 17 RGI emoji sequence analysis and layout",
        "qualified-text-world",
        "emoji-conformance",
        "worth_ui_text::analysis::emoji",
        "emoji-sequence-decomposition",
        "rgi-sequences"
    ),
    contract!(
        "P4-FALLBACK-01",
        "worth-ui-text",
        "deterministic cluster font fallback",
        "qualified-text-world",
        "font-oracle",
        "worth_ui_text::font_collection",
        "cluster-split",
        "fallback-probes"
    ),
    contract!(
        "P4-FONT-COLLECTION-01",
        "worth-ui-text",
        "qualified default and application font collections",
        "qualified-text-world",
        "manifest-qualification",
        "worth_ui_text::font_collection",
        "system-font",
        "font-faces"
    ),
    contract!(
        "P4-LINE-LAYOUT-01",
        "worth-ui-text",
        "Unicode line fitting and overflow",
        "qualified-text-world",
        "layout-oracle",
        "worth_ui_text::layout",
        "cluster-break",
        "lines"
    ),
    contract!(
        "P4-MEASUREMENT-IDENTITY-01",
        "worth-ui-text",
        "canonical layout measurement identity",
        "qualified-text-world",
        "identity-model",
        "worth_ui_text::layout",
        "duplicate-shaper",
        "layout-identities"
    ),
    contract!(
        "P4-ORIGINAL-RANGE-01",
        "worth-ui-text",
        "original UTF-8 range and caret geometry",
        "qualified-text-world",
        "range-oracle",
        "worth_ui_text::analysis",
        "range-normalization",
        "original-ranges"
    ),
    contract!(
        "P4-SHAPING-01",
        "worth-ui-text",
        "qualified complex script shaping",
        "qualified-text-world",
        "harfbuzz-fixture",
        "worth_ui_text::layout::shaping",
        "glyph-substitution",
        "shaped-glyphs"
    ),
    contract!(
        "P4-TEXT-COST-01",
        "worth-ui-text",
        "incremental paragraph analysis and layout",
        "qualified-text-world",
        "slope-model",
        "worth_ui_text::layout::cost",
        "paragraph-rescan",
        "retained-scans"
    ),
    contract!(
        "P4-TEXT-PROFILE-01",
        "worth-ui-text",
        "global Unicode text profile qualification",
        "text-profile-qualification-world",
        "manifest-qualification",
        "worth_ui_text::profile",
        "profile-drift",
        "qualified-assets"
    ),
    contract!(
        "P4-TEXT-RECONSTRUCTION-01",
        "worth-ui-text",
        "layout reconstruction from admitted text",
        "qualified-text-world",
        "reconstruction",
        "worth_ui_text::layout",
        "derived-state-reuse",
        "reconstructed-layouts"
    ),
    contract!(
        "P4-UNICODE-SEGMENTATION-01",
        "worth-ui-text",
        "Unicode grapheme word and line segmentation",
        "qualified-text-world",
        "unicode-conformance",
        "worth_ui_text::analysis",
        "emoji-sequence-split",
        "conformance-cases"
    ),
    contract!(
        "P4-UNCHANGED-01",
        "worth-ui-text",
        "unchanged paragraph layout identity reuse",
        "qualified-text-world",
        "slope-model",
        "worth_ui_text::layout::retention",
        "unchanged-analysis",
        "unchanged-analysis"
    ),
];

pub(super) fn validate_open_claim(row: &Row, contract: &RequirementContract) -> Result<(), String> {
    if !matches!(row["phase"].as_str(), "3" | "4") {
        return Ok(());
    }
    let scenario = scenario_delta(&row["requirement"])
        .ok_or_else(|| "future requirement omits its mutation case".to_owned())?;
    let mutation = format!("family={};case={scenario}", contract.mutation_family);
    if row["scenario_delta"] != scenario
        || row["mutation_control"] != mutation
        || row["fault_injection_boundary"] != fault_boundary(&row["requirement"])
    {
        return Err("future requirement claim drifted".to_owned());
    }
    let open_counter = format!("{}=open", contract.counter_family);
    if row["result"] == "OPEN" && row["structural_counters"] != open_counter {
        return Err("open future requirement has the wrong counter family".to_owned());
    }
    Ok(())
}

fn scenario_delta(requirement: &str) -> Option<&'static str> {
    Some(match requirement {
        "P3-BASELINE-REPLAY-01" => "opaque-baseline-clear",
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
        "P4-FONT-COLLECTION-01" => "ambient-font-fallback",
        "P4-LINE-LAYOUT-01" => "mid-cluster-wrap",
        "P4-MEASUREMENT-IDENTITY-01" => "independent-measurement-pass",
        "P4-ORIGINAL-RANGE-01" => "normalized-offset-substitution",
        "P4-SHAPING-01" => "one-run-latin",
        "P4-TEXT-COST-01" => "complete-document-rescan",
        "P4-TEXT-PROFILE-01" => "font-or-unicode-digest-drift",
        "P4-TEXT-RECONSTRUCTION-01" => "stale-layout-reuse",
        "P4-UNICODE-SEGMENTATION-01" => "zwj-or-flag-split",
        "P4-UNCHANGED-01" => "unchanged-paragraph-rescan",
        _ => return None,
    })
}

fn fault_boundary(requirement: &str) -> &'static str {
    match requirement {
        "P3-BASELINE-REPLAY-01"
        | "P3-HP02-WORLD-01"
        | "P3-PHYSICAL-AMPLIFICATION-01"
        | "P3-TRANSACTION-01" => "after-effects-may-have-begun",
        _ => "not-applicable",
    }
}
