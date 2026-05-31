use super::mutation_records::TopologyDeclaredMutationRecord;
use super::TopologyDeclaredMutationSequence;

pub(crate) struct TopologyDeclaredMutationSequenceBuilder {
    pub(in crate::topology_operators) records: Vec<TopologyDeclaredMutationRecord>,
}

impl TopologyDeclaredMutationSequenceBuilder {
    pub(crate) fn builder() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub(crate) fn finish(self) -> TopologyDeclaredMutationSequence {
        TopologyDeclaredMutationSequence::new(self.records)
    }
}
