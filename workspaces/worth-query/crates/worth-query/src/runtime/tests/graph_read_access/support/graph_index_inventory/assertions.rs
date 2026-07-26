use crate::runtime::{
    WorthQueryGraphIndexInventoryMatch, WorthQueryGraphIndexInventoryMatchOutcome,
    WorthQueryGraphIndexPosture, WorthQueryGraphReadAccessRequirementKind,
    WorthQueryGraphReadAccessRequirementSet,
};

pub fn requirement_row_digest_for_kind(
    requirements: &WorthQueryGraphReadAccessRequirementSet,
    requirement_kind: WorthQueryGraphReadAccessRequirementKind,
) -> String {
    requirements
        .rows()
        .iter()
        .find(|row| row.kind() == &requirement_kind)
        .expect("expected requirement row should exist")
        .digest_part()
}

pub fn missing_match_for_requirement<'a>(
    matches: &'a [WorthQueryGraphIndexInventoryMatch],
    requirement_row_digest: &str,
) -> &'a WorthQueryGraphIndexInventoryMatch {
    matches
        .iter()
        .find(|row| {
            row.requirement_row_digest() == requirement_row_digest
                && row.outcome() == &WorthQueryGraphIndexInventoryMatchOutcome::MissingSupportRow
        })
        .expect("expected localized missing support match should exist")
}

pub fn support_match_for_kind(
    matches: &[WorthQueryGraphIndexInventoryMatch],
    requirement_kind: WorthQueryGraphReadAccessRequirementKind,
    posture: WorthQueryGraphIndexPosture,
) -> &WorthQueryGraphIndexInventoryMatch {
    matches
        .iter()
        .find(|row| {
            row.requirement_kind() == &requirement_kind && row.support_posture() == &posture
        })
        .expect("expected support posture match should exist")
}
