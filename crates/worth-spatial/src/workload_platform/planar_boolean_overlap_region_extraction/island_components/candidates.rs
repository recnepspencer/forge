use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::{
    Left, Right,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapCellContainmentEvidenceKind::BoundaryOnly;
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapCellWindingEvidenceKind::{
    BoundaryTopologyAndSupportingIslandTopology, SupportingIslandTopology,
};

use super::counters::PlanarBooleanOverlapIslandComponentCounters;
use super::identity::{candidate_identity, candidate_set_identity, island_identity};
use super::input::PlanarBooleanOverlapIslandCandidateInput;
use super::lookup::PlanarBooleanOverlapCellEvidenceLookup;
use super::product::PlanarBooleanOverlapIslandCandidateSet;
use super::rows::{
    PlanarBooleanOverlapIslandCandidateKind as Kind, PlanarBooleanOverlapIslandCandidateRow,
};
use super::validation::{
    contradictory, unsupported, validate_cell_overlap_signal, validate_input_identities,
};
use super::PlanarBooleanOverlapIslandComponentDenial;

pub(super) fn build_island_candidate_set(
    input: PlanarBooleanOverlapIslandCandidateInput<'_>,
) -> Result<PlanarBooleanOverlapIslandCandidateSet, PlanarBooleanOverlapIslandComponentDenial> {
    let mut counters = PlanarBooleanOverlapIslandComponentCounters::default();
    validate_input_identities(input, &mut counters)?;
    let arrangement = input.arrangement_graph();
    let lookup = PlanarBooleanOverlapCellEvidenceLookup::default()
        .with_containment_rows(input.containment_map().rows())
        .with_winding_rows(input.winding_field().rows());

    let mut rows = Vec::new();
    for cell in arrangement.cell_set().cells() {
        validate_cell_overlap_signal(cell.cell_identity(), &lookup, &mut counters)?;
        let left_containment = lookup
            .containment_row(cell.cell_identity(), Left)
            .expect("validated left containment row should exist");
        let right_containment = lookup
            .containment_row(cell.cell_identity(), Right)
            .expect("validated right containment row should exist");
        let left_winding = lookup
            .winding_row(cell.cell_identity(), Left)
            .expect("validated left winding row should exist");
        let right_winding = lookup
            .winding_row(cell.cell_identity(), Right)
            .expect("validated right winding row should exist");
        let left_inside = left_winding.winding_number() != 0;
        let right_inside = right_winding.winding_number() != 0;
        let has_boundary_contact = left_containment.evidence_kind() == BoundaryOnly
            || right_containment.evidence_kind() == BoundaryOnly;

        let kind = classify_candidate_kind(
            left_inside,
            right_inside,
            has_boundary_contact,
            cell.supporting_island_identity().is_some(),
            left_winding.evidence_kind(),
            right_winding.evidence_kind(),
        )
        .ok_or_else(|| {
            if left_inside || right_inside {
                contradictory(cell.cell_identity(), &mut counters)
            } else {
                unsupported(cell.cell_identity(), &mut counters)
            }
        })?;

        rows.push(PlanarBooleanOverlapIslandCandidateRow::new(
            candidate_identity(cell.cell_identity(), kind),
            island_identity(cell.neighborhood_identity()),
            cell.cell_identity().to_string(),
            cell.neighborhood_identity().to_string(),
            cell.boundary_component_identities().to_vec(),
            cell.boundary_segment_identities().to_vec(),
            cell.source_loop_identities().to_vec(),
            cell.propagated_persistent_name_identities().to_vec(),
            kind,
        ));
        counters.admitted_candidate();
    }

    Ok(PlanarBooleanOverlapIslandCandidateSet::new(
        candidate_set_identity(
            arrangement.request_identity(),
            rows.iter().map(|row| row.candidate_identity().to_string()),
        ),
        arrangement.request_identity().to_string(),
        arrangement.arrangement_graph_identity().to_string(),
        arrangement.cell_set().cell_set_identity().to_string(),
        arrangement.ordering_basis_identity().to_string(),
        rows,
        counters,
    ))
}

fn classify_candidate_kind(
    left_inside: bool,
    right_inside: bool,
    has_boundary_contact: bool,
    has_supporting_island: bool,
    left_winding_evidence: crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapCellWindingEvidenceKind,
    right_winding_evidence: crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapCellWindingEvidenceKind,
) -> Option<Kind> {
    match (left_inside, right_inside, has_boundary_contact) {
        (true, true, false) => Some(Kind::AreaOverlap),
        (true, true, true)
            if supports_area_overlap_topology(left_winding_evidence)
                || supports_area_overlap_topology(right_winding_evidence) =>
        {
            Some(Kind::AreaOverlap)
        }
        (true, true, true) if has_supporting_island => Some(Kind::AreaOverlap),
        (true, true, true) | (false, false, true) | (true, false, true) | (false, true, true) => {
            Some(Kind::BoundaryContact)
        }
        (true, false, false) | (false, true, false) => None,
        (false, false, false) => None,
    }
}

fn supports_area_overlap_topology(
    evidence_kind: crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapCellWindingEvidenceKind,
) -> bool {
    matches!(
        evidence_kind,
        SupportingIslandTopology | BoundaryTopologyAndSupportingIslandTopology
    )
}

#[cfg(test)]
mod tests {
    use super::classify_candidate_kind;
    use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapCellWindingEvidenceKind::{
        BoundaryTopology, BoundaryTopologyAndSupportingIslandTopology, SupportingIslandTopology,
    };
    use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapIslandCandidateKind::{
        AreaOverlap, BoundaryContact,
    };

    #[test]
    fn inside_both_boundary_only_contact_stays_boundary_contact_without_supporting_topology() {
        let kind =
            classify_candidate_kind(true, true, true, false, BoundaryTopology, BoundaryTopology);

        assert_eq!(kind, Some(BoundaryContact));
    }

    #[test]
    fn inside_both_boundary_contact_with_supporting_topology_stays_area_overlap() {
        let left_supported = classify_candidate_kind(
            true,
            true,
            true,
            true,
            SupportingIslandTopology,
            BoundaryTopology,
        );
        let both_supported = classify_candidate_kind(
            true,
            true,
            true,
            true,
            BoundaryTopologyAndSupportingIslandTopology,
            BoundaryTopologyAndSupportingIslandTopology,
        );

        assert_eq!(left_supported, Some(AreaOverlap));
        assert_eq!(both_supported, Some(AreaOverlap));
    }

    #[test]
    fn inside_both_boundary_contact_with_source_witness_is_area_overlap() {
        let kind =
            classify_candidate_kind(true, true, true, true, BoundaryTopology, BoundaryTopology);

        assert_eq!(kind, Some(AreaOverlap));
    }
}
