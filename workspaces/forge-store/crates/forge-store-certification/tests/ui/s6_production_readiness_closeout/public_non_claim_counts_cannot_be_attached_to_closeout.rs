use forge_store_certification::{
    S6CertificationEvidenceAdoptionReceipt, S6ProductionReadinessClosureInput,
};

struct CallerSuppliedNonClaimCounts;

fn main() {
    let adoption: S6CertificationEvidenceAdoptionReceipt = todo!();
    let _ = S6ProductionReadinessClosureInput::from_phase13_adoption(adoption)
        .with_later_milestone_non_claims(CallerSuppliedNonClaimCounts);
}
