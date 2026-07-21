//! Identity match graph builder — orchestration plus named semantic steps.
//!
//! guard admission → index nodes → classify kinds → build match graph → report

mod admission_guards;
mod denial_assembly;
mod index_nodes;
mod match_graph;
mod structure_digests;
mod types;

use crate::runtime::active::WorthUiActiveArtifact;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiIdentityMatchCounters, WorthUiIdentityMatchDenial,
    WorthUiIdentityMatchNodeSide, WorthUiIdentityMatchReport, WorthUiRuntimeImpactNarrowing,
};

use admission_guards::{reject_mismatched_active_basis, reject_mismatched_candidate};
use index_nodes::index_artifact_nodes;
use match_graph::build_match_graph;

#[derive(Clone, Debug, Default)]
pub struct WorthUiIdentityMatchGraphBuilder;

impl WorthUiIdentityMatchGraphBuilder {
    pub(crate) fn build(
        active_artifact: &WorthUiActiveArtifact,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiIdentityMatchReport, WorthUiIdentityMatchDenial> {
        let mut counters = WorthUiIdentityMatchCounters::default();
        reject_mismatched_active_basis(active_artifact, narrowing, counters)?;
        reject_mismatched_candidate(narrowing, admitted, counters)?;

        let active_index = index_artifact_nodes(
            active_artifact.artifact(),
            WorthUiIdentityMatchNodeSide::Active,
            &mut counters,
        )?;
        let candidate_index = index_artifact_nodes(
            admitted.artifact_bundle().artifact(),
            WorthUiIdentityMatchNodeSide::Candidate,
            &mut counters,
        )?;
        let graph = build_match_graph(active_index, candidate_index, counters)?;

        Ok(WorthUiIdentityMatchReport::new(
            narrowing.active_artifact_digest(),
            narrowing.candidate_artifact_digest(),
            graph,
        ))
    }
}
