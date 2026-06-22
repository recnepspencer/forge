#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapStormCounters {
    topology_entity_count: usize,
    topology_face_count: usize,
    topology_relation_count: usize,
    projected_entity_count: usize,
    transform_step_count: usize,
    transform_cancellation_step_count: usize,
    retained_artifact_count: usize,
    replay_checkpoint_count: usize,
    operator_input_count: usize,
    operator_receipt_count: usize,
    overlap_extraction_receipt_count: usize,
    overlap_candidate_pair_breadth: usize,
    overlap_segment_contacts_certified: usize,
    overlap_shared_intervals: usize,
    overlap_islands: usize,
    overlap_policy_required_exits: usize,
    overlap_ambiguous_contacts: usize,
}

impl CoplanarOverlapStormCounters {
    pub(crate) fn new(input: CoplanarOverlapStormCounterInput) -> Self {
        Self {
            topology_entity_count: input.topology_entity_count,
            topology_face_count: input.topology_face_count,
            topology_relation_count: input.topology_relation_count,
            projected_entity_count: input.projected_entity_count,
            transform_step_count: input.transform_step_count,
            transform_cancellation_step_count: input.transform_cancellation_step_count,
            retained_artifact_count: input.retained_artifact_count,
            replay_checkpoint_count: input.replay_checkpoint_count,
            operator_input_count: input.operator_input_count,
            operator_receipt_count: input.operator_receipt_count,
            overlap_extraction_receipt_count: input.overlap_extraction_receipt_count,
            overlap_candidate_pair_breadth: input.overlap_candidate_pair_breadth,
            overlap_segment_contacts_certified: input.overlap_segment_contacts_certified,
            overlap_shared_intervals: input.overlap_shared_intervals,
            overlap_islands: input.overlap_islands,
            overlap_policy_required_exits: input.overlap_policy_required_exits,
            overlap_ambiguous_contacts: input.overlap_ambiguous_contacts,
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

    pub fn projected_entity_count(self) -> usize {
        self.projected_entity_count
    }

    pub fn transform_step_count(self) -> usize {
        self.transform_step_count
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

    pub fn overlap_extraction_receipt_count(self) -> usize {
        self.overlap_extraction_receipt_count
    }

    pub fn overlap_candidate_pair_breadth(self) -> usize {
        self.overlap_candidate_pair_breadth
    }

    pub fn overlap_segment_contacts_certified(self) -> usize {
        self.overlap_segment_contacts_certified
    }

    pub fn overlap_shared_intervals(self) -> usize {
        self.overlap_shared_intervals
    }

    pub fn overlap_islands(self) -> usize {
        self.overlap_islands
    }

    pub fn overlap_policy_required_exits(self) -> usize {
        self.overlap_policy_required_exits
    }

    pub fn overlap_ambiguous_contacts(self) -> usize {
        self.overlap_ambiguous_contacts
    }
}

pub(crate) struct CoplanarOverlapStormCounterInput {
    pub(crate) topology_entity_count: usize,
    pub(crate) topology_face_count: usize,
    pub(crate) topology_relation_count: usize,
    pub(crate) projected_entity_count: usize,
    pub(crate) transform_step_count: usize,
    pub(crate) transform_cancellation_step_count: usize,
    pub(crate) retained_artifact_count: usize,
    pub(crate) replay_checkpoint_count: usize,
    pub(crate) operator_input_count: usize,
    pub(crate) operator_receipt_count: usize,
    pub(crate) overlap_extraction_receipt_count: usize,
    pub(crate) overlap_candidate_pair_breadth: usize,
    pub(crate) overlap_segment_contacts_certified: usize,
    pub(crate) overlap_shared_intervals: usize,
    pub(crate) overlap_islands: usize,
    pub(crate) overlap_policy_required_exits: usize,
    pub(crate) overlap_ambiguous_contacts: usize,
}
