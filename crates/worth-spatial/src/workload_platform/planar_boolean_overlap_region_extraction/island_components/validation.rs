use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::{
    Left, Right,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapCellContainmentEvidenceKind::Inside;

use super::counters::PlanarBooleanOverlapIslandComponentCounters;
use super::denial::{
    PlanarBooleanOverlapIslandComponentDenial, PlanarBooleanOverlapIslandComponentDenialKind as Kind,
};
use super::input::PlanarBooleanOverlapIslandCandidateInput;
use super::lookup::PlanarBooleanOverlapCellEvidenceLookup;

pub(super) fn validate_input_identities(
    input: PlanarBooleanOverlapIslandCandidateInput<'_>,
    counters: &mut PlanarBooleanOverlapIslandComponentCounters,
) -> Result<(), PlanarBooleanOverlapIslandComponentDenial> {
    let arrangement = input.arrangement_graph();
    let containment = input.containment_map();
    let winding = input.winding_field();
    if arrangement.request_identity() != containment.request_identity()
        || arrangement.request_identity() != winding.request_identity()
        || arrangement.arrangement_graph_identity() != containment.arrangement_graph_identity()
        || arrangement.arrangement_graph_identity() != winding.arrangement_graph_identity()
        || arrangement.cell_set().cell_set_identity() != containment.cell_set_identity()
        || arrangement.cell_set().cell_set_identity() != winding.cell_set_identity()
        || arrangement.ordering_basis_identity() != containment.ordering_basis_identity()
        || arrangement.ordering_basis_identity() != winding.ordering_basis_identity()
    {
        counters.denied_partition();
        return Err(PlanarBooleanOverlapIslandComponentDenial::new(
            Kind::InputIdentityMismatchDenied,
            arrangement.arrangement_graph_identity(),
            *counters,
            "overlap island extraction denies arrangement, containment, or winding products that do not share one admitted identity basis",
        ));
    }
    Ok(())
}

pub(super) fn validate_cell_overlap_signal(
    cell_identity: &str,
    lookup: &PlanarBooleanOverlapCellEvidenceLookup<'_>,
    counters: &mut PlanarBooleanOverlapIslandComponentCounters,
) -> Result<(), PlanarBooleanOverlapIslandComponentDenial> {
    let left_containment = lookup
        .containment_row(cell_identity, Left)
        .ok_or_else(|| missing_containment(cell_identity, counters))?;
    let right_containment = lookup
        .containment_row(cell_identity, Right)
        .ok_or_else(|| missing_containment(cell_identity, counters))?;
    let left_winding = lookup
        .winding_row(cell_identity, Left)
        .ok_or_else(|| missing_winding(cell_identity, counters))?;
    let right_winding = lookup
        .winding_row(cell_identity, Right)
        .ok_or_else(|| missing_winding(cell_identity, counters))?;

    if left_containment.evidence_kind() == Inside && left_winding.winding_number() == 0 {
        return Err(unsupported(cell_identity, counters));
    }
    if right_containment.evidence_kind() == Inside && right_winding.winding_number() == 0 {
        return Err(unsupported(cell_identity, counters));
    }
    Ok(())
}

fn missing_containment(
    cell_identity: &str,
    counters: &mut PlanarBooleanOverlapIslandComponentCounters,
) -> PlanarBooleanOverlapIslandComponentDenial {
    counters.denied_partition();
    PlanarBooleanOverlapIslandComponentDenial::new(
        Kind::MissingCellContainmentEvidenceDenied,
        cell_identity,
        *counters,
        "overlap island extraction denies cell partition without explicit operand containment evidence",
    )
}

fn missing_winding(
    cell_identity: &str,
    counters: &mut PlanarBooleanOverlapIslandComponentCounters,
) -> PlanarBooleanOverlapIslandComponentDenial {
    counters.denied_partition();
    PlanarBooleanOverlapIslandComponentDenial::new(
        Kind::MissingCellWindingEvidenceDenied,
        cell_identity,
        *counters,
        "overlap island extraction denies cell partition without explicit operand-local winding evidence",
    )
}

pub(super) fn unsupported(
    cell_identity: &str,
    counters: &mut PlanarBooleanOverlapIslandComponentCounters,
) -> PlanarBooleanOverlapIslandComponentDenial {
    counters.denied_partition();
    PlanarBooleanOverlapIslandComponentDenial::new(
        Kind::UnsupportedCellOverlapSignalDenied,
        cell_identity,
        *counters,
        "overlap island extraction denies cells that are neither inside-both area overlap nor explicit boundary contact",
    )
}

pub(super) fn contradictory(
    rejected_identity: &str,
    counters: &mut PlanarBooleanOverlapIslandComponentCounters,
) -> PlanarBooleanOverlapIslandComponentDenial {
    counters.denied_partition();
    PlanarBooleanOverlapIslandComponentDenial::new(
        Kind::ContradictoryComponentMembershipDenied,
        rejected_identity,
        *counters,
        "overlap island extraction denies a cell or component basis that would force contradictory typed component membership",
    )
}

pub(super) fn mixed_partition(
    island_identity: &str,
    counters: &mut PlanarBooleanOverlapIslandComponentCounters,
) -> PlanarBooleanOverlapIslandComponentDenial {
    counters.denied_partition();
    PlanarBooleanOverlapIslandComponentDenial::new(
        Kind::MixedIslandPartitionDenied,
        island_identity,
        *counters,
        "overlap island extraction denies mixed islands whose contact structure cannot yet be partitioned honestly into boundary-contact and area-overlap components",
    )
}
