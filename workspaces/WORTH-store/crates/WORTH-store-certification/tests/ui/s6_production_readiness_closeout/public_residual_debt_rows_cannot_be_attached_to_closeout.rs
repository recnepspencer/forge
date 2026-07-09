use worth_store_certification::{
    S6CertificationEvidenceAdoptionReceipt, S6ProductionReadinessClosureInput, S6ResidualDebtLedger,
};

fn main() {
    let adoption: S6CertificationEvidenceAdoptionReceipt = todo!();
    let ledger: S6ResidualDebtLedger = todo!();
    let _ = S6ProductionReadinessClosureInput::from_phase13_adoption(adoption)
        .with_residual_debt(ledger);
}
