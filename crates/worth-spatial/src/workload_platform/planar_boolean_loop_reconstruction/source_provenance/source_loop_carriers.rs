use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_events::PlanarBooleanLoopRole;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopSourceCarrierRow {
    source_loop_carrier_identity: String,
    recovered_carrier_identity: String,
    carrier_identity: String,
    source_face_identity: String,
    source_loop_identity: String,
    source_edge_identity: String,
    loop_role: PlanarBooleanLoopRole,
}

impl PlanarBooleanLoopSourceCarrierRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        source_loop_carrier_identity: String,
        recovered_carrier_identity: String,
        carrier_identity: String,
        source_face_identity: String,
        source_loop_identity: String,
        source_edge_identity: String,
        loop_role: PlanarBooleanLoopRole,
    ) -> Self {
        Self {
            source_loop_carrier_identity,
            recovered_carrier_identity,
            carrier_identity,
            source_face_identity,
            source_loop_identity,
            source_edge_identity,
            loop_role,
        }
    }

    pub fn source_loop_carrier_identity(&self) -> &str {
        &self.source_loop_carrier_identity
    }

    pub fn recovered_carrier_identity(&self) -> &str {
        &self.recovered_carrier_identity
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn source_face_identity(&self) -> &str {
        &self.source_face_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn loop_role(&self) -> PlanarBooleanLoopRole {
        self.loop_role
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopSourceCarrierSet {
    carrier_set_identity: String,
    request_identity: String,
    split_ledger_receipt_identity: String,
    rows: Vec<PlanarBooleanLoopSourceCarrierRow>,
    carrier_offsets: BTreeMap<String, usize>,
}

impl PlanarBooleanLoopSourceCarrierSet {
    pub(crate) fn new(
        carrier_set_identity: String,
        request_identity: String,
        split_ledger_receipt_identity: String,
        rows: Vec<PlanarBooleanLoopSourceCarrierRow>,
    ) -> Self {
        let carrier_offsets = rows
            .iter()
            .enumerate()
            .map(|(offset, row)| (row.carrier_identity().to_string(), offset))
            .collect();
        Self {
            carrier_set_identity,
            request_identity,
            split_ledger_receipt_identity,
            rows,
            carrier_offsets,
        }
    }

    pub fn carrier_set_identity(&self) -> &str {
        &self.carrier_set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn split_ledger_receipt_identity(&self) -> &str {
        &self.split_ledger_receipt_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopSourceCarrierRow] {
        &self.rows
    }

    pub fn carrier_for_identity(
        &self,
        carrier_identity: &str,
    ) -> Option<&PlanarBooleanLoopSourceCarrierRow> {
        self.carrier_offsets
            .get(carrier_identity)
            .and_then(|offset| self.rows.get(*offset))
    }
}
