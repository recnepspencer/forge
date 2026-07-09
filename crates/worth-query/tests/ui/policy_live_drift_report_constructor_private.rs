use worth_query::facade::{
    PolicyLiveDensityEvidence, PolicyLiveDriftEvidenceReport, PolicyLiveEpochEvidence,
};

fn main() {
    let _ = PolicyLiveDriftEvidenceReport {
        epoch_evidence: PolicyLiveEpochEvidence::new("p1", "t1", "p1", "t1"),
        density_evidence: PolicyLiveDensityEvidence::new(2, 1, 1),
        digest: "fabricated".to_string(),
    };
}
