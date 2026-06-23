use std::collections::{BTreeMap, BTreeSet};

use crate::construction::tests::support::compound_lane_support::{
    compound_authoring_order_parity_verified, compound_canonical_rows,
    compound_scenario_stable_across_orders,
};
use crate::construction::tests::support::compound_row_support::{
    exhaustion_reason, grazing_digest, grazing_kind, motion_digest, motion_kind, row_digest,
};
use crate::construction::tests::support::compound_runtime::{
    compound_parity_registry, exhaustion_witness_kind_for,
    PrimitiveConstructionCompoundAdversarialLanes,
    PrimitiveConstructionCompoundAdversarialSiegeError,
    PrimitiveConstructionCompoundExhaustionWitnessParityRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundMotionParityRow,
    PrimitiveConstructionCompoundRow,
};
use crate::construction::tests::support::compound_specialized_rows::{
    derive_specialized_rows, require_specialized_row_field,
};
use crate::construction::tests::support::evidence_reports::sealed_report_identity;
use crate::construction::tests::support::realization::prepare_primitive_construction_realization_exhaustion_witness_report;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCompoundExhaustionInventoryCoverage {
    pub inventory_matches: bool,
    pub siege_row_digest_uniqueness_verified: bool,
    pub witness_row_digest_uniqueness_verified: bool,
}

pub(crate) fn build_motion_parity_rows_from_siege(
    siege: &PrimitiveConstructionCompoundAdversarialLanes,
) -> Result<
    Vec<PrimitiveConstructionCompoundMotionParityRow>,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    derive_specialized_rows(
        compound_canonical_rows(siege).iter(),
        |row| motion_kind(row).is_some() || motion_digest(row).is_some(),
        |row: &PrimitiveConstructionCompoundRow| {
            let motion_kind = require_specialized_row_field(
                row.scenario_id(),
                "motion kind",
                motion_kind(row),
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow,
            )?;
            let motion_digest = require_specialized_row_field(
                row.scenario_id(),
                "motion digest",
                motion_digest(row),
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow,
            )?;
            Ok(PrimitiveConstructionCompoundMotionParityRow::new(
                row.scenario_id().to_string(),
                motion_kind,
                motion_digest,
            ))
        },
    )
}

pub(crate) fn build_grazing_boundary_rows_from_siege(
    siege: &PrimitiveConstructionCompoundAdversarialLanes,
) -> Result<
    Vec<PrimitiveConstructionCompoundGrazingBoundaryRow>,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    derive_specialized_rows(
        compound_canonical_rows(siege).iter(),
        |row| grazing_kind(row).is_some() || grazing_digest(row).is_some(),
        |row: &PrimitiveConstructionCompoundRow| {
            let grazing_kind = require_specialized_row_field(
                row.scenario_id(),
                "grazing kind",
                grazing_kind(row),
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow,
            )?;
            let grazing_digest = require_specialized_row_field(
                row.scenario_id(),
                "grazing digest",
                grazing_digest(row),
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow,
            )?;
            Ok(PrimitiveConstructionCompoundGrazingBoundaryRow::new(
                row.scenario_id().to_string(),
                grazing_kind,
                grazing_digest,
            ))
        },
    )
}

pub(crate) fn build_exhaustion_witness_parity_rows_from_siege(
    siege: &PrimitiveConstructionCompoundAdversarialLanes,
) -> Result<
    Vec<PrimitiveConstructionCompoundExhaustionWitnessParityRow>,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let witness_report = prepare_primitive_construction_realization_exhaustion_witness_report();
    siege
        .iter()
        .find(|(lane, _)| lane.as_str() == "canonical")
        .map(|(_, rows)| rows.iter())
        .into_iter()
        .flatten()
        .filter_map(|row| exhaustion_witness_kind_for(row.scenario_id()).map(|kind| (row, kind)))
        .map(|(row, witness_kind)| {
            let witness_row = witness_report.row_for(witness_kind).ok_or_else(|| {
                PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow(format!(
                    "compound exhaustion row '{}' is missing lower-layer witness row",
                    row.scenario_id()
                ))
            })?;
            if witness_row.exhaustion_reason()
                != exhaustion_reason(row).ok_or_else(|| {
                    PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow(
                        format!(
                            "compound exhaustion row '{}' is missing exhaustion reason",
                            row.scenario_id()
                        ),
                    )
                })?
            {
                return Err(
                    PrimitiveConstructionCompoundAdversarialSiegeError::InvalidSpecializedRow(
                        format!(
                            "compound exhaustion row '{}' drifted from lower-layer witness truth",
                            row.scenario_id()
                        ),
                    ),
                );
            }
            Ok(
                PrimitiveConstructionCompoundExhaustionWitnessParityRow::new(
                    row.scenario_id().to_string(),
                    witness_kind,
                    row_digest(row),
                    witness_row.row_digest().to_string(),
                ),
            )
        })
        .collect()
}

pub(crate) fn exact_motion_inventory_matches(
    rows: &[PrimitiveConstructionCompoundMotionParityRow],
) -> bool {
    let registry = compound_parity_registry();
    let actual = rows
        .iter()
        .map(|row| (row.scenario_id().to_string(), row.motion_kind()))
        .collect::<BTreeMap<_, _>>();
    actual.len() == rows.len()
        && actual == *registry.motion_inventory()
        && rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<BTreeSet<_>>()
            .len()
            == rows.len()
}

