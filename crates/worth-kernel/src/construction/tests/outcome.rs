use super::super::intent::PrimitiveConstructionIntent;
use super::super::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionGeometryError,
    PrimitiveConstructionPhaseError,
};
use super::super::result::{
    prepare_primitive_construction_result, PrimitiveConstructionResultError,
};
use super::super::specs::{OrthotopeSpec, SimplexSolidSpec, WireBodySpec};
use super::{
    prepare_primitive_construction_outcome, rejected_outcome, PrimitiveConstructionPreparedOutcome,
    PrimitiveConstructionRecoveryAction, PrimitiveConstructionRejectionClass,
    PrimitiveConstructionRejectionLocality,
};
use topology::facade::{
    TopologyConstructionQueryAdmittedHandoffError, TopologyConstructionQueryEnvelopeError,
    TopologyConstructionQueryHandoffError, TopologyConstructionQueryReceiptError,
};

#[test]
fn prepared_outcome_tracks_accepted_artifact_identity() {
    let outcome = prepare_primitive_construction_outcome(PrimitiveConstructionIntent::orthotope(
        OrthotopeSpec {
            half_extents: [1.0, 2.0, 3.0],
        },
    ));

    match outcome {
        PrimitiveConstructionPreparedOutcome::Accepted(accepted) => {
            assert_eq!(accepted.family(), PrimitiveConstructionFamily::Orthotope);
            assert_eq!(accepted.topology_birth_class(), "closed_orthotope_body");
            assert!(!accepted.canonical_artifact_digest().is_empty());
            assert!(!accepted.outcome_digest().is_empty());
        }
        PrimitiveConstructionPreparedOutcome::Rejected(_) => panic!("orthotope should be accepted"),
    }
}

#[test]
fn prepared_outcome_tracks_rejected_request_locality() {
    let intent = PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 });
    let error = prepare_primitive_construction_result(intent.clone())
        .expect_err("invalid wire body should reject");
    match error {
        PrimitiveConstructionResultError::Phase(
            PrimitiveConstructionPhaseError::InvalidRequest { family, reason },
        ) => {
            assert_eq!(family, PrimitiveConstructionFamily::WireBody);
            assert_eq!(
                reason,
                "polygonal construction families require at least three edges"
            );
        }
        other => panic!("expected invalid wire_body request, got {other:?}"),
    }

    match prepare_primitive_construction_outcome(intent) {
        PrimitiveConstructionPreparedOutcome::Accepted(_) => {
            panic!("invalid wire body should be rejected")
        }
        PrimitiveConstructionPreparedOutcome::Rejected(rejected) => {
            assert_eq!(rejected.family(), PrimitiveConstructionFamily::WireBody);
            assert_eq!(
                rejected.rejection_class(),
                PrimitiveConstructionRejectionClass::InvalidRequest
            );
            assert_eq!(
                rejected.rejection_locality(),
                PrimitiveConstructionRejectionLocality::Admission
            );
            assert_eq!(
                rejected.recovery_actions(),
                &[PrimitiveConstructionRecoveryAction::CorrectRequestFamilyOrCounts]
            );
            assert!(!rejected.failure_digest().is_empty());
        }
    }
}

