use forge_store::{
    SupportDomainCertificationBundle, SupportDomainCertificationCounterSnapshot,
};

fn main() {
    let _ = SupportDomainCertificationBundle {
        rows: Vec::new(),
        batch_plan: forged_plan(),
        counter_snapshot: SupportDomainCertificationCounterSnapshot::new(0, 0, 0, 0, 0, 0, 0),
        domain_certification_digest: String::new(),
    };
}

fn forged_plan() -> forge_store::SupportDomainCertificationBatchPlan {
    panic!("constructor privacy check only")
}
