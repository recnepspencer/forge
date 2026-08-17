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
        "P4-PREDECESSOR-01",
        "worth-ui-certification",
        "current Phase 1-3 source handoff",
        "phase-four-ledger-world",
        "operational-revalidation",
        "worth_ui_certification::phase_four_ledger",
        "stale-predecessor",
        "requirements"
    ),
    contract!(
        "P4-BIDI-INTERACTION-01",
        "worth-ui-text",
        "bidi caret hit and selection geometry",
        "qualified-text-world",
        "interaction-oracle",
        "worth_ui_text::layout::interaction",
        "caret-affinity-swap",
        "interaction-records"
    ),
    contract!(
        "P4-TEXT-CONTENT-LOCALITY-01",
        "worth-ui-text",
        "content-only paragraph locality",
        "qualified-text-world",
        "slope-model",
        "worth_ui_text::layout::retention",
        "content-global-rescan",
        "analyzed-bytes"
    ),
    contract!(
        "P4-TEXT-WIDTH-LOCALITY-01",
        "worth-ui-text",
        "paragraph width locality",
        "qualified-text-world",
        "slope-model",
        "worth_ui_text::layout::retention",
        "paragraph-width-global-rescan",
        "relayout-paragraphs"
    ),
    contract!(
        "P4-ACCESSIBILITY-GEOMETRY-01",
        "worth-ui-text",
        "shared accessibility layout geometry",
        "qualified-text-world",
        "consumer-identity",
        "worth_ui_text::layout::consumers",
        "accessibility-reshape",
        "layout-identities"
    ),
    contract!(
        "P4-COLOR-FONT-ADMISSION-01",
        "worth-ui-text",
        "qualified color font admission",
        "text-profile-qualification-world",
        "manifest-qualification",
        "worth_ui_text::font_collection",
        "unsupported-color-table",
        "color-formats"
    ),
    contract!(
        "P4-CLOSE-01",
        "worth-ui-certification",
        "phase four final source closure",
        "phase-four-ledger-world",
        "ledger-closure",
        "worth_ui_certification::phase_four_ledger",
        "ledger",
        "requirements"
    ),
];
