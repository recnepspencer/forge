use worth_store_recovery_physics::{
    FoundationalRecoveryEvidenceBundle, RecoveredPhysicalState,
};

fn requires_recovered_state(_: RecoveredPhysicalState) {}

fn main() {
    let evidence: FoundationalRecoveryEvidenceBundle = todo!();
    requires_recovered_state(evidence);
}
