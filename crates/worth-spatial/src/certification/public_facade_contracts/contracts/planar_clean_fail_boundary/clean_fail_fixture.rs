use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarCleanFailBoundary, PlanarCleanFailBoundaryContracts, PlanarCleanFailBoundaryReceipt,
    PlanarCleanFailInput, PlanarDirtyInputKind, PlanarOpenInputKind,
};
use worth_spatial::facade::planar_contracts::{
    planar_admission_matrix, PlanarAdmissionFamily, PlanarRuntimeConcern,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleReceipt,
    PlanarDiagnosticSubject, PlanarDiagnosticTriggerLocality,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoveryPostureReceipt,
    PlanarRecoverySource,
};

use crate::public_api_planar_diagnostics::contract_subject::{causal_reference, topology_surface};
use crate::public_api_planar_motion_posture::contract_subject::cancellation_motion_receipt;

use super::runtime_handles::{clean_fail_handle, diagnostic_handle, recovery_handle};

pub(crate) fn certify_clean_fail_boundary(
    world: &'static str,
    input: PlanarCleanFailInput,
    recovery: PlanarRecoveryPostureReceipt,
    diagnostics: PlanarDiagnosticBundleReceipt,
) -> PlanarCleanFailBoundaryReceipt {
    PlanarCleanFailBoundary::from_planar_input(input)
        .recovery_posture(recovery)
        .diagnostics(diagnostics)
        .certify_clean_fail_boundary()
        .compile(&PlanarCleanFailBoundaryContracts::new(clean_fail_handle(
            world,
        )))
        .expect("clean-fail boundary plan")
        .certify()
        .expect("clean-fail boundary receipt")
}

pub(crate) fn dirty_input(world: &'static str, source: &'static str) -> PlanarCleanFailInput {
    dirty_input_with_kind(world, source, PlanarDirtyInputKind::SelfIntersectingLoop)
}

pub(crate) fn dirty_input_with_kind(
    world: &'static str,
    source: &'static str,
    kind: PlanarDirtyInputKind,
) -> PlanarCleanFailInput {
    let input = match kind {
        PlanarDirtyInputKind::SelfIntersectingLoop => {
            PlanarCleanFailInput::dirty_planar_loop(source)
        }
        PlanarDirtyInputKind::NonManifoldWire => PlanarCleanFailInput::non_manifold_wire(source),
        PlanarDirtyInputKind::ThinWall => PlanarCleanFailInput::thin_wall(source),
        PlanarDirtyInputKind::OrientationInconsistency => {
            PlanarCleanFailInput::orientation_inconsistency(source)
        }
    };
    input
        .with_topology_identity("stable-dirty-topology-id")
        .with_transform_posture(cancellation_motion_receipt(world))
        .with_admission_row(admission_row(
            PlanarAdmissionFamily::DirtyPlanarInput,
            PlanarRuntimeConcern::DiagnosticsLocalization,
        ))
}

pub(crate) fn unbounded_input(world: &'static str, source: &'static str) -> PlanarCleanFailInput {
    open_input_with_kind(world, source, PlanarOpenInputKind::HalfSpaceGroup)
}

pub(crate) fn open_input_with_kind(
    world: &'static str,
    source: &'static str,
    kind: PlanarOpenInputKind,
) -> PlanarCleanFailInput {
    let input = match kind {
        PlanarOpenInputKind::HalfSpaceGroup => PlanarCleanFailInput::unbounded_half_space(source),
        PlanarOpenInputKind::OpenPlanarDomain => PlanarCleanFailInput::open_planar_domain(source),
    };
    input
        .with_topology_identity("stable-unbounded-topology-id")
        .with_transform_posture(cancellation_motion_receipt(world))
        .with_admission_row(admission_row(
            PlanarAdmissionFamily::UnboundedPlanarDomain,
            PlanarRuntimeConcern::BooleanReadinessBundle,
        ))
}

pub(crate) fn dirty_recovery(
    world: &'static str,
    source: &'static str,
) -> PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(PlanarRecoverySource::dirty_input(source))
        .prepare_next_step()
        .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(world)))
        .expect("dirty recovery plan")
        .certify()
        .expect("dirty recovery receipt")
}

pub(crate) fn unbounded_recovery(
    world: &'static str,
    source: &'static str,
) -> PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(PlanarRecoverySource::unbounded_or_open(
        source,
    ))
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(world)))
    .expect("unbounded recovery plan")
    .certify()
    .expect("unbounded recovery receipt")
}

pub(crate) fn diagnostic(
    world: &'static str,
    subject: PlanarDiagnosticSubject,
) -> PlanarDiagnosticBundleReceipt {
    let locality = subject.trigger_locality();
    let mut bundle = PlanarDiagnosticBundle::explain_planar_failure(subject);
    bundle = match locality {
        PlanarDiagnosticTriggerLocality::TopologyContract => bundle
            .with_topology_declared_surface(topology_surface(world))
            .with_query_causal_inspection(causal_reference(world)),
        PlanarDiagnosticTriggerLocality::RetainedTransformStep
        | PlanarDiagnosticTriggerLocality::MotionOrRotationPosture => bundle
            .with_motion_posture(cancellation_motion_receipt(world))
            .with_query_causal_inspection(causal_reference(world)),
        _ => bundle,
    };
    bundle
        .inspect_failure_locality()
        .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
            world,
        )))
        .expect("clean-fail diagnostic plan")
        .certify()
        .expect("clean-fail diagnostic receipt")
}

fn admission_row(
    family: PlanarAdmissionFamily,
    concern: PlanarRuntimeConcern,
) -> worth_spatial::facade::planar_contracts::PlanarAdmissionRow {
    planar_admission_matrix()
        .row(family, concern)
        .expect("planar admission row")
        .clone()
}
