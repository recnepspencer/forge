mod event_ledger;
mod receipt_stage_lookup;
mod segment_pair_enumeration;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorkloadEvidenceStageCounters {
    topology_entity_count: usize,
    topology_face_count: usize,
    topology_relation_count: usize,
    binding_target_count: usize,
    surface_support_count: usize,
    projected_entity_count: usize,
    local_basis_part_count: usize,
    transform_step_count: usize,
    transform_changed_coordinate_count: usize,
    transform_cancellation_step_count: usize,
    retained_artifact_count: usize,
    replay_checkpoint_count: usize,
    operator_input_count: usize,
    operator_receipt_count: usize,
    boolean_declaration_count: usize,
    boolean_route_count: usize,
    boolean_operand_pair_count: usize,
    boolean_blocker_count: usize,
    boolean_precision_agreement_count: usize,
    boolean_shared_plane_identity_count: usize,
    boolean_local_frame_selection_count: usize,
    boolean_operand_a_projection_consumption_count: usize,
    boolean_operand_b_projection_consumption_count: usize,
    boolean_reduced_operand_pair_count: usize,
    boolean_event_extraction_request_count: usize,
    boolean_segment_pair_enumeration_count: usize,
    boolean_segment_pair_left_segment_count: usize,
    boolean_segment_pair_right_segment_count: usize,
    boolean_segment_pair_expected_breadth: usize,
    boolean_segment_pair_emitted_breadth: usize,
    boolean_segment_pair_skipped_count: usize,
    boolean_segment_pair_query_index_candidate_count: usize,
    boolean_segment_pair_query_index_culled_count: usize,
    boolean_segment_pair_envelope_expanded_count: usize,
    boolean_segment_pair_broad_phase_comparison_count: usize,
    boolean_segment_pair_degenerate_skip_count: usize,
    boolean_segment_pair_fallback_used_count: usize,
    boolean_event_ledger_count: usize,
    boolean_event_ledger_point_event_count: usize,
    boolean_event_ledger_interval_event_count: usize,
    boolean_event_ledger_group_count: usize,
    boolean_event_ledger_relation_diagnostic_count: usize,
    boolean_split_count: usize,
    boolean_loop_reconstruction_count: usize,
    boolean_classify_count: usize,
    boolean_assemble_count: usize,
    boolean_cleanup_count: usize,
    diagnostic_count: usize,
    user_outcome_count: usize,
}

impl WorkloadEvidenceStageCounters {
    pub(crate) fn topology(entity_count: usize, face_count: usize, relation_count: usize) -> Self {
        Self {
            topology_entity_count: entity_count,
            topology_face_count: face_count,
            topology_relation_count: relation_count,
            ..Self::default()
        }
    }

    pub(crate) fn binding(binding_target_count: usize) -> Self {
        Self {
            binding_target_count,
            ..Self::default()
        }
    }

    pub(crate) fn surface_support(surface_support_count: usize) -> Self {
        Self {
            surface_support_count,
            ..Self::default()
        }
    }

    pub(crate) fn projection(projected_entity_count: usize, local_basis_part_count: usize) -> Self {
        Self {
            projected_entity_count,
            local_basis_part_count,
            ..Self::default()
        }
    }

    pub(crate) fn transform(
        transform_step_count: usize,
        transform_changed_coordinate_count: usize,
        transform_cancellation_step_count: usize,
    ) -> Self {
        Self {
            transform_step_count,
            transform_changed_coordinate_count,
            transform_cancellation_step_count,
            ..Self::default()
        }
    }

    pub(crate) fn retained_replay(
        retained_artifact_count: usize,
        replay_checkpoint_count: usize,
    ) -> Self {
        Self {
            retained_artifact_count,
            replay_checkpoint_count,
            ..Self::default()
        }
    }

    pub(crate) fn diagnostics(diagnostic_count: usize) -> Self {
        Self {
            diagnostic_count,
            ..Self::default()
        }
    }

    pub(crate) fn response(user_outcome_count: usize) -> Self {
        Self {
            user_outcome_count,
            ..Self::default()
        }
    }

    pub fn operator(operator_input_count: usize, operator_receipt_count: usize) -> Self {
        Self {
            operator_input_count,
            operator_receipt_count,
            ..Self::default()
        }
    }

