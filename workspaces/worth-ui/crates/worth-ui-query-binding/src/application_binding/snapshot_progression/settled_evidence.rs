use worth_query::facade::installed::{conditional, operation};

use super::{
    WorthUiExactSettledSnapshotEvidence, WorthUiQueryConsumerRequirements,
    WorthUiSettledSnapshotFact, WorthUiSettledSnapshotProjection,
};

impl WorthUiSettledSnapshotProjection {
    pub(crate) fn attach_source_coordinates(
        &mut self,
        generation: super::super::WorthUiSettledSnapshotSourceGeneration,
        order: super::super::WorthUiSettledSnapshotSourceOrder,
    ) {
        std::sync::Arc::get_mut(&mut self.fact)
            .expect("source coordinates attach before settled fact sharing")
            .attach_source_coordinates(generation, order);
    }

    pub fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.reference
    }

    pub fn ui_requirements(&self) -> WorthUiQueryConsumerRequirements {
        self.requirements
    }

    pub fn fact(&self) -> &WorthUiSettledSnapshotFact {
        &self.fact
    }

    pub(crate) fn shared_fact(&self) -> std::sync::Arc<WorthUiSettledSnapshotFact> {
        std::sync::Arc::clone(&self.fact)
    }

    pub fn execution_warnings(&self) -> &[operation::WorthQueryOperationExecutionWarning] {
        self.settled.warnings()
    }

    pub fn projection_warnings(
        &self,
    ) -> Option<&worth_query::facade::foundation::ProjectionConsumptionWarnings> {
        self.settled.projection_warnings()
    }

    pub fn result_state(&self) -> operation::WorthQueryOperationResultState {
        self.settled.result_state()
    }

    pub fn counters(&self) -> operation::WorthQueryOperationExecutionCounters {
        self.settled.counters()
    }

    pub fn resource_admission_counters(
        &self,
    ) -> operation::WorthQueryExecutionResourceAdmissionCounters {
        self.settled.resources().counters()
    }

    pub fn publication_receipt(&self) -> &operation::WorthQueryDerivedPublicationReceipt {
        self.settled.publication_receipt()
    }

    pub fn conditional_provenance(&self) -> &[conditional::WorthQueryConditionalProvenance] {
        self.settled.conditional_provenance()
    }

    pub fn exact_evidence(&self) -> WorthUiExactSettledSnapshotEvidence {
        WorthUiExactSettledSnapshotEvidence {
            installed_reference: self.reference.clone(),
            binding_reference: self.fact.binding_reference().clone(),
            settlement_reference: self.fact.settlement_reference().clone(),
            installation_is_current: self.reference.installation_is_current(),
            ui_requirements: self.requirements,
        }
    }
}

impl WorthUiExactSettledSnapshotEvidence {
    pub fn installed_reference(&self) -> &crate::WorthUiInstalledQueryBindingReference {
        &self.installed_reference
    }

    pub fn binding_reference(&self) -> &super::WorthUiAdmittedQueryBindingReference {
        &self.binding_reference
    }

    pub fn settlement_reference(&self) -> &super::WorthUiAdmittedQuerySettlementReference {
        &self.settlement_reference
    }

    pub fn installation_is_current(&self) -> bool {
        self.installation_is_current
    }

    pub fn ui_requirements(&self) -> WorthUiQueryConsumerRequirements {
        self.ui_requirements
    }
}
