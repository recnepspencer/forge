#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPortalBindingSuccessionDenial {
    TransactionPermitMismatch,
    MissingRequestBinding {
        request_identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
    },
    StalePredecessorReceipt {
        request_identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
        expected_identity_digest: u64,
        observed_identity_digest: u64,
        expected_generation_digest: u64,
        observed_generation_digest: u64,
    },
    MissingCommittedReceipt {
        neighborhood_identity_digest: u64,
    },
    MissingCanonicalPortalInput {
        receipt_identity_digest: u64,
    },
    SuccessorContractDenied {
        request_identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
    },
    CounterExhausted,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiPortalBindingSuccessionCounters {
    consequences_visited: u16,
    binding_lookups: u16,
    receipt_lookups: u16,
    binding_replacements: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPortalBindingSuccessionLineage {
    request_identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
    predecessor_receipt_digest: u64,
    successor_receipt_digest: u64,
    predecessor_evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
    successor_evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
    neighborhood_identity_digest: u64,
    graph_generation: crate::graph::UiGraphGeneration,
}

#[derive(Clone, Debug)]
pub(crate) struct UiPreparedPortalBindingSuccession {
    pub(super) successor: super::UiPortalInvalidationBindingIndex,
    predecessor_identity_digest: u64,
    receipt: UiPortalBindingSuccessionReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiPortalBindingSuccessionReceipt {
    lineage: Box<[UiPortalBindingSuccessionLineage]>,
    counters: UiPortalBindingSuccessionCounters,
}

impl UiPortalBindingSuccessionCounters {
    fn increment(value: &mut u16) -> Result<(), UiPortalBindingSuccessionDenial> {
        *value = value
            .checked_add(1)
            .ok_or(UiPortalBindingSuccessionDenial::CounterExhausted)?;
        Ok(())
    }
    pub(crate) fn visit(&mut self) -> Result<(), UiPortalBindingSuccessionDenial> {
        Self::increment(&mut self.consequences_visited)
    }
    pub(crate) fn binding_lookup(&mut self) -> Result<(), UiPortalBindingSuccessionDenial> {
        Self::increment(&mut self.binding_lookups)
    }
    pub(crate) fn receipt_lookup(&mut self) -> Result<(), UiPortalBindingSuccessionDenial> {
        Self::increment(&mut self.receipt_lookups)
    }
    pub(crate) fn replacement(&mut self) -> Result<(), UiPortalBindingSuccessionDenial> {
        Self::increment(&mut self.binding_replacements)
    }
    pub fn consequences_visited(self) -> u16 {
        self.consequences_visited
    }
    pub fn binding_lookups(self) -> u16 {
        self.binding_lookups
    }
    pub fn receipt_lookups(self) -> u16 {
        self.receipt_lookups
    }
    pub fn binding_replacements(self) -> u16 {
        self.binding_replacements
    }
}

impl UiPortalBindingSuccessionLineage {
    pub(super) fn new(
        request_identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
        prior: &super::UiAdmittedPortalInvalidationBinding,
        receipt: &crate::runtime::UiAllocationReceipt,
        portal: &crate::runtime::UiPortalAllocationPlanningBasis,
    ) -> Self {
        Self {
            request_identity,
            predecessor_receipt_digest: prior.receipt_identity().identity_digest(),
            successor_receipt_digest: receipt.identity().identity_digest(),
            predecessor_evidence_generation: prior.evidence_generation(),
            successor_evidence_generation: portal.observation().evidence_generation(),
            neighborhood_identity_digest: portal.neighborhood_identity_digest(),
            graph_generation: receipt.generation().neighborhood_generation(),
        }
    }
    pub fn predecessor_evidence_generation(
        self,
    ) -> worth_ui_inspection::UiEvidenceAuthorityGeneration {
        self.predecessor_evidence_generation
    }
    pub fn successor_evidence_generation(
        self,
    ) -> worth_ui_inspection::UiEvidenceAuthorityGeneration {
        self.successor_evidence_generation
    }
    pub fn neighborhood_identity_digest(self) -> u64 {
        self.neighborhood_identity_digest
    }
    pub fn request_identity(self) -> worth_ui_host_contract::UiMeasurementRequestIdentity {
        self.request_identity
    }
    pub fn predecessor_receipt_digest(self) -> u64 {
        self.predecessor_receipt_digest
    }
    pub fn successor_receipt_digest(self) -> u64 {
        self.successor_receipt_digest
    }
    pub fn graph_generation(self) -> crate::graph::UiGraphGeneration {
        self.graph_generation
    }
}

impl UiPreparedPortalBindingSuccession {
    pub(crate) fn new(
        predecessor_identity_digest: u64,
        successor: super::UiPortalInvalidationBindingIndex,
        lineage: Vec<UiPortalBindingSuccessionLineage>,
        counters: UiPortalBindingSuccessionCounters,
    ) -> Self {
        Self {
            successor,
            predecessor_identity_digest,
            receipt: UiPortalBindingSuccessionReceipt {
                lineage: lineage.into_boxed_slice(),
                counters,
            },
        }
    }
    pub(crate) fn receipt(&self) -> UiPortalBindingSuccessionReceipt {
        self.receipt.clone()
    }
    pub(crate) fn predecessor_identity_digest(&self) -> u64 {
        self.predecessor_identity_digest
    }
}

impl UiPortalBindingSuccessionReceipt {
    pub fn lineage(&self) -> &[UiPortalBindingSuccessionLineage] {
        &self.lineage
    }
    pub fn counters(&self) -> UiPortalBindingSuccessionCounters {
        self.counters
    }
}
