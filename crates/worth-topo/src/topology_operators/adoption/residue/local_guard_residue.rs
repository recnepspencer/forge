#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TopologyOperatorLocalGuardResidueClass {
    ExistingEntityIncomingRelationCountMismatch,
}

impl TopologyOperatorLocalGuardResidueClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExistingEntityIncomingRelationCountMismatch => {
                "existing-entity-incoming-relation-count-mismatch"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyOperatorLocalGuardResidueRow {
    class: TopologyOperatorLocalGuardResidueClass,
    source_label: &'static str,
    source_path: &'static str,
    occurrence_count: usize,
}

impl TopologyOperatorLocalGuardResidueRow {
    const fn new(
        source_label: &'static str,
        source_path: &'static str,
        occurrence_count: usize,
    ) -> Self {
        Self {
            class:
                TopologyOperatorLocalGuardResidueClass::ExistingEntityIncomingRelationCountMismatch,
            source_label,
            source_path,
            occurrence_count,
        }
    }

    pub fn class(&self) -> TopologyOperatorLocalGuardResidueClass {
        self.class
    }

    pub fn source_label(&self) -> &'static str {
        self.source_label
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn occurrence_count(&self) -> usize {
        self.occurrence_count
    }
}

pub fn topology_operator_local_guard_residue_inventory() -> Vec<TopologyOperatorLocalGuardResidueRow>
{
    vec![
        TopologyOperatorLocalGuardResidueRow::new(
            "boundary-wiring.adjacency-support",
            "crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/adjacency_support.rs",
            1,
        ),
        TopologyOperatorLocalGuardResidueRow::new(
            "sheet-wire-laminar.wire-membership-program",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/wire_membership_program.rs",
            2,
        ),
        TopologyOperatorLocalGuardResidueRow::new(
            "sheet-wire-laminar.shell-membership-program",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/shell_membership_program.rs",
            2,
        ),
        TopologyOperatorLocalGuardResidueRow::new(
            "sheet-wire-laminar.shell-split-program",
            "crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/shell_split_program.rs",
            1,
        ),
    ]
}

pub fn topology_operator_local_guard_residue_total() -> usize {
    topology_operator_local_guard_residue_inventory()
        .iter()
        .map(TopologyOperatorLocalGuardResidueRow::occurrence_count)
        .sum()
}
