use crate::construction::diagnostics::PrimitiveConstructionBlockingBoundary;
use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::{
    rejected_outcome, PrimitiveConstructionRejectionClass, PrimitiveConstructionRejectionLocality,
};
use crate::construction::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionGeometryError,
    PrimitiveConstructionPhaseError,
};
use crate::construction::result::PrimitiveConstructionResultError;
use topology::facade::{
    TopologyConstructionQueryAdmittedHandoffError, TopologyConstructionQueryEnvelopeError,
    TopologyConstructionQueryHandoffError, TopologyConstructionQueryReceiptError,
};
use worth_geom::facade::Plane;
use worth_spatial::facade::bindings::{
    evaluate_primitive_construction_birth_consequence, plan_primitive_construction_birth,
    PrimitiveConstructionBirthFamily, PrimitiveConstructionBirthScaffoldInput,
    RejectedPrimitiveConstructionBirthConsequence, SpatialConstructionBirthConsequence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCorpusRejectionWitnessRow {
    witness_id: String,
    family: PrimitiveConstructionFamily,
    rejection_class: PrimitiveConstructionRejectionClass,
    rejection_locality: PrimitiveConstructionRejectionLocality,
    blocking_boundary: PrimitiveConstructionBlockingBoundary,
    failure_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionCorpusRejectionWitnessRow {
    pub fn witness_id(&self) -> &str {
        &self.witness_id
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn rejection_class(&self) -> PrimitiveConstructionRejectionClass {
        self.rejection_class
    }

    pub fn rejection_locality(&self) -> PrimitiveConstructionRejectionLocality {
        self.rejection_locality
    }

    pub fn blocking_boundary(&self) -> PrimitiveConstructionBlockingBoundary {
        self.blocking_boundary
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(super) fn primitive_construction_rejection_witness_rows(
) -> Vec<PrimitiveConstructionCorpusRejectionWitnessRow> {
    vec![
        witness(
            "admission_invalid_wire",
            PrimitiveConstructionFamily::WireBody,
            PrimitiveConstructionResultError::Phase(
                PrimitiveConstructionPhaseError::InvalidRequest {
                    family: PrimitiveConstructionFamily::WireBody,
                    reason: "polygonal construction families require at least three edges",
                },
            ),
        ),
        witness(
            "scaffold_geometry_failure",
            PrimitiveConstructionFamily::RegularPyramid,
            PrimitiveConstructionResultError::Phase(PrimitiveConstructionPhaseError::Geometry(
                PrimitiveConstructionGeometryError::GeometryFailure(
                    "degenerate scaffold".to_string(),
                ),
            )),
        ),
        witness(
            "spatial_birth_completeness_failure",
            PrimitiveConstructionFamily::ShellWithHole,
            PrimitiveConstructionResultError::Phase(
                PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff(
                    TopologyConstructionQueryAdmittedHandoffError::BirthCompleteness(
                        "birth mismatch".to_string(),
                    ),
                ),
            ),
        ),
        witness(
            "impossible_birth_attachment_failure",
            PrimitiveConstructionFamily::RegularPyramid,
            PrimitiveConstructionResultError::Phase(
                PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff(
                    TopologyConstructionQueryAdmittedHandoffError::ImpossibleBirthAttachment(
                        impossible_birth_attachment_witness().reason().to_string(),
                    ),
                ),
            ),
        ),
        witness(
            "topology_execution_failure",
            PrimitiveConstructionFamily::RegularPrism,
            PrimitiveConstructionResultError::Phase(
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
        ),
    ]
}

fn witness(
    witness_id: &str,
    family: PrimitiveConstructionFamily,
    error: PrimitiveConstructionResultError,
) -> PrimitiveConstructionCorpusRejectionWitnessRow {
    let rejected = rejected_outcome(family, &error);
    let blocking_boundary = match rejected.rejection_locality() {
        PrimitiveConstructionRejectionLocality::Admission => {
            PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission
        }
        PrimitiveConstructionRejectionLocality::Scaffold => {
            PrimitiveConstructionBlockingBoundary::KernelIntent
        }
        PrimitiveConstructionRejectionLocality::SpatialBirth => {
            PrimitiveConstructionBlockingBoundary::SpatialBirth
        }
        PrimitiveConstructionRejectionLocality::Execution => {
            PrimitiveConstructionBlockingBoundary::TopologyLegality
        }
    };
    let row_digest = digest_owned_parts(&[
        witness_id.to_string(),
        rejected.family().as_str().to_string(),
        rejected.rejection_class().as_str().to_string(),
        rejected.rejection_locality().as_str().to_string(),
        blocking_boundary.as_str().to_string(),
        rejected.failure_digest().to_string(),
    ]);
    PrimitiveConstructionCorpusRejectionWitnessRow {
        witness_id: witness_id.to_string(),
        family: rejected.family(),
        rejection_class: rejected.rejection_class(),
        rejection_locality: rejected.rejection_locality(),
        blocking_boundary,
        failure_digest: rejected.failure_digest().to_string(),
        row_digest,
    }
}

fn impossible_birth_attachment_witness() -> RejectedPrimitiveConstructionBirthConsequence {
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
        "bad_birth_class",
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
    match evaluate_primitive_construction_birth_consequence(&mismatched, &plan) {
        SpatialConstructionBirthConsequence::Admitted(_) => {
            panic!("mismatched birth should be rejected")
        }
        SpatialConstructionBirthConsequence::Rejected(rejected) => rejected,
    }
}

fn plane() -> Plane {
    Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")
}
