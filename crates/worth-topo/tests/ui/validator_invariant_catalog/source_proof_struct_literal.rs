use topology::facade::WorthTopologyLegalityFamilySourceProof;

fn main() {
    let _ = WorthTopologyLegalityFamilySourceProof {
        authority_kind: panic!("private source authority unavailable"),
        source_identity_digest: String::new(),
        rule_name: String::new(),
        semantic_version: String::new(),
        execution_point: None,
        applicability_digest: String::new(),
        enforcement_phase: panic!("private enforcement phase unavailable"),
        witness_posture: panic!("private witness posture unavailable"),
        proof_digest: String::new(),
    };
}
