use crate::boundary::errors::ForgeSignalJsError;
use crate::recipe::model::TransactionOp;
use crate::runtime::adapters::RuntimeDefinitionEnvelope;
use crate::runtime::core::RuntimeCore;
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::summaries::{ObservationSurfaceSummary, RuntimeAsyncLifecycleCertification};
use forge_signal::facade::history::{RuntimeBranch, RuntimeSnapshot};
use forge_signal::facade::runtime::ObservationHandle;

use super::{
    committed_truth_digest_for_runtime, publish_definition_envelope_into_worker_runtime,
    WorkerBranchTruthEnvelope, WorkerBrowserHistoryIngress, WorkerBrowserHistoryIngressReport,
    WorkerCommittedTransactionEnvelope, WorkerGraphPublicationSummary, WorkerHostBoundaryCausality,
    WorkerHostCapabilityIngressBatch, WorkerHostCapabilityIngressReport,
    WorkerHostEffectAcknowledgement, WorkerHostEffectAcknowledgementReport,
    WorkerHostEffectRequest, WorkerHostEffectRequestEnvelope,
    WorkerMainThreadHostBridgeCertificationPackage, WorkerPortableGraphPublication,
    WorkerRuntimeBootstrapRecord, WorkerRuntimeShellLock,
};

pub struct WorkerRuntimeShell {
    core: RuntimeCore,
    next_host_boundary_sequence: u64,
    latest_host_capability_report: Option<WorkerHostCapabilityIngressReport>,
    latest_browser_history_report: Option<WorkerBrowserHistoryIngressReport>,
    latest_host_effect_request: Option<WorkerHostEffectRequestEnvelope>,
    latest_host_effect_acknowledgement: Option<WorkerHostEffectAcknowledgementReport>,
}

impl WorkerRuntimeShell {
    pub fn new(policy: RuntimePolicySpec) -> Result<Self, ForgeSignalJsError> {
        Ok(Self {
            core: RuntimeCore::new(policy)?,
            next_host_boundary_sequence: 0,
            latest_host_capability_report: None,
            latest_browser_history_report: None,
            latest_host_effect_request: None,
            latest_host_effect_acknowledgement: None,
        })
    }

    pub fn bootstrap_record(&self) -> WorkerRuntimeBootstrapRecord {
        WorkerRuntimeBootstrapRecord::worker_first_portable_runtime()
    }

    pub fn shell_lock(&self) -> WorkerRuntimeShellLock {
        WorkerRuntimeShellLock::dedicated_worker_runtime_shell()
    }

    pub fn publish_graph(
        &mut self,
        publication: WorkerPortableGraphPublication,
    ) -> Result<WorkerGraphPublicationSummary, ForgeSignalJsError> {
        self.publish_definition_envelope(publication.into_definition_envelope())
    }

    pub fn publish_definition_envelope(
        &mut self,
        envelope: RuntimeDefinitionEnvelope,
    ) -> Result<WorkerGraphPublicationSummary, ForgeSignalJsError> {
        let summary = publish_definition_envelope_into_worker_runtime(&mut self.core, envelope)?;
        self.clear_main_thread_host_bridge_certification_evidence();
        Ok(summary)
    }

    pub fn apply_committed_transaction(
        &mut self,
        ops: Vec<TransactionOp>,
    ) -> Result<WorkerCommittedTransactionEnvelope, ForgeSignalJsError> {
        let run_summary = self.core.apply_transaction(ops)?;
        let branch = self.core.current_branch();
        let committed_truth_digest = committed_truth_digest_for_runtime(&self.core)?;
        let envelope = WorkerCommittedTransactionEnvelope::from_committed_worker_transaction(
            branch.id.0,
            committed_truth_digest,
            run_summary,
        );
        self.clear_main_thread_host_bridge_certification_evidence();
        Ok(envelope)
    }

    pub fn admit_host_capability_ingress(
        &mut self,
        batch: WorkerHostCapabilityIngressBatch,
    ) -> Result<WorkerHostCapabilityIngressReport, ForgeSignalJsError> {
        super::worker_host_capability_ingress::reject_malformed_host_capability_updates(
            batch.updates.as_slice(),
        )?;
        let submitted_artifact_counts =
            super::worker_host_capability_ingress::host_capability_artifact_counts(
                batch.updates.as_slice(),
            );
        let coalesced_updates =
            super::worker_host_capability_ingress::coalesce_host_capability_updates(batch.updates);
        let runtime_values =
            super::worker_host_capability_ingress::runtime_values_for_host_capability_admission(
                coalesced_updates.as_slice(),
            )?;
        let runtime_mutation_breadth = if runtime_values.is_empty() {
            0
        } else {
            self.core
                .apply_transaction(vec![TransactionOp::SetMany {
                    values: runtime_values,
                }])?
                .touched_nodes
        };
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&self.core)?;

