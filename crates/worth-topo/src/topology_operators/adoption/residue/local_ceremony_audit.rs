use forge_query::facade::consumer_kit::{
    ForgeQueryBoundaryAuditSourceSet, ForgeQueryGraphObligationLocalCeremonyAudit,
};

use super::audit_sources::topology_operator_phase_seventeen_audit_sources;

pub fn topology_operator_graph_obligation_local_ceremony_audit(
) -> ForgeQueryGraphObligationLocalCeremonyAudit {
    ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
        &topology_operator_graph_obligation_audit_sources(),
    )
}

pub fn topology_operator_graph_obligation_audit_sources() -> ForgeQueryBoundaryAuditSourceSet {
    let mut sources = ForgeQueryBoundaryAuditSourceSet::new("worth-topo");
    for (label, path, source) in topology_operator_phase_seventeen_audit_sources() {
        sources = sources.source_file(label, path, source);
    }
    sources
}
