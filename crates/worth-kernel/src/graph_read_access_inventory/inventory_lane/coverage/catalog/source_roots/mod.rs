mod kernel;
mod spatial;
mod test_support;
mod topology;

use super::WorthGraphReadAccessCoveredSource;

pub(crate) const COVERED_GRAPH_READ_SOURCES: &[WorthGraphReadAccessCoveredSource] = &[
    topology::TOPOLOGY_READ_DOMAIN,
    topology::TOPOLOGY_READ_EXECUTION,
    topology::TOPOLOGY_READ_PROOF_SUPPORT,
    spatial::SPATIAL_EVIDENCE_LEDGER,
    spatial::SPATIAL_BOOLEAN_LOOP_RECONSTRUCTION,
    spatial::SPATIAL_BOOLEAN_EVENTS,
    kernel::KERNEL_GRAPH_READ_ADOPTION,
    kernel::KERNEL_WORKLOAD_COMPOSITION,
    kernel::KERNEL_BINDING_ROOT,
    test_support::TOPOLOGY_READ_TEST_SUPPORT,
    test_support::SPATIAL_LOOP_RECONSTRUCTION_TEST_SUPPORT,
    test_support::KERNEL_BINDING_TEST_SUPPORT,
];
