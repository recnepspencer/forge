use crate::runtime::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphObligationSupportMatrix,
    ForgeQueryGraphTouchDescriptor,
};

use super::error::{
    ForgeQueryGraphObligationConsumerKitError, ForgeQueryGraphObligationConsumerKitErrorKind,
};
use super::{
    ForgeQueryGraphObligationAdoptionManifest, ForgeQueryGraphObligationAdoptionProof,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationExecutionProof, ForgeQueryGraphObligationInMemoryProof,
    ForgeQueryGraphObligationInMemoryTestWorkspace, ForgeQueryGraphObligationLocalCeremonyAudit,
    ForgeQueryGraphObligationResidueManifest, ForgeQueryGraphObligationSelectorCoverageDeclaration,
    ForgeQueryGraphObligationSupportPin,
};

#[derive(Clone, Debug, Default)]
pub struct ForgeQueryGraphObligationConsumerKit {
    consumer_name: String,
    registration: Option<ForgeQueryGraphObligationConsumerRegistrationDeclaration>,
    selector_coverage: Option<ForgeQueryGraphObligationSelectorCoverageDeclaration>,
    support_pin: Option<ForgeQueryGraphObligationSupportPin>,
    support_matrix: Option<ForgeQueryGraphObligationSupportMatrix>,
    local_ceremony_audit: Option<ForgeQueryGraphObligationLocalCeremonyAudit>,
    residue_manifest: Option<ForgeQueryGraphObligationResidueManifest>,
    in_memory_proof: Option<ForgeQueryGraphObligationInMemoryProof>,
    execution_proof: Option<ForgeQueryGraphObligationExecutionProof>,
}

pub fn graph_obligation_consumer_kit(
    consumer_name: impl Into<String>,
) -> ForgeQueryGraphObligationConsumerKit {
    ForgeQueryGraphObligationConsumerKit::new(consumer_name)
}

impl ForgeQueryGraphObligationConsumerKit {
    pub fn new(consumer_name: impl Into<String>) -> Self {
        Self {
            consumer_name: consumer_name.into(),
            ..Self::default()
        }
    }

    pub fn register_obligations(
        mut self,
        registration: ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ) -> Self {
        self.registration = Some(registration);
        self
    }

    pub fn declare_selector_coverage(
        mut self,
        selector_coverage: ForgeQueryGraphObligationSelectorCoverageDeclaration,
    ) -> Self {
        self.selector_coverage = Some(selector_coverage);
        self
    }

    pub fn pin_support(mut self, support_pin: ForgeQueryGraphObligationSupportPin) -> Self {
        self.support_pin = Some(support_pin);
        self
    }

    pub fn against_support_matrix(
        mut self,
        support_matrix: ForgeQueryGraphObligationSupportMatrix,
    ) -> Self {
        self.support_matrix = Some(support_matrix);
        self
    }

    pub fn audit_local_ceremony(
        mut self,
        local_ceremony_audit: ForgeQueryGraphObligationLocalCeremonyAudit,
    ) -> Self {
        self.local_ceremony_audit = Some(local_ceremony_audit);
        self
    }

    pub fn account_for_residue(
        mut self,
        residue_manifest: ForgeQueryGraphObligationResidueManifest,
    ) -> Self {
        self.residue_manifest = Some(residue_manifest);
        self
    }

    pub fn prove_in_memory(mut self, proof: ForgeQueryGraphObligationInMemoryProof) -> Self {
        self.in_memory_proof = Some(proof);
        self
    }

    pub fn prove_execution(mut self, proof: ForgeQueryGraphObligationExecutionProof) -> Self {
        self.in_memory_proof = Some(proof.selection_proof().clone());
        self.execution_proof = Some(proof);
        self
    }

