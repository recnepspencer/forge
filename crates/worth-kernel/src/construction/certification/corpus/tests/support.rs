use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};

use crate::construction::certification::corpus::{
    prepare_primitive_construction_corpus_replay_siege,
    PrimitiveConstructionCorpusReplaySiegeReport,
};
use forge_query::facade::ForgeQueryWorkspace;

pub(super) fn siege_report(label: &str) -> PrimitiveConstructionCorpusReplaySiegeReport {
    let mut workspace = siege_workspace(label);
    prepare_primitive_construction_corpus_replay_siege(&mut workspace).expect("siege report")
}

pub(super) fn siege_workspace(label: &str) -> ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        format!("worth-kernel.{label}"),
    )
    .expect("workspace")
}
