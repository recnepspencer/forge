use std::collections::{BTreeMap, BTreeSet};

use crate::classification::{ProofDisposition, ProofFamily, ProofOwner};
use crate::preservation::{
    validate_current_reachability, validate_ledger, ProofPreservationLedger, ProofPreservationRow,
    ProofReplacementRecord,
};

use super::{current_inventory, workspace_root};

#[test]
fn replacement_requires_reachable_predicate_parity() {
    let current = current_inventory(&workspace_root());
    let replacement = current
        .inventory()
        .proofs
        .iter()
        .find(|proof| !proof.case.assertion_predicates.is_empty())
        .unwrap();
    let replacement_predicate = replacement.case.assertion_predicates[0].clone();
    let old_predicate = "old rejection assertion".to_owned();
    let mut row = ProofPreservationRow {
        stable_case_id: "removed::proof::identity".to_owned(),
        owner: ProofOwner {
            package: replacement.case.identity.package.clone(),
            responsibility: replacement.case.identity.responsibility.clone(),
        },
        family: ProofFamily::CrossOwnerIntegration,
        products: BTreeSet::from(["store-ci:replacement-mutant".to_owned()]),
        disposition: ProofDisposition::ReplaceWithStrongerProof,
        assertion_predicates: vec![old_predicate.clone()],
        original_target_identity: None,
        admitted_target_identity: None,
        physical_reality_audit_required: false,
        amendment_rationale: String::new(),
        quarantine: None,
        replacement: Some(ProofReplacementRecord {
            replacement_case_id: replacement.case.identity.stable_id.clone(),
            predicate_parity: BTreeMap::from([(old_predicate.clone(), "missing".to_owned())]),
        }),
    };
    let ledger = ProofPreservationLedger {
        schema_version: 1,
        rows: vec![row.clone()],
    };
    assert!(validate_current_reachability(&ledger, &current, &BTreeSet::new()).is_err());

    row.replacement.as_mut().unwrap().predicate_parity =
        BTreeMap::from([(old_predicate, replacement_predicate)]);
    row.products = replacement.products.clone();
    let ledger = ProofPreservationLedger {
        schema_version: 1,
        rows: vec![row],
    };
    assert!(validate_current_reachability(&ledger, &current, &BTreeSet::new()).is_ok());
}

#[test]
fn executable_suffix_cannot_forge_historical_aggregate_status() {
    let current = current_inventory(&workspace_root());
    let proof = &current.inventory().proofs[0];
    let ledger = ProofPreservationLedger {
        schema_version: 1,
        rows: vec![ProofPreservationRow {
            stable_case_id: "legitimate::owner::case::executable".to_owned(),
            owner: proof.owner.clone(),
            family: proof.family,
            products: proof.products.clone(),
            disposition: ProofDisposition::PreserveUnchanged,
            assertion_predicates: vec!["must_remain_reachable".to_owned()],
            original_target_identity: proof.case.target_identity.clone(),
            admitted_target_identity: None,
            physical_reality_audit_required: false,
            amendment_rationale: String::new(),
            quarantine: None,
            replacement: None,
        }],
    };
    let denials = validate_current_reachability(&ledger, &current, &BTreeSet::new()).unwrap_err();
    assert!(denials
        .iter()
        .any(|denial| denial.contains("legitimate::owner::case::executable")));
}

#[test]
fn replacement_disposition_without_authority_is_rejected() {
    let current = current_inventory(&workspace_root());
    let proof = &current.inventory().proofs[0];
    let ledger = ProofPreservationLedger {
        schema_version: 1,
        rows: vec![ProofPreservationRow {
            stable_case_id: proof.case.identity.stable_id.clone(),
            owner: proof.owner.clone(),
            family: proof.family,
            products: proof.products.clone(),
            disposition: ProofDisposition::ReplaceWithStrongerProof,
            assertion_predicates: proof.case.assertion_predicates.clone(),
            original_target_identity: proof.case.target_identity.clone(),
            admitted_target_identity: None,
            physical_reality_audit_required: false,
            amendment_rationale: String::new(),
            quarantine: None,
            replacement: None,
        }],
    };
    let denials = validate_ledger(&current, &ledger).unwrap_err();
    assert!(denials.iter().any(|denial| {
        denial.contains("replacement disposition lacks replacement authority")
            && denial.contains(&proof.case.identity.stable_id)
    }));
}
