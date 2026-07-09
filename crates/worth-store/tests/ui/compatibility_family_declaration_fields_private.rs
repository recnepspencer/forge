use worth_store::{
    ArtifactFamilyId, CompatibilityAuthorityClassification, CompatibilityFamilyDeclaration,
    CompatibilityFamilyKind,
};

fn main() {
    let _ = CompatibilityFamilyDeclaration {
        kind: CompatibilityFamilyKind::CommitEnvelope,
        family_id: ArtifactFamilyId::new("commit_envelope"),
        authority_classification: CompatibilityAuthorityClassification::Authoritative,
        manifest: unreachable!(),
        restore_posture: String::new(),
        rolling_posture: String::new(),
        counter_family_id: String::new(),
        certification_lane_id: String::new(),
    };
}