        let report = WorkerHostCapabilityIngressReport::from_coalesced_updates(
            coalesced_updates,
            submitted_artifact_counts,
            self.next_host_boundary_causality(),
            worker_first_truth_digest,
            runtime_mutation_breadth,
        )?;
        self.latest_host_capability_report = Some(report.clone());
        Ok(report)
    }

    pub fn admit_browser_history_ingress(
        &mut self,
        ingress: WorkerBrowserHistoryIngress,
    ) -> Result<WorkerBrowserHistoryIngressReport, ForgeSignalJsError> {
        let runtime_values =
            super::worker_browser_history_ingress::runtime_values_for_browser_history_admission(
                &ingress,
            )?;
        let admission_width =
            super::worker_browser_history_ingress::browser_history_admission_width(&ingress);
        let runtime_mutation_breadth = if runtime_values.is_empty() {
            0
        } else {
            self.core
                .apply_transaction(vec![TransactionOp::SetMany {
                    values: runtime_values,
                }])?
                .touched_nodes
        };
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&self.core)?;

        let report = WorkerBrowserHistoryIngressReport::from_ingress(
            ingress,
            self.next_host_boundary_causality(),
            admission_width.runtime_admitted_route_count,
            admission_width.runtime_admitted_continuity_count,
            runtime_mutation_breadth,
            worker_first_truth_digest,
        )?;
        self.latest_browser_history_report = Some(report.clone());
        Ok(report)
    }

    pub fn issue_host_effect_request(
        &mut self,
        request: WorkerHostEffectRequest,
    ) -> Result<WorkerHostEffectRequestEnvelope, ForgeSignalJsError> {
        let envelope = WorkerHostEffectRequestEnvelope::from_request(
            request,
            self.next_host_boundary_causality(),
        )?;
        self.latest_host_effect_request = Some(envelope.clone());
        Ok(envelope)
    }

    pub fn admit_host_effect_acknowledgement(
        &mut self,
        acknowledgement: WorkerHostEffectAcknowledgement,
    ) -> Result<WorkerHostEffectAcknowledgementReport, ForgeSignalJsError> {
        let bridged_acknowledgement =
            super::worker_host_effect_boundary::bridge_host_effect_acknowledgement(acknowledgement);
        let readmitted_lifecycle =
            super::worker_host_effect_boundary::readmit_host_effect_acknowledgement(
                bridged_acknowledgement,
            )?;
        let runtime_admitted_lifecycle_count = readmitted_lifecycle.runtime_values.len() as u64;
        let runtime_mutation_breadth = if readmitted_lifecycle.runtime_values.is_empty() {
            0
        } else {
            self.core
                .apply_transaction(vec![TransactionOp::SetMany {
                    values: readmitted_lifecycle.runtime_values,
                }])?
                .touched_nodes
        };
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&self.core)?;

        let report = WorkerHostEffectAcknowledgementReport::from_acknowledgement(
            readmitted_lifecycle.acknowledgement,
            self.next_host_boundary_causality(),
            runtime_admitted_lifecycle_count,
            runtime_mutation_breadth,
            worker_first_truth_digest,
        )?;
        self.latest_host_effect_acknowledgement = Some(report.clone());
        Ok(report)
    }

    pub fn certify_main_thread_host_bridge(
        &self,
    ) -> Result<WorkerMainThreadHostBridgeCertificationPackage, ForgeSignalJsError> {
        WorkerMainThreadHostBridgeCertificationPackage::from_boundary_reports(
            self.latest_host_capability_report()?,
            self.latest_browser_history_report()?,
            self.latest_host_effect_request()?,
            self.latest_host_effect_acknowledgement()?,
        )
    }

    pub fn observe_signal_for_runtime_certification(
        &mut self,
        id: &str,
    ) -> Result<ObservationHandle, ForgeSignalJsError> {
        self.core.observe_signal_for_runtime_certification(id)
    }

    pub fn latest_observation_summary(
        &self,
    ) -> Result<Option<ObservationSurfaceSummary>, ForgeSignalJsError> {
        self.core.latest_observation()
    }

    pub fn diagnostics_summary_now(
        &self,
    ) -> Result<forge_signal::facade::diagnostics::GraphSummary, ForgeSignalJsError> {
        self.core.diagnostics_summary_now()
    }

    pub fn certify_async_lifecycle(
        &mut self,
        id: &str,
        payload_contract_id: u64,
        payload_byte_len: u64,
    ) -> Result<RuntimeAsyncLifecycleCertification, ForgeSignalJsError> {
        self.core
            .certify_runtime_async_lifecycle(id, payload_contract_id, payload_byte_len)
    }

    pub fn create_branch(&mut self, name: String) -> Result<RuntimeBranch, ForgeSignalJsError> {
        self.core.create_branch(name)
    }

    pub fn switch_branch(
        &mut self,
        branch_id: u64,
    ) -> Result<WorkerBranchTruthEnvelope, ForgeSignalJsError> {
        self.core.switch_branch(branch_id)?;
        self.clear_main_thread_host_bridge_certification_evidence();
        self.branch_truth_envelope()
    }

    pub fn branch_snapshot(
        &mut self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshot, ForgeSignalJsError> {
        self.core.branch_snapshot(branch_id)
    }

    pub fn restore_branch_snapshot(
        &mut self,
        branch_id: u64,
        snapshot: RuntimeSnapshot,
    ) -> Result<WorkerBranchTruthEnvelope, ForgeSignalJsError> {
        self.core.restore_branch_snapshot(branch_id, snapshot)?;
        self.clear_main_thread_host_bridge_certification_evidence();
        self.branch_truth_envelope_for_branch(branch_id)
    }

    #[cfg(test)]
    pub fn read_value(
        &mut self,
        id: &str,
    ) -> Result<crate::expression::model::SignalValue, ForgeSignalJsError> {
        self.core.read_value(id)
    }

    pub fn branch_truth_envelope(&self) -> Result<WorkerBranchTruthEnvelope, ForgeSignalJsError> {
        let branch = self.core.current_branch();
        self.branch_truth_envelope_for_branch(branch.id.0)
    }

    fn branch_truth_envelope_for_branch(
        &self,
        branch_id: u64,
    ) -> Result<WorkerBranchTruthEnvelope, ForgeSignalJsError> {
        let proof = self.core.branch_state_proof(branch_id)?;
        Ok(WorkerBranchTruthEnvelope::from_worker_branch(
            proof.branch_id,
            proof.branch_name,
            proof.snapshot_id,
            proof.state_digest,
        ))
    }

    fn next_host_boundary_causality(&mut self) -> WorkerHostBoundaryCausality {
        let causality = WorkerHostBoundaryCausality::new(self.next_host_boundary_sequence);
        self.next_host_boundary_sequence = self.next_host_boundary_sequence.saturating_add(1);
        causality
    }

    fn clear_main_thread_host_bridge_certification_evidence(&mut self) {
        self.latest_host_capability_report = None;
        self.latest_browser_history_report = None;
        self.latest_host_effect_request = None;
        self.latest_host_effect_acknowledgement = None;
    }

    fn latest_host_capability_report(
        &self,
    ) -> Result<&WorkerHostCapabilityIngressReport, ForgeSignalJsError> {
        self.latest_host_capability_report.as_ref().ok_or_else(|| {
            ForgeSignalJsError::invalid_input(
                "main thread host bridge certification requires host capability ingress evidence",
            )
        })
    }

    fn latest_browser_history_report(
        &self,
    ) -> Result<&WorkerBrowserHistoryIngressReport, ForgeSignalJsError> {
        self.latest_browser_history_report.as_ref().ok_or_else(|| {
            ForgeSignalJsError::invalid_input(
                "main thread host bridge certification requires browser history ingress evidence",
            )
        })
    }

    fn latest_host_effect_request(
        &self,
    ) -> Result<&WorkerHostEffectRequestEnvelope, ForgeSignalJsError> {
        self.latest_host_effect_request.as_ref().ok_or_else(|| {
            ForgeSignalJsError::invalid_input(
                "main thread host bridge certification requires host effect request evidence",
            )
        })
    }

    fn latest_host_effect_acknowledgement(
        &self,
    ) -> Result<&WorkerHostEffectAcknowledgementReport, ForgeSignalJsError> {
        self.latest_host_effect_acknowledgement
            .as_ref()
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(
                    "main thread host bridge certification requires host effect acknowledgement evidence",
                )
            })
    }
}
