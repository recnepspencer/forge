use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::family_declaration::{
    BatchAdmissionAdvisoryWitnessShape, BatchAdmissionFamilyDeclaration,
    BatchAdmissionFamilyDeclarationInput, BatchAdmissionFamilyIdentity,
    BatchAdmissionFamilyPosture, BatchAdmissionIndependenceRequirement,
};
use crate::workload_composition::ConflictPlanDownstreamProofCategory;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionFamilyCatalog {
    declarations: Vec<BatchAdmissionFamilyDeclaration>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionFamilyCatalogCloseout {
    catalog: BatchAdmissionFamilyCatalog,
    catalog_digest: String,
}

pub fn current_batch_admission_family_catalog_closeout() -> BatchAdmissionFamilyCatalogCloseout {
    BatchAdmissionFamilyCatalogCloseout::close(BatchAdmissionFamilyCatalog::new(vec![
        BatchAdmissionFamilyDeclaration::new(BatchAdmissionFamilyDeclarationInput {
            identity: BatchAdmissionFamilyIdentity::ParallelProjectionConsumption,
            posture: BatchAdmissionFamilyPosture::ParallelAdmit,
            accepted_overlap_categories: vec![
                ConflictOverlapCategory::Aspect,
                ConflictOverlapCategory::Evidence,
                ConflictOverlapCategory::Locality,
            ],
            accepted_downstream_proof_categories: vec![
                ConflictPlanDownstreamProofCategory::ProjectionConsumption,
            ],
            require_all_selected_plans_admitted: true,
            independence_requirement: BatchAdmissionIndependenceRequirement::CompleteParallelProof,
            advisory_witness_shape: None,
        }),
        BatchAdmissionFamilyDeclaration::new(BatchAdmissionFamilyDeclarationInput {
            identity: BatchAdmissionFamilyIdentity::AdvisoryQueryBoundaryParallel,
            posture: BatchAdmissionFamilyPosture::AdvisorySerialAdmit,
            accepted_overlap_categories: vec![
                ConflictOverlapCategory::ReplayUndo,
                ConflictOverlapCategory::Transaction,
            ],
            accepted_downstream_proof_categories: vec![
                ConflictPlanDownstreamProofCategory::QueryBoundaryEnvelope,
            ],
            require_all_selected_plans_admitted: true,
            independence_requirement: BatchAdmissionIndependenceRequirement::CompleteParallelProof,
            advisory_witness_shape: Some(
                BatchAdmissionAdvisoryWitnessShape::QueryBoundarySerialCoordination,
            ),
        }),
        BatchAdmissionFamilyDeclaration::new(BatchAdmissionFamilyDeclarationInput {
            identity: BatchAdmissionFamilyIdentity::SerializableGroupedOverlap,
            posture: BatchAdmissionFamilyPosture::SerialAdmit,
            accepted_overlap_categories: vec![
                ConflictOverlapCategory::Aspect,
                ConflictOverlapCategory::Evidence,
                ConflictOverlapCategory::ReplayUndo,
                ConflictOverlapCategory::Transaction,
            ],
            accepted_downstream_proof_categories: vec![
                ConflictPlanDownstreamProofCategory::ProjectionConsumption,
                ConflictPlanDownstreamProofCategory::QueryBoundaryEnvelope,
            ],
            require_all_selected_plans_admitted: true,
            independence_requirement:
                BatchAdmissionIndependenceRequirement::CompleteSerializableOrBetterProof,
            advisory_witness_shape: None,
        }),
        BatchAdmissionFamilyDeclaration::new(BatchAdmissionFamilyDeclarationInput {
            identity: BatchAdmissionFamilyIdentity::DeniedGroupedOverlap,
            posture: BatchAdmissionFamilyPosture::Denied,
            accepted_overlap_categories: vec![
                ConflictOverlapCategory::Entity,
                ConflictOverlapCategory::Relation,
                ConflictOverlapCategory::Aspect,
                ConflictOverlapCategory::Locality,
                ConflictOverlapCategory::Evidence,
                ConflictOverlapCategory::Validator,
                ConflictOverlapCategory::ReplayUndo,
                ConflictOverlapCategory::Transaction,
            ],
            accepted_downstream_proof_categories: vec![
                ConflictPlanDownstreamProofCategory::ProjectionConsumption,
                ConflictPlanDownstreamProofCategory::QueryBoundaryEnvelope,
            ],
            require_all_selected_plans_admitted: false,
            independence_requirement: BatchAdmissionIndependenceRequirement::MissingOrDeniedProof,
            advisory_witness_shape: None,
        }),
    ]))
}

impl BatchAdmissionFamilyCatalog {
    pub(crate) fn new(mut declarations: Vec<BatchAdmissionFamilyDeclaration>) -> Self {
        declarations.sort_by_key(|declaration| declaration.identity().as_str());
        Self { declarations }
    }

    pub fn declarations(&self) -> &[BatchAdmissionFamilyDeclaration] {
        &self.declarations
    }
}

impl BatchAdmissionFamilyCatalogCloseout {
    pub(crate) fn close(catalog: BatchAdmissionFamilyCatalog) -> Self {
        let catalog_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &catalog
                .declarations()
                .iter()
                .map(|declaration| declaration.declaration_digest().to_string())
                .chain(std::iter::once(
                    "worth-kernel:batch-admission-family-catalog:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        Self {
            catalog,
            catalog_digest,
        }
    }

    pub const fn catalog(&self) -> &BatchAdmissionFamilyCatalog {
        &self.catalog
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}
