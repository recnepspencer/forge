pub const TOPOLOGY_OPERATOR_INCOMING_RELATION_COUNT_GUARD_PATTERN: &str =
    "ExistingEntityIncomingRelationCountMismatch";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyOperatorLegacyGuardAudit {
    rows: Vec<TopologyOperatorLegacyGuardAuditRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyOperatorLegacyGuardAuditRow {
    source_label: &'static str,
    source_path: &'static str,
    pattern: &'static str,
    occurrence_count: usize,
}

impl TopologyOperatorLegacyGuardAudit {
    pub(crate) fn evaluate_sources(
        sources: impl IntoIterator<Item = (&'static str, &'static str, &'static str)>,
    ) -> Self {
        let rows = sources
            .into_iter()
            .filter_map(|(source_label, source_path, source)| {
                let occurrence_count = source
                    .matches(TOPOLOGY_OPERATOR_INCOMING_RELATION_COUNT_GUARD_PATTERN)
                    .count();
                (occurrence_count > 0).then_some(TopologyOperatorLegacyGuardAuditRow {
                    source_label,
                    source_path,
                    pattern: TOPOLOGY_OPERATOR_INCOMING_RELATION_COUNT_GUARD_PATTERN,
                    occurrence_count,
                })
            })
            .collect();
        Self { rows }
    }

    pub fn rows(&self) -> &[TopologyOperatorLegacyGuardAuditRow] {
        &self.rows
    }

    pub fn total_occurrence_count(&self) -> usize {
        self.rows.iter().map(|row| row.occurrence_count).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

impl TopologyOperatorLegacyGuardAuditRow {
    pub fn source_label(&self) -> &'static str {
        self.source_label
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn pattern(&self) -> &'static str {
        self.pattern
    }

    pub fn occurrence_count(&self) -> usize {
        self.occurrence_count
    }
}

pub fn topology_operator_legacy_guard_audit() -> TopologyOperatorLegacyGuardAudit {
    TopologyOperatorLegacyGuardAudit::evaluate_sources(topology_operator_legacy_guard_sources())
}

fn topology_operator_legacy_guard_sources(
) -> impl IntoIterator<Item = (&'static str, &'static str, &'static str)> {
    [
        (
            "boundary-wiring.adjacency-support",
            "crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/adjacency_support.rs",
            include_str!("../../local_rewrites/boundary_wiring/adjacency_support.rs"),
        ),
        (
            "boundary-wiring.composed-successor-program",
            "crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/composed_successor_program.rs",
            include_str!("../../local_rewrites/boundary_wiring/composed_successor_program.rs"),
        ),
        (
            "boundary-wiring.membership",
            "crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/membership.rs",
            include_str!("../../local_rewrites/boundary_wiring/membership.rs"),
        ),
        (
            "boundary-wiring.relation-update",
            "crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/relation_update.rs",
            include_str!("../../local_rewrites/boundary_wiring/relation_update.rs"),
        ),
        (
            "boundary-wiring.successor-admission",
            "crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/successor_admission.rs",
            include_str!("../../local_rewrites/boundary_wiring/successor_admission.rs"),
        ),
        (
            "sheet-wire-laminar.membership-admission",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_admission.rs",
            include_str!("../../local_rewrites/sheet_wire_laminar/membership_admission.rs"),
        ),
        (
            "sheet-wire-laminar.shell-face-rehome-support",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/shell_face_rehome_support.rs",
            include_str!("../../local_rewrites/sheet_wire_laminar/shell_face_rehome_support.rs"),
        ),
        (
            "sheet-wire-laminar.wire-rehome-support",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/wire_rehome_support.rs",
            include_str!("../../local_rewrites/sheet_wire_laminar/wire_rehome_support.rs"),
        ),
        (
            "sheet-wire-laminar.face-inner-loop-program",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/face_inner_loop_program.rs",
            include_str!("../../local_rewrites/sheet_wire_laminar/membership_programs/face_inner_loop_program.rs"),
        ),
        (
            "sheet-wire-laminar.membership-programs.shared",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/shared.rs",
            include_str!("../../local_rewrites/sheet_wire_laminar/membership_programs/shared.rs"),
        ),
        (
            "sheet-wire-laminar.shell-membership-program",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/shell_membership_program.rs",
            include_str!("../../local_rewrites/sheet_wire_laminar/membership_programs/shell_membership_program.rs"),
        ),
        (
            "sheet-wire-laminar.shell-split-program",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/shell_split_program.rs",
            include_str!("../../local_rewrites/sheet_wire_laminar/membership_programs/shell_split_program.rs"),
        ),
        (
            "sheet-wire-laminar.wire-membership-program",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/wire_membership_program.rs",
            include_str!("../../local_rewrites/sheet_wire_laminar/membership_programs/wire_membership_program.rs"),
        ),
    ]
}
