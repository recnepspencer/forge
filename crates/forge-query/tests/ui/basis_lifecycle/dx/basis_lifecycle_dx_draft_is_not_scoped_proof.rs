use forge_query::facade::{basis_lifecycle, readmit_lower_runtime_evidence, LowerRuntimeBasisEvidence};

fn main() {
    let draft = basis_lifecycle().current_head();
    let _ = readmit_lower_runtime_evidence(
        draft,
        LowerRuntimeBasisEvidence::from_runtime_basis("runtime-current-head", "evidence", 1),
    );
}
