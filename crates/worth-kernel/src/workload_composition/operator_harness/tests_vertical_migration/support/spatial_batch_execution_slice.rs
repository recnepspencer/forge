use worth_spatial::facade::replay_undo_semantic_graph::{
    boolean_event_ledger_spatial_boundary_fixture, projection_receipt_spatial_boundary_fixture,
};
use worth_spatial::touched_graph_conflict::{
    current_spatial_conflict_family_catalog_closeout, SpatialConflictFamilyIdentity,
};

use crate::workload_composition::{
    admit_batch_admission_grouped_input, admit_spatial_conflict_input,
    current_batch_admission_family_catalog_closeout, execute_selected_batch_admission_plan,
    lower_selected_batch_admission_plan, lower_selected_spatial_conflict_plan,
    prove_spatial_conflict_independence, BatchAdmissionCandidate, BatchAdmissionExecutionReceipt,
    BatchAdmissionGroupedInput, BatchAdmissionPairwiseIndependenceProof,
    BatchAdmissionSupportingConflictLane, ConflictIndependenceDisposition,
    SelectedSpatialConflictPlan, SpatialConflictIndependenceRequest, SpatialConflictInputRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DerivedSpatialBatchExecutionSlice {
    execution_receipt: BatchAdmissionExecutionReceipt,
    pub(super) independence_disposition: ConflictIndependenceDisposition,
    pub(super) authority_participant_identities: Vec<String>,
}

pub(crate) fn disjoint_parallel_spatial_batch_execution_slice() -> DerivedSpatialBatchExecutionSlice
{
    let closeout = current_spatial_conflict_family_catalog_closeout().expect("catalog closes");
    let left_fixture = projection_receipt_spatial_boundary_fixture();
    let right_fixture = boolean_event_ledger_spatial_boundary_fixture();
    let left = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(left_fixture.authority()).with_evidence_lookup(
            left_fixture.workload_handoff(),
            left_fixture.execution_receipt(),
        ),
    )
    .expect("left spatial conflict input admits");
    let right = admit_spatial_conflict_input(
        SpatialConflictInputRequest::new(right_fixture.authority()).with_evidence_lookup(
            right_fixture.workload_handoff(),
            right_fixture.execution_receipt(),
        ),
    )
    .expect("right spatial conflict input admits");
    let left_plan = lower_selected_spatial_conflict_plan(&closeout, &left);
    let right_plan = lower_selected_spatial_conflict_plan(&closeout, &right);
    let proof = prove_spatial_conflict_independence(SpatialConflictIndependenceRequest::new(
        &left_plan,
        &right_plan,
    ));
    DerivedSpatialBatchExecutionSlice::from_spatial_proof(
        &left_plan,
        &right_plan,
        proof.disposition(),
        Some(BatchAdmissionPairwiseIndependenceProof::Spatial(&proof)),
    )
}

impl DerivedSpatialBatchExecutionSlice {
    pub fn execution_receipt(&self) -> &BatchAdmissionExecutionReceipt {
        &self.execution_receipt
    }

    pub(crate) fn from_spatial_proof(
        left_plan: &SelectedSpatialConflictPlan<'_>,
        right_plan: &SelectedSpatialConflictPlan<'_>,
        independence_disposition: ConflictIndependenceDisposition,
        pairwise_independence: Option<BatchAdmissionPairwiseIndependenceProof<'_>>,
    ) -> Self {
        let mut authority_participant_identities = vec![
            left_plan
                .authority()
                .conflict_participant_identity()
                .expect("left authority participant identity")
                .canonical_part(),
            right_plan
                .authority()
                .conflict_participant_identity()
                .expect("right authority participant identity")
                .canonical_part(),
        ];
        authority_participant_identities.sort();
        authority_participant_identities.dedup();

        let grouped_input = match pairwise_independence {
            Some(pairwise_independence) => BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Spatial(left_plan),
                BatchAdmissionCandidate::Spatial(right_plan),
            ])
            .with_pairwise_independence(pairwise_independence),
            None => BatchAdmissionGroupedInput::new([
                BatchAdmissionCandidate::Spatial(left_plan),
                BatchAdmissionCandidate::Spatial(right_plan),
            ]),
        };
        let selected_plan = lower_selected_batch_admission_plan(
            &current_batch_admission_family_catalog_closeout(),
            &admit_batch_admission_grouped_input(grouped_input).expect("group admits"),
        );
        let execution_receipt = execute_selected_batch_admission_plan(&selected_plan);

        assert_eq!(
            execution_receipt
                .counters()
                .topology_supporting_conflict_family_row_count(),
            0
        );
        assert_eq!(
            execution_receipt
                .counters()
                .spatial_supporting_conflict_family_row_count(),
            2
        );
        assert!(execution_receipt
            .supporting_conflict_family_rows()
            .iter()
            .all(|row| {
                row.conflict_lane() == BatchAdmissionSupportingConflictLane::Spatial
                    && row.conflict_family_identity()
                        == SpatialConflictFamilyIdentity::EvidenceSelection.as_str()
            }));

        Self {
            execution_receipt,
            independence_disposition,
            authority_participant_identities,
        }
    }
}
