use super::{
    kit_digest, WorthQueryGraphObligationConsumerRegistrationDeclaration,
    WorthQueryGraphObligationExecutionProof, WorthQueryGraphObligationInMemoryProof,
    WorthQueryGraphObligationLocalCeremonyAudit, WorthQueryGraphObligationResidueManifest,
    WorthQueryGraphObligationSelectorCoverageDeclaration, WorthQueryGraphObligationSupportPin,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationAdoptionManifest {
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

impl WorthQueryGraphObligationAdoptionManifest {
    pub(super) fn new(
        consumer_name: impl Into<String>,
        registration: &WorthQueryGraphObligationConsumerRegistrationDeclaration,
        selector_coverage: &WorthQueryGraphObligationSelectorCoverageDeclaration,
        support_pin: &WorthQueryGraphObligationSupportPin,
        support_matrix_digest: &str,
        residue_manifest: &WorthQueryGraphObligationResidueManifest,
        local_ceremony_audit: &WorthQueryGraphObligationLocalCeremonyAudit,
        in_memory_proof: &WorthQueryGraphObligationInMemoryProof,
        execution_proof: Option<&WorthQueryGraphObligationExecutionProof>,
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
pub struct WorthQueryGraphObligationAdoptionProof {
    manifest: WorthQueryGraphObligationAdoptionManifest,
    support_pin: WorthQueryGraphObligationSupportPin,
    local_ceremony_audit: WorthQueryGraphObligationLocalCeremonyAudit,
    residue_manifest: WorthQueryGraphObligationResidueManifest,
    in_memory_proof: WorthQueryGraphObligationInMemoryProof,
    execution_proof: Option<WorthQueryGraphObligationExecutionProof>,
}

impl WorthQueryGraphObligationAdoptionProof {
    pub(super) fn new(
        manifest: WorthQueryGraphObligationAdoptionManifest,
        support_pin: WorthQueryGraphObligationSupportPin,
        local_ceremony_audit: WorthQueryGraphObligationLocalCeremonyAudit,
        residue_manifest: WorthQueryGraphObligationResidueManifest,
        in_memory_proof: WorthQueryGraphObligationInMemoryProof,
        execution_proof: Option<WorthQueryGraphObligationExecutionProof>,
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

    pub fn manifest(&self) -> &WorthQueryGraphObligationAdoptionManifest {
        &self.manifest
    }

    pub fn support_pin(&self) -> &WorthQueryGraphObligationSupportPin {
        &self.support_pin
    }

    pub fn local_ceremony_audit(&self) -> &WorthQueryGraphObligationLocalCeremonyAudit {
        &self.local_ceremony_audit
    }

    pub fn residue_manifest(&self) -> &WorthQueryGraphObligationResidueManifest {
        &self.residue_manifest
    }

    pub fn in_memory_proof(&self) -> &WorthQueryGraphObligationInMemoryProof {
        &self.in_memory_proof
    }

    pub fn execution_proof(&self) -> Option<&WorthQueryGraphObligationExecutionProof> {
        self.execution_proof.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationExecutionBackedAdoptionProof {
    adoption_proof: WorthQueryGraphObligationAdoptionProof,
    execution_proof: WorthQueryGraphObligationExecutionProof,
}

impl WorthQueryGraphObligationExecutionBackedAdoptionProof {
    pub(super) fn new(
        adoption_proof: WorthQueryGraphObligationAdoptionProof,
        execution_proof: WorthQueryGraphObligationExecutionProof,
    ) -> Self {
        Self {
            adoption_proof,
            execution_proof,
        }
    }

    pub fn adoption_proof(&self) -> &WorthQueryGraphObligationAdoptionProof {
        &self.adoption_proof
    }

    pub fn manifest(&self) -> &WorthQueryGraphObligationAdoptionManifest {
        self.adoption_proof.manifest()
    }

    pub fn support_pin(&self) -> &WorthQueryGraphObligationSupportPin {
        self.adoption_proof.support_pin()
    }

    pub fn local_ceremony_audit(&self) -> &WorthQueryGraphObligationLocalCeremonyAudit {
        self.adoption_proof.local_ceremony_audit()
    }

    pub fn residue_manifest(&self) -> &WorthQueryGraphObligationResidueManifest {
        self.adoption_proof.residue_manifest()
    }

    pub fn in_memory_proof(&self) -> &WorthQueryGraphObligationInMemoryProof {
        self.adoption_proof.in_memory_proof()
    }

    pub fn execution_proof(&self) -> &WorthQueryGraphObligationExecutionProof {
        &self.execution_proof
    }
}
