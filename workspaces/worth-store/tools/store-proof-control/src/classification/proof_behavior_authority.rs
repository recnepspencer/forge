use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::ClassifiedInventory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofBehaviorAuthority {
    pub schema_version: u32,
    pub declarations: Vec<ProofBehaviorDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofBehaviorDeclaration {
    pub stable_case_id: String,
    pub behavior_fingerprint: String,
}

impl ProofBehaviorAuthority {
    pub fn from_inventory(inventory: &ClassifiedInventory) -> Self {
        let mut declarations: Vec<_> = inventory
            .proofs
            .iter()
            .filter(|proof| proof.case.kind != crate::discovery::CaseKind::TestExecutable)
            .map(|proof| ProofBehaviorDeclaration {
                stable_case_id: proof.case.identity.stable_id.clone(),
                behavior_fingerprint: proof.case.behavior_fingerprint.clone(),
            })
            .collect();
        declarations.sort_by(|left, right| left.stable_case_id.cmp(&right.stable_case_id));
        Self {
            schema_version: 1,
            declarations,
        }
    }
}

pub fn validate_proof_behavior_authority(
    authority: &ProofBehaviorAuthority,
    inventory: &ClassifiedInventory,
) -> Result<(), Vec<String>> {
    let mut violations = Vec::new();
    if authority.schema_version != 1 {
        violations.push(format!(
            "unsupported proof behavior authority schema: {}",
            authority.schema_version
        ));
    }
    let mut declared = BTreeMap::new();
    for declaration in &authority.declarations {
        if declared
            .insert(
                declaration.stable_case_id.as_str(),
                declaration.behavior_fingerprint.as_str(),
            )
            .is_some()
        {
            violations.push(format!(
                "proof behavior authority duplicates identity: {}",
                declaration.stable_case_id
            ));
        }
    }
    let current: BTreeMap<_, _> = inventory
        .proofs
        .iter()
        .filter(|proof| proof.case.kind != crate::discovery::CaseKind::TestExecutable)
        .map(|proof| {
            (
                proof.case.identity.stable_id.as_str(),
                proof.case.behavior_fingerprint.as_str(),
            )
        })
        .collect();
    for (identity, fingerprint) in &current {
        match declared.get(identity) {
            None => violations.push(format!(
                "proof has no sealed current behavior fingerprint: {identity}"
            )),
            Some(expected) if expected != fingerprint => violations.push(format!(
                "proof behavior drifted from sealed fingerprint: {identity}"
            )),
            _ => {}
        }
    }
    for identity in declared
        .keys()
        .filter(|identity| !current.contains_key(*identity))
    {
        violations.push(format!(
            "sealed proof behavior is no longer reachable: {identity}"
        ));
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}
