use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryRecoveryExplanation {
    checked_topology: ForgeQueryOrdinaryCheckedTopology,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: Option<String>,
    receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
}

impl ForgeQueryRecoveryExplanation {
    pub(crate) fn new(checked_topology: ForgeQueryOrdinaryCheckedTopology) -> Self {
        Self {
            checked_topology,
            route_governing_reason: None,
            route_denial_cause: None,
            receipt_governing_reason: None,
            receipt_denial_cause: None,
        }
    }

    pub(crate) fn with_route_context(
        mut self,
        route_governing_reason: impl Into<String>,
        route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    ) -> Self {
        self.route_governing_reason = Some(route_governing_reason.into());
        self.route_denial_cause = route_denial_cause;
        self
    }

    pub(crate) fn with_receipt_context(
        mut self,
        receipt_governing_reason: impl Into<String>,
        receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    ) -> Self {
        self.receipt_governing_reason = Some(receipt_governing_reason.into());
        self.receipt_denial_cause = receipt_denial_cause;
        self
    }

    pub fn checked_topology(&self) -> &ForgeQueryOrdinaryCheckedTopology {
        &self.checked_topology
    }

    pub fn stop_stage(&self) -> Option<ForgeQueryDeclarationEntryOrchestrationStage> {
        self.checked_topology.orchestration_stop_stage()
    }

    pub fn retained_digest(&self) -> Option<&str> {
        self.checked_topology
            .orchestration_retained_digest()
            .or_else(|| {
                self.checked_topology
                    .contribution_composed_digest()
                    .or_else(|| {
                        self.checked_topology
                            .binding_linked_artifacts()?
                            .envelope_digest()
                    })
                    .or_else(|| {
                        self.checked_topology
                            .continuation_linked_artifacts()?
                            .envelope_digest()
                    })
                    .or_else(|| {
                        self.checked_topology
                            .signal_compatibility_orchestration_linked_artifacts()?
                            .envelope_digest()
                    })
            })
    }

    pub fn refusal_class(&self) -> Option<ForgeQueryDeclarationEntryOrchestrationRefusalClass> {
        self.checked_topology.orchestration_refusal_class()
    }

    pub fn route_governing_reason(&self) -> Option<&str> {
        self.route_governing_reason.as_deref()
    }

    pub fn route_denial_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        self.route_denial_cause
    }

    pub fn receipt_governing_reason(&self) -> Option<&str> {
        self.receipt_governing_reason.as_deref()
    }

    pub fn receipt_denial_cause(&self) -> Option<ForgeQueryDeclarationReceiptDenialCause> {
        self.receipt_denial_cause
    }

    pub fn contribution_digest(&self) -> Option<&str> {
        self.checked_topology.contribution_composed_digest()
    }
}
