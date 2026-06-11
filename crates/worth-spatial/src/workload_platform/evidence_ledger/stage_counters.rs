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
            + self.diagnostic_count
            + self.user_outcome_count
    }
}
