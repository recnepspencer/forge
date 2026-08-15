use super::super::requirement_contract::RequirementContract;

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
        "P5-PREDECESSOR-01",
        "worth-ui-certification",
        "current Phase 1-4 source handoff",
        "phase-five-ledger-world",
        "operational-revalidation",
        "worth_ui_certification::phase_five_ledger",
        "stale-predecessor",
        "requirements"
    ),
    contract!(
        "P5-GLYPH-RASTER-01",
        "worth-ui-text",
        "typed alpha and color glyph raster batches",
        "qualified-text-world",
        "raster-oracle",
        "worth_ui_text::raster",
        "raster-authority",
        "raster-batches"
    ),
    contract!(
        "P5-COLOR-EMOJI-01",
        "worth-ui-text",
        "intrinsic color emoji raster without cluster split",
        "qualified-text-world",
        "emoji-conformance",
        "worth_ui_text::raster::color",
        "color-layer-drop",
        "rgi-sequences"
    ),
    contract!(
        "P5-ATLAS-01",
        "worth-ui-host-native",
        "separate bounded alpha and RGBA atlas lifecycle",
        "qualified-text-world",
        "atlas-lifecycle",
        "worth_ui_host_native::atlas",
        "atlas-lifecycle",
        "atlas-kinds"
    ),
    contract!(
        "P5-ATLAS-PINNING-01",
        "worth-ui-host-native",
        "live-layout atlas pinning and deterministic eviction",
        "qualified-text-world",
        "atlas-pinning",
        "worth_ui_host_native::atlas::pinning",
        "pin-eviction",
        "pinned-layouts"
    ),
    contract!(
        "P5-TEXT-DPI-01",
        "worth-ui-text",
        "pure DPI raster replacement without relayout",
        "qualified-text-world",
        "dpi-replacement",
        "worth_ui_text::raster::dpi",
        "dpi-reuse",
        "dpi-replacements"
    ),
    contract!(
        "P5-TEXT-SPAN-PAINT-01",
        "worth-ui-runtime",
        "paint-span identity and logical foreground RGBA",
        "qualified-text-world",
        "paint-span-oracle",
        "worth_ui_runtime::mounting::text_paint",
        "paint-span",
        "paint-spans"
    ),
    contract!(
        "P5-TEXT-PIXELS-01",
        "worth-ui-host-native",
        "native and headless paint-span pixel identity",
        "qualified-text-world",
        "pixel-identity",
        "worth_ui_host_native::presentation::text",
        "pixel-identity",
        "pixel-observations"
    ),
    contract!(
        "P5-TEXT-RECONSTRUCTION-01",
        "worth-ui-runtime",
        "layout raster and atlas reconstruction from mounted authority",
        "qualified-text-world",
        "reconstruction",
        "worth_ui_runtime::mounting::text",
        "derived-state-reuse",
        "reconstructed-atlases"
    ),
    contract!(
        "P5-TEXT-COST-01",
        "worth-ui-text",
        "ordinary versus reconstructive text raster cost",
        "qualified-text-world",
        "slope-model",
        "worth_ui_text::raster::cost",
        "retained-scan",
        "retained-scans"
    ),
    contract!(
        "P5-CLOSE-01",
        "worth-ui-certification",
        "phase five final source closure",
        "phase-five-ledger-world",
        "ledger-closure",
        "worth_ui_certification::phase_five_ledger",
        "ledger",
        "requirements"
    ),
];
