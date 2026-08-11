use crate::policy_narrowing::{
    classify_saved_policy_narrowing_reuse, SavedPolicyNarrowingReuseDescriptor,
    SavedPolicyNarrowingReuseDisposition,
};

#[test]
fn saved_policy_narrowing_reuse_requires_exact_projection_and_proof_match() {
    let exact = SavedPolicyNarrowingReuseDescriptor::new(
        "saved-a",
        "narrowed-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
    );
    let fresh = SavedPolicyNarrowingReuseDescriptor::new(
        "saved-a",
        "narrowed-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-b",
        "proof-a",
    );
    let drift = SavedPolicyNarrowingReuseDescriptor::new(
        "saved-a",
        "narrowed-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
        "policy-b",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
    );

    assert_eq!(
        classify_saved_policy_narrowing_reuse(&exact),
        SavedPolicyNarrowingReuseDisposition::LegalNoSemanticChange
    );
    assert_eq!(
        classify_saved_policy_narrowing_reuse(&fresh),
        SavedPolicyNarrowingReuseDisposition::LegalRequiresFreshNarrowing
    );
    assert_eq!(
        classify_saved_policy_narrowing_reuse(&drift),
        SavedPolicyNarrowingReuseDisposition::IllegalSemanticDrift
    );
}
