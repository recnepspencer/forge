use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceStage, WorkloadEvidenceStageLookupCounters,
};

use super::identity::stage_link_set_identity;
use super::link::WorkloadEvidenceStageLink;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceStageLinkSet {
    stage_index_identity: String,
    links: Vec<WorkloadEvidenceStageLink>,
    stage_offsets: [Option<usize>; WorkloadEvidenceStage::STAGE_COUNT],
    lookup_counters: WorkloadEvidenceStageLookupCounters,
    link_set_identity: String,
}

impl WorkloadEvidenceStageLinkSet {
    pub(crate) fn new(stage_index_identity: String, links: Vec<WorkloadEvidenceStageLink>) -> Self {
        let stage_offsets = build_stage_offsets(&links);
        let lookup_counters = WorkloadEvidenceStageLookupCounters::indexed(links.len());
        let link_set_identity = stage_link_set_identity(&stage_index_identity, &links);
        Self {
            stage_index_identity,
            links,
            stage_offsets,
            lookup_counters,
            link_set_identity,
        }
    }

    pub fn stage_index_identity(&self) -> &str {
        &self.stage_index_identity
    }

    pub fn link_set_identity(&self) -> &str {
        &self.link_set_identity
    }

    pub fn links(&self) -> &[WorkloadEvidenceStageLink] {
        &self.links
    }

    pub fn lookup_counters(&self) -> WorkloadEvidenceStageLookupCounters {
        self.lookup_counters
    }

    pub fn link_for_stage(
        &self,
        stage: WorkloadEvidenceStage,
    ) -> Option<&WorkloadEvidenceStageLink> {
        self.stage_offsets
            .get(stage.index_slot())
            .and_then(|offset| offset.map(|link_index| &self.links[link_index]))
    }

    pub fn links_to(&self, stage: WorkloadEvidenceStage) -> bool {
        self.link_for_stage(stage).is_some()
    }

    pub fn links_to_identity(&self, stage: WorkloadEvidenceStage, evidence_identity: &str) -> bool {
        self.link_for_stage(stage)
            .is_some_and(|link| link.evidence_identity() == evidence_identity)
    }

    pub fn evidence_identities(&self) -> Vec<String> {
        self.links
            .iter()
            .map(|link| link.evidence_identity().to_string())
            .collect()
    }
}

fn build_stage_offsets(
    links: &[WorkloadEvidenceStageLink],
) -> [Option<usize>; WorkloadEvidenceStage::STAGE_COUNT] {
    let mut stage_offsets = [None; WorkloadEvidenceStage::STAGE_COUNT];
    for (link_index, link) in links.iter().enumerate() {
        stage_offsets[link.stage().index_slot()] = Some(link_index);
    }
    stage_offsets
}
