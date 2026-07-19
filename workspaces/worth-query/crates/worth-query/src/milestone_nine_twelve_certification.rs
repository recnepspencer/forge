use crate::basis_lifecycle::{
    basis_lifecycle, basis_lifecycle_migration_audit, basis_lifecycle_support_matrix,
    emit_certification_basis_receipt, readmit_lower_runtime_evidence, LowerRuntimeBasisEvidence,
};
use crate::consumer_kit::{
    hard_prohibition_registry, worth_query_public_authority_surface_rows,
    WorthQueryPublicAuthoritySurfaceClass,
};
use crate::identity::hash_parts;
use crate::intent_admission::certify_intent_admission;
use crate::projection_consumption::certify_projection_consumption_closeout_core;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryMilestoneNineTwelveCertificationBundle {
    public_authority_manifest_digest: String,
    ordinary_facade_digest: String,
    prohibition_registry_digest: String,
    support_matrix_digest: String,
    intent_admission_certification_digest: String,
    projection_consumption_certification_digest: String,
    hostile_authority_matrix_digest: String,
    reference_consumer_adoption_digest: String,
    certification_digest: String,
    hostile_case_count: usize,
    ordinary_facade_violation_count: usize,
    compatibility_debt_count: usize,
}

impl WorthQueryMilestoneNineTwelveCertificationBundle {
    pub fn public_authority_manifest_digest(&self) -> &str {
        &self.public_authority_manifest_digest
    }
    pub fn ordinary_facade_digest(&self) -> &str {
        &self.ordinary_facade_digest
    }
    pub fn prohibition_registry_digest(&self) -> &str {
        &self.prohibition_registry_digest
    }
    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }
    pub fn intent_admission_certification_digest(&self) -> &str {
        &self.intent_admission_certification_digest
    }
    pub fn projection_consumption_certification_digest(&self) -> &str {
        &self.projection_consumption_certification_digest
    }
    pub fn hostile_authority_matrix_digest(&self) -> &str {
        &self.hostile_authority_matrix_digest
    }
    pub fn reference_consumer_adoption_digest(&self) -> &str {
        &self.reference_consumer_adoption_digest
    }
    pub fn certification_digest(&self) -> &str {
        &self.certification_digest
    }
    pub fn hostile_case_count(&self) -> usize {
        self.hostile_case_count
    }
    pub fn ordinary_facade_violation_count(&self) -> usize {
        self.ordinary_facade_violation_count
    }
    pub fn compatibility_debt_count(&self) -> usize {
        self.compatibility_debt_count
    }
    pub fn is_closed(&self) -> bool {
        self.hostile_case_count == 6
            && self.ordinary_facade_violation_count == 0
            && self.compatibility_debt_count == 0
            && !self.reference_consumer_adoption_digest.is_empty()
            && !self.intent_admission_certification_digest.is_empty()
            && !self.projection_consumption_certification_digest.is_empty()
    }
}

pub fn certify_milestone_nine_twelve(
    reference_consumer_adoption_digest: impl Into<String>,
) -> WorthQueryMilestoneNineTwelveCertificationBundle {
    let reference_consumer_adoption_digest = reference_consumer_adoption_digest.into();
    assert!(
        !reference_consumer_adoption_digest.trim().is_empty(),
        "reference-consumer adoption evidence is required"
    );
    let rows = worth_query_public_authority_surface_rows();
    let public_authority_manifest_digest = hash_parts(
        &rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}",
                    row.symbol(),
                    row.owner().as_str(),
                    row.target_class().as_str(),
                    row.replacement()
                )
            })
            .collect::<Vec<_>>(),
    );
    let ordinary_facade_rows = rows
        .iter()
        .filter(|row| {
            row.facade_path().is_some() && row.facade_probe() != Some("pub mod certification")
        })
        .collect::<Vec<_>>();
    let ordinary_facade_violation_count = ordinary_facade_rows
        .iter()
        .filter(|row| {
            matches!(
                row.target_class(),
                WorthQueryPublicAuthoritySurfaceClass::CertificationOnlyApi
                    | WorthQueryPublicAuthoritySurfaceClass::InternalAdapter
                    | WorthQueryPublicAuthoritySurfaceClass::DeleteBeforeCloseout
                    | WorthQueryPublicAuthoritySurfaceClass::RemovedSurface
            )
        })
        .count();
    let ordinary_facade_digest = hash_parts(
        &ordinary_facade_rows
            .iter()
            .map(|row| format!("{}:{}", row.symbol(), row.facade_path().unwrap_or("none")))
            .collect::<Vec<_>>(),
    );
    let prohibition_registry_digest = hash_parts(
        &hard_prohibition_registry()
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}",
                    row.seam_key(),
                    row.enforcement_tier().as_str(),
                    row.replacement_lane()
                )
            })
            .collect::<Vec<_>>(),
    );
    let support_matrix_digest = basis_lifecycle_support_matrix().matrix_digest().to_string();
    let intent_admission_certification_digest = certify_intent_admission()
        .certification_bundle_digest()
        .to_string();
    let projection_consumption_certification_digest =
        certify_projection_consumption_closeout_core()
            .certification_bundle_digest()
            .to_string();
    let (hostile_authority_matrix_digest, hostile_case_count) = hostile_authority_matrix();
    let compatibility_debt_count = basis_lifecycle_migration_audit()
        .counters()
        .compatibility_debt_count();
    let certification_digest = hash_parts(&[
        public_authority_manifest_digest.clone(),
        ordinary_facade_digest.clone(),
        prohibition_registry_digest.clone(),
        support_matrix_digest.clone(),
        intent_admission_certification_digest.clone(),
        projection_consumption_certification_digest.clone(),
        hostile_authority_matrix_digest.clone(),
        reference_consumer_adoption_digest.clone(),
        format!("ordinary_facade_violations:{ordinary_facade_violation_count}"),
        format!("compatibility_debt:{compatibility_debt_count}"),
    ]);
    WorthQueryMilestoneNineTwelveCertificationBundle {
        public_authority_manifest_digest,
        ordinary_facade_digest,
        prohibition_registry_digest,
        support_matrix_digest,
        intent_admission_certification_digest,
        projection_consumption_certification_digest,
        hostile_authority_matrix_digest,
        reference_consumer_adoption_digest,
        certification_digest,
        hostile_case_count,
        ordinary_facade_violation_count,
        compatibility_debt_count,
    }
}

