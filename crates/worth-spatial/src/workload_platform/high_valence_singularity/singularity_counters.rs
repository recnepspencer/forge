#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HighValenceSingularityCounters {
    topology_entity_count: usize,
    topology_face_count: usize,
    topology_relation_count: usize,
    binding_target_count: usize,
    surface_support_count: usize,
    neighborhood_valence: usize,
    projected_entity_count: usize,
    local_basis_part_count: usize,
    transform_step_count: usize,
    local_rebuild_evidence_row_count: usize,
    retained_artifact_count: usize,
    replay_checkpoint_count: usize,
    diagnostic_count: usize,
    user_outcome_count: usize,
}

impl HighValenceSingularityCounters {
    pub(crate) fn new(input: HighValenceSingularityCounterInput) -> Self {
        Self {
            topology_entity_count: input.topology_entity_count,
            topology_face_count: input.topology_face_count,
            topology_relation_count: input.topology_relation_count,
            binding_target_count: input.binding_target_count,
            surface_support_count: input.surface_support_count,
            neighborhood_valence: input.neighborhood_valence,
            projected_entity_count: input.projected_entity_count,
            local_basis_part_count: input.local_basis_part_count,
            transform_step_count: input.transform_step_count,
            local_rebuild_evidence_row_count: input.local_rebuild_evidence_row_count,
            retained_artifact_count: input.retained_artifact_count,
            replay_checkpoint_count: input.replay_checkpoint_count,
            diagnostic_count: input.diagnostic_count,
            user_outcome_count: input.user_outcome_count,
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

    pub fn neighborhood_valence(self) -> usize {
        self.neighborhood_valence
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

    pub fn local_rebuild_evidence_row_count(self) -> usize {
        self.local_rebuild_evidence_row_count
    }

    pub fn retained_artifact_count(self) -> usize {
        self.retained_artifact_count
    }

    pub fn replay_checkpoint_count(self) -> usize {
        self.replay_checkpoint_count
    }

    pub fn diagnostic_count(self) -> usize {
        self.diagnostic_count
    }

    pub fn user_outcome_count(self) -> usize {
        self.user_outcome_count
    }
}

pub(crate) struct HighValenceSingularityCounterInput {
    pub(crate) topology_entity_count: usize,
    pub(crate) topology_face_count: usize,
    pub(crate) topology_relation_count: usize,
    pub(crate) binding_target_count: usize,
    pub(crate) surface_support_count: usize,
    pub(crate) neighborhood_valence: usize,
    pub(crate) projected_entity_count: usize,
    pub(crate) local_basis_part_count: usize,
    pub(crate) transform_step_count: usize,
    pub(crate) local_rebuild_evidence_row_count: usize,
    pub(crate) retained_artifact_count: usize,
    pub(crate) replay_checkpoint_count: usize,
    pub(crate) diagnostic_count: usize,
    pub(crate) user_outcome_count: usize,
}