pub(crate) fn motion_parity_verified(
    siege: &PrimitiveConstructionCompoundAdversarialLanes,
    rows: &[PrimitiveConstructionCompoundMotionParityRow],
) -> bool {
    compound_authoring_order_parity_verified(siege)
        && exact_motion_inventory_matches(rows)
        && rows.iter().all(|row| {
            compound_scenario_stable_across_orders(siege, row.scenario_id())
                && compound_parity_registry()
                    .motion_inventory()
                    .get(row.scenario_id())
                    .is_some_and(|kind| row.motion_kind() == *kind)
        })
}

pub(crate) fn motion_report_digest(
    siege: &PrimitiveConstructionCompoundAdversarialLanes,
    rows: &[PrimitiveConstructionCompoundMotionParityRow],
) -> String {
    sealed_report_identity(
        "worth-kernel.construction.compound-parity",
        "motion-parity-report",
        |report| {
            report
                .value_sequence_participating(
                    "row-identities",
                    rows.iter().map(|row| row.row_digest().to_string()),
                )?
                .bool_participating("parity-verified", motion_parity_verified(siege, rows))
        },
    )
}

pub(crate) fn exact_grazing_inventory_matches(
    rows: &[PrimitiveConstructionCompoundGrazingBoundaryRow],
) -> bool {
    let registry = compound_parity_registry();
    let actual = rows
        .iter()
        .map(|row| (row.scenario_id().to_string(), row.grazing_kind()))
        .collect::<BTreeMap<_, _>>();
    actual.len() == rows.len()
        && actual == *registry.grazing_inventory()
        && rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<BTreeSet<_>>()
            .len()
            == rows.len()
}

pub(crate) fn grazing_parity_verified(
    siege: &PrimitiveConstructionCompoundAdversarialLanes,
    rows: &[PrimitiveConstructionCompoundGrazingBoundaryRow],
) -> bool {
    compound_authoring_order_parity_verified(siege)
        && exact_grazing_inventory_matches(rows)
        && rows.iter().all(|row| {
            compound_scenario_stable_across_orders(siege, row.scenario_id())
                && compound_parity_registry()
                    .grazing_inventory()
                    .get(row.scenario_id())
                    .is_some_and(|kind| row.grazing_kind() == *kind)
        })
}

pub(crate) fn grazing_report_digest(
    siege: &PrimitiveConstructionCompoundAdversarialLanes,
    rows: &[PrimitiveConstructionCompoundGrazingBoundaryRow],
) -> String {
    sealed_report_identity(
        "worth-kernel.construction.compound-parity",
        "grazing-parity-report",
        |report| {
            report
                .value_sequence_participating(
                    "row-identities",
                    rows.iter().map(|row| row.row_digest().to_string()),
                )?
                .bool_participating("parity-verified", grazing_parity_verified(siege, rows))
        },
    )
}

pub(crate) fn exact_exhaustion_inventory_matches(
    rows: &[PrimitiveConstructionCompoundExhaustionWitnessParityRow],
) -> PrimitiveConstructionCompoundExhaustionInventoryCoverage {
    let registry = compound_parity_registry();
    let actual = rows
        .iter()
        .map(|row| (row.scenario_id().to_string(), row.witness_kind()))
        .collect::<BTreeMap<_, _>>();
    let inventory_matches =
        actual.len() == rows.len() && actual == *registry.exhaustion_inventory();
    let siege_row_digest_uniqueness_verified = rows
        .iter()
        .map(|row| row.siege_row_digest().to_string())
        .collect::<BTreeSet<_>>()
        .len()
        == rows.len();
    let witness_row_digest_uniqueness_verified = rows
        .iter()
        .map(|row| row.witness_row_digest().to_string())
        .collect::<BTreeSet<_>>()
        .len()
        == rows.len();
    PrimitiveConstructionCompoundExhaustionInventoryCoverage {
        inventory_matches,
        siege_row_digest_uniqueness_verified,
        witness_row_digest_uniqueness_verified,
    }
}

pub(crate) fn exhaustion_parity_verified(
    siege: &PrimitiveConstructionCompoundAdversarialLanes,
    rows: &[PrimitiveConstructionCompoundExhaustionWitnessParityRow],
) -> bool {
    let coverage = exact_exhaustion_inventory_matches(rows);
    compound_authoring_order_parity_verified(siege)
        && coverage.inventory_matches
        && coverage.siege_row_digest_uniqueness_verified
        && coverage.witness_row_digest_uniqueness_verified
        && rows
            .iter()
            .all(|row| compound_scenario_stable_across_orders(siege, row.scenario_id()))
}

pub(crate) fn exhaustion_report_digest(
    siege: &PrimitiveConstructionCompoundAdversarialLanes,
    rows: &[PrimitiveConstructionCompoundExhaustionWitnessParityRow],
) -> String {
    sealed_report_identity(
        "worth-kernel.construction.compound-parity",
        "exhaustion-parity-report",
        |report| {
            report
                .value_sequence_participating(
                    "row-identities",
                    rows.iter().map(|row| row.row_digest().to_string()),
                )?
                .bool_participating("parity-verified", exhaustion_parity_verified(siege, rows))
        },
    )
}
