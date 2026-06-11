use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticSubject,
    PlanarDiagnosticTriggerLocality, PlanarDiagnosticTruthEffect,
};

use super::contract_subject::{causal_reference, diagnostic_planar_parts, topology_surface};
use super::runtime_handles::diagnostic_handle;
use crate::public_api_planar_motion_posture::contract_subject::cancellation_motion_receipt;

#[test]
fn planar_diagnostics_localize_predicate_topology_binding_policy_and_transform_failures() {
    let direct_cases = [
        (
            PlanarDiagnosticSubject::predicate_failure("predicate:near-graze"),
            PlanarDiagnosticTriggerLocality::PredicateAuthority,
        ),
        (
            PlanarDiagnosticSubject::binding_failure("binding:stale-rebind"),
            PlanarDiagnosticTriggerLocality::BindingOrRebinding,
        ),
        (
            PlanarDiagnosticSubject::policy_required("policy:manual-review"),
            PlanarDiagnosticTriggerLocality::PolicyBoundary,
        ),
        (
            PlanarDiagnosticSubject::unsupported_planar_class("unsupported:open-shell"),
            PlanarDiagnosticTriggerLocality::UnsupportedPlanarClass,
        ),
    ];
    for (subject, expected_locality) in direct_cases {
        let receipt = PlanarDiagnosticBundle::explain_planar_failure(subject)
            .inspect_failure_locality()
            .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
                "planar-diagnostic-direct-locality",
            )))
            .expect("direct diagnostic plan")
            .certify()
            .expect("direct diagnostic receipt");
        assert_eq!(receipt.trigger_locality(), expected_locality);
        assert_eq!(
            receipt.truth_effect(),
            PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth
        );
    }

    let topology = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::topology_failure("topology:loop-gap"),
    )
    .with_topology_declared_surface(topology_surface("planar-diagnostic-topology"))
    .with_query_causal_inspection(causal_reference("planar-diagnostic-topology"))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "planar-diagnostic-topology",
    )))
    .expect("topology diagnostic plan")
    .certify()
    .expect("topology diagnostic receipt");
    assert_eq!(
        topology.trigger_locality(),
        PlanarDiagnosticTriggerLocality::TopologyContract
    );
    assert_eq!(topology.counters().topology_surfaces_inspected(), 1);
    assert_eq!(topology.counters().causal_references_resolved(), 1);

    let parts = diagnostic_planar_parts("planar-diagnostic-projection");
    let projection = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::projection_failure("projection:off-plane"),
    )
    .with_projection_consumed_planar_facts(parts.projected)
    .with_query_causal_inspection(causal_reference("planar-diagnostic-projection"))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "planar-diagnostic-projection",
    )))
    .expect("projection diagnostic plan")
    .certify()
    .expect("projection diagnostic receipt");
    assert_eq!(
        projection.trigger_locality(),
        PlanarDiagnosticTriggerLocality::ProjectionBasis
    );
    assert_eq!(projection.counters().causal_references_resolved(), 1);

    let motion = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::motion_failure("motion:late-rotation"),
    )
    .with_motion_posture(cancellation_motion_receipt("planar-diagnostic-motion"))
    .with_query_causal_inspection(causal_reference("planar-diagnostic-motion"))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "planar-diagnostic-motion",
    )))
    .expect("motion diagnostic plan")
    .certify()
    .expect("motion diagnostic receipt");
    assert_eq!(
        motion.trigger_locality(),
        PlanarDiagnosticTriggerLocality::MotionOrRotationPosture
    );
    assert_eq!(motion.counters().source_receipts_inspected(), 3);
    assert_eq!(motion.counters().causal_references_resolved(), 1);
}

#[test]
fn topology_failure_stays_topology_local_even_with_valid_planar_evidence() {
    let parts = diagnostic_planar_parts("planar-diagnostic-topology-vs-planar");
    let receipt = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::topology_failure("topology:missing-shell"),
    )
    .with_retained_planar_facts(parts.retained)
    .with_projection_consumed_planar_facts(parts.projected)
    .with_topology_declared_surface(topology_surface("planar-diagnostic-topology-vs-planar"))
    .with_query_causal_inspection(causal_reference("planar-diagnostic-topology-vs-planar"))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        "planar-diagnostic-topology-vs-planar",
    )))
    .expect("mixed diagnostic plan")
    .certify()
    .expect("mixed diagnostic receipt");

    assert_eq!(
        receipt.trigger_locality(),
        PlanarDiagnosticTriggerLocality::TopologyContract
    );
    assert_eq!(
        receipt.truth_effect(),
        PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth
    );
}
