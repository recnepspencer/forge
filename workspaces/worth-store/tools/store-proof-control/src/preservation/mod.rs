use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::classification::{ProofDisposition, ProofFamily, ProofOwner};
use crate::ValidatedProofInventory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofPreservationRow {
    pub stable_case_id: String,
    pub owner: ProofOwner,
    pub family: ProofFamily,
    pub products: BTreeSet<String>,
    pub disposition: ProofDisposition,
    pub assertion_predicates: Vec<String>,
    pub original_target_identity: Option<String>,
    #[serde(default)]
    pub admitted_target_identity: Option<String>,
    pub physical_reality_audit_required: bool,
    #[serde(default)]
    pub amendment_rationale: String,
    #[serde(default)]
    pub quarantine: Option<InvalidClaimQuarantineRecord>,
    #[serde(default)]
    pub replacement: Option<ProofReplacementRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidClaimQuarantineRecord {
    pub rationale: String,
    pub follow_on_owner: ProofOwner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofReplacementRecord {
    pub replacement_case_id: String,
    pub predicate_parity: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofPreservationLedger {
    pub schema_version: u32,
    pub rows: Vec<ProofPreservationRow>,
}

pub fn build_ledger(inventory: &ValidatedProofInventory) -> ProofPreservationLedger {
    let rows = inventory
        .inventory()
        .proofs
        .iter()
        .map(|proof| ProofPreservationRow {
            stable_case_id: proof.case.identity.stable_id.clone(),
            owner: proof.owner.clone(),
            family: proof.family,
            products: proof.products.clone(),
            disposition: proof.disposition,
            assertion_predicates: proof.case.assertion_predicates.clone(),
            original_target_identity: proof.case.target_identity.clone(),
            admitted_target_identity: None,
            physical_reality_audit_required: proof.physical_reality_audit_required,
            amendment_rationale: String::new(),
            quarantine: None,
            replacement: None,
        })
        .collect();
    ProofPreservationLedger {
        schema_version: 1,
        rows,
    }
}

pub fn validate_ledger(
    inventory: &ValidatedProofInventory,
    ledger: &ProofPreservationLedger,
) -> Result<(), Vec<String>> {
    let inventory_rows: BTreeMap<_, _> = inventory
        .inventory()
        .proofs
        .iter()
        .map(|proof| (proof.case.identity.stable_id.as_str(), proof))
        .collect();
    let inventory_ids: BTreeSet<_> = inventory_rows.keys().copied().collect();
    let mut ledger_ids = BTreeSet::new();
    let mut violations = Vec::new();
    if ledger.schema_version != 1 {
        violations.push(format!(
            "unsupported proof preservation ledger schema: {}",
            ledger.schema_version
        ));
    }
    for row in &ledger.rows {
        if !inventory_ids.contains(row.stable_case_id.as_str()) {
            violations.push(format!(
                "ledger contains phantom proof: {}",
                row.stable_case_id
            ));
        }
        if !ledger_ids.insert(row.stable_case_id.as_str()) {
            violations.push(format!("ledger duplicates proof: {}", row.stable_case_id));
        }
        if let Some(proof) = inventory_rows.get(row.stable_case_id.as_str()) {
            let semantics_changed = row.owner != proof.owner
                || row.family != proof.family
                || row.products != proof.products
                || row.disposition != proof.disposition
                || row.original_target_identity != proof.case.target_identity
                || row.assertion_predicates != proof.case.assertion_predicates
                || row.physical_reality_audit_required != proof.physical_reality_audit_required;
            if semantics_changed
                && matches!(
                    row.disposition,
                    ProofDisposition::PreserveUnchanged | ProofDisposition::PreserveAndConsolidate
                )
            {
                violations.push(format!(
                    "unamended ledger row drifted from frozen inventory: {}",
                    row.stable_case_id
                ));
            }
            if matches!(
                row.disposition,
                ProofDisposition::PreserveAndMove | ProofDisposition::PreserveAndReclassify
            ) && row.amendment_rationale.trim().is_empty()
            {
                violations.push(format!(
                    "amended proof lacks a rationale: {}",
                    row.stable_case_id
                ));
            }
        }
        if row.assertion_predicates.is_empty() {
            violations.push(format!(
                "ledger lost assertion surface: {}",
                row.stable_case_id
            ));
        }
        if row.disposition == ProofDisposition::InvalidClaimQuarantine && row.quarantine.is_none() {
            violations.push(format!(
                "quarantined proof lacks rationale and follow-on owner: {}",
                row.stable_case_id
            ));
        }
        let replacement_disposition = matches!(
            row.disposition,
            ProofDisposition::ReplaceWithStrongerProof
                | ProofDisposition::DuplicateProofRemoveAfterParity
        );
        if replacement_disposition && row.replacement.is_none() {
            violations.push(format!(
                "replacement disposition lacks replacement authority: {}",
                row.stable_case_id
            ));
        }
        if let Some(replacement) = &row.replacement {
            let expected: BTreeSet<_> = row.assertion_predicates.iter().cloned().collect();
            let mapped: BTreeSet<_> = replacement.predicate_parity.keys().cloned().collect();
            if expected != mapped {
                violations.push(format!(
                    "replacement predicate parity is incomplete for {}",
                    row.stable_case_id
                ));
            }
            if replacement.replacement_case_id == row.stable_case_id {
                violations.push(format!(
                    "replacement proof points to itself: {}",
                    row.stable_case_id
                ));
            }
        }
    }
    for missing in inventory_ids.difference(&ledger_ids) {
        violations.push(format!("ledger omits discovered proof: {missing}"));
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

pub fn validate_current_reachability(
    ledger: &ProofPreservationLedger,
    current: &ValidatedProofInventory,
    historical_non_case_aggregates: &BTreeSet<String>,
) -> Result<(), Vec<String>> {
    let current_rows: std::collections::BTreeMap<_, _> = current
        .inventory()
        .proofs
        .iter()
        .map(|proof| (proof.case.identity.stable_id.as_str(), proof))
        .collect();
    let mut violations = Vec::new();
    for baseline in &ledger.rows {
        if let Some(replacement) = &baseline.replacement {
            validate_replacement_reachability(
                baseline,
                replacement,
                &current_rows,
                &mut violations,
            );
        }
        let Some(current_proof) = current_rows.get(baseline.stable_case_id.as_str()) else {
            if historical_non_case_aggregates.contains(&baseline.stable_case_id) {
                continue;
            }
            if baseline.disposition == ProofDisposition::InvalidClaimQuarantine {
                continue;
            }
            if baseline.replacement.is_some() {
                continue;
            }
            violations.push(format!(
                "preserved proof is unreachable: {}",
                baseline.stable_case_id
            ));
            continue;
        };
        if baseline.disposition == ProofDisposition::PreserveUnchanged
            && current_proof.case.kind != crate::discovery::CaseKind::UiFixture
            && current_proof.case.target_identity != baseline.original_target_identity
        {
            violations.push(format!(
                "unchanged proof moved execution target for {}: expected {:?}, observed {:?}",
                baseline.stable_case_id,
                baseline.original_target_identity,
                current_proof.case.target_identity
            ));
        }
        if matches!(
            baseline.disposition,
            ProofDisposition::PreserveAndConsolidate | ProofDisposition::PreserveAndMove
        ) {
            let expected_target = baseline
                .admitted_target_identity
                .as_deref()
                .or_else(|| admitted_consolidated_target(&baseline.products));
            if expected_target != current_proof.case.target_identity.as_deref() {
                violations.push(format!(
                    "consolidated proof reached an unadmitted destination for {}: expected {:?}, observed {:?}",
                    baseline.stable_case_id,
                    expected_target,
                    current_proof.case.target_identity
                ));
            }
        }
        if current_proof.products != baseline.products {
            violations.push(format!(
                "proof-product membership drifted for {}: expected {:?}, observed {:?}",
                baseline.stable_case_id, baseline.products, current_proof.products
            ));
        }
        if current_proof.owner != baseline.owner {
            violations.push(format!(
                "proof owner drifted for {}: expected {:?}, observed {:?}",
                baseline.stable_case_id, baseline.owner, current_proof.owner
            ));
        }
        if current_proof.family != baseline.family {
            violations.push(format!(
                "proof family drifted for {}: expected {:?}, observed {:?}",
                baseline.stable_case_id, baseline.family, current_proof.family
            ));
        }
        if current_proof.case.assertion_predicates != baseline.assertion_predicates {
            violations.push(format!(
                "assertion surface drifted for {}: expected {:?}, observed {:?}",
                baseline.stable_case_id,
                baseline.assertion_predicates,
                current_proof.case.assertion_predicates
            ));
        }
        if current_proof.case.target_identity.is_none()
            && current_proof.case.kind != crate::discovery::CaseKind::UiFixture
        {
            violations.push(format!(
                "preserved proof has no executable target: {}",
                baseline.stable_case_id
            ));
        }
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

pub fn historical_non_case_aggregate_ids(
    baseline: &crate::classification::ClassifiedInventory,
) -> BTreeSet<String> {
    baseline
        .proofs
        .iter()
        .filter(|proof| {
            matches!(
                proof.case.kind,
                crate::discovery::CaseKind::TestExecutable
                    | crate::discovery::CaseKind::DoctestSurface
            )
        })
        .map(|proof| proof.case.identity.stable_id.clone())
        .collect()
}

pub fn semantic_authority_from_ledger(
    baseline: &crate::classification::ClassifiedInventory,
    ledger: &ProofPreservationLedger,
) -> Result<crate::classification::ClassifiedInventory, String> {
    let rows: BTreeMap<_, _> = ledger
        .rows
        .iter()
        .map(|row| (row.stable_case_id.as_str(), row))
        .collect();
    let mut authority = baseline.clone();
    for proof in &mut authority.proofs {
        let row = rows
            .get(proof.case.identity.stable_id.as_str())
            .ok_or_else(|| {
                format!(
                    "ledger omits semantic authority for {}",
                    proof.case.identity.stable_id
                )
            })?;
        proof.owner = row.owner.clone();
        proof.family = row.family;
        proof.products = row.products.clone();
        proof.disposition = row.disposition;
        proof.physical_reality_audit_required = row.physical_reality_audit_required;
    }
    Ok(authority)
}

fn admitted_consolidated_target(products: &BTreeSet<String>) -> Option<&'static str> {
    [
        (
            "store-ci:recovery",
            "worth-store-certification::test::durability_recovery",
        ),
        (
            "store-ci:physical_isolation",
            "worth-store-certification::test::physical_isolation",
        ),
        (
            "store-ci:scheduling",
            "worth-store-certification::test::io_scheduling",
        ),
        (
            "store-ci:layout",
            "worth-store-certification::test::layout_access",
        ),
        (
            "store-ci:blobs",
            "worth-store-certification::test::blob_chunks",
        ),
        (
            "store-ci:security",
            "worth-store-certification::test::operational_security",
        ),
    ]
    .into_iter()
    .find_map(|(product, target)| products.contains(product).then_some(target))
}

fn validate_replacement_reachability(
    baseline: &ProofPreservationRow,
    replacement: &ProofReplacementRecord,
    current_rows: &std::collections::BTreeMap<&str, &crate::classification::ClassifiedProof>,
    violations: &mut Vec<String>,
) {
    let Some(current) = current_rows.get(replacement.replacement_case_id.as_str()) else {
        violations.push(format!(
            "replacement proof is unreachable for {}: {}",
            baseline.stable_case_id, replacement.replacement_case_id
        ));
        return;
    };
    let current_predicates: BTreeSet<_> = current.case.assertion_predicates.iter().collect();
    for (old, new) in &replacement.predicate_parity {
        if !current_predicates.contains(new) {
            violations.push(format!(
                "replacement proof {} loses mapped predicate {:?} -> {:?}",
                replacement.replacement_case_id, old, new
            ));
        }
    }
    if !baseline.products.is_subset(&current.products) {
        violations.push(format!(
            "replacement proof {} is unreachable from original products {:?}",
            replacement.replacement_case_id, baseline.products
        ));
    }
}