#[test]
fn rejected_outcome_maps_phase_and_runtime_failures_to_exact_localities() {
    let geometry = rejected_outcome(
        PrimitiveConstructionFamily::Orthotope,
        &PrimitiveConstructionResultError::Phase(PrimitiveConstructionPhaseError::Geometry(
            PrimitiveConstructionGeometryError::GeometryFailure("bad scaffold".to_string()),
        )),
    );
    let execution = rejected_outcome(
        PrimitiveConstructionFamily::RegularPrism,
        &PrimitiveConstructionResultError::Phase(
            PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff(
                TopologyConstructionQueryAdmittedHandoffError::Handoff(
                    TopologyConstructionQueryHandoffError::Envelope(
                        TopologyConstructionQueryEnvelopeError::Receipt(
                            TopologyConstructionQueryReceiptError::UnsupportedBirthClass(
                                "bad surface",
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    assert_eq!(
        (geometry.rejection_class(), geometry.rejection_locality()),
        (
            PrimitiveConstructionRejectionClass::GeometryScaffold,
            PrimitiveConstructionRejectionLocality::Scaffold,
        )
    );
    assert_eq!(
        (execution.rejection_class(), execution.rejection_locality()),
        (
            PrimitiveConstructionRejectionClass::TopologyExecution,
            PrimitiveConstructionRejectionLocality::Execution,
        )
    );
    assert_eq!(
        geometry.recovery_actions(),
        &[PrimitiveConstructionRecoveryAction::ReviseGeometryScaffold]
    );
    assert_eq!(
        execution.recovery_actions(),
        &[PrimitiveConstructionRecoveryAction::RetryTopologyExecution]
    );
}

#[test]
fn rejected_outcome_maps_spatial_birth_and_impossible_attachment_distinctly() {
    let impossible_reason =
        "topology birth class mismatch: wrong_class cannot satisfy planar_wire_body".to_string();
    let completeness = rejected_outcome(
        PrimitiveConstructionFamily::WireBody,
        &PrimitiveConstructionResultError::Phase(
            PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff(
                TopologyConstructionQueryAdmittedHandoffError::BirthCompleteness(
                    "birth mismatch".to_string(),
                ),
            ),
        ),
    );
    let impossible = rejected_outcome(
        PrimitiveConstructionFamily::WireBody,
        &PrimitiveConstructionResultError::Phase(
            PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff(
                TopologyConstructionQueryAdmittedHandoffError::ImpossibleBirthAttachment(
                    impossible_reason,
                ),
            ),
        ),
    );

    assert_eq!(
        (
            completeness.rejection_class(),
            completeness.rejection_locality()
        ),
        (
            PrimitiveConstructionRejectionClass::SpatialBirth,
            PrimitiveConstructionRejectionLocality::SpatialBirth,
        )
    );
    assert_eq!(
        (
            impossible.rejection_class(),
            impossible.rejection_locality()
        ),
        (
            PrimitiveConstructionRejectionClass::ImpossibleBirthAttachment,
            PrimitiveConstructionRejectionLocality::SpatialBirth,
        )
    );
    assert_eq!(
        completeness.recovery_actions(),
        &[PrimitiveConstructionRecoveryAction::CorrectBirthAttachment]
    );
    assert_eq!(
        impossible.recovery_actions(),
        &[PrimitiveConstructionRecoveryAction::CorrectBirthAttachment]
    );
}

#[test]
fn rejected_outcome_covers_every_major_failure_boundary_with_typed_locality() {
    let admission = rejected_outcome(
        PrimitiveConstructionFamily::WireBody,
        &PrimitiveConstructionResultError::Phase(PrimitiveConstructionPhaseError::InvalidRequest {
            family: PrimitiveConstructionFamily::WireBody,
            reason: "polygonal construction families require at least three edges",
        }),
    );
    let scaffold = rejected_outcome(
        PrimitiveConstructionFamily::RegularPyramid,
        &PrimitiveConstructionResultError::Phase(PrimitiveConstructionPhaseError::Geometry(
            PrimitiveConstructionGeometryError::GeometryFailure("degenerate scaffold".to_string()),
        )),
    );
    let spatial_birth = rejected_outcome(
        PrimitiveConstructionFamily::ShellWithHole,
        &PrimitiveConstructionResultError::Phase(
            PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff(
                TopologyConstructionQueryAdmittedHandoffError::BirthCompleteness(
                    "birth mismatch".to_string(),
                ),
            ),
        ),
    );
    let topology = rejected_outcome(
        PrimitiveConstructionFamily::RegularPrism,
        &PrimitiveConstructionResultError::Phase(
            PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff(
                TopologyConstructionQueryAdmittedHandoffError::Handoff(
                    TopologyConstructionQueryHandoffError::Envelope(
                        TopologyConstructionQueryEnvelopeError::Receipt(
                            TopologyConstructionQueryReceiptError::UnsupportedBirthClass(
                                "bad surface",
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );

    assert_eq!(
        (admission.rejection_class(), admission.rejection_locality()),
        (
            PrimitiveConstructionRejectionClass::InvalidRequest,
            PrimitiveConstructionRejectionLocality::Admission,
        )
    );
    assert_eq!(
        (scaffold.rejection_class(), scaffold.rejection_locality()),
        (
            PrimitiveConstructionRejectionClass::GeometryScaffold,
            PrimitiveConstructionRejectionLocality::Scaffold,
        )
    );
    assert_eq!(
        (
            spatial_birth.rejection_class(),
            spatial_birth.rejection_locality()
        ),
        (
            PrimitiveConstructionRejectionClass::SpatialBirth,
            PrimitiveConstructionRejectionLocality::SpatialBirth,
        )
    );
    assert_eq!(
        (topology.rejection_class(), topology.rejection_locality()),
        (
            PrimitiveConstructionRejectionClass::TopologyExecution,
            PrimitiveConstructionRejectionLocality::Execution,
        )
    );
    assert_ne!(admission.failure_digest(), scaffold.failure_digest());
    assert_ne!(spatial_birth.failure_digest(), topology.failure_digest());
}

#[test]
fn rejected_outcome_maps_conditioning_exhaustion_to_typed_recovery_action() {
    let outcome = prepare_primitive_construction_outcome(
        PrimitiveConstructionIntent::simplex_solid(
            SimplexSolidSpec::new(1.0e-240).with_auxiliary_altitude_component(1.0e-280),
        )
        .at([2f64.powi(548), -2f64.powi(548), 2f64.powi(548)]),
    );

    match outcome {
        PrimitiveConstructionPreparedOutcome::Accepted(_) => {
            panic!("degenerate orthotope should be rejected")
        }
        PrimitiveConstructionPreparedOutcome::Rejected(rejected) => {
            assert_eq!(
                rejected.rejection_class(),
                PrimitiveConstructionRejectionClass::ConditioningExhaustion
            );
            assert_eq!(
                rejected.recovery_actions(),
                &[PrimitiveConstructionRecoveryAction::EscalateRealizationConditioning]
            );
        }
    }
}