fn hostile_authority_matrix() -> (String, usize) {
    let certification_use_path = basis_lifecycle()
        .current_head()
        .for_certification()
        .expect("certification intent normalizes")
        .admit()
        .expect("certification basis admits");
    assert!(!certification_use_path
        .capability()
        .capability_digest()
        .is_empty());
    let certification_basis = certification_use_path.scope();
    assert_eq!(
        certification_basis
            .counters()
            .scoped_capability_construction_count(),
        1
    );
    let certification_bound_basis = readmit_lower_runtime_evidence(
        certification_basis,
        LowerRuntimeBasisEvidence::from_runtime_basis(
            "runtime-current-head",
            "certification-evidence",
            1,
        ),
    )
    .expect("matching certification basis readmits");
    let certification_receipt = emit_certification_basis_receipt(certification_bound_basis);

    let current_left = basis_lifecycle()
        .current_head()
        .observe()
        .expect("current basis admits");
    let current_right = basis_lifecycle()
        .current_head()
        .observe()
        .expect("equivalent basis admits");
    assert_eq!(current_left, current_right);

    let branch_a = basis_lifecycle()
        .branch_head("branch-a", true)
        .observe()
        .expect("branch a admits");
    let branch_b = basis_lifecycle()
        .branch_head("branch-b", true)
        .observe()
        .expect("branch b admits");
    assert_ne!(
        branch_a.scoped_basis_digest(),
        branch_b.scoped_basis_digest()
    );

    let generation_a = basis_lifecycle()
        .runtime_snapshot("generation-a", "runtime:generation-a")
        .observe()
        .expect("generation a admits");
    let generation_b = basis_lifecycle()
        .runtime_snapshot("generation-b", "runtime:generation-b")
        .observe()
        .expect("generation b admits");
    assert_ne!(
        generation_a.scoped_basis_digest(),
        generation_b.scoped_basis_digest()
    );

    let matching = readmit_lower_runtime_evidence(
        generation_a.clone(),
        LowerRuntimeBasisEvidence::from_runtime_bridge_facade(
            "runtime:generation-a",
            "evidence-a",
            1,
        ),
    )
    .expect("matching runtime generation readmits");
    assert!(!matching.lower_runtime_binding_digest().is_empty());
    assert!(readmit_lower_runtime_evidence(
        generation_a.clone(),
        LowerRuntimeBasisEvidence::from_runtime_bridge_facade(
            "runtime:generation-b",
            "evidence-b",
            1
        ),
    )
    .is_err());
    assert!(readmit_lower_runtime_evidence(
        generation_a,
        LowerRuntimeBasisEvidence::stale_runtime_snapshot(
            "runtime:generation-a",
            "evidence-stale",
            1
        ),
    )
    .is_err());
    assert!(basis_lifecycle()
        .policy_scoped("policy-a", "tenant-a", "branch-a", "schema-a")
        .policy_masks_operation()
        .observe()
        .is_err());

    let evidence = vec![
        format!("equivalent:{}", current_left.scoped_basis_digest()),
        format!(
            "cross_basis:{}:{}",
            branch_a.scoped_basis_digest(),
            branch_b.scoped_basis_digest()
        ),
        format!(
            "cross_generation:{}:{}",
            generation_b.scoped_basis_digest(),
            matching.lower_runtime_binding_digest()
        ),
        "matching_readmission:admitted".to_string(),
        "foreign_and_stale_readmission:denied".to_string(),
        "policy_mask:denied_before_scope".to_string(),
    ];
    let hostile_case_count = evidence.len();
    let mut digest_parts = evidence;
    digest_parts.push(format!(
        "certification_receipt:{}",
        certification_receipt.receipt_digest()
    ));
    (hash_parts(&digest_parts), hostile_case_count)
}

#[cfg(test)]
mod tests {
    use super::certify_milestone_nine_twelve;

    #[test]
    fn certification_closes_all_runtime_backed_authority_seams() {
        let bundle = certify_milestone_nine_twelve("reference-consumer-adoption-v1");
        assert!(bundle.is_closed(), "bundle: {bundle:#?}");
        assert!(!bundle.certification_digest().is_empty());
    }
}
