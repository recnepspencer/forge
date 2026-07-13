use worth_query::facade::runtime::{WorthQueryAuthorityLane, WorthQueryRuntimeInspectionEvidence};

fn main() {
    let _ = WorthQueryRuntimeInspectionEvidence {
        artifact_family: String::new(),
        authority_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
        evidence: Vec::new(),
    };
}
