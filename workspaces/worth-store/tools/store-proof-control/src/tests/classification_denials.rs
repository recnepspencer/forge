use crate::classification::{validate, ProofFamily};

use super::{current_inventory, workspace_root};

#[test]
fn duplicate_identity_and_owner_leakage_are_denied() {
    let current = current_inventory(&workspace_root());

    let mut duplicate = current.inventory().clone();
    duplicate.proofs.push(duplicate.proofs[0].clone());
    let duplicate_denials =
        validate(crate::ClassifiedProofInventory::from_discovered(duplicate)).unwrap_err();
    assert!(duplicate_denials
        .iter()
        .any(|denial| denial.contains("duplicate proof identity")));

    let mut leaked = current.inventory().clone();
    let proof = leaked
        .proofs
        .iter_mut()
        .find(|proof| {
            matches!(
                proof.family,
                ProofFamily::OwnerBehavior | ProofFamily::OwnerInvariant
            )
        })
        .unwrap();
    proof.owner.package = "worth-store-test-support".to_owned();
    let leakage_denials =
        validate(crate::ClassifiedProofInventory::from_discovered(leaked)).unwrap_err();
    assert!(leakage_denials
        .iter()
        .any(|denial| denial.contains("owner-local proof") && denial.contains("leaked")));
}

#[test]
fn hidden_source_and_unregistered_ui_fixture_are_denied() {
    let current = current_inventory(&workspace_root());

    let mut hidden = current.inventory().clone();
    let hidden_path = hidden.proofs[0].case.source_path.clone();
    hidden.proofs[0].case.current_invocation = "unregistered".to_owned();
    hidden.proofs[0].case.registration_authority = "unregistered".to_owned();
    let hidden_denials =
        validate(crate::ClassifiedProofInventory::from_discovered(hidden)).unwrap_err();
    assert!(hidden_denials
        .iter()
        .any(|denial| denial.contains(&hidden_path)));

    let mut ignored_ui = current.inventory().clone();
    let ui = ignored_ui
        .proofs
        .iter_mut()
        .find(|proof| proof.case.kind == crate::discovery::CaseKind::UiFixture)
        .unwrap();
    let ui_path = ui.case.source_path.clone();
    ui.case.target_identity = None;
    ui.case.registration_authority = "unregistered".to_owned();
    let ui_denials =
        validate(crate::ClassifiedProofInventory::from_discovered(ignored_ui)).unwrap_err();
    assert!(ui_denials.iter().any(|denial| denial.contains(&ui_path)));
}
