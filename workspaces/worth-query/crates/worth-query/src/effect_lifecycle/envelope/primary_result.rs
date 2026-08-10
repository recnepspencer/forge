#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectEnvelopePrimaryResult {
    MutationCommitted,
    MergeCommitted,
    WritebackCommitted,
    BatchMutationCommitted,
}

impl EffectEnvelopePrimaryResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MutationCommitted => "mutation_committed",
            Self::MergeCommitted => "merge_committed",
            Self::WritebackCommitted => "writeback_committed",
            Self::BatchMutationCommitted => "batch_mutation_committed",
        }
    }
}
