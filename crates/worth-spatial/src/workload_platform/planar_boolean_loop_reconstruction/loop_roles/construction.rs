use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_events::PlanarBooleanLoopRole;
use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanSourceLoopSplitAttributionKind;

use super::counters::PlanarBooleanLoopRoleOutcomeBoundaryCounters;
use super::identity::{
    containment_posture_identity, containment_posture_set_identity, role_outcome_identity,
    role_outcome_set_identity,
};
use super::input::PlanarBooleanLoopRoleOutcomeBoundaryInput;
use super::product::{
    PlanarBooleanLoopContainmentEvidencePostureSet, PlanarBooleanLoopRoleOutcomeBoundary,
    PlanarBooleanLoopRoleOutcomeSet,
};
use super::row::{
    PlanarBooleanLoopClassifiedProductKind, PlanarBooleanLoopContainmentEvidencePosture,
    PlanarBooleanLoopContainmentEvidencePostureKind, PlanarBooleanLoopRoleOutcome,
    PlanarBooleanLoopRoleOutcomeKind,
};

pub(crate) fn classify_loop_role_outcomes(
    input: PlanarBooleanLoopRoleOutcomeBoundaryInput<'_>,
) -> PlanarBooleanLoopRoleOutcomeBoundary {
    let request_identity = input
        .reconstructed_loop_boundary()
        .reconstructed_loops()
        .request_identity()
        .to_string();
    let source_role_index = source_role_index(input);
    let island_membership_index = island_membership_index(input);
    let split_attribution_index = split_attribution_index(input);
    let mut counters = PlanarBooleanLoopRoleOutcomeBoundaryCounters::default();
    let mut role_outcomes = Vec::new();
    let mut containment_postures = Vec::new();

    for row in input
        .reconstructed_loop_boundary()
        .reconstructed_loops()
        .rows()
    {
        counters.consumed_reconstructed_loop();
        let source_loop_identities = vec![row.source_loop_identity().to_string()];
        let island_identities =
            island_identities_for(&island_membership_index, row.reconstructed_loop_identity());
        let summary = summarize_source_role_evidence(&source_loop_identities, &source_role_index);
        let role_kind = classify_reconstructed_role_kind(&summary);
        counters.emitted_role_outcome(role_kind);
        let preserved_source_role = preserved_source_role_for(role_kind, &summary);
        role_outcomes.push(PlanarBooleanLoopRoleOutcome::new(
            role_outcome_identity(
                &request_identity,
                row.reconstructed_loop_identity(),
                &source_loop_identities,
                &island_identities,
            ),
            row.reconstructed_loop_identity().to_string(),
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
            island_identities.clone(),
            source_loop_identities.clone(),
            preserved_source_role,
            role_kind,
        ));

        let containment_kind = classify_reconstructed_containment_kind(
            &summary,
            split_attribution_index.get(row.source_loop_identity()),
        );
        counters.emitted_containment_posture();
        containment_postures.push(PlanarBooleanLoopContainmentEvidencePosture::new(
            containment_posture_identity(
                &request_identity,
                row.reconstructed_loop_identity(),
                &source_loop_identities,
                &island_identities,
            ),
            row.reconstructed_loop_identity().to_string(),
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
            island_identities,
            source_loop_identities,
            containment_kind,
        ));
    }

    for row in input.reconstructed_loop_boundary().born_loops().rows() {
        counters.consumed_born_loop();
        let source_loop_identities = row.source_loop_identities().to_vec();
        let island_identities =
            island_identities_for(&island_membership_index, row.born_loop_identity());
        let summary = summarize_source_role_evidence(&source_loop_identities, &source_role_index);
        let role_kind = classify_born_role_kind(&summary, row.source_loop_identities().len());
        counters.emitted_role_outcome(role_kind);
        let preserved_source_role = preserved_source_role_for(role_kind, &summary);
        role_outcomes.push(PlanarBooleanLoopRoleOutcome::new(
            role_outcome_identity(
                &request_identity,
                row.born_loop_identity(),
                &source_loop_identities,
                &island_identities,
            ),
            row.born_loop_identity().to_string(),
            PlanarBooleanLoopClassifiedProductKind::BornLoop,
            island_identities.clone(),
            source_loop_identities.clone(),
            preserved_source_role,
            role_kind,
        ));

        let containment_kind =
            classify_born_containment_kind(&summary, row.source_loop_identities().len());
        counters.emitted_containment_posture();
        containment_postures.push(PlanarBooleanLoopContainmentEvidencePosture::new(
            containment_posture_identity(
                &request_identity,
                row.born_loop_identity(),
                &source_loop_identities,
                &island_identities,
            ),
            row.born_loop_identity().to_string(),
            PlanarBooleanLoopClassifiedProductKind::BornLoop,
            island_identities,
            source_loop_identities,
            containment_kind,
        ));
    }

    role_outcomes.sort_by(|left, right| {
        left.loop_kind()
            .cmp(&right.loop_kind())
            .then_with(|| left.loop_identity().cmp(right.loop_identity()))
    });
    containment_postures.sort_by(|left, right| {
        left.loop_kind()
            .cmp(&right.loop_kind())
            .then_with(|| left.loop_identity().cmp(right.loop_identity()))
    });

    PlanarBooleanLoopRoleOutcomeBoundary::new(
        PlanarBooleanLoopRoleOutcomeSet::new(
            role_outcome_set_identity(&request_identity, &role_outcomes),
            request_identity.clone(),
            role_outcomes,
        ),
        PlanarBooleanLoopContainmentEvidencePostureSet::new(
            containment_posture_set_identity(&request_identity, &containment_postures),
            request_identity,
            containment_postures,
        ),
        counters,
    )
}

