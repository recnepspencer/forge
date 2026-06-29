use std::collections::BTreeSet;

use super::family_catalog::BatchAdmissionFamilyCatalogCloseout;
use super::family_declaration::{
    BatchAdmissionAdvisoryWitnessShape, BatchAdmissionFamilyDeclaration,
    BatchAdmissionIndependenceRequirement,
};
use super::grouped_input::AdmittedBatchAdmissionGroupedInput;
use super::selected_plan::{
    BatchAdmissionPlanAdvisory, BatchAdmissionPlanDenial, BatchAdmissionPlanDenialKind,
    BatchAdmissionPlanEdge, BatchAdmissionSelectedFamilyRow, SelectedBatchAdmissionPlan,
};
use crate::workload_composition::ConflictIndependenceDisposition;

pub fn lower_selected_batch_admission_plan<'a>(
    closeout: &BatchAdmissionFamilyCatalogCloseout,
    admitted: &AdmittedBatchAdmissionGroupedInput<'a>,
) -> SelectedBatchAdmissionPlan {
    let authority_digests = admitted
        .candidates()
        .iter()
        .map(|candidate| candidate.authority_digest().to_string())
        .collect::<Vec<_>>();
    let selected_conflict_plan_digests = admitted
        .candidates()
        .iter()
        .map(|candidate| candidate.selected_plan_digest().to_string())
        .collect::<Vec<_>>();
    let overlap_identity_digests = admitted
        .candidates()
        .iter()
        .map(|candidate| candidate.overlap_identity_digest().to_string())
        .collect::<Vec<_>>();
    let locality_footprint_digests = admitted
        .candidates()
        .iter()
        .map(|candidate| candidate.locality_footprint_digest().to_string())
        .collect::<Vec<_>>();
    let participant_identities = admitted
        .candidates()
        .iter()
        .map(|candidate| candidate.selected_plan_digest().to_string())
        .collect::<Vec<_>>();
    let proof_summary = ProofSummary::from_group(admitted);
    let declaration = select_exact_batch_admission_family(closeout, admitted, &proof_summary);
    let selected_family_rows = vec![BatchAdmissionSelectedFamilyRow::new(
        declaration.identity(),
        declaration.declaration_digest(),
        declaration.posture(),
    )];
    let supporting_conflict_family_rows = admitted
        .candidates()
        .iter()
        .flat_map(|candidate| candidate.supporting_conflict_family_rows())
        .collect::<Vec<_>>();
    let advisory = proof_summary.advisory_for(declaration);
    let denial = proof_summary.denial_for(declaration);
    SelectedBatchAdmissionPlan::new(
        declaration.posture(),
        authority_digests,
        selected_conflict_plan_digests,
        overlap_identity_digests,
        locality_footprint_digests,
        participant_identities,
        selected_family_rows,
        supporting_conflict_family_rows,
        proof_summary.parallel_edges,
        proof_summary.serial_edges,
        proof_summary.denied_proof_identities,
        advisory,
        denial,
        admitted.grouped_input_digest(),
    )
}

#[derive(Default)]
struct ProofSummary {
    parallel_edges: Vec<BatchAdmissionPlanEdge>,
    serial_edges: Vec<BatchAdmissionPlanEdge>,
    denied_proof_identities: Vec<String>,
    complete_pair_coverage: bool,
    saw_serializable_only: bool,
    saw_denied_proof: bool,
    saw_selected_plan_denial: bool,
}

impl ProofSummary {
    fn from_group(admitted: &AdmittedBatchAdmissionGroupedInput<'_>) -> Self {
        let mut summary = Self::default();
        summary.saw_selected_plan_denial = admitted
            .candidates()
            .iter()
            .any(|candidate| candidate.execution_admission().is_denied());
        let expected_pair_count =
            admitted.candidates().len() * (admitted.candidates().len() - 1) / 2;
        let mut covered_pairs = BTreeSet::new();
        for proof in admitted.pairwise_independence() {
            covered_pairs.insert(canonical_pair_key(
                proof.left_plan_digest(),
                proof.right_plan_digest(),
            ));
            match proof.disposition() {
                ConflictIndependenceDisposition::Disjoint
                | ConflictIndependenceDisposition::CompatibleAspectOverlap => {
                    summary.parallel_edges.push(BatchAdmissionPlanEdge::new(
                        proof.left_plan_digest(),
                        proof.right_plan_digest(),
                        proof.proof_digest(),
                    ));
                }
                ConflictIndependenceDisposition::SerializableOnly => {
                    summary.saw_serializable_only = true;
                    summary.serial_edges.push(BatchAdmissionPlanEdge::new(
                        proof.left_plan_digest(),
                        proof.right_plan_digest(),
                        proof.proof_digest(),
                    ));
                }
                ConflictIndependenceDisposition::Denied => {
                    summary.saw_denied_proof = true;
                    summary
                        .denied_proof_identities
                        .push(proof.proof_digest().to_string());
                }
            }
        }
        summary.complete_pair_coverage = covered_pairs.len() == expected_pair_count;
        summary.parallel_edges.sort_by_key(|edge| {
            format!(
                "{}:{}",
                edge.left_participant_identity(),
                edge.right_participant_identity()
            )
        });
        summary.serial_edges.sort_by_key(|edge| {
            format!(
                "{}:{}",
                edge.left_participant_identity(),
                edge.right_participant_identity()
            )
        });
        summary.denied_proof_identities.sort();
        summary
    }

