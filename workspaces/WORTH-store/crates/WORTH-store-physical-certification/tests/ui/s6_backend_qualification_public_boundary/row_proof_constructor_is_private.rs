use worth_store_physical_backend::{AdmittedBackendCapabilityWitness, BackendCapabilityKind};
use worth_store_physical_certification::{
    BackendQualificationRow, QualificationHarnessProof, S6IoPressureHarnessEvidence,
};

fn main() {
    let witness: AdmittedBackendCapabilityWitness = todo!();
    let evidence: S6IoPressureHarnessEvidence = todo!();
    let proof: QualificationHarnessProof = todo!();
    let _ = BackendQualificationRow::from_admitted_backend_witness_with_proof(
        &witness,
        BackendCapabilityKind::Fsync,
        &evidence,
        proof,
    );
}
