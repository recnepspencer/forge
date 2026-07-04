use super::super::consumer_class::KernelCompiledProductConsumerResponsibility;
use super::super::coverage_target::KernelCompiledProductConsumerCoverageTarget;
use super::super::dependency_row::KernelCompiledProductConsumerClusterIdentity;
use super::super::error::KernelCompiledProductConsumerDependencyError;
use super::super::family_class::KernelCompiledProductFamilyClass;
use super::super::future_cutover_lane::KernelCompiledProductFutureCutoverLane;
use super::super::proof_basis::KernelCompiledProductProofBasis;
use crate::workload_composition::CompiledProductReuseSurfaceIdentity as Surface;

pub(super) fn current_topology_consumer_rows() -> Result<
    Vec<KernelCompiledProductConsumerCoverageTarget>,
    KernelCompiledProductConsumerDependencyError,
> {
    Ok(vec![
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::TopologyDerivedProjectionEquivalence,
            "crates/worth-topo/src/derived_topology/compiled_product_consumer_cutover/topology_derived_cluster/admitted_contract.rs",
            "build_derived_equivalence_contract",
            KernelCompiledProductConsumerResponsibility::TopologyDerived,
            KernelCompiledProductFamilyClass::TopologyDerivedEquivalenceContract,
            KernelCompiledProductFutureCutoverLane::TopologyDerivedConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "historical read-basis authority plus topology compiled-product family admission",
                "materialized, interpreted, and validation topology surfaces",
                "selected topology equivalence family plus compiled-product lowering",
                "typed derived-equivalence report and topology reuse-decision inputs",
                "topology.selected-equivalence.derived-semantic-parity",
            ),
            None,
            "ordinary topology projection consumers must route through the shared compiled-product lane instead of local digest comparison helpers",
            &[
                Surface::BuildDerivedEquivalenceContract,
                Surface::BuildDerivedEquivalenceContractReport,
                Surface::CompareDerivedEquivalenceContracts,
            ],
        ),
        KernelCompiledProductConsumerCoverageTarget::new(
            KernelCompiledProductConsumerClusterIdentity::TopologyDerivedInvalidationDisposition,
            "crates/worth-topo/src/derived_topology/compiled_product_consumer_cutover/topology_derived_cluster/reuse_decision_contract.rs",
            "topology_cutover_planned_disposition_from_update_posture",
            KernelCompiledProductConsumerResponsibility::TopologyDerived,
            KernelCompiledProductFamilyClass::TopologyDerivedInvalidationDisposition,
            KernelCompiledProductFutureCutoverLane::TopologyDerivedConsumerCutover,
            KernelCompiledProductProofBasis::new(
                "derived-topology family catalog update posture authority",
                "selected invalidation row footprint",
                "topology family selection plus shared planned disposition lowering",
                "typed invalidation row planned disposition rather than local rebuild suppression",
                "topology.selected-equivalence.derived-semantic-parity",
            ),
            None,
            "ordinary topology invalidation consumers must stop translating bounded rebuild posture locally once the shared topology lane exists",
            &[Surface::DerivedInvalidationPlannedDispositionFromUpdatePosture],
        ),
    ])
}
