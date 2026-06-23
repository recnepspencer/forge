use super::construction::partition_loop_islands;
use super::counters::PlanarBooleanLoopIslandPartitionCounters;
use super::input::PlanarBooleanLoopIslandPartitionInput;
use super::row::PlanarBooleanLoopIslandPartitionRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopIslandPartition {
    partition_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanLoopIslandPartitionRow>,
    counters: PlanarBooleanLoopIslandPartitionCounters,
}

impl PlanarBooleanLoopIslandPartition {
    pub fn partition(input: PlanarBooleanLoopIslandPartitionInput<'_>) -> Self {
        partition_loop_islands(input)
    }

    pub(crate) fn new(
        partition_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanLoopIslandPartitionRow>,
        counters: PlanarBooleanLoopIslandPartitionCounters,
    ) -> Self {
        Self {
            partition_identity,
            request_identity,
            rows,
            counters,
        }
    }

    pub fn partition_identity(&self) -> &str {
        &self.partition_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopIslandPartitionRow] {
        &self.rows
    }

    pub fn counters(&self) -> PlanarBooleanLoopIslandPartitionCounters {
        self.counters
    }
}
