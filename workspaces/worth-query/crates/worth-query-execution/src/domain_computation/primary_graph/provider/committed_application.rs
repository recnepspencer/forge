use worth_relational::facade::history::CommitId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryPrimaryGraphCommittedApplication {
    runtime_instance_id: u64,
    commit_id: CommitId,
    changed_record_count: usize,
    emitted_effect_count: usize,
}

impl WorthQueryPrimaryGraphCommittedApplication {
    pub(in crate::domain_computation::primary_graph) const fn new(
        runtime_instance_id: u64,
        commit_id: CommitId,
        changed_record_count: usize,
        emitted_effect_count: usize,
    ) -> Self {
        Self {
            runtime_instance_id,
            commit_id,
            changed_record_count,
            emitted_effect_count,
        }
    }

    pub(in crate::domain_computation::primary_graph) const fn runtime_instance_id(self) -> u64 {
        self.runtime_instance_id
    }

    pub(in crate::domain_computation::primary_graph) const fn commit_id(self) -> CommitId {
        self.commit_id
    }

    pub(in crate::domain_computation::primary_graph) const fn changed_record_count(self) -> usize {
        self.changed_record_count
    }

    pub(in crate::domain_computation::primary_graph) const fn emitted_effect_count(self) -> usize {
        self.emitted_effect_count
    }
}