    pub fn boolean_declaration() -> Self {
        Self {
            boolean_declaration_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_route() -> Self {
        Self {
            boolean_route_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_operand_pair() -> Self {
        Self {
            boolean_operand_pair_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_blocker() -> Self {
        Self {
            boolean_blocker_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_precision_agreement() -> Self {
        Self {
            boolean_precision_agreement_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_shared_plane_identity() -> Self {
        Self {
            boolean_shared_plane_identity_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_local_frame_selection() -> Self {
        Self {
            boolean_local_frame_selection_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_operand_a_projection_consumption() -> Self {
        Self {
            boolean_operand_a_projection_consumption_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_operand_b_projection_consumption() -> Self {
        Self {
            boolean_operand_b_projection_consumption_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_reduced_operand_pair() -> Self {
        Self {
            boolean_reduced_operand_pair_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_event_extraction_request() -> Self {
        Self {
            boolean_event_extraction_request_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_split() -> Self {
        Self {
            boolean_split_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_loop_reconstruction() -> Self {
        Self {
            boolean_loop_reconstruction_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_classify() -> Self {
        Self {
            boolean_classify_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_assemble() -> Self {
        Self {
            boolean_assemble_count: 1,
            ..Self::default()
        }
    }

    pub fn boolean_cleanup() -> Self {
        Self {
            boolean_cleanup_count: 1,
            ..Self::default()
        }
    }

    pub fn topology_entity_count(self) -> usize {
        self.topology_entity_count
    }

    pub fn topology_face_count(self) -> usize {
        self.topology_face_count
    }

    pub fn topology_relation_count(self) -> usize {
        self.topology_relation_count
    }

    pub fn binding_target_count(self) -> usize {
        self.binding_target_count
    }

    pub fn surface_support_count(self) -> usize {
        self.surface_support_count
    }

    pub fn projected_entity_count(self) -> usize {
        self.projected_entity_count
    }

    pub fn local_basis_part_count(self) -> usize {
        self.local_basis_part_count
    }

    pub fn transform_step_count(self) -> usize {
        self.transform_step_count
    }

    pub fn transform_changed_coordinate_count(self) -> usize {
        self.transform_changed_coordinate_count
    }

    pub fn transform_cancellation_step_count(self) -> usize {
        self.transform_cancellation_step_count
    }

    pub fn retained_artifact_count(self) -> usize {
        self.retained_artifact_count
    }

    pub fn replay_checkpoint_count(self) -> usize {
        self.replay_checkpoint_count
    }

    pub fn operator_input_count(self) -> usize {
        self.operator_input_count
    }

    pub fn operator_receipt_count(self) -> usize {
        self.operator_receipt_count
    }

    pub fn boolean_declaration_count(self) -> usize {
        self.boolean_declaration_count
    }

    pub fn boolean_route_count(self) -> usize {
        self.boolean_route_count
    }

    pub fn boolean_operand_pair_count(self) -> usize {
        self.boolean_operand_pair_count
    }

    pub fn boolean_blocker_count(self) -> usize {
        self.boolean_blocker_count
    }

    pub fn boolean_precision_agreement_count(self) -> usize {
        self.boolean_precision_agreement_count
    }

    pub fn boolean_shared_plane_identity_count(self) -> usize {
        self.boolean_shared_plane_identity_count
    }

    pub fn boolean_local_frame_selection_count(self) -> usize {
        self.boolean_local_frame_selection_count
    }

    pub fn boolean_operand_a_projection_consumption_count(self) -> usize {
        self.boolean_operand_a_projection_consumption_count
    }

    pub fn boolean_operand_b_projection_consumption_count(self) -> usize {
        self.boolean_operand_b_projection_consumption_count
    }

    pub fn boolean_reduced_operand_pair_count(self) -> usize {
        self.boolean_reduced_operand_pair_count
    }

    pub fn boolean_event_extraction_request_count(self) -> usize {
        self.boolean_event_extraction_request_count
    }

    pub fn boolean_split_count(self) -> usize {
        self.boolean_split_count
    }

    pub fn boolean_loop_reconstruction_count(self) -> usize {
        self.boolean_loop_reconstruction_count
    }

    pub fn boolean_classify_count(self) -> usize {
        self.boolean_classify_count
    }

    pub fn boolean_assemble_count(self) -> usize {
        self.boolean_assemble_count
    }

    pub fn boolean_cleanup_count(self) -> usize {
        self.boolean_cleanup_count
    }

    pub fn diagnostic_count(self) -> usize {
        self.diagnostic_count
    }

    pub fn user_outcome_count(self) -> usize {
        self.user_outcome_count
    }

    pub fn total_receipt_backed_counters(self) -> usize {
        self.topology_entity_count
            + self.topology_face_count
            + self.topology_relation_count
            + self.binding_target_count
            + self.surface_support_count
            + self.projected_entity_count
            + self.local_basis_part_count
            + self.transform_step_count
            + self.transform_changed_coordinate_count
            + self.transform_cancellation_step_count
            + self.retained_artifact_count
            + self.replay_checkpoint_count
            + self.operator_input_count
            + self.operator_receipt_count
            + self.boolean_declaration_count
            + self.boolean_route_count
            + self.boolean_operand_pair_count
            + self.boolean_blocker_count
            + self.boolean_precision_agreement_count
            + self.boolean_shared_plane_identity_count
            + self.boolean_local_frame_selection_count
            + self.boolean_operand_a_projection_consumption_count
            + self.boolean_operand_b_projection_consumption_count
            + self.boolean_reduced_operand_pair_count
            + self.boolean_event_extraction_request_count
            + self.boolean_segment_pair_enumeration_count
            + self.boolean_event_ledger_count
            + self.boolean_split_count
            + self.boolean_loop_reconstruction_count
            + self.boolean_classify_count
            + self.boolean_assemble_count
            + self.boolean_cleanup_count
            + self.diagnostic_count
            + self.user_outcome_count
    }
}
