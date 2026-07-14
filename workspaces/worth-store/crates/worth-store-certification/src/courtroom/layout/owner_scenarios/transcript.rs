use super::LayoutOwnerObservationLedger;

#[derive(Debug)]
pub struct LayoutOwnerScenarioTranscript {
    observations: LayoutOwnerObservationLedger,
    performance: worth_store_layout_indexes::LayoutAccessPerformanceReceipt,
    durable: super::durable_observation::LayoutDurableObservationSource,
}

impl LayoutOwnerScenarioTranscript {
    pub const fn observations(&self) -> &LayoutOwnerObservationLedger {
        &self.observations
    }

    pub const fn performance(&self) -> &worth_store_layout_indexes::LayoutAccessPerformanceReceipt {
        &self.performance
    }

    pub(crate) fn into_evidence_parts(
        self,
    ) -> (
        LayoutOwnerObservationLedger,
        worth_store_layout_indexes::LayoutAccessPerformanceReceipt,
        super::durable_observation::LayoutDurableObservationSource,
    ) {
        (self.observations, self.performance, self.durable)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutOwnerScenarioExecutionDenial {
    CurrentSecurityScopeUnavailable,
}

pub fn execute_declaration_owner_scenarios(
) -> Result<LayoutOwnerScenarioTranscript, LayoutOwnerScenarioExecutionDenial> {
    let mut ledger = LayoutOwnerObservationLedger::default();
    super::declarations::execute(&mut ledger)?;
    super::materialization::execute(&mut ledger);
    super::planning::execute(&mut ledger);
    let access = super::access::execute(&mut ledger);
    super::evolution::execute(&mut ledger);
    super::durable::execute(&mut ledger);
    super::maintenance::execute(&mut ledger);
    super::integrity::execute(&mut ledger);
    let durable = super::durable_observation::observe_durable_artifacts(access.btree);
    Ok(LayoutOwnerScenarioTranscript {
        observations: ledger,
        performance: access.performance,
        durable,
    })
}
