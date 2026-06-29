use worth_kernel::workload_composition::{BatchAdmissionExecutionCounters, BatchAdmissionExecutionReceipt, BatchAdmissionFamilyPosture};

fn main() {
    let _ = BatchAdmissionExecutionReceipt {
        execution_receipt_digest: String::new(),
        selected_batch_plan_digest: String::new(),
        posture: BatchAdmissionFamilyPosture::ParallelAdmit,
        participant_identities: Vec::new(),
        selected_conflict_plan_identities: Vec::new(),
        independence_proof_identities: Vec::new(),
        selected_family_rows: Vec::new(),
        supporting_conflict_family_rows: Vec::new(),
        advisory: None,
        denial: None,
        counters: fake_counters(),
    };
}

fn fake_counters() -> BatchAdmissionExecutionCounters {
    panic!("private fields should reject construction before this helper matters");
}
