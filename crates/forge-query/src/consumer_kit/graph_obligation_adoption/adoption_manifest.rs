use super::{
    kit_digest, ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationExecutionProof, ForgeQueryGraphObligationInMemoryProof,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationSelectorCoverageDeclaration, ForgeQueryGraphObligationSupportPin,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationAdoptionManifest {
    consumer_name: String,
    registration_declaration_digest: String,
    selector_coverage_digest: String,
    support_pin_digest: String,
    support_matrix_digest: String,
    residue_manifest_digest: String,
    local_ceremony_audit_digest: String,
    in_memory_proof_digest: String,
    execution_proof_digest: Option<String>,
    manifest_digest: String,
}

impl ForgeQueryGraphObligationAdoptionManifest {
    pub(super) fn new(
        consumer_name: impl Into<String>,
        registration: &ForgeQueryGraphObligationConsumerRegistrationDeclaration,
        selector_coverage: &ForgeQueryGraphObligationSelectorCoverageDeclaration,
        support_pin: &ForgeQueryGraphObligationSupportPin,
        support_matrix_digest: &str,
        residue_manifest: &ForgeQueryGraphObligationResidueManifest,
        local_ceremony_audit: &ForgeQueryGraphObligationLocalCeremonyAudit,
        in_memory_proof: &ForgeQueryGraphObligationInMemoryProof,
        execution_proof: Option<&ForgeQueryGraphObligationExecutionProof>,
    ) -> Self {
        let consumer_name = consumer_name.into();
        let registration_declaration_digest = registration.declaration_digest().to_string();
        let selector_coverage_digest = selector_coverage.declaration_digest().to_string();
        let support_pin_digest = support_pin.pin_digest().to_string();
        let support_matrix_digest = support_matrix_digest.to_string();
        let residue_manifest_digest = residue_manifest.manifest_digest().to_string();
        let local_ceremony_audit_digest = local_ceremony_audit.audit_digest().to_string();
        let in_memory_proof_digest = in_memory_proof.proof_digest().to_string();
        let execution_proof_digest = execution_proof.map(|proof| proof.proof_digest().to_string());
        let manifest_digest = kit_digest(
            "adoption-manifest",
            [
                consumer_name.as_str(),
                registration_declaration_digest.as_str(),
                selector_coverage_digest.as_str(),
                support_pin_digest.as_str(),
                support_matrix_digest.as_str(),
                residue_manifest_digest.as_str(),
                local_ceremony_audit_digest.as_str(),
                in_memory_proof_digest.as_str(),
                execution_proof_digest
                    .as_deref()
                    .unwrap_or("no-execution-proof"),
            ],
        );
        Self {
            consumer_name,
            registration_declaration_digest,
            selector_coverage_digest,
            support_pin_digest,
            support_matrix_digest,
            residue_manifest_digest,
            local_ceremony_audit_digest,
            in_memory_proof_digest,
            execution_proof_digest,
            manifest_digest,
        }
    }

    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn registration_declaration_digest(&self) -> &str {
        &self.registration_declaration_digest
    }

    pub fn selector_coverage_digest(&self) -> &str {
        &self.selector_coverage_digest
    }

    pub fn support_pin_digest(&self) -> &str {
        &self.support_pin_digest
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn residue_manifest_digest(&self) -> &str {
        &self.residue_manifest_digest
    }

    pub fn local_ceremony_audit_digest(&self) -> &str {
        &self.local_ceremony_audit_digest
    }

    pub fn in_memory_proof_digest(&self) -> &str {
        &self.in_memory_proof_digest
    }

    pub fn execution_proof_digest(&self) -> Option<&str> {
        self.execution_proof_digest.as_deref()
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationAdoptionProof {
    manifest: ForgeQueryGraphObligationAdoptionManifest,
    support_pin: ForgeQueryGraphObligationSupportPin,
    local_ceremony_audit: ForgeQueryGraphObligationLocalCeremonyAudit,
    residue_manifest: ForgeQueryGraphObligationResidueManifest,
    in_memory_proof: ForgeQueryGraphObligationInMemoryProof,
    execution_proof: Option<ForgeQueryGraphObligationExecutionProof>,
}

impl ForgeQueryGraphObligationAdoptionProof {
    pub(super) fn new(
        manifest: ForgeQueryGraphObligationAdoptionManifest,
        support_pin: ForgeQueryGraphObligationSupportPin,
        local_ceremony_audit: ForgeQueryGraphObligationLocalCeremonyAudit,
        residue_manifest: ForgeQueryGraphObligationResidueManifest,
        in_memory_proof: ForgeQueryGraphObligationInMemoryProof,
        execution_proof: Option<ForgeQueryGraphObligationExecutionProof>,
    ) -> Self {
        Self {
            manifest,
            support_pin,
            local_ceremony_audit,
            residue_manifest,
            in_memory_proof,
            execution_proof,
        }
    }

    pub fn manifest(&self) -> &ForgeQueryGraphObligationAdoptionManifest {
        &self.manifest
    }

    pub fn support_pin(&self) -> &ForgeQueryGraphObligationSupportPin {
        &self.support_pin
    }

    pub fn local_ceremony_audit(&self) -> &ForgeQueryGraphObligationLocalCeremonyAudit {
        &self.local_ceremony_audit
    }

    pub fn residue_manifest(&self) -> &ForgeQueryGraphObligationResidueManifest {
        &self.residue_manifest
    }

    pub fn in_memory_proof(&self) -> &ForgeQueryGraphObligationInMemoryProof {
        &self.in_memory_proof
    }

    pub fn execution_proof(&self) -> Option<&ForgeQueryGraphObligationExecutionProof> {
        self.execution_proof.as_ref()
    }
}
