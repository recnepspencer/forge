use std::collections::BTreeMap;

use crate::boundary::errors::ForgeSignalJsError;
use crate::recipe::model::TransactionOp;
use crate::runtime::adapters::RuntimeDefinitionEnvelope;
use crate::runtime::core::RuntimeCore;
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::summaries::RuntimeAsyncLifecycleCertification;

use super::{
    committed_truth_digest_for_runtime, publish_definition_envelope_into_worker_runtime,
    WorkerBrowserHistoryIngress, WorkerBrowserHistoryIngressReport,
    WorkerCallbackCapabilityExportCertificationPackage, WorkerDefinitionEnvelopePublicationReport,
    WorkerDiagnosticsSummaryReadPacket, WorkerGraphPublicationSummary, WorkerHostBoundaryCausality,
    WorkerHostCapabilityIngressBatch, WorkerHostCapabilityIngressReport,
    WorkerHostEffectAcknowledgement, WorkerHostEffectAcknowledgementReport,
    WorkerHostEffectRequest, WorkerHostEffectRequestEnvelope,
    WorkerMainThreadHostBridgeCertificationPackage, WorkerMainThreadHostedCallbackRequestEnvelope,
    WorkerMainThreadHostedCallbackResultReport, WorkerObservationDeliveryPacket,
    WorkerOutputDeliveryPacket, WorkerPortableGraphPublication,
    WorkerReplayCheckpointRetainedHistoryCertificationPackage,
    WorkerReplayCheckpointRetainedHistoryReport, WorkerReplayRestoreCapabilityCertificationPackage,
    WorkerReplayRestoreCapabilityReport, WorkerRuntimeBootstrapRecord,
    WorkerRuntimeEnvelopeImportReport, WorkerRuntimeShellLock,
};
use super::{
    WorkerImportExportCallbackUnavailabilityCertificationPackage, WorkerLifecycleControlPacket,
    WorkerObservationDeliverySubscription,
};

