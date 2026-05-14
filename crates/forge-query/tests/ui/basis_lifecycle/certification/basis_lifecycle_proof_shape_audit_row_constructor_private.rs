use forge_query::facade::{
    BasisLifecycleProofShapeAuditRow, BasisLifecycleProofShapeEnforcement,
    BasisLifecycleProofShapeViolation,
};

fn main() {
    let _row = BasisLifecycleProofShapeAuditRow {
        violation: BasisLifecycleProofShapeViolation::PhaseSkipping,
        attempted_shortcut: "skip",
        required_prior_artifact: "admitted capability",
        rejected_artifact: "draft",
        enforcement: BasisLifecycleProofShapeEnforcement::CompileFailFixture,
        enforcement_proof: "compile_fail",
        row_digest: String::new(),
    };
}
