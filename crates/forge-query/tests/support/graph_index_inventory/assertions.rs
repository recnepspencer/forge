use forge_query::facade::runtime::{
    ForgeQueryGraphIndexInventoryMatch, ForgeQueryGraphIndexInventoryMatchOutcome,
    ForgeQueryGraphIndexPosture, ForgeQueryGraphReadAccessRequirementKind,
    ForgeQueryGraphReadAccessRequirementSet,
};

pub fn requirement_row_digest_for_kind(
    requirements: &ForgeQueryGraphReadAccessRequirementSet,
    requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
) -> String {
    requirements
        .rows()
        .iter()
        .find(|row| row.kind() == &requirement_kind)
        .expect("expected requirement row should exist")
        .digest_part()
}

pub fn missing_match_for_requirement<'a>(
    matches: &'a [ForgeQueryGraphIndexInventoryMatch],
    requirement_row_digest: &str,
) -> &'a ForgeQueryGraphIndexInventoryMatch {
    matches
        .iter()
        .find(|row| {
            row.requirement_row_digest() == requirement_row_digest
                && row.outcome() == &ForgeQueryGraphIndexInventoryMatchOutcome::MissingSupportRow
        })
        .expect("expected localized missing support match should exist")
}

pub fn support_match_for_kind<'a>(
    matches: &'a [ForgeQueryGraphIndexInventoryMatch],
    requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
    posture: ForgeQueryGraphIndexPosture,
) -> &'a ForgeQueryGraphIndexInventoryMatch {
    matches
        .iter()
        .find(|row| {
            row.requirement_kind() == &requirement_kind && row.support_posture() == &posture
        })
        .expect("expected support posture match should exist")
}
