use serde::Serialize;

use crate::evidence::sha256_serialized;

use super::{
    C2QuarantinedClaim, CloseoutArtifactReference, ProductionSubject, StableProofCommand,
    TestArchitectureCloseoutBundle,
};

/// A C.2 handoff can only be issued from the complete C.1 closeout conjunction.
///
/// Timing or a green smoke run has no constructor path:
///
/// ```compile_fail
/// use store_proof_control::closeout::C2TestArchitectureReadiness;
/// let _forged = C2TestArchitectureReadiness::from_smoke_success(42);
/// ```
///
/// Its fields are also sealed against structural fabrication:
///
/// ```compile_fail
/// use store_proof_control::closeout::C2TestArchitectureReadiness;
/// let _forged = C2TestArchitectureReadiness {};
/// ```
#[derive(Debug, Serialize)]
pub struct C2TestArchitectureReadiness {
    schema_version: u32,
    readiness_identity: String,
    closeout_identity: String,
    proof_inventory: CloseoutArtifactReference,
    preserved_assertion_inventory: String,
    quarantined_physical_claims: Vec<C2QuarantinedClaim>,
    production_subject_map: Vec<ProductionSubject>,
    stable_proof_products: Vec<StableProofCommand>,
    process_probe_roles: Vec<String>,
    preflight_predicates: Vec<String>,
    evidence_and_cost_contracts: Vec<String>,
    authority_posture: String,
    seal: ReadinessSeal,
}

#[derive(Debug, Serialize)]
struct ReadinessSeal;

impl C2TestArchitectureReadiness {
    pub(crate) fn issue(bundle: &TestArchitectureCloseoutBundle) -> Result<Self, String> {
        bundle.validate()?;
        let process_probe_roles = bundle
            .developer_iteration()
            .cases()
            .iter()
            .filter(|case| case.warm.process_probe_receipts > 0)
            .map(|case| format!("{:?}", case.edit.case))
            .collect();
        let mut readiness = Self {
            schema_version: 1,
            readiness_identity: String::new(),
            closeout_identity: bundle.evidence_identity().to_owned(),
            proof_inventory: bundle.proof_inventory().clone(),
            preserved_assertion_inventory: bundle
                .preservation()
                .evidence_identity()
                .to_owned(),
            quarantined_physical_claims: bundle.residual_quarantines().to_vec(),
            production_subject_map: bundle.preservation().production_subjects().to_vec(),
            stable_proof_products: bundle.stable_commands().to_vec(),
            process_probe_roles,
            preflight_predicates: vec![
                "Boundary".to_owned(),
                "AgentContext".to_owned(),
                "Inventory".to_owned(),
                "Preservation".to_owned(),
                "Feature".to_owned(),
                "Dependency".to_owned(),
                "LineCap".to_owned(),
                "Naming".to_owned(),
                "AdmittedResidue".to_owned(),
            ],
            evidence_and_cost_contracts: vec![
                bundle.mutation_sensitivity().evidence_identity().to_owned(),
                bundle.developer_iteration().evidence_identity().to_owned(),
                bundle.ci_identity().to_owned(),
                bundle.artifact_lifecycle_identity().to_owned(),
            ],
            authority_posture: "test-architecture-readiness-only; grants no physical runtime, persistence, recovery, or promotion authority".to_owned(),
            seal: ReadinessSeal,
        };
        readiness.readiness_identity = readiness.expected_identity()?;
        readiness.validate()?;
        Ok(readiness)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1
            || self.closeout_identity.len() != 64
            || self.production_subject_map.is_empty()
            || self.stable_proof_products.is_empty()
            || self.process_probe_roles.is_empty()
            || self.preflight_predicates.is_empty()
            || self.evidence_and_cost_contracts.len() != 4
            || self.authority_posture
                != "test-architecture-readiness-only; grants no physical runtime, persistence, recovery, or promotion authority"
        {
            return Err("C2 readiness handoff is incomplete or overclaims authority".to_owned());
        }
        if self.expected_identity()? != self.readiness_identity {
            return Err("C2 readiness identity does not match its contents".to_owned());
        }
        Ok(())
    }

    pub fn readiness_identity(&self) -> &str {
        &self.readiness_identity
    }

    pub fn closeout_identity(&self) -> &str {
        &self.closeout_identity
    }

    pub fn quarantined_physical_claims(&self) -> &[C2QuarantinedClaim] {
        &self.quarantined_physical_claims
    }

    fn expected_identity(&self) -> Result<String, String> {
        let mut basis = Self {
            schema_version: self.schema_version,
            readiness_identity: String::new(),
            closeout_identity: self.closeout_identity.clone(),
            proof_inventory: self.proof_inventory.clone(),
            preserved_assertion_inventory: self.preserved_assertion_inventory.clone(),
            quarantined_physical_claims: self.quarantined_physical_claims.clone(),
            production_subject_map: self.production_subject_map.clone(),
            stable_proof_products: self.stable_proof_products.clone(),
            process_probe_roles: self.process_probe_roles.clone(),
            preflight_predicates: self.preflight_predicates.clone(),
            evidence_and_cost_contracts: self.evidence_and_cost_contracts.clone(),
            authority_posture: self.authority_posture.clone(),
            seal: ReadinessSeal,
        };
        basis.readiness_identity.clear();
        sha256_serialized(&basis)
    }
}
