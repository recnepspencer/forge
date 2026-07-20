use crate::runtime::{
    WorthUiCandidateAdmissionCounters, WorthUiCounterCaptureRichness,
    WorthUiDurableStateReconciliationCounters, WorthUiExecutionPlanEquivalenceCounters,
    WorthUiImpactLookupCounters, WorthUiMeasurementCounterPacket, WorthUiPlanLoweringCounters,
    WorthUiPlanTopologyCounters, WorthUiQueryLiveRebindCounters, WorthUiQuerySupportReceipt,
    WorthUiRuntimeArtifactComparisonCounters, WorthUiRuntimeCounterFamily,
    WorthUiRuntimeHandleAllocationCounters,
};

use super::counter_schema;
use super::denial::{WorthUiReloadCounterBoundaryDenial, WorthUiReloadCounterBoundaryDenialReason};
use super::phase_rows;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiReloadCounterStopStage {
    CandidateAdmission,
    ArtifactComparison,
    ImpactNarrowing,
    IdentityMatching,
    StateReconciliation,
    QueryRebindPlanning,
    PlanLowering,
    PlanAssembly,
    PlanEquivalence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadLoweringCounterReceipt {
    stopped_at: WorthUiReloadCounterStopStage,
    packets: Vec<WorthUiMeasurementCounterPacket>,
    carried_query_contract_identities: Vec<u64>,
    query_support_rediscovery_count: u32,
    context: Option<WorthUiReloadCostContext>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadCostContext {
    active_generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    candidate_generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    active_plan_digest: u64,
    candidate_plan_digest: u64,
    affected_scope_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCertifiedReloadLoweringCounterReceipt {
    receipt: WorthUiReloadLoweringCounterReceipt,
}

#[derive(Clone, Debug)]
pub struct WorthUiReloadLoweringCounterReceiptBuilder {
    stopped_at: WorthUiReloadCounterStopStage,
    capture_richness: WorthUiCounterCaptureRichness,
    packets: Vec<WorthUiMeasurementCounterPacket>,
    pending_query_rebind_counters: Option<WorthUiQueryLiveRebindCounters>,
    carried_query_contract_identities: Vec<u64>,
    query_support_rediscovery_count: u32,
    construction_denial: Option<WorthUiReloadCounterBoundaryDenialReason>,
    context: Option<WorthUiReloadCostContext>,
}

impl WorthUiReloadLoweringCounterReceiptBuilder {
    pub(crate) fn new(stopped_at: WorthUiReloadCounterStopStage) -> Self {
        Self {
            stopped_at,
            capture_richness: WorthUiCounterCaptureRichness::Standard,
            packets: Vec::new(),
            pending_query_rebind_counters: None,
            carried_query_contract_identities: Vec::new(),
            query_support_rediscovery_count: 0,
            construction_denial: None,
            context: None,
        }
    }

    pub fn with_capture_richness(mut self, richness: WorthUiCounterCaptureRichness) -> Self {
        self.capture_richness = richness;
        self
    }

    pub(crate) fn with_cost_context(mut self, context: WorthUiReloadCostContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn record_admission_counters(
        mut self,
        counters: WorthUiCandidateAdmissionCounters,
    ) -> Self {
        self.push_packet(
            WorthUiRuntimeCounterFamily::ReloadCandidateAdmission,
            phase_rows::admission_rows(counters),
            false,
        );
        self
    }

    pub fn record_artifact_comparison_counters(
        mut self,
        counters: WorthUiRuntimeArtifactComparisonCounters,
    ) -> Self {
        self.push_packet(
            WorthUiRuntimeCounterFamily::ArtifactComparison,
            phase_rows::artifact_comparison_rows(counters),
            false,
        );
        self
    }

    pub fn record_impact_narrowing_counters(
        mut self,
        counters: WorthUiImpactLookupCounters,
    ) -> Self {
        self.push_packet(
            WorthUiRuntimeCounterFamily::ImpactNarrowing,
            phase_rows::impact_narrowing_rows(counters),
            false,
        );
        self
    }

    pub fn record_identity_match_counters(
        mut self,
        counters: crate::runtime::WorthUiIdentityMatchCounters,
    ) -> Self {
        self.push_packet(
            WorthUiRuntimeCounterFamily::IdentityReplacement,
            phase_rows::identity_rows(counters),
            false,
        );
        self
    }

    pub fn record_reconciliation_counters(
        mut self,
        counters: WorthUiDurableStateReconciliationCounters,
    ) -> Self {
        self.push_packet(
            WorthUiRuntimeCounterFamily::DurableStateReconciliation,
            phase_rows::reconciliation_rows(counters),
            false,
        );
        self
    }

    pub fn record_query_rebind_counters(
        mut self,
        counters: WorthUiQueryLiveRebindCounters,
    ) -> Self {
        self.pending_query_rebind_counters = Some(counters);
        self
    }

    pub fn record_carried_query_support_receipt(
        mut self,
        receipt: WorthUiQuerySupportReceipt,
    ) -> Self {
        self.carried_query_contract_identities
            .push(receipt.contract_identity().as_u64());
        self.carried_query_contract_identities.sort();
        self.carried_query_contract_identities.dedup();
        self
    }

    pub fn record_plan_lowering_counters(mut self, counters: WorthUiPlanLoweringCounters) -> Self {
        self.push_packet(
            WorthUiRuntimeCounterFamily::PlanLowering,
            phase_rows::plan_lowering_rows(counters),
            false,
        );
        self
    }

    pub fn record_plan_assembly_counters(
        mut self,
        handle: WorthUiRuntimeHandleAllocationCounters,
        topology: WorthUiPlanTopologyCounters,
        equivalence: WorthUiExecutionPlanEquivalenceCounters,
    ) -> Self {
        self.push_packet(
            WorthUiRuntimeCounterFamily::PlanAssembly,
            phase_rows::plan_assembly_rows(handle, topology, equivalence),
            false,
        );
        self
    }

    pub fn record_query_support_rediscovery(mut self) -> Self {
        self.query_support_rediscovery_count += 1;
        self
    }

    #[cfg(test)]
    pub(crate) fn record_measurement_packet_for_test(
        mut self,
        packet: WorthUiMeasurementCounterPacket,
    ) -> Self {
        self.packets.push(packet);
        self
    }

    pub fn seal(
        mut self,
    ) -> Result<WorthUiReloadLoweringCounterReceipt, WorthUiReloadCounterBoundaryDenial> {
        self.materialize_pending_query_rebind_packet();
        if let Some(reason) = self.construction_denial {
            return Err(WorthUiReloadCounterBoundaryDenial::new(reason));
        }
        if self.packets.is_empty() {
            return Err(WorthUiReloadCounterBoundaryDenial::new(
                WorthUiReloadCounterBoundaryDenialReason::EmptyCounterReceipt,
            ));
        }
        if self.query_support_rediscovery_count > 0 {
            return Err(WorthUiReloadCounterBoundaryDenial::new(
                WorthUiReloadCounterBoundaryDenialReason::RepeatedQuerySupportRediscovery,
            ));
        }
        if self.packets.iter().any(has_forbidden_broad_work) {
            return Err(WorthUiReloadCounterBoundaryDenial::new(
                WorthUiReloadCounterBoundaryDenialReason::FullArtifactScanDetected,
            ));
        }
        counter_schema::validate_receipt_packet_schema(&self.packets)?;
        Ok(WorthUiReloadLoweringCounterReceipt {
            stopped_at: self.stopped_at,
            packets: self.packets,
            carried_query_contract_identities: self.carried_query_contract_identities,
            query_support_rediscovery_count: self.query_support_rediscovery_count,
            context: self.context,
        })
    }

    fn push_packet(
        &mut self,
        family: WorthUiRuntimeCounterFamily,
        rows: Vec<crate::runtime::WorthUiFrameCostCounter>,
        needs_query_evidence: bool,
    ) {
        let mut builder = family
            .at_boundary(phase_rows::boundary_for_family(family))
            .with_capture_richness(self.capture_richness);
        for row in rows {
            builder = builder.record(row);
        }
        if needs_query_evidence {
            for evidence in phase_rows::query_evidence_rows(&self.carried_query_contract_identities)
            {
                builder = builder.with_query_evidence(evidence);
            }
        }
        match builder.seal() {
            Ok(packet) => self.packets.push(packet),
            Err(denial) => {
                self.construction_denial = Some(
                    WorthUiReloadCounterBoundaryDenialReason::MeasurementCertification(denial),
                );
            }
        }
    }

    fn materialize_pending_query_rebind_packet(&mut self) {
        if let Some(counters) = self.pending_query_rebind_counters.take() {
            self.push_packet(
                WorthUiRuntimeCounterFamily::QueryRebindPlanning,
                phase_rows::query_rebind_rows(counters),
                true,
            );
        }
    }
}

impl WorthUiReloadLoweringCounterReceipt {
    pub fn stopped_at(&self) -> WorthUiReloadCounterStopStage {
        self.stopped_at
    }

    pub fn packets(&self) -> &[WorthUiMeasurementCounterPacket] {
        &self.packets
    }

    pub fn carried_query_receipt_count(&self) -> u32 {
        self.carried_query_contract_identities.len() as u32
    }

    pub fn carried_query_contract_identities(&self) -> &[u64] {
        &self.carried_query_contract_identities
    }

    pub fn query_support_rediscovery_count(&self) -> u32 {
        self.query_support_rediscovery_count
    }

    pub fn context(&self) -> Option<&WorthUiReloadCostContext> {
        self.context.as_ref()
    }

    pub fn certify(
        self,
    ) -> Result<WorthUiCertifiedReloadLoweringCounterReceipt, WorthUiReloadCounterBoundaryDenial>
    {
        counter_schema::validate_receipt_packet_schema(&self.packets)?;
        Ok(WorthUiCertifiedReloadLoweringCounterReceipt { receipt: self })
    }
}

impl WorthUiReloadCostContext {
    pub(crate) fn new(
        active_generation: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
        candidate_generation: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        active_plan_digest: u64,
        candidate_plan_digest: u64,
        affected_scope_count: usize,
    ) -> Self {
        Self {
            active_generation,
            candidate_generation,
            active_artifact_digest,
            candidate_artifact_digest,
            active_plan_digest,
            candidate_plan_digest,
            affected_scope_count,
        }
    }

    pub fn active_generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.active_generation
    }

    pub fn candidate_generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.candidate_generation
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }
    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }
    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }
    pub fn candidate_plan_digest(&self) -> u64 {
        self.candidate_plan_digest
    }
    pub fn affected_scope_count(&self) -> usize {
        self.affected_scope_count
    }
}

impl WorthUiCertifiedReloadLoweringCounterReceipt {
    pub fn receipt(&self) -> &WorthUiReloadLoweringCounterReceipt {
        &self.receipt
    }
}

fn has_forbidden_broad_work(packet: &WorthUiMeasurementCounterPacket) -> bool {
    packet.counters().iter().any(|counter| {
        counter.value() > 0
            && matches!(
                counter.name(),
                "reload.impact_narrowing.full_artifact_scans"
                    | "plan.assembly.handle_broad_registry_scans"
                    | "plan.assembly.topology_artifact_tree_scans"
                    | "plan.assembly.topology_broad_registry_scans"
                    | "plan.assembly.equivalence_artifact_tree_scans"
            )
    })
}
