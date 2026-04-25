use forge_store::{SupportCertificationCoverageWitness, SupportCertificationSummary};

fn main() {
    let _ = SupportCertificationCoverageWitness {
        summary: forged_summary(),
    };
}

fn forged_summary() -> SupportCertificationSummary {
    loop {}
}
