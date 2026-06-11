#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapPerformanceCounters {
    candidate_pair_breadth: usize,
    segment_contacts_certified: usize,
    overlap_islands: usize,
    shared_intervals: usize,
    containment_relations: usize,
    policy_required_exits: usize,
    basis_part_count: usize,
}

impl CoplanarOverlapPerformanceCounters {
    pub(crate) fn certified(
        candidate_pair_breadth: usize,
        segment_contacts_certified: usize,
        overlap_islands: usize,
        shared_intervals: usize,
        containment_relations: usize,
        policy_required_exits: usize,
        basis_part_count: usize,
    ) -> Self {
        Self {
            candidate_pair_breadth,
            segment_contacts_certified,
            overlap_islands,
            shared_intervals,
            containment_relations,
            policy_required_exits,
            basis_part_count,
        }
    }

    pub fn candidate_pair_breadth(self) -> usize {
        self.candidate_pair_breadth
    }

    pub fn segment_contacts_certified(self) -> usize {
        self.segment_contacts_certified
    }

    pub fn overlap_islands(self) -> usize {
        self.overlap_islands
    }

    pub fn shared_intervals(self) -> usize {
        self.shared_intervals
    }

    pub fn containment_relations(self) -> usize {
        self.containment_relations
    }

    pub fn policy_required_exits(self) -> usize {
        self.policy_required_exits
    }

    pub fn basis_part_count(self) -> usize {
        self.basis_part_count
    }
}
