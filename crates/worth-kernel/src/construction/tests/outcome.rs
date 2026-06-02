use super::{
    prepare_primitive_construction_outcome, rejected_outcome, PrimitiveConstructionPreparedOutcome,
    PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use crate::construction::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionGeometryError,
    PrimitiveConstructionPhaseError,
};
use crate::construction::result::PrimitiveConstructionResultError;
use crate::construction::{OrthotopeSpec, PrimitiveConstructionIntent, WireBodySpec};
use topology::facade::{
    TopologyConstructionQueryAdmittedHandoffError, TopologyConstructionQueryEnvelopeError,
    TopologyConstructionQueryHandoffError, TopologyConstructionQueryReceiptError,
};
use worth_geom::facade::Plane;
use worth_spatial::facade::{
    impossible_primitive_construction_birth_attachment, plan_primitive_construction_birth,
    PrimitiveConstructionBirthFamily, PrimitiveConstructionBirthScaffoldInput,
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
    let outcome = prepare_primitive_construction_outcome(PrimitiveConstructionIntent::wire_body(
        WireBodySpec { edge_count: 2 },
    ));

    match outcome {
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
            assert!(rejected.reason().contains("invalid wire_body request"));
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
}

#[test]
fn rejected_outcome_maps_spatial_birth_and_impossible_attachment_distinctly() {
    let input = PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionBirthFamily::WireBody,
        "planar_wire_body",
        "wire-scaffold".to_string(),
        vec![plane()],
        vec![
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
        4,
        4,
        1,
        1,
        0,
        0,
        1,
    );
    let mismatched = PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionBirthFamily::WireBody,
        "wrong_class",
        "wire-scaffold".to_string(),
        vec![plane()],
        vec![
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
        4,
        4,
        1,
        1,
        0,
        0,
        1,
    );
    let plan = plan_primitive_construction_birth(input).expect("birth plan");
    let impossible = impossible_primitive_construction_birth_attachment(&mismatched, &plan)
        .expect("impossible attachment");
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
                    impossible.reason().to_string(),
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
}

fn plane() -> Plane {
    Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")
}
