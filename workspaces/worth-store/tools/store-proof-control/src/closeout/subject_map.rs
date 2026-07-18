use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::classification::ProofOwner;
use crate::ValidatedProofInventory;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProductionSubject {
    pub stable_case_id: String,
    pub owner: ProofOwner,
    pub source_path: String,
    pub invocation: String,
    pub products: BTreeSet<String>,
}

pub(super) fn production_subjects(inventory: &ValidatedProofInventory) -> Vec<ProductionSubject> {
    let mut subjects: Vec<_> = inventory
        .inventory()
        .proofs
        .iter()
        .filter(|proof| proof.physical_reality_audit_required)
        .map(|proof| ProductionSubject {
            stable_case_id: proof.case.identity.stable_id.clone(),
            owner: proof.owner.clone(),
            source_path: proof.case.source_path.clone(),
            invocation: proof.case.current_invocation.clone(),
            products: proof.products.clone(),
        })
        .collect();
    subjects.sort();
    subjects
}