fn source_role_index(
    input: PlanarBooleanLoopRoleOutcomeBoundaryInput<'_>,
) -> BTreeMap<String, Vec<PlanarBooleanLoopRole>> {
    let mut index = BTreeMap::<String, BTreeMap<&'static str, PlanarBooleanLoopRole>>::new();
    for row in input.source_provenance().source_loop_carriers().rows() {
        index
            .entry(row.source_loop_identity().to_string())
            .or_default()
            .insert(row.loop_role().query_key(), row.loop_role());
    }
    index
        .into_iter()
        .map(|(source_loop_identity, roles)| {
            (
                source_loop_identity,
                roles.into_values().collect::<Vec<PlanarBooleanLoopRole>>(),
            )
        })
        .collect()
}

fn island_membership_index(
    input: PlanarBooleanLoopRoleOutcomeBoundaryInput<'_>,
) -> BTreeMap<String, Vec<String>> {
    let mut index = BTreeMap::<String, BTreeSet<String>>::new();
    for row in input.island_partition().rows() {
        for loop_identity in row.member_loop_identities() {
            index
                .entry(loop_identity.clone())
                .or_default()
                .insert(row.island_identity().to_string());
        }
    }
    index
        .into_iter()
        .map(|(loop_identity, island_ids)| {
            (
                loop_identity,
                island_ids.into_iter().collect::<Vec<String>>(),
            )
        })
        .collect()
}

fn split_attribution_index<'a>(
    input: PlanarBooleanLoopRoleOutcomeBoundaryInput<'a>,
) -> BTreeMap<String, &'a crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanSourceLoopSplitAttributionRow>
{
    input
        .source_loop_split_attribution()
        .rows()
        .iter()
        .map(|row| (row.source_loop_identity().to_string(), row))
        .collect()
}

fn island_identities_for(
    island_membership_index: &BTreeMap<String, Vec<String>>,
    loop_identity: &str,
) -> Vec<String> {
    island_membership_index
        .get(loop_identity)
        .cloned()
        .unwrap_or_default()
}

#[derive(Default)]
struct SourceRoleEvidenceSummary {
    unique_roles: Vec<PlanarBooleanLoopRole>,
    missing_sources: Vec<String>,
}

fn summarize_source_role_evidence(
    source_loop_identities: &[String],
    source_role_index: &BTreeMap<String, Vec<PlanarBooleanLoopRole>>,
) -> SourceRoleEvidenceSummary {
    let mut summary = SourceRoleEvidenceSummary::default();
    let mut unique_roles = BTreeMap::<&'static str, PlanarBooleanLoopRole>::new();
    for source_loop_identity in source_loop_identities {
        let Some(roles) = source_role_index.get(source_loop_identity) else {
            summary.missing_sources.push(source_loop_identity.clone());
            continue;
        };
        for role in roles {
            unique_roles.insert(role.query_key(), *role);
        }
    }
    summary.unique_roles = unique_roles.into_values().collect();
    summary
}