    pub fn prove_in_memory_selection(
        self,
        touch_descriptor: &ForgeQueryGraphTouchDescriptor,
        operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
    ) -> Result<Self, ForgeQueryGraphObligationConsumerKitError> {
        let registration = self.registration.as_ref().ok_or_else(|| {
            ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingRegistrationDeclaration,
                "graph obligation adoption requires registered obligations before in-memory proof",
            )
        })?;
        let workspace = ForgeQueryGraphObligationInMemoryTestWorkspace::from_registrations(
            registration.registrations().iter().cloned(),
        )?;
        Ok(self.prove_in_memory(workspace.prove_selection(touch_descriptor, operating_world)))
    }

    pub fn prove_execution_with(
        self,
        touch_descriptor: &ForgeQueryGraphTouchDescriptor,
        operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
    ) -> Result<Self, ForgeQueryGraphObligationConsumerKitError> {
        let registration = self.registration.as_ref().ok_or_else(|| {
            ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingRegistrationDeclaration,
                "graph obligation adoption requires registered obligations before execution proof",
            )
        })?;
        let workspace = ForgeQueryGraphObligationInMemoryTestWorkspace::from_registrations(
            registration.registrations().iter().cloned(),
        )?;
        Ok(self.prove_execution(workspace.prove_execution(touch_descriptor, operating_world)))
    }

    pub fn prove_adoption_with_execution(
        self,
    ) -> Result<ForgeQueryGraphObligationAdoptionProof, ForgeQueryGraphObligationConsumerKitError>
    {
        if self.execution_proof.is_none() {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingInMemoryProof,
                "graph obligation adoption closeout requires a real execution proof",
            ));
        }
        self.prove_adoption()
    }

    pub fn prove_adoption(
        self,
    ) -> Result<ForgeQueryGraphObligationAdoptionProof, ForgeQueryGraphObligationConsumerKitError>
    {
        let consumer_name = self.consumer_name.trim().to_string();
        if consumer_name.is_empty() {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::BlankConsumerName,
                "graph obligation adoption requires a consumer name",
            ));
        }
        let registration = self.registration.ok_or_else(|| {
            ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingRegistrationDeclaration,
                "graph obligation adoption requires a registration declaration",
            )
        })?;
        let selector_coverage = self.selector_coverage.ok_or_else(|| {
            ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingSelectorCoverage,
                "graph obligation adoption requires selector coverage declaration",
            )
        })?;
        if selector_coverage.row_count() == 0 {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingSelectorCoverage,
                "graph obligation adoption requires at least one covered selector",
            ));
        }
        if !selector_coverage.covers_registration_declaration(&registration) {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::SelectorCoverageMismatch,
                "graph obligation selector coverage must cover every registered obligation selector",
            ));
        }
        let support_pin = self.support_pin.ok_or_else(|| {
            ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingSupportPins,
                "graph obligation adoption requires support pins",
            )
        })?;
        if support_pin.row_count() == 0 {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingSupportPins,
                "graph obligation adoption requires at least one support pin",
            ));
        }
        let support_matrix = self
            .support_matrix
            .unwrap_or_else(ForgeQueryGraphObligationSupportMatrix::assembly_selection_foundation);
        support_pin.evaluate_for_registrations(&support_matrix, registration.registrations())?;
        let local_ceremony_audit = self.local_ceremony_audit.ok_or_else(|| {
            ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingLocalCeremonyAudit,
                "graph obligation adoption requires a local ceremony audit",
            )
        })?;
        if !local_ceremony_audit.is_evaluated() {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::UnevaluatedLocalCeremonyAudit,
                "graph obligation adoption requires an evaluated source audit, not a synthetic clean artifact",
            ));
        }
        if !local_ceremony_audit.is_clean() {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::LocalCeremonyDetected,
                "graph obligation adoption cannot pass while consumer-local ceremony is present",
            ));
        }
        let in_memory_proof = self.in_memory_proof.ok_or_else(|| {
            ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::MissingInMemoryProof,
                "graph obligation adoption requires an in-memory selection proof",
            )
        })?;
        if in_memory_proof.selected_obligation_count() == 0 {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::EmptyInMemoryProof,
                "graph obligation adoption requires an in-memory proof that selects at least one obligation",
            ));
        }
        if !proof_selects_declared_registrations(&in_memory_proof, &registration) {
            return Err(ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::InMemoryProofRegistrationMismatch,
                "graph obligation in-memory proof selected obligations outside the declared registration set",
            ));
        }
        let residue_manifest = self
            .residue_manifest
            .unwrap_or_else(ForgeQueryGraphObligationResidueManifest::empty);
        let manifest = ForgeQueryGraphObligationAdoptionManifest::new(
            consumer_name,
            &registration,
            &selector_coverage,
            &support_pin,
            support_matrix.matrix_digest(),
            &residue_manifest,
            &local_ceremony_audit,
            &in_memory_proof,
            self.execution_proof.as_ref(),
        );
        Ok(ForgeQueryGraphObligationAdoptionProof::new(
            manifest,
            support_pin,
            local_ceremony_audit,
            residue_manifest,
            in_memory_proof,
            self.execution_proof,
        ))
    }
}

fn proof_selects_declared_registrations(
    proof: &ForgeQueryGraphObligationInMemoryProof,
    registration: &ForgeQueryGraphObligationConsumerRegistrationDeclaration,
) -> bool {
    proof
        .selected_registration_digests()
        .all(|selected_digest| {
            registration
                .registrations()
                .iter()
                .any(|registration| registration.registration_digest() == selected_digest)
        })
}
