use crate::identity::hash_parts;

use super::super::proof_artifacts::ProjectionConsumptionCompileFailProof;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionProofShapeViolation {
    PhaseSkipping,
    NonAdmittedPromotion,
    RawSourceBypass,
    GenericExtractionShortcut,
    ForgedOperationalArtifact,
    ForgedCertificationArtifact,
}

impl ProjectionConsumptionProofShapeViolation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PhaseSkipping => "phase_skipping",
            Self::NonAdmittedPromotion => "non_admitted_promotion",
            Self::RawSourceBypass => "raw_source_bypass",
            Self::GenericExtractionShortcut => "generic_extraction_shortcut",
            Self::ForgedOperationalArtifact => "forged_operational_artifact",
            Self::ForgedCertificationArtifact => "forged_certification_artifact",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionProofShapeEnforcement {
    CompileFailFixture,
    CompileFailBoundary,
}

impl ProjectionConsumptionProofShapeEnforcement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CompileFailFixture => "compile_fail_fixture",
            Self::CompileFailBoundary => "compile_fail_boundary",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionProofShapeAuditRow {
    violation: ProjectionConsumptionProofShapeViolation,
    attempted_shortcut: &'static str,
    required_prior_artifact: &'static str,
    rejected_artifact: &'static str,
    enforcement: ProjectionConsumptionProofShapeEnforcement,
    enforcement_proof: ProjectionConsumptionCompileFailProof,
    row_digest: String,
}

impl ProjectionConsumptionProofShapeAuditRow {
    pub fn violation(&self) -> ProjectionConsumptionProofShapeViolation {
        self.violation
    }

    pub fn required_prior_artifact(&self) -> &'static str {
        self.required_prior_artifact
    }

    pub fn enforcement_proof(&self) -> ProjectionConsumptionCompileFailProof {
        self.enforcement_proof
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionProofShapeAudit {
    rows: Vec<ProjectionConsumptionProofShapeAuditRow>,
    proof_shape_digest: String,
    phase_progression_digest: String,
}

impl ProjectionConsumptionProofShapeAudit {
    pub fn rows(&self) -> &[ProjectionConsumptionProofShapeAuditRow] {
        &self.rows
    }

    pub fn proof_shape_digest(&self) -> &str {
        &self.proof_shape_digest
    }

    pub fn phase_progression_digest(&self) -> &str {
        &self.phase_progression_digest
    }
}

pub fn projection_consumption_proof_shape_audit() -> ProjectionConsumptionProofShapeAudit {
    use ProjectionConsumptionCompileFailProof::*;
    use ProjectionConsumptionProofShapeEnforcement::*;
    use ProjectionConsumptionProofShapeViolation::*;
    let rows = vec![
        row(
            PhaseSkipping,
            "raw authoring or source artifact used as a later-phase proof",
            "Declaration -> Eligibility -> Contract -> FactSet -> Receipt -> Envelope -> Certification",
            "unscoped source or draft artifact",
            CompileFailFixture,
            DeclarationConstructorPrivate,
        ),
        row(
            NonAdmittedPromotion,
            "denied/deferred/source-mismatch artifact bound into a contract",
            "AdmittedProjectionConsumption",
            "non-admitted eligibility artifact",
            CompileFailFixture,
            NonAdmittedCannotBindContract,
        ),
        row(
            RawSourceBypass,
            "row-like source artifact treated as consumed facts directly",
            "MaterializedProjectionContract",
            "raw source artifact",
            CompileFailBoundary,
            RawSourceHasNoConsumedFactAccessors,
        ),
        row(
            GenericExtractionShortcut,
            "generic extract() over source-specific extraction contract",
            "source-aware extract_from_* method",
            "generic extraction shortcut",
            CompileFailFixture,
            ContractHasNoGenericExtract,
        ),
        row(
            ForgedOperationalArtifact,
            "caller-minted fact set, receipt, or envelope",
            "sealed extraction and receipt shaping path",
            "forged operational artifact",
            CompileFailFixture,
            ReceiptConstructorPrivate,
        ),
        row(
            ForgedCertificationArtifact,
            "caller-minted certification bundle",
            "certify_projection_consumption_closeout_core()",
            "forged certification artifact",
            CompileFailFixture,
            CertificationBundleConstructorPrivate,
        ),
    ];
    let proof_shape_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    let phase_progression_digest = hash_parts(&[
        "projection_consumption_phase_progression_v1".to_string(),
        proof_shape_digest.clone(),
        "declaration>eligibility>contract>fact_set>receipt>envelope>certification".to_string(),
    ]);
    ProjectionConsumptionProofShapeAudit {
        rows,
        proof_shape_digest,
        phase_progression_digest,
    }
}

pub fn projection_consumption_phase_progression_digest() -> String {
    projection_consumption_proof_shape_audit()
        .phase_progression_digest()
        .to_string()
}

fn row(
    violation: ProjectionConsumptionProofShapeViolation,
    attempted_shortcut: &'static str,
    required_prior_artifact: &'static str,
    rejected_artifact: &'static str,
    enforcement: ProjectionConsumptionProofShapeEnforcement,
    enforcement_proof: ProjectionConsumptionCompileFailProof,
) -> ProjectionConsumptionProofShapeAuditRow {
    let row_digest = hash_parts(&[
        "projection_consumption_proof_shape_row_v1".to_string(),
        format!("violation:{}", violation.as_str()),
        format!("attempted:{attempted_shortcut}"),
        format!("required:{required_prior_artifact}"),
        format!("rejected:{rejected_artifact}"),
        format!("enforcement:{}", enforcement.as_str()),
        format!("proof:{}", enforcement_proof.as_str()),
    ]);
    ProjectionConsumptionProofShapeAuditRow {
        violation,
        attempted_shortcut,
        required_prior_artifact,
        rejected_artifact,
        enforcement,
        enforcement_proof,
        row_digest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_shape_audit_binds_every_named_violation_to_a_real_proof() {
        let audit = projection_consumption_proof_shape_audit();
        assert_eq!(audit.rows().len(), 6);
        assert!(audit.rows().iter().all(|row| !row.row_digest().is_empty()));
        assert!(!audit.proof_shape_digest().is_empty());
        assert!(!audit.phase_progression_digest().is_empty());
    }
}
