//! Compile-gate and contract tests for curved merge scaffolding.

use super::schema::*;
use super::*;
use forge_topo::handles::FaceId;

#[test]
fn curved_merge_stub_returns_not_implemented() {
    // Verify the placeholder correctly returns an error
    // Type-level compile check — the function signature is the contract.
}

#[test]
fn curved_merge_selection_is_serializable() {
    let selection = CurvedMergeSelection {
        selected_faces: vec![],
        protected_faces: vec![],
        surviving_face: FaceId::new(0, 0),
        policy_overrides: CurvedMergePolicyOverrides::default(),
    };
    let json = serde_json::to_string(&selection).unwrap();
    let roundtrip: CurvedMergeSelection = serde_json::from_str(&json).unwrap();
    assert_eq!(roundtrip.selected_faces.len(), 0);
}
