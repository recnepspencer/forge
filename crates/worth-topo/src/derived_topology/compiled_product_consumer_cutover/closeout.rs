#[cfg(test)]
use crate::derived_topology::compiled_product_consumer_cutover::residue_manifest::{
    current_topology_consumer_residue_manifest, TopologyConsumerResidueDisposition,
    TopologyConsumerResidueOwner,
};

#[cfg(test)]
pub(crate) fn require_exact_topology_consumer_closeout() {
    let residue = current_topology_consumer_residue_manifest();
    assert_eq!(residue.len(), 2);
    assert_eq!(
        residue[0].source_path(),
        "crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs"
    );
    assert_eq!(
        residue[0].current_surface(),
        "HistoricalEvaluationRequest::retained_snapshot(... HistoricalPathReuseDescriptor::retained_reuse())"
    );
    assert_eq!(residue[0].owner(), TopologyConsumerResidueOwner::ForgeQuery);
    assert_eq!(
        residue[0].disposition(),
        TopologyConsumerResidueDisposition::ExplicitResidue
    );
    assert_eq!(
        residue[1].source_path(),
        "crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs"
    );
    assert_eq!(
        residue[1].current_surface(),
        "HistoricalCapabilityDescriptor::retained_snapshot(... HistoricalPathReuseDescriptor::retained_reuse())"
    );
    assert_eq!(residue[1].owner(), TopologyConsumerResidueOwner::ForgeQuery);
    assert_eq!(
        residue[1].disposition(),
        TopologyConsumerResidueDisposition::QueryGap
    );
    assert!(residue.iter().all(|row| {
        row.disposition() != TopologyConsumerResidueDisposition::AuthoritativeOrdinaryConsumer
    }));
}
