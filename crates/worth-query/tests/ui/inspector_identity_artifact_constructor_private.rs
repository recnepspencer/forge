use worth_query::facade::{
    BranchLocalityClass, InspectorIdentityArtifact, InspectorIdentityClassification,
    InspectorIdentityDigest,
};

fn main() {
    let _ = InspectorIdentityArtifact {
        digest: InspectorIdentityDigest::from_parts(&["identity".to_string()]),
        classification: InspectorIdentityClassification::IdentityBreak,
        branch_locality_class: BranchLocalityClass::CrossBranchAuthoritative,
        authoritative: false,
        identity_break: true,
        replay_stable_digest: "identity-break".to_string(),
    };
}
