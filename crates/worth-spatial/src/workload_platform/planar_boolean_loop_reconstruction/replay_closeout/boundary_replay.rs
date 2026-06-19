use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopIslandPartition, PlanarBooleanReconstructedLoopBoundary,
    PlanarBooleanSourceLoopSplitAttribution,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionReplayCounters {
    compared_reconstructed_loops: usize,
    compared_born_loops: usize,
    compared_island_partitions: usize,
    compared_split_attributions: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopReconstructionReplayDenialKind {
    ReconstructedLoopMismatch,
    BornLoopMismatch,
    IslandPartitionMismatch,
    SplitAttributionMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionReplayDenial {
    kind: PlanarBooleanLoopReconstructionReplayDenialKind,
    counters: PlanarBooleanLoopReconstructionReplayCounters,
}

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanLoopReconstructionReplayInput<'a> {
    original: &'a PlanarBooleanReconstructedLoopBoundary,
    replayed: &'a PlanarBooleanReconstructedLoopBoundary,
    original_partition: &'a PlanarBooleanLoopIslandPartition,
    replayed_partition: &'a PlanarBooleanLoopIslandPartition,
    original_split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
    replayed_split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionReplayReceipt {
    replay_identity: String,
    counters: PlanarBooleanLoopReconstructionReplayCounters,
}

pub struct ComparePlanarBooleanLoopReconstructionReplay;

impl PlanarBooleanLoopReconstructionReplayCounters {
    fn compared_reconstructed_loops(&mut self) {
        self.compared_reconstructed_loops += 1;
    }

    fn compared_born_loops(&mut self) {
        self.compared_born_loops += 1;
    }

    fn compared_island_partitions(&mut self) {
        self.compared_island_partitions += 1;
    }

    fn compared_split_attributions(&mut self) {
        self.compared_split_attributions += 1;
    }
}

impl PlanarBooleanLoopReconstructionReplayDenial {
    fn new(
        kind: PlanarBooleanLoopReconstructionReplayDenialKind,
        counters: PlanarBooleanLoopReconstructionReplayCounters,
    ) -> Self {
        Self { kind, counters }
    }

    pub fn kind(&self) -> PlanarBooleanLoopReconstructionReplayDenialKind {
        self.kind
    }

    pub fn counters(&self) -> PlanarBooleanLoopReconstructionReplayCounters {
        self.counters
    }
}

impl<'a> PlanarBooleanLoopReconstructionReplayInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_boundaries(
        original: &'a PlanarBooleanReconstructedLoopBoundary,
        replayed: &'a PlanarBooleanReconstructedLoopBoundary,
        original_partition: &'a PlanarBooleanLoopIslandPartition,
        replayed_partition: &'a PlanarBooleanLoopIslandPartition,
        original_split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
        replayed_split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
    ) -> Self {
        Self {
            original,
            replayed,
            original_partition,
            replayed_partition,
            original_split_attribution,
            replayed_split_attribution,
        }
    }

    fn original(self) -> &'a PlanarBooleanReconstructedLoopBoundary {
        self.original
    }

    fn replayed(self) -> &'a PlanarBooleanReconstructedLoopBoundary {
        self.replayed
    }

    fn original_partition(self) -> &'a PlanarBooleanLoopIslandPartition {
        self.original_partition
    }

    fn replayed_partition(self) -> &'a PlanarBooleanLoopIslandPartition {
        self.replayed_partition
    }

    fn original_split_attribution(self) -> &'a PlanarBooleanSourceLoopSplitAttribution {
        self.original_split_attribution
    }

    fn replayed_split_attribution(self) -> &'a PlanarBooleanSourceLoopSplitAttribution {
        self.replayed_split_attribution
    }
}

impl PlanarBooleanLoopReconstructionReplayReceipt {
    fn new(
        replay_identity: String,
        counters: PlanarBooleanLoopReconstructionReplayCounters,
    ) -> Self {
        Self {
            replay_identity,
            counters,
        }
    }

    pub fn replay_identity(&self) -> &str {
        &self.replay_identity
    }

    pub fn counters(&self) -> PlanarBooleanLoopReconstructionReplayCounters {
        self.counters
    }
}

impl ComparePlanarBooleanLoopReconstructionReplay {
    pub fn compare(
        input: PlanarBooleanLoopReconstructionReplayInput<'_>,
    ) -> Result<
        PlanarBooleanLoopReconstructionReplayReceipt,
        PlanarBooleanLoopReconstructionReplayDenial,
    > {
        let mut counters = PlanarBooleanLoopReconstructionReplayCounters::default();
        counters.compared_reconstructed_loops();
        if input.original().reconstructed_loops().rows()
            != input.replayed().reconstructed_loops().rows()
        {
            return Err(PlanarBooleanLoopReconstructionReplayDenial::new(
                PlanarBooleanLoopReconstructionReplayDenialKind::ReconstructedLoopMismatch,
                counters,
            ));
        }
        counters.compared_born_loops();
        if input.original().born_loops().rows() != input.replayed().born_loops().rows() {
            return Err(PlanarBooleanLoopReconstructionReplayDenial::new(
                PlanarBooleanLoopReconstructionReplayDenialKind::BornLoopMismatch,
                counters,
            ));
        }
        counters.compared_island_partitions();
        if input.original_partition().rows() != input.replayed_partition().rows() {
            return Err(PlanarBooleanLoopReconstructionReplayDenial::new(
                PlanarBooleanLoopReconstructionReplayDenialKind::IslandPartitionMismatch,
                counters,
            ));
        }
        counters.compared_split_attributions();
        if input.original_split_attribution().rows() != input.replayed_split_attribution().rows() {
            return Err(PlanarBooleanLoopReconstructionReplayDenial::new(
                PlanarBooleanLoopReconstructionReplayDenialKind::SplitAttributionMismatch,
                counters,
            ));
        }

        let replay_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-loop-reconstruction-replay".to_string(),
                format!(
                    "reconstructed:{}",
                    input
                        .original()
                        .reconstructed_loops()
                        .reconstructed_loop_set_identity()
                ),
                format!(
                    "born:{}",
                    input.original().born_loops().born_loop_set_identity()
                ),
                format!(
                    "islands:{}",
                    input.original_partition().partition_identity()
                ),
                format!(
                    "split-attribution:{}",
                    input.original_split_attribution().attribution_identity()
                ),
            ],
        );

        Ok(PlanarBooleanLoopReconstructionReplayReceipt::new(
            replay_identity,
            counters,
        ))
    }
}
