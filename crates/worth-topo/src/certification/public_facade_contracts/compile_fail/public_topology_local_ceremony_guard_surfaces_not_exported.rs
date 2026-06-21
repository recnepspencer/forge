use topology::facade::{
    topology_operator_graph_obligation_audit_sources,
    topology_operator_graph_obligation_local_ceremony_audit,
    topology_operator_legacy_guard_audit, topology_operator_local_guard_residue_inventory,
    topology_operator_local_guard_residue_total, TopologyOperatorLegacyGuardAudit,
    TopologyOperatorLegacyGuardAuditRow, TopologyOperatorLocalGuardResidueClass,
    TopologyOperatorLocalGuardResidueRow,
};

fn main() {
    let _ = topology_operator_graph_obligation_local_ceremony_audit();
    let _ = topology_operator_graph_obligation_audit_sources();
    let _ = topology_operator_legacy_guard_audit();
    let _ = topology_operator_local_guard_residue_inventory();
    let _ = topology_operator_local_guard_residue_total();
    let _: Option<TopologyOperatorLegacyGuardAudit> = None;
    let _: Option<TopologyOperatorLegacyGuardAuditRow> = None;
    let _: Option<TopologyOperatorLocalGuardResidueClass> = None;
    let _: Option<TopologyOperatorLocalGuardResidueRow> = None;
}