fn classify_reconstructed_role_kind(
    summary: &SourceRoleEvidenceSummary,
) -> PlanarBooleanLoopRoleOutcomeKind {
    if !summary.missing_sources.is_empty() {
        return PlanarBooleanLoopRoleOutcomeKind::MissingSourceRoleEvidence;
    }
    if summary.unique_roles.len() > 1 {
        return PlanarBooleanLoopRoleOutcomeKind::ContradictorySourceRoleEvidence;
    }
    PlanarBooleanLoopRoleOutcomeKind::PreservedSourceRole
}

fn classify_born_role_kind(
    summary: &SourceRoleEvidenceSummary,
    source_loop_count: usize,
) -> PlanarBooleanLoopRoleOutcomeKind {
    if !summary.missing_sources.is_empty() {
        return PlanarBooleanLoopRoleOutcomeKind::MissingSourceRoleEvidence;
    }
    if summary.unique_roles.len() > 1 {
        return PlanarBooleanLoopRoleOutcomeKind::ContradictorySourceRoleEvidence;
    }
    if source_loop_count > 1 {
        return PlanarBooleanLoopRoleOutcomeKind::BornLoopRoleAmbiguous;
    }
    PlanarBooleanLoopRoleOutcomeKind::SingleSourceBornLoopRoleDerivedFromEvidence
}

fn classify_reconstructed_containment_kind(
    summary: &SourceRoleEvidenceSummary,
    attribution: Option<
        &&crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanSourceLoopSplitAttributionRow,
    >,
) -> PlanarBooleanLoopContainmentEvidencePostureKind {
    if !summary.missing_sources.is_empty() {
        return PlanarBooleanLoopContainmentEvidencePostureKind::MissingSourceContainmentEvidence;
    }
    if summary.unique_roles.len() > 1 {
        return PlanarBooleanLoopContainmentEvidencePostureKind::ContradictorySourceContainmentEvidence;
    }
    match attribution.map(|row| row.kind()) {
        Some(PlanarBooleanSourceLoopSplitAttributionKind::SplitIntoMultipleIslands) => {
            PlanarBooleanLoopContainmentEvidencePostureKind::SplitSourceContainmentEvidence
        }
        Some(PlanarBooleanSourceLoopSplitAttributionKind::ContributedToBornLoop) => {
            PlanarBooleanLoopContainmentEvidencePostureKind::MultiSourceBornLoopContainmentEvidence
        }
        Some(PlanarBooleanSourceLoopSplitAttributionKind::Preserved) | None => {
            PlanarBooleanLoopContainmentEvidencePostureKind::PreservedSourceContainmentEvidence
        }
    }
}

fn classify_born_containment_kind(
    summary: &SourceRoleEvidenceSummary,
    source_loop_count: usize,
) -> PlanarBooleanLoopContainmentEvidencePostureKind {
    if !summary.missing_sources.is_empty() {
        return PlanarBooleanLoopContainmentEvidencePostureKind::MissingSourceContainmentEvidence;
    }
    if summary.unique_roles.len() > 1 {
        return PlanarBooleanLoopContainmentEvidencePostureKind::ContradictorySourceContainmentEvidence;
    }
    if source_loop_count > 1 {
        return PlanarBooleanLoopContainmentEvidencePostureKind::MultiSourceBornLoopContainmentEvidence;
    }
    PlanarBooleanLoopContainmentEvidencePostureKind::SingleSourceBornLoopContainmentEvidence
}

fn preserved_source_role_for(
    kind: PlanarBooleanLoopRoleOutcomeKind,
    summary: &SourceRoleEvidenceSummary,
) -> Option<PlanarBooleanLoopRole> {
    match kind {
        PlanarBooleanLoopRoleOutcomeKind::PreservedSourceRole
        | PlanarBooleanLoopRoleOutcomeKind::SingleSourceBornLoopRoleDerivedFromEvidence => {
            summary.unique_roles.first().copied()
        }
        PlanarBooleanLoopRoleOutcomeKind::BornLoopRoleAmbiguous
        | PlanarBooleanLoopRoleOutcomeKind::ContradictorySourceRoleEvidence
        | PlanarBooleanLoopRoleOutcomeKind::MissingSourceRoleEvidence => None,
    }
}
