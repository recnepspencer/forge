use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::{
    classify_case, ClassifiedInventory, ClassifiedProof, ProofDisposition, ProofFamily, ProofOwner,
};
use crate::discovery::{CaseKind, TestCaseSurface};
use crate::{ClassifiedProofInventory, DiscoveredTestSurface};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostBaselineProofAuthority {
    pub schema_version: u32,
    pub declarations: Vec<ProofSemanticDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofSemanticDeclaration {
    pub stable_case_id: String,
    pub family: ProofFamily,
    pub owner: ProofOwner,
    pub products: BTreeSet<String>,
    pub disposition: ProofDisposition,
    pub expected_evidence: Vec<String>,
    pub physical_reality_audit_required: bool,
}

pub fn classify(discovered: DiscoveredTestSurface) -> ClassifiedProofInventory {
    let mut proofs: Vec<_> = discovered
        .inventory()
        .cases
        .iter()
        .cloned()
        .map(classify_case)
        .collect();
    proofs.sort_by(|left, right| left.case.identity.cmp(&right.case.identity));
    ClassifiedProofInventory::from_discovered(ClassifiedInventory {
        schema_version: 1,
        discovered: discovered.into_inventory(),
        proofs,
    })
}

pub fn classify_from_authority(
    discovered: DiscoveredTestSurface,
    authority: &ClassifiedInventory,
    post_baseline: &PostBaselineProofAuthority,
) -> Result<ClassifiedProofInventory, Vec<String>> {
    if post_baseline.schema_version != 1 {
        return Err(vec![format!(
            "unsupported post-baseline proof authority schema: {}",
            post_baseline.schema_version
        )]);
    }
    let baseline_by_id: BTreeMap<_, _> = authority
        .proofs
        .iter()
        .map(|proof| (proof.case.identity.stable_id.as_str(), proof))
        .collect();
    let post_baseline_by_id: BTreeMap<_, _> = post_baseline
        .declarations
        .iter()
        .map(|declaration| (declaration.stable_case_id.as_str(), declaration))
        .collect();
    let mut violations = authority_violations(
        &baseline_by_id,
        &post_baseline_by_id,
        post_baseline.declarations.len(),
    );
    let mut proofs = Vec::new();
    for case in discovered.inventory().cases.iter().cloned() {
        if let Some(declared) = baseline_by_id.get(case.identity.stable_id.as_str()) {
            proofs.push(from_baseline(case, declared));
        } else if case.identity.package == "store-proof-control" && case.kind == CaseKind::RustTest
        {
            proofs.push(controller_case(case));
        } else if let Some(declared) = post_baseline_by_id.get(case.identity.stable_id.as_str()) {
            proofs.push(from_declaration(case, declared));
        } else if matches!(
            case.kind,
            CaseKind::DoctestRunnable | CaseKind::DoctestCompileFail | CaseKind::DoctestIgnored
        ) {
            proofs.push(classify_case(case));
        } else {
            violations.push(format!(
                "proof has no reviewed semantic authority: {}",
                case.identity.stable_id
            ));
        }
    }
    if !violations.is_empty() {
        return Err(violations);
    }
    proofs.sort_by(|left, right| left.case.identity.cmp(&right.case.identity));
    Ok(ClassifiedProofInventory::from_discovered(
        ClassifiedInventory {
            schema_version: 2,
            discovered: discovered.into_inventory(),
            proofs,
        },
    ))
}

fn authority_violations(
    baseline: &BTreeMap<&str, &ClassifiedProof>,
    post_baseline: &BTreeMap<&str, &ProofSemanticDeclaration>,
    declaration_count: usize,
) -> Vec<String> {
    let mut violations = Vec::new();
    if post_baseline.len() != declaration_count {
        violations
            .push("post-baseline semantic authority contains duplicate identities".to_owned());
    }
    for duplicate in baseline.keys().filter(|id| post_baseline.contains_key(*id)) {
        violations.push(format!(
            "post-baseline authority attempts to override frozen proof: {duplicate}"
        ));
    }
    violations
}

fn from_baseline(case: TestCaseSurface, declared: &ClassifiedProof) -> ClassifiedProof {
    ClassifiedProof {
        case,
        family: declared.family,
        owner: declared.owner.clone(),
        products: declared.products.clone(),
        disposition: declared.disposition,
        expected_evidence: declared.expected_evidence.clone(),
        physical_reality_audit_required: declared.physical_reality_audit_required,
    }
}

fn from_declaration(case: TestCaseSurface, declared: &ProofSemanticDeclaration) -> ClassifiedProof {
    ClassifiedProof {
        case,
        family: declared.family,
        owner: declared.owner.clone(),
        products: declared.products.clone(),
        disposition: declared.disposition,
        expected_evidence: declared.expected_evidence.clone(),
        physical_reality_audit_required: declared.physical_reality_audit_required,
    }
}

fn controller_case(case: TestCaseSurface) -> ClassifiedProof {
    ClassifiedProof {
        owner: ProofOwner {
            package: "store-proof-control".to_owned(),
            responsibility: case.identity.responsibility.clone(),
        },
        case,
        family: ProofFamily::OwnerInvariant,
        products: BTreeSet::from(["store-ci:test-control".to_owned()]),
        disposition: ProofDisposition::PreserveUnchanged,
        expected_evidence: vec![
            "behavioral_verdict".to_owned(),
            "assertion_predicates".to_owned(),
        ],
        physical_reality_audit_required: false,
    }
}
