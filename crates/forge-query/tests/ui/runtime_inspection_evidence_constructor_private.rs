use forge_query::facade::{ForgeQueryAuthorityLane, ForgeQueryRuntimeInspectionEvidence};

fn main() {
    let _ = ForgeQueryRuntimeInspectionEvidence {
        artifact_family: String::new(),
        authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        evidence: Vec::new(),
    };
}
