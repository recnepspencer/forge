use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::ConflictPlanDownstreamProofCategory;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BatchAdmissionFamilyIdentity {
    ParallelProjectionConsumption,
    AdvisoryQueryBoundaryParallel,
    SerializableGroupedOverlap,
    DeniedGroupedOverlap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchAdmissionFamilyPosture {
    ParallelAdmit,
    SerialAdmit,
    AdvisorySerialAdmit,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchAdmissionIndependenceRequirement {
    CompleteParallelProof,
    CompleteSerializableOrBetterProof,
    MissingOrDeniedProof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchAdmissionAdvisoryWitnessShape {
    QueryBoundarySerialCoordination,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionFamilyDeclarationInput {
    pub identity: BatchAdmissionFamilyIdentity,
    pub posture: BatchAdmissionFamilyPosture,
    pub accepted_overlap_categories: Vec<ConflictOverlapCategory>,
    pub accepted_downstream_proof_categories: Vec<ConflictPlanDownstreamProofCategory>,
    pub require_all_selected_plans_admitted: bool,
    pub independence_requirement: BatchAdmissionIndependenceRequirement,
    pub advisory_witness_shape: Option<BatchAdmissionAdvisoryWitnessShape>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionFamilyDeclaration {
    identity: BatchAdmissionFamilyIdentity,
    posture: BatchAdmissionFamilyPosture,
    accepted_overlap_categories: Vec<ConflictOverlapCategory>,
    accepted_downstream_proof_categories: Vec<ConflictPlanDownstreamProofCategory>,
    require_all_selected_plans_admitted: bool,
    independence_requirement: BatchAdmissionIndependenceRequirement,
    advisory_witness_shape: Option<BatchAdmissionAdvisoryWitnessShape>,
    declaration_digest: String,
}

impl BatchAdmissionFamilyIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParallelProjectionConsumption => "parallel-projection-consumption",
            Self::AdvisoryQueryBoundaryParallel => "advisory-query-boundary-parallel",
            Self::SerializableGroupedOverlap => "serializable-grouped-overlap",
            Self::DeniedGroupedOverlap => "denied-grouped-overlap",
        }
    }
}

impl BatchAdmissionFamilyPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParallelAdmit => "parallel-admit",
            Self::SerialAdmit => "serial-admit",
            Self::AdvisorySerialAdmit => "advisory-serial-admit",
            Self::Denied => "denied",
        }
    }
}

impl BatchAdmissionIndependenceRequirement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompleteParallelProof => "complete-parallel-proof",
            Self::CompleteSerializableOrBetterProof => "complete-serializable-or-better-proof",
            Self::MissingOrDeniedProof => "missing-or-denied-proof",
        }
    }
}

impl BatchAdmissionAdvisoryWitnessShape {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryBoundarySerialCoordination => "query-boundary-serial-coordination",
        }
    }
}

impl BatchAdmissionFamilyDeclaration {
    pub(crate) fn new(input: BatchAdmissionFamilyDeclarationInput) -> Self {
        match (input.posture, input.advisory_witness_shape) {
            (
                BatchAdmissionFamilyPosture::AdvisorySerialAdmit,
                Some(BatchAdmissionAdvisoryWitnessShape::QueryBoundarySerialCoordination),
            ) => {}
            (BatchAdmissionFamilyPosture::AdvisorySerialAdmit, None) => {
                panic!(
                    "advisory batch-admission family declarations must declare an advisory witness shape"
                );
            }
            (_, Some(_)) => {
                panic!(
                    "non-advisory batch-admission family declarations cannot declare an advisory witness shape"
                );
            }
            (_, None) => {}
        }
        let mut overlap_categories = input.accepted_overlap_categories;
        overlap_categories.sort();
        overlap_categories.dedup();
        let mut downstream_categories = input.accepted_downstream_proof_categories;
        downstream_categories.sort_by_key(|category| category.as_str());
        downstream_categories.dedup_by_key(|category| category.as_str());
        let declaration_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:batch-admission-family-declaration:v1".to_string(),
                format!("identity:{}", input.identity.as_str()),
                format!("posture:{}", input.posture.as_str()),
                format!(
                    "require-admitted:{}",
                    input.require_all_selected_plans_admitted
                ),
                format!("independence:{}", input.independence_requirement.as_str()),
                format!(
                    "advisory:{}",
                    input
                        .advisory_witness_shape
                        .map(BatchAdmissionAdvisoryWitnessShape::as_str)
                        .unwrap_or("none")
                ),
                format!(
                    "overlap:{}",
                    overlap_categories
                        .iter()
                        .map(|category| category.as_str())
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                format!(
                    "downstream:{}",
                    downstream_categories
                        .iter()
                        .map(|category| category.as_str())
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            ],
        );
        Self {
            identity: input.identity,
            posture: input.posture,
            accepted_overlap_categories: overlap_categories,
            accepted_downstream_proof_categories: downstream_categories,
            require_all_selected_plans_admitted: input.require_all_selected_plans_admitted,
            independence_requirement: input.independence_requirement,
            advisory_witness_shape: input.advisory_witness_shape,
            declaration_digest,
        }
    }

    pub const fn identity(&self) -> BatchAdmissionFamilyIdentity {
        self.identity
    }

    pub const fn posture(&self) -> BatchAdmissionFamilyPosture {
        self.posture
    }

    pub fn accepted_overlap_categories(&self) -> &[ConflictOverlapCategory] {
        &self.accepted_overlap_categories
    }

    pub fn accepted_downstream_proof_categories(&self) -> &[ConflictPlanDownstreamProofCategory] {
        &self.accepted_downstream_proof_categories
    }

    pub const fn require_all_selected_plans_admitted(&self) -> bool {
        self.require_all_selected_plans_admitted
    }

    pub const fn independence_requirement(&self) -> BatchAdmissionIndependenceRequirement {
        self.independence_requirement
    }

    pub const fn advisory_witness_shape(&self) -> Option<BatchAdmissionAdvisoryWitnessShape> {
        self.advisory_witness_shape
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }
}
