#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiAdmittedPortalInvalidationBinding {
    contract: crate::runtime::portal_anchored_allocation::UiAdmittedPortalAnchorContract,
    target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
    receipt_identity: crate::runtime::UiAllocationReceiptIdentity,
    receipt_generation: crate::runtime::UiAllocationReceiptGeneration,
    activation_witness: crate::evidence::UiHostMeasurementAuthorityWitness,
    measurement_basis: crate::evidence::UiMeasurementBasis,
    evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
}

impl Eq for UiAdmittedPortalInvalidationBinding {}

impl UiAdmittedPortalInvalidationBinding {
    pub(super) fn seal(
        contract: crate::runtime::portal_anchored_allocation::UiAdmittedPortalAnchorContract,
        target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
        receipt: &crate::runtime::UiAllocationReceipt,
        activation_witness: crate::evidence::UiHostMeasurementAuthorityWitness,
        measurement_basis: &crate::evidence::UiMeasurementBasis,
    ) -> Option<Self> {
        let previous =
            measurement_basis.host_measurement_result(activation_witness.request_identity())?;
        (contract.graph_generation() == target.primary().graph_generation()
            && contract.neighborhood_identity() == target.primary().neighborhood_identity()
            && receipt.identity().graph_node_identity() == target.primary().graph_node_identity())
        .then(|| Self {
            contract,
            target,
            receipt_identity: receipt.identity().clone(),
            receipt_generation: receipt.generation(),
            activation_witness,
            measurement_basis: measurement_basis.clone(),
            evidence_generation: previous.evidence_generation(),
        })
    }

    pub(super) fn seal_successor(
        &self,
        portal: &crate::runtime::UiPortalAllocationPlanningBasis,
        receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Option<Self> {
        let allocation = receipt.committed_allocation();
        let neighborhood = allocation.allocation_neighborhood();
        let constraint = allocation.planning_basis().allocation_constraint_set()?;
        let input = constraint.portal_anchor_planning_input()?;
        let observation = portal.observation();
        let contract =
            crate::runtime::portal_anchored_allocation::UiAdmittedPortalAnchorContract::seal(
                observation.identity(),
                allocation.measurement_basis(),
                neighborhood,
                input.identity_digest(),
                observation.evidence_generation().as_u64(),
                observation.authority_witness(),
            );
        Self::seal(
            contract,
            self.target.clone(),
            receipt,
            observation.authority_witness(),
            allocation.measurement_basis(),
        )
        .map(|mut binding| {
            binding.evidence_generation = observation.evidence_generation();
            binding
        })
    }

    pub(crate) fn contract(
        &self,
    ) -> &crate::runtime::portal_anchored_allocation::UiAdmittedPortalAnchorContract {
        &self.contract
    }

    pub(crate) fn target(&self) -> &crate::graph::UiAdmittedAllocationInvalidationTargetSet {
        &self.target
    }

    pub(crate) fn receipt_identity(&self) -> &crate::runtime::UiAllocationReceiptIdentity {
        &self.receipt_identity
    }

    pub(crate) fn receipt_generation(&self) -> crate::runtime::UiAllocationReceiptGeneration {
        self.receipt_generation
    }

    pub(crate) fn activation_witness(&self) -> crate::evidence::UiHostMeasurementAuthorityWitness {
        self.activation_witness
    }

    pub(crate) fn evidence_generation(&self) -> worth_ui_inspection::UiEvidenceAuthorityGeneration {
        self.evidence_generation
    }
    pub(crate) fn identity_digest(&self) -> u64 {
        self.contract.identity_digest()
            ^ self.receipt_identity.identity_digest().rotate_left(17)
            ^ self.receipt_generation.identity_digest().rotate_left(31)
            ^ self.evidence_generation.as_u64().rotate_left(47)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiAdmittedPortalMovement {
    binding: UiAdmittedPortalInvalidationBinding,
    measurement_result: crate::evidence::UiMeasurementResult,
    observation: crate::runtime::UiAdmittedPortalAnchorObservation,
    identity_transition: crate::runtime::UiPortalAnchorIdentityTransition,
    authority_probes: u16,
}

impl Eq for UiAdmittedPortalMovement {}

impl UiAdmittedPortalMovement {
    pub(super) fn seal(
        binding: UiAdmittedPortalInvalidationBinding,
        result: &crate::evidence::UiMeasurementResult,
        authority_probes: u16,
    ) -> Result<Self, crate::runtime::UiPortalAnchorSuccessorDenial> {
        let observation = crate::runtime::UiAdmittedPortalAnchorObservation::admit(result)
            .ok_or(crate::runtime::UiPortalAnchorSuccessorDenial::ObservationInvalid)?;
        if binding.activation_witness().evidence_category() != result.evidence_category() {
            return Err(crate::runtime::UiPortalAnchorSuccessorDenial::EvidenceCategoryMismatch);
        }
        if result.evidence_generation().as_u64() <= binding.evidence_generation().as_u64() {
            return Err(crate::runtime::UiPortalAnchorSuccessorDenial::StaleEvidenceGeneration);
        }
        if !binding
            .activation_witness()
            .same_normalization_authority(result.authority_witness())
        {
            return Err(
                crate::runtime::UiPortalAnchorSuccessorDenial::NormalizationAuthorityMismatch,
            );
        }
        let identity_transition = crate::runtime::UiPortalAnchorIdentityTransition::classify(
            binding.contract().identity(),
            observation.identity(),
        );
        Ok(Self {
            binding,
            measurement_result: result.clone(),
            observation,
            identity_transition,
            authority_probes,
        })
    }

    pub fn observation(&self) -> crate::runtime::UiAdmittedPortalAnchorObservation {
        self.observation
    }
    pub(crate) fn measurement_result(&self) -> &crate::evidence::UiMeasurementResult {
        &self.measurement_result
    }
    pub fn identity_transition(&self) -> crate::runtime::UiPortalAnchorIdentityTransition {
        self.identity_transition
    }
    pub fn target(&self) -> &crate::graph::UiAdmittedAllocationInvalidationTargetSet {
        self.binding.target()
    }
    pub fn receipt_identity(&self) -> &crate::runtime::UiAllocationReceiptIdentity {
        self.binding.receipt_identity()
    }
    pub fn receipt_generation(&self) -> crate::runtime::UiAllocationReceiptGeneration {
        self.binding.receipt_generation()
    }
    pub fn authority_probes(&self) -> u16 {
        self.authority_probes
    }
}
