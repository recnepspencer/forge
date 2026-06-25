use topology::facade::WorthTopologyValidatorFamilyRecord;

fn main() {
    let _ = WorthTopologyValidatorFamilyRecord {
        identity: panic!("private identity unavailable"),
        query_obligation_kind: panic!("private Query kind unavailable"),
        touched_applicability: panic!("private applicability unavailable"),
        required_access_posture: panic!("private access posture unavailable"),
        enforcement_phase: panic!("private enforcement phase unavailable"),
        witness_posture: panic!("private witness posture unavailable"),
        diagnostic_projection: panic!("private diagnostic projection unavailable"),
        query_support_posture: panic!("private support posture unavailable"),
        family_digest: String::new(),
    };
}
