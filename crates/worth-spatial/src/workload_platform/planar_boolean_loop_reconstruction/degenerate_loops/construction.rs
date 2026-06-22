use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanBornLoop,
    PlanarBooleanLoopClassifiedProductKind, PlanarBooleanLoopContainmentEvidencePosture,
    PlanarBooleanLoopContainmentEvidencePostureKind, PlanarBooleanLoopRoleOutcome,
    PlanarBooleanLoopRoleOutcomeKind,
};

use super::counters::PlanarBooleanDegenerateLoopOutcomeBoundaryCounters;
use super::geometry::DegenerateLoopGeometryIndex;
use super::identity::{degenerate_loop_outcome_identity, degenerate_loop_outcome_set_identity};
use super::input::PlanarBooleanDegenerateLoopOutcomeBoundaryInput;
use super::product::{
    PlanarBooleanDegenerateLoopOutcomeBoundary, PlanarBooleanDegenerateLoopOutcomeSet,
};
use super::row::{PlanarBooleanDegenerateLoopOutcome, PlanarBooleanDegenerateLoopOutcomeKind};

pub(crate) fn classify_degenerate_loop_outcomes(
    input: PlanarBooleanDegenerateLoopOutcomeBoundaryInput<'_>,
) -> PlanarBooleanDegenerateLoopOutcomeBoundary {
    let request_identity = input.reconstructed_loops().request_identity().to_string();
    let role_index = role_outcome_index(input);
    let containment_index = containment_posture_index(input);
    let geometry_index =
        DegenerateLoopGeometryIndex::new(input.source_loop_carriers(), input.split_fragments());
    let mut counters = PlanarBooleanDegenerateLoopOutcomeBoundaryCounters::default();
    let mut rows = Vec::new();

    for loop_row in input.reconstructed_loops().rows() {
        counters.consumed_reconstructed_loop();
        rows.push(classify_reconstructed_loop(
            &request_identity,
            loop_row,
            role_index.get(loop_row.reconstructed_loop_identity()),
            containment_index.get(loop_row.reconstructed_loop_identity()),
            &geometry_index,
            &mut counters,
        ));
    }

    for loop_row in input.born_loops().rows() {
        counters.consumed_born_loop();
        rows.push(classify_born_loop(
            &request_identity,
            loop_row,
            role_index.get(loop_row.born_loop_identity()),
            containment_index.get(loop_row.born_loop_identity()),
            &geometry_index,
            &mut counters,
        ));
    }

    rows.sort_by(|left, right| {
        left.loop_kind()
            .cmp(&right.loop_kind())
            .then_with(|| left.loop_identity().cmp(right.loop_identity()))
    });

    PlanarBooleanDegenerateLoopOutcomeBoundary::new(
        PlanarBooleanDegenerateLoopOutcomeSet::new(
            degenerate_loop_outcome_set_identity(&request_identity, &rows),
            request_identity,
            rows,
        ),
        counters,
    )
}

fn classify_reconstructed_loop(
    request_identity: &str,
    loop_row: &PlanarBooleanAdmittedReconstructedLoop,
    role_outcome: Option<&&PlanarBooleanLoopRoleOutcome>,
    containment_posture: Option<&&PlanarBooleanLoopContainmentEvidencePosture>,
    geometry_index: &DegenerateLoopGeometryIndex<'_>,
    counters: &mut PlanarBooleanDegenerateLoopOutcomeBoundaryCounters,
) -> PlanarBooleanDegenerateLoopOutcome {
    classify_loop(
        request_identity,
        loop_row.reconstructed_loop_identity(),
        PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
        vec![loop_row.source_loop_identity().to_string()],
        loop_row.local_frame_identity().to_string(),
        loop_row.precision_basis_identity().to_string(),
        loop_row.fragment_identities().to_vec(),
        loop_row.split_vertex_identities().to_vec(),
        role_outcome.copied(),
        containment_posture.copied(),
        geometry_index,
        counters,
    )
}

fn classify_born_loop(
    request_identity: &str,
    loop_row: &PlanarBooleanBornLoop,
    role_outcome: Option<&&PlanarBooleanLoopRoleOutcome>,
    containment_posture: Option<&&PlanarBooleanLoopContainmentEvidencePosture>,
    geometry_index: &DegenerateLoopGeometryIndex<'_>,
    counters: &mut PlanarBooleanDegenerateLoopOutcomeBoundaryCounters,
) -> PlanarBooleanDegenerateLoopOutcome {
    classify_loop(
        request_identity,
        loop_row.born_loop_identity(),
        PlanarBooleanLoopClassifiedProductKind::BornLoop,
        loop_row.source_loop_identities().to_vec(),
        loop_row.local_frame_identity().to_string(),
        loop_row.precision_basis_identity().to_string(),
        loop_row.fragment_identities().to_vec(),
        loop_row.split_vertex_identities().to_vec(),
        role_outcome.copied(),
        containment_posture.copied(),
        geometry_index,
        counters,
    )
}

