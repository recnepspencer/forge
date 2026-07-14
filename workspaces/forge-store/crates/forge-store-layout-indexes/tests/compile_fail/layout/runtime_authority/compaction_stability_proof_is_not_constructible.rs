use forge_store_physical_isolation::{
    CompactionCutoverStabilityProof, CompactionRewritePublication,
};
use forge_store_recovery_physics::CompactionCutoverRecoveryPosture;

#[allow(dead_code)]
fn forge(
    publication: CompactionRewritePublication,
    recovery_posture: CompactionCutoverRecoveryPosture,
) {
    let _ = CompactionCutoverStabilityProof {
        publication,
        recovery_posture,
    };
}

fn main() {}
