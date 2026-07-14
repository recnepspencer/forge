use worth_store_physical_isolation::{
    CompactionCutoverStabilityProof, CompactionRewritePublication,
};
use worth_store_recovery_physics::CompactionCutoverRecoveryPosture;

#[allow(dead_code)]
fn worth(
    publication: CompactionRewritePublication,
    recovery_posture: CompactionCutoverRecoveryPosture,
) {
    let _ = CompactionCutoverStabilityProof {
        publication,
        recovery_posture,
    };
}

fn main() {}
