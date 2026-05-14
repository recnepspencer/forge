use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecyclePhaseArtifact {
    RawIntent,
    NormalizedIntent,
    Eligibility,
    AdmittedCapability,
    ScopedBasis,
    LowerRuntimeBinding,
    UseReceipt,
    SelfDescribingEnvelope,
    CertificationBundle,
}

impl BasisLifecyclePhaseArtifact {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RawIntent => "raw_basis_intent",
            Self::NormalizedIntent => "normalized_basis_intent",
            Self::Eligibility => "basis_eligibility",
            Self::AdmittedCapability => "admitted_basis_capability",
            Self::ScopedBasis => "scoped_execution_or_observation_basis",
            Self::LowerRuntimeBinding => "lower_runtime_bound_basis",
            Self::UseReceipt => "basis_use_receipt",
            Self::SelfDescribingEnvelope => "self_describing_basis_envelope",
            Self::CertificationBundle => "basis_lifecycle_certification_bundle",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecyclePhaseManifestRow {
    artifact: BasisLifecyclePhaseArtifact,
    producer: &'static str,
    required_input: &'static str,
    next_consumer: &'static str,
    enforcement_proof: &'static str,
    row_digest: String,
}

impl BasisLifecyclePhaseManifestRow {
    fn new(
        artifact: BasisLifecyclePhaseArtifact,
        producer: &'static str,
        required_input: &'static str,
        next_consumer: &'static str,
        enforcement_proof: &'static str,
    ) -> Self {
        let row_digest = hash_parts(&[
            "basis_lifecycle_phase_manifest_row_v1".to_string(),
            format!("artifact:{}", artifact.as_str()),
            format!("producer:{producer}"),
            format!("required_input:{required_input}"),
            format!("next_consumer:{next_consumer}"),
            format!("proof:{enforcement_proof}"),
        ]);
        Self {
            artifact,
            producer,
            required_input,
            next_consumer,
            enforcement_proof,
            row_digest,
        }
    }

    pub fn artifact(&self) -> BasisLifecyclePhaseArtifact {
        self.artifact
    }

    pub fn producer(&self) -> &'static str {
        self.producer
    }

    pub fn required_input(&self) -> &'static str {
        self.required_input
    }

    pub fn next_consumer(&self) -> &'static str {
        self.next_consumer
    }

    pub fn enforcement_proof(&self) -> &'static str {
        self.enforcement_proof
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecyclePhaseManifest {
    rows: Vec<BasisLifecyclePhaseManifestRow>,
    manifest_digest: String,
    typestate_transition_digest: String,
}

impl BasisLifecyclePhaseManifest {
    fn new(rows: Vec<BasisLifecyclePhaseManifestRow>) -> Self {
        let manifest_digest = hash_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        let typestate_transition_digest = hash_parts(
            &rows
                .windows(2)
                .map(|pair| {
                    format!(
                        "{}->{}",
                        pair[0].artifact().as_str(),
                        pair[1].artifact().as_str()
                    )
                })
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            manifest_digest,
            typestate_transition_digest,
        }
    }

    pub fn rows(&self) -> &[BasisLifecyclePhaseManifestRow] {
        &self.rows
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn typestate_transition_digest(&self) -> &str {
        &self.typestate_transition_digest
    }
}

pub fn basis_lifecycle_phase_manifest() -> BasisLifecyclePhaseManifest {
    use BasisLifecyclePhaseArtifact::*;
    BasisLifecyclePhaseManifest::new(vec![
        row(
            RawIntent,
            "RawBasisIntent public authoring",
            "caller intent only",
            "normalize_raw_basis_intent",
            "basis_lifecycle_dx_draft_is_not_scoped_proof",
        ),
        row(
            NormalizedIntent,
            "normalize_raw_basis_intent",
            "RawBasisIntent plus operation lane witness",
            "evaluate_basis_*_eligibility",
            "basis_lifecycle_dx_draft_is_not_scoped_proof",
        ),
        row(
            Eligibility,
            "evaluate_basis_*_eligibility",
            "NormalizedBasisIntent",
            "admit_basis_capability",
            "basis_lifecycle_advisory_cannot_be_admitted",
        ),
        row(
            AdmittedCapability,
            "admit_basis_capability",
            "BasisEligibility",
            "scope_basis_for_*",
            "basis_lifecycle_admitted_capability_constructor_private",
        ),
        row(
            ScopedBasis,
            "scope_basis_for_*",
            "AdmittedBasisCapability",
            "readmit_lower_runtime_evidence",
            "basis_lifecycle_scoped_basis_constructor_private",
        ),
        row(
            LowerRuntimeBinding,
            "readmit_lower_runtime_evidence",
            "ScopedExecutionOrObservationBasis plus facade evidence",
            "emit_*_basis_receipt",
            "basis_lifecycle_lower_runtime_bound_basis_constructor_private",
        ),
        row(
            UseReceipt,
            "emit_*_basis_receipt",
            "LowerRuntimeBoundBasis",
            "SelfDescribingBasisEnvelope::from_receipt",
            "basis_lifecycle_use_receipt_constructor_private",
        ),
        row(
            SelfDescribingEnvelope,
            "SelfDescribingBasisEnvelope::from_receipt",
            "BasisUseReceipt",
            "certify_basis_lifecycle",
            "basis_lifecycle_envelope_constructor_private",
        ),
        row(
            CertificationBundle,
            "certify_basis_lifecycle",
            "representative certified rows",
            "9.3.2 closeout evidence",
            "basis_lifecycle_certification_bundle_constructor_private",
        ),
    ])
}

pub fn basis_lifecycle_phase_artifact_manifest_digest() -> String {
    basis_lifecycle_phase_manifest()
        .manifest_digest()
        .to_string()
}

pub fn basis_lifecycle_typestate_transition_digest() -> String {
    basis_lifecycle_phase_manifest()
        .typestate_transition_digest()
        .to_string()
}

fn row(
    artifact: BasisLifecyclePhaseArtifact,
    producer: &'static str,
    required_input: &'static str,
    next_consumer: &'static str,
    enforcement_proof: &'static str,
) -> BasisLifecyclePhaseManifestRow {
    BasisLifecyclePhaseManifestRow::new(
        artifact,
        producer,
        required_input,
        next_consumer,
        enforcement_proof,
    )
}

#[cfg(test)]
mod tests;