#[allow(clippy::too_many_arguments)]
fn classify_loop(
    request_identity: &str,
    loop_identity: &str,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    source_loop_identities: Vec<String>,
    local_frame_identity: String,
    precision_basis_identity: String,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    role_outcome: Option<&PlanarBooleanLoopRoleOutcome>,
    containment_posture: Option<&PlanarBooleanLoopContainmentEvidencePosture>,
    geometry_index: &DegenerateLoopGeometryIndex<'_>,
    counters: &mut PlanarBooleanDegenerateLoopOutcomeBoundaryCounters,
) -> PlanarBooleanDegenerateLoopOutcome {
    let (kind, reason) = if is_tiny_cardinality(&fragment_identities, &split_vertex_identities) {
        counters.emitted_tiny_cardinality();
        (
            PlanarBooleanDegenerateLoopOutcomeKind::DeniedTinyCardinality,
            "loop degeneracy classification denies loop candidates that do not establish at least three fragment and split-vertex positions before identity minting".to_string(),
        )
    } else if has_self_touching_split_vertex(&split_vertex_identities) {
        counters.emitted_self_touching();
        (
            PlanarBooleanDegenerateLoopOutcomeKind::DeniedSelfTouching,
            "loop degeneracy classification denies split-vertex reuse inside a single loop because the same vertex identity appears more than once in the loop walk".to_string(),
        )
    } else {
        match geometry_index.classify_zero_area(loop_identity, &fragment_identities) {
            Ok(Some(reason)) => {
                counters.emitted_zero_area();
                (
                    PlanarBooleanDegenerateLoopOutcomeKind::DeniedZeroArea,
                    reason,
                )
            }
            Err(reason) => {
                counters.emitted_geometry_policy_required();
                (
                    PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredGeometryEvidence,
                    reason,
                )
            }
            Ok(None) if role_requires_policy(role_outcome) => {
                counters.emitted_policy_required();
                (
                    PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredRoleEvidence,
                    "loop degeneracy classification requires explicit policy when role evidence is missing, contradictory, or ambiguous before loop identity minting".to_string(),
                )
            }
            Ok(None) if containment_requires_policy(containment_posture) => {
                counters.emitted_policy_required();
                (
                    PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredContainmentEvidence,
                    "loop degeneracy classification requires explicit policy when containment evidence is missing, contradictory, or multi-source ambiguous before loop identity minting".to_string(),
                )
            }
            Ok(None) => {
                counters.emitted_admitted();
                (
                    PlanarBooleanDegenerateLoopOutcomeKind::AdmittedForIdentityMinting,
                    "loop degeneracy classification found no topology-evident collapsed or poisoned posture, so the loop may advance to identity minting".to_string(),
                )
            }
        }
    };

    PlanarBooleanDegenerateLoopOutcome::new(
        degenerate_loop_outcome_identity(
            request_identity,
            loop_identity,
            kind.as_str(),
            &fragment_identities,
            &split_vertex_identities,
        ),
        loop_identity.to_string(),
        loop_kind,
        source_loop_identities,
        local_frame_identity,
        precision_basis_identity,
        fragment_identities,
        split_vertex_identities,
        role_outcome.map(|row| row.role_outcome_identity().to_string()),
        containment_posture.map(|row| row.containment_posture_identity().to_string()),
        kind,
        reason,
    )
}

fn role_outcome_index(
    input: PlanarBooleanDegenerateLoopOutcomeBoundaryInput<'_>,
) -> BTreeMap<String, &PlanarBooleanLoopRoleOutcome> {
    input
        .role_outcomes()
        .rows()
        .iter()
        .map(|row| (row.loop_identity().to_string(), row))
        .collect()
}

fn containment_posture_index(
    input: PlanarBooleanDegenerateLoopOutcomeBoundaryInput<'_>,
) -> BTreeMap<String, &PlanarBooleanLoopContainmentEvidencePosture> {
    input
        .containment_postures()
        .rows()
        .iter()
        .map(|row| (row.loop_identity().to_string(), row))
        .collect()
}

fn is_tiny_cardinality(fragment_identities: &[String], split_vertex_identities: &[String]) -> bool {
    fragment_identities.len() < 3 || unique_count(split_vertex_identities) < 3
}

fn has_self_touching_split_vertex(split_vertex_identities: &[String]) -> bool {
    unique_count(split_vertex_identities) != split_vertex_identities.len()
}

fn unique_count(values: &[String]) -> usize {
    values.iter().collect::<BTreeSet<_>>().len()
}

fn role_requires_policy(role_outcome: Option<&PlanarBooleanLoopRoleOutcome>) -> bool {
    match role_outcome.map(|row| row.kind()) {
        Some(PlanarBooleanLoopRoleOutcomeKind::PreservedSourceRole)
        | Some(PlanarBooleanLoopRoleOutcomeKind::SingleSourceBornLoopRoleDerivedFromEvidence) => {
            false
        }
        Some(PlanarBooleanLoopRoleOutcomeKind::BornLoopRoleAmbiguous)
        | Some(PlanarBooleanLoopRoleOutcomeKind::ContradictorySourceRoleEvidence)
        | Some(PlanarBooleanLoopRoleOutcomeKind::MissingSourceRoleEvidence)
        | None => true,
    }
}

fn containment_requires_policy(
    containment_posture: Option<&PlanarBooleanLoopContainmentEvidencePosture>,
) -> bool {
    match containment_posture.map(|row| row.kind()) {
        Some(PlanarBooleanLoopContainmentEvidencePostureKind::PreservedSourceContainmentEvidence)
        | Some(PlanarBooleanLoopContainmentEvidencePostureKind::SplitSourceContainmentEvidence)
        | Some(PlanarBooleanLoopContainmentEvidencePostureKind::SingleSourceBornLoopContainmentEvidence) => false,
        Some(PlanarBooleanLoopContainmentEvidencePostureKind::MultiSourceBornLoopContainmentEvidence)
        | Some(PlanarBooleanLoopContainmentEvidencePostureKind::ContradictorySourceContainmentEvidence)
        | Some(PlanarBooleanLoopContainmentEvidencePostureKind::MissingSourceContainmentEvidence)
        | None => true,
    }
}