pub struct WorkerRuntimeShell {
    pub(in crate::runtime::worker_host) core: RuntimeCore,
    pub(in crate::runtime::worker_host) next_host_boundary_sequence: u64,
    pub(in crate::runtime::worker_host) latest_host_capability_report:
        Option<WorkerHostCapabilityIngressReport>,
    pub(in crate::runtime::worker_host) latest_browser_history_report:
        Option<WorkerBrowserHistoryIngressReport>,
    pub(in crate::runtime::worker_host) latest_host_effect_request:
        Option<WorkerHostEffectRequestEnvelope>,
    pub(in crate::runtime::worker_host) latest_host_effect_acknowledgement:
        Option<WorkerHostEffectAcknowledgementReport>,
    pub(in crate::runtime::worker_host) latest_main_thread_hosted_callback_request:
        Option<WorkerMainThreadHostedCallbackRequestEnvelope>,
    pub(in crate::runtime::worker_host) latest_main_thread_hosted_callback_report:
        Option<WorkerMainThreadHostedCallbackResultReport>,
    pub(in crate::runtime::worker_host) latest_worker_runtime_envelope_import_report:
        Option<WorkerRuntimeEnvelopeImportReport>,
    pub(in crate::runtime::worker_host) latest_worker_callback_capability_export_certification:
        Option<WorkerCallbackCapabilityExportCertificationPackage>,
    pub(in crate::runtime::worker_host) latest_worker_runtime_envelope_import_denial_report:
        Option<WorkerRuntimeEnvelopeImportReport>,
    pub(in crate::runtime::worker_host) latest_worker_runtime_envelope_import_reattachment_report:
        Option<WorkerRuntimeEnvelopeImportReport>,
    pub(in crate::runtime::worker_host) latest_worker_definition_publication_report:
        Option<WorkerDefinitionEnvelopePublicationReport>,
    pub(in crate::runtime::worker_host) latest_worker_observation_delivery_packet:
        Option<WorkerObservationDeliveryPacket>,
    pub(in crate::runtime::worker_host) latest_worker_output_delivery_packet:
        Option<WorkerOutputDeliveryPacket>,
    pub(in crate::runtime::worker_host) latest_worker_diagnostics_summary_read_packet:
        Option<WorkerDiagnosticsSummaryReadPacket>,
    pub(in crate::runtime::worker_host) latest_worker_lifecycle_control_packet:
        Option<WorkerLifecycleControlPacket>,
    pub(in crate::runtime::worker_host) latest_worker_replay_restore_capability_report:
        Option<WorkerReplayRestoreCapabilityReport>,
    pub(in crate::runtime::worker_host) latest_worker_replay_checkpoint_retained_history_report:
        Option<WorkerReplayCheckpointRetainedHistoryReport>,
    pub(in crate::runtime::worker_host) latest_worker_replay_restore_capability_certification:
        Option<WorkerReplayRestoreCapabilityCertificationPackage>,
    pub(in crate::runtime::worker_host) latest_worker_replay_checkpoint_retained_history_certification:
        Option<WorkerReplayCheckpointRetainedHistoryCertificationPackage>,
    pub(in crate::runtime::worker_host) latest_worker_import_export_callback_unavailability_certification:
        Option<WorkerImportExportCallbackUnavailabilityCertificationPackage>,
    pub(in crate::runtime::worker_host) next_worker_lifecycle_subscription_id: u64,
    pub(in crate::runtime::worker_host) worker_observation_delivery_subscriptions:
        BTreeMap<u64, WorkerObservationDeliverySubscription>,
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
            latest_main_thread_hosted_callback_request: None,
            latest_main_thread_hosted_callback_report: None,
            latest_worker_runtime_envelope_import_report: None,
            latest_worker_callback_capability_export_certification: None,
            latest_worker_runtime_envelope_import_denial_report: None,
            latest_worker_runtime_envelope_import_reattachment_report: None,
            latest_worker_definition_publication_report: None,
            latest_worker_observation_delivery_packet: None,
            latest_worker_output_delivery_packet: None,
            latest_worker_diagnostics_summary_read_packet: None,
            latest_worker_lifecycle_control_packet: None,
            latest_worker_replay_restore_capability_report: None,
            latest_worker_replay_checkpoint_retained_history_report: None,
            latest_worker_replay_restore_capability_certification: None,
            latest_worker_replay_checkpoint_retained_history_certification: None,
            latest_worker_import_export_callback_unavailability_certification: None,
            next_worker_lifecycle_subscription_id: 1,
            worker_observation_delivery_subscriptions: BTreeMap::new(),
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
        publication.validate_public_output_ids()?;
        let output_ids = publication.output_ids.clone();
        let summary = self.publish_definition_envelope(publication.into_definition_envelope())?;
        self.core.mark_worker_public_outputs(output_ids)?;
        Ok(summary)
    }

    pub fn publish_definition_envelope(
        &mut self,
        envelope: RuntimeDefinitionEnvelope,
    ) -> Result<WorkerGraphPublicationSummary, ForgeSignalJsError> {
        let summary = publish_definition_envelope_into_worker_runtime(&mut self.core, envelope)?;
        self.clear_worker_boundary_certification_evidence();
        Ok(summary)
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

    pub fn certify_async_lifecycle(
        &mut self,
        id: &str,
        payload_contract_id: u64,
        payload_byte_len: u64,
    ) -> Result<RuntimeAsyncLifecycleCertification, ForgeSignalJsError> {
        self.core
            .certify_runtime_async_lifecycle(id, payload_contract_id, payload_byte_len)
    }

    pub(in crate::runtime::worker_host) fn next_host_boundary_causality(
        &mut self,
    ) -> WorkerHostBoundaryCausality {
        let causality = WorkerHostBoundaryCausality::new(self.next_host_boundary_sequence);
        self.next_host_boundary_sequence = self.next_host_boundary_sequence.saturating_add(1);
        causality
    }

    pub(in crate::runtime::worker_host) fn clear_worker_boundary_certification_evidence(&mut self) {
        self.latest_host_capability_report = None;
        self.latest_browser_history_report = None;
        self.latest_host_effect_request = None;
        self.latest_host_effect_acknowledgement = None;
        self.latest_main_thread_hosted_callback_request = None;
        self.latest_main_thread_hosted_callback_report = None;
        self.latest_worker_runtime_envelope_import_report = None;
        self.latest_worker_callback_capability_export_certification = None;
        self.latest_worker_runtime_envelope_import_denial_report = None;
        self.latest_worker_runtime_envelope_import_reattachment_report = None;
        self.latest_worker_definition_publication_report = None;
        self.latest_worker_observation_delivery_packet = None;
        self.latest_worker_output_delivery_packet = None;
        self.latest_worker_diagnostics_summary_read_packet = None;
        self.latest_worker_lifecycle_control_packet = None;
        self.latest_worker_replay_restore_capability_report = None;
        self.latest_worker_replay_checkpoint_retained_history_report = None;
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
