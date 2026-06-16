use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceStage, WorkloadEvidenceStageBinding, WorkloadEvidenceStageCounters,
    WorkloadEvidenceStageLink, WorkloadEvidenceStageLinkSet,
};

fn main() {
    let _binding = WorkloadEvidenceStageBinding {
        upstream_stage: WorkloadEvidenceStage::Projection,
        upstream_evidence_identity: "projection".to_string(),
    };
    let link = WorkloadEvidenceStageLink {
        stage: WorkloadEvidenceStage::Projection,
        evidence_identity: "projection".to_string(),
        link_identity: "link".to_string(),
        counters: WorkloadEvidenceStageCounters::default(),
    };
    let _ = WorkloadEvidenceStageLinkSet {
        stage_index_identity: "index".to_string(),
        links: vec![link],
        stage_offsets: [None; 26],
        link_set_identity: "set".to_string(),
    };
}