    fn supports_parallel_only(&self) -> bool {
        self.complete_pair_coverage && !self.saw_serializable_only && !self.saw_denied_proof
    }

    fn supports_serializable_or_better(&self) -> bool {
        self.complete_pair_coverage && !self.saw_denied_proof
    }

    fn denial_for(
        &self,
        declaration: &BatchAdmissionFamilyDeclaration,
    ) -> Option<BatchAdmissionPlanDenial> {
        if !matches!(
            declaration.independence_requirement(),
            BatchAdmissionIndependenceRequirement::MissingOrDeniedProof
        ) {
            return None;
        }
        Some(if self.saw_selected_plan_denial {
            BatchAdmissionPlanDenial::new(
                BatchAdmissionPlanDenialKind::SelectedPlanDenied,
                "batch admission cannot admit grouped work after one selected conflict plan already denied execution",
            )
        } else if self.saw_denied_proof {
            BatchAdmissionPlanDenial::new(
                BatchAdmissionPlanDenialKind::DeclaredDenied,
                "batch admission received an explicit denied pairwise independence proof",
            )
        } else {
            BatchAdmissionPlanDenial::new(
                BatchAdmissionPlanDenialKind::MissingExplicitIndependenceProof,
                "batch admission requires complete explicit independence proof before grouped posture may admit parallel or serial execution",
            )
        })
    }

    fn advisory_for(
        &self,
        declaration: &BatchAdmissionFamilyDeclaration,
    ) -> Option<BatchAdmissionPlanAdvisory> {
        declaration
            .advisory_witness_shape()
            .map(|shape| BatchAdmissionPlanAdvisory::new(shape, advisory_detail_for(shape)))
    }
}

fn canonical_pair_key(left: &str, right: &str) -> String {
    if left <= right {
        format!("{left}|{right}")
    } else {
        format!("{right}|{left}")
    }
}

fn advisory_detail_for(shape: BatchAdmissionAdvisoryWitnessShape) -> &'static str {
    match shape {
        BatchAdmissionAdvisoryWitnessShape::QueryBoundarySerialCoordination =>
            "grouped work crosses query boundary envelopes and must preserve explicit serial coordination despite complete independence proof",
    }
}

fn select_exact_batch_admission_family<'a>(
    closeout: &'a BatchAdmissionFamilyCatalogCloseout,
    admitted: &AdmittedBatchAdmissionGroupedInput<'_>,
    proof_summary: &ProofSummary,
) -> &'a BatchAdmissionFamilyDeclaration {
    let matches = closeout
        .catalog()
        .declarations()
        .iter()
        .filter(|declaration| matches_declaration(declaration, admitted, proof_summary))
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [declaration] => declaration,
        [] => panic!("current batch-admission family catalog must always match one grouped plan"),
        _ => panic!(
            "current batch-admission family catalog must match exactly one grouped plan declaration"
        ),
    }
}

fn matches_declaration(
    declaration: &BatchAdmissionFamilyDeclaration,
    admitted: &AdmittedBatchAdmissionGroupedInput<'_>,
    proof_summary: &ProofSummary,
) -> bool {
    if declaration.require_all_selected_plans_admitted()
        && admitted
            .candidates()
            .iter()
            .any(|candidate| candidate.execution_admission().is_denied())
    {
        return false;
    }
    if admitted.candidates().iter().any(|candidate| {
        !declaration
            .accepted_overlap_categories()
            .contains(&candidate.overlap_category())
            || !declaration
                .accepted_downstream_proof_categories()
                .contains(&candidate.downstream_proof_category())
    }) {
        return false;
    }
    match declaration.independence_requirement() {
        BatchAdmissionIndependenceRequirement::CompleteParallelProof => {
            proof_summary.supports_parallel_only()
        }
        BatchAdmissionIndependenceRequirement::CompleteSerializableOrBetterProof => {
            proof_summary.supports_serializable_or_better() && proof_summary.saw_serializable_only
        }
        BatchAdmissionIndependenceRequirement::MissingOrDeniedProof => {
            proof_summary.saw_selected_plan_denial
                || proof_summary.saw_denied_proof
                || !proof_summary.complete_pair_coverage
        }
    }
}
