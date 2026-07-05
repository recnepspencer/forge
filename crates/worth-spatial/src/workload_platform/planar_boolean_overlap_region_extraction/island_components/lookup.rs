use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapCellContainmentRow, PlanarBooleanOverlapCellWindingRow,
};

#[derive(Default)]
pub(crate) struct PlanarBooleanOverlapCellEvidenceLookup<'a> {
    containment_rows: BTreeMap<String, &'a PlanarBooleanOverlapCellContainmentRow>,
    winding_rows: BTreeMap<String, &'a PlanarBooleanOverlapCellWindingRow>,
}

impl<'a> PlanarBooleanOverlapCellEvidenceLookup<'a> {
    pub(crate) fn with_containment_rows(
        mut self,
        rows: &'a [PlanarBooleanOverlapCellContainmentRow],
    ) -> Self {
        for row in rows {
            self.containment_rows
                .insert(evidence_key(row.cell_identity(), row.operand_side()), row);
        }
        self
    }

    pub(crate) fn with_winding_rows(
        mut self,
        rows: &'a [PlanarBooleanOverlapCellWindingRow],
    ) -> Self {
        for row in rows {
            self.winding_rows
                .insert(evidence_key(row.cell_identity(), row.operand_side()), row);
        }
        self
    }

    pub(crate) fn containment_row(
        &self,
        cell_identity: &'a str,
        operand_side: PlanarBooleanCommonPlaneOperandSide,
    ) -> Option<&'a PlanarBooleanOverlapCellContainmentRow> {
        self.containment_rows
            .get(&evidence_key(cell_identity, operand_side))
            .copied()
    }

    pub(crate) fn winding_row(
        &self,
        cell_identity: &'a str,
        operand_side: PlanarBooleanCommonPlaneOperandSide,
    ) -> Option<&'a PlanarBooleanOverlapCellWindingRow> {
        self.winding_rows
            .get(&evidence_key(cell_identity, operand_side))
            .copied()
    }
}

fn evidence_key(
    cell_identity: &str,
    operand_side: PlanarBooleanCommonPlaneOperandSide,
) -> String {
    format!("{cell_identity}:{operand_side:?}")
}
