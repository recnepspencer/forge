use forge_query::facade::{LowerRuntimeBasisEvidence, LowerRuntimeEvidenceAuthority};

fn main() {
    let _ = LowerRuntimeBasisEvidence {
        authority: LowerRuntimeEvidenceAuthority::Runtime,
        basis_digest: String::new(),
        evidence_digest: String::new(),
        retained_evidence_lookup_width: 0,
        stale_runtime_snapshot: false,
        missing_signal_observation: false,
        unsupported_capability: false,
    };
}
