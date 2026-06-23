use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanFragmentMembershipRow {
    membership_identity: String,
    fragment_identity: String,
    carrier_identity: String,
    source_loop_carrier_identity: String,
    recovered_carrier_identity: String,
    source_face_identity: String,
    source_loop_identity: String,
    source_edge_identity: String,
    local_frame_identity: String,
    precision_basis_identity: String,
    source_senses: Vec<PlanarBooleanSourceIntervalSense>,
}

impl PlanarBooleanFragmentMembershipRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        membership_identity: String,
        fragment_identity: String,
        carrier_identity: String,
        source_loop_carrier_identity: String,
        recovered_carrier_identity: String,
        source_face_identity: String,
        source_loop_identity: String,
        source_edge_identity: String,
        local_frame_identity: String,
        precision_basis_identity: String,
        source_senses: Vec<PlanarBooleanSourceIntervalSense>,
    ) -> Self {
        Self {
            membership_identity,
            fragment_identity,
            carrier_identity,
            source_loop_carrier_identity,
            recovered_carrier_identity,
            source_face_identity,
            source_loop_identity,
            source_edge_identity,
            local_frame_identity,
            precision_basis_identity,
            source_senses,
        }
    }

    pub fn membership_identity(&self) -> &str {
        &self.membership_identity
    }

    pub fn fragment_identity(&self) -> &str {
        &self.fragment_identity
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn source_loop_carrier_identity(&self) -> &str {
        &self.source_loop_carrier_identity
    }

    pub fn recovered_carrier_identity(&self) -> &str {
        &self.recovered_carrier_identity
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

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn source_senses(&self) -> &[PlanarBooleanSourceIntervalSense] {
        &self.source_senses
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanFragmentMembershipMap {
    membership_map_identity: String,
    request_identity: String,
    fragment_set_identity: String,
    rows: Vec<PlanarBooleanFragmentMembershipRow>,
    fragment_offsets: BTreeMap<String, usize>,
}

impl PlanarBooleanFragmentMembershipMap {
    pub(crate) fn new(
        membership_map_identity: String,
        request_identity: String,
        fragment_set_identity: String,
        rows: Vec<PlanarBooleanFragmentMembershipRow>,
    ) -> Self {
        let fragment_offsets = rows
            .iter()
            .enumerate()
            .map(|(offset, row)| (row.fragment_identity().to_string(), offset))
            .collect();
        Self {
            membership_map_identity,
            request_identity,
            fragment_set_identity,
            rows,
            fragment_offsets,
        }
    }

    pub fn membership_map_identity(&self) -> &str {
        &self.membership_map_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn fragment_set_identity(&self) -> &str {
        &self.fragment_set_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanFragmentMembershipRow] {
        &self.rows
    }

    pub fn membership_for_fragment_identity(
        &self,
        fragment_identity: &str,
    ) -> Option<&PlanarBooleanFragmentMembershipRow> {
        self.fragment_offsets
            .get(fragment_identity)
            .and_then(|offset| self.rows.get(*offset))
    }
}
