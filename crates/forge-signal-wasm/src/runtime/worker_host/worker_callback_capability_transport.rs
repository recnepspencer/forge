use serde::Serialize;

use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::adapters::{
    RuntimeDefinitionEnvelope, RuntimeEnvelope, UnavailableCallbackArtifact,
};
use crate::runtime::core::ExactRuntimeRestoreArtifact;
pub(crate) use crate::runtime::core::RuntimeEnvelopeCallbackReattachment;
use crate::runtime::summaries::RuntimeStoreSnapshot;

use super::{
    canonical_worker_certification_digest, committed_truth_digest_for_runtime, WorkerRuntimeShell,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCallbackCapabilityExportCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub exported_callback_count: u64,
    pub unavailable_callback_count: u64,
    pub host_capability_transport_count: u64,
    pub fallback_count: u64,
    pub placement_digest: String,
    pub replay_import_compatibility_digest: String,
    pub capability_transport_digest: String,
    pub certification_digest: String,
    pub unavailable_callbacks: Vec<UnavailableCallbackArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerRuntimeEnvelopeImportReport {
    pub envelope_family: &'static str,
    pub import_outcome: &'static str,
    pub rejected_callback_count: u64,
    pub reattached_callback_count: u64,
    pub host_capability_transport_count: u64,
    pub fallback_count: u64,
    pub rejected_callback_ids: Vec<String>,
    pub reattached_callback_ids: Vec<String>,
    pub worker_first_truth_digest: String,
    pub import_digest: String,
}

impl WorkerCallbackCapabilityExportCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_runtime_export(
        shell: &mut WorkerRuntimeShell,
    ) -> Result<Self, ForgeSignalJsError> {
        let envelope = shell.core.export_runtime_envelope()?;
        let placement = shell.core.worker_callback_placement_eligibility()?;
        let unavailable_callbacks = envelope.definitions.unavailable_callbacks;
        let host_capability_transport_count =
            host_capability_transport_count(&unavailable_callbacks);
        let capability_transport_digest = canonical_worker_certification_digest(&(
            "workerCallbackCapabilityExport",
            &unavailable_callbacks,
            host_capability_transport_count,
            0_u64,
        ))?;
        let certification_digest = canonical_worker_certification_digest(&(
            "workerCallbackCapabilityExportCertification",
            placement.placement_digest.as_str(),
            placement.replay_import_compatibility_digest.as_str(),
            capability_transport_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerCallbackCapabilityExportCertification",
            covered_suite_count: 1,
            exported_callback_count: unavailable_callbacks.len() as u64,
            unavailable_callback_count: unavailable_callbacks.len() as u64,
            host_capability_transport_count,
            fallback_count: 0,
            placement_digest: placement.placement_digest,
            replay_import_compatibility_digest: placement.replay_import_compatibility_digest,
            capability_transport_digest,
            certification_digest,
            unavailable_callbacks,
        })
    }
}

impl WorkerRuntimeEnvelopeImportReport {
    pub(in crate::runtime::worker_host) fn rejected_callbacks(
        unavailable_callbacks: &[UnavailableCallbackArtifact],
        worker_first_truth_digest: String,
    ) -> Result<Self, ForgeSignalJsError> {
        let rejected_callback_ids = unavailable_callbacks
            .iter()
            .map(|artifact| artifact.id.clone())
            .collect::<Vec<_>>();
        let host_capability_transport_count =
            host_capability_transport_count(unavailable_callbacks);
        let import_digest = canonical_worker_certification_digest(&(
            "workerRuntimeEnvelopeImport",
            "Denied",
            &rejected_callback_ids,
            Vec::<String>::new(),
            0_u64,
            host_capability_transport_count,
            0_u64,
            worker_first_truth_digest.as_str(),
        ))?;
        Ok(Self {
            envelope_family: "workerRuntimeEnvelopeImport",
            import_outcome: "Denied",
            rejected_callback_count: rejected_callback_ids.len() as u64,
            reattached_callback_count: 0,
            host_capability_transport_count,
            fallback_count: 0,
            rejected_callback_ids,
            reattached_callback_ids: Vec::new(),
            worker_first_truth_digest,
            import_digest,
        })
    }

    pub(in crate::runtime::worker_host) fn admitted(
        import_outcome: &'static str,
        reattached_callback_ids: Vec<String>,
        reattached_callback_count: u64,
        host_capability_transport_count: u64,
        worker_first_truth_digest: String,
    ) -> Result<Self, ForgeSignalJsError> {
        let import_digest = canonical_worker_certification_digest(&(
            "workerRuntimeEnvelopeImport",
            import_outcome,
            Vec::<String>::new(),
            &reattached_callback_ids,
            reattached_callback_count,
            host_capability_transport_count,
            0_u64,
            worker_first_truth_digest.as_str(),
        ))?;
        Ok(Self {
            envelope_family: "workerRuntimeEnvelopeImport",
            import_outcome,
            rejected_callback_count: 0,
            reattached_callback_count,
            host_capability_transport_count,
            fallback_count: 0,
            rejected_callback_ids: Vec::new(),
            reattached_callback_ids,
            worker_first_truth_digest,
            import_digest,
        })
    }

    pub(in crate::runtime::worker_host) fn rejected(
        envelope: &RuntimeEnvelope,
        worker_first_truth_digest: String,
    ) -> Result<Self, ForgeSignalJsError> {
        Self::rejected_callbacks(
            &envelope.definitions.unavailable_callbacks,
            worker_first_truth_digest,
        )
    }
}

impl WorkerRuntimeShell {
    pub fn export_worker_runtime_envelope(
        &mut self,
    ) -> Result<RuntimeEnvelope, ForgeSignalJsError> {
        self.core.export_runtime_envelope()
    }

    pub fn export_exact_worker_runtime_restore_artifact(
        &mut self,
    ) -> Result<ExactRuntimeRestoreArtifact, ForgeSignalJsError> {
        self.core.export_exact_runtime_restore_artifact()
    }

    pub fn certify_worker_callback_capability_export(
        &mut self,
    ) -> Result<WorkerCallbackCapabilityExportCertificationPackage, ForgeSignalJsError> {
        let package =
            WorkerCallbackCapabilityExportCertificationPackage::from_runtime_export(self)?;
        self.latest_worker_callback_capability_export_certification = Some(package.clone());
        Ok(package)
    }

    pub fn admit_worker_runtime_envelope_import(
        &mut self,
        envelope: RuntimeEnvelope,
    ) -> Result<WorkerRuntimeEnvelopeImportReport, ForgeSignalJsError> {
        if !envelope.definitions.unavailable_callbacks.is_empty() {
            let report = WorkerRuntimeEnvelopeImportReport::rejected(
                &envelope,
                committed_truth_digest_for_runtime(&self.core)?,
            )?;
            self.latest_worker_runtime_envelope_import_report = Some(report.clone());
            self.latest_worker_runtime_envelope_import_denial_report = Some(report.clone());
            return Ok(report);
        }
        self.core.replace_runtime_envelope(envelope)?;
        self.clear_worker_boundary_certification_evidence();
        let report = WorkerRuntimeEnvelopeImportReport::admitted(
            "Admitted",
            Vec::new(),
            0,
            0,
            committed_truth_digest_for_runtime(&self.core)?,
        )?;
        self.latest_worker_runtime_envelope_import_report = Some(report.clone());
        Ok(report)
    }

    pub fn admit_exact_worker_runtime_restore_artifact(
        &mut self,
        artifact: ExactRuntimeRestoreArtifact,
    ) -> Result<WorkerRuntimeEnvelopeImportReport, ForgeSignalJsError> {
        self.core.replace_runtime_envelope_exact(artifact)?;
        self.clear_worker_boundary_certification_evidence();
        let report = WorkerRuntimeEnvelopeImportReport::admitted(
            "AdmittedExact",
            Vec::new(),
            0,
            0,
            committed_truth_digest_for_runtime(&self.core)?,
        )?;
        self.latest_worker_runtime_envelope_import_report = Some(report.clone());
        Ok(report)
    }

    pub fn admit_worker_runtime_envelope_import_portable_artifact(
        &mut self,
        definitions: RuntimeDefinitionEnvelope,
        state: RuntimeStoreSnapshot,
    ) -> Result<WorkerRuntimeEnvelopeImportReport, ForgeSignalJsError> {
        if !definitions.unavailable_callbacks.is_empty() {
            let report = WorkerRuntimeEnvelopeImportReport::rejected_callbacks(
                &definitions.unavailable_callbacks,
                committed_truth_digest_for_runtime(&self.core)?,
            )?;
            self.latest_worker_runtime_envelope_import_report = Some(report.clone());
            self.latest_worker_runtime_envelope_import_denial_report = Some(report.clone());
            return Ok(report);
        }
        self.core
            .replace_runtime_envelope_portable_artifact(definitions, state)?;
        self.clear_worker_boundary_certification_evidence();
        let report = WorkerRuntimeEnvelopeImportReport::admitted(
            "Admitted",
            Vec::new(),
            0,
            0,
            committed_truth_digest_for_runtime(&self.core)?,
        )?;
        self.latest_worker_runtime_envelope_import_report = Some(report.clone());
        Ok(report)
    }

    pub fn admit_worker_runtime_envelope_import_with_callback_reattachments(
        &mut self,
        envelope: RuntimeEnvelope,
        reattachments: Vec<RuntimeEnvelopeCallbackReattachment>,
    ) -> Result<WorkerRuntimeEnvelopeImportReport, ForgeSignalJsError> {
        let host_capability_transport_count =
            host_capability_transport_count(&envelope.definitions.unavailable_callbacks);
        let retained_export = self
            .latest_worker_callback_capability_export_certification
            .clone();
        let retained_denial = self
            .latest_worker_runtime_envelope_import_denial_report
            .clone();
        let reattached_callback_ids = envelope
            .definitions
            .unavailable_callbacks
            .iter()
            .map(|artifact| artifact.id.clone())
            .collect::<Vec<_>>();
        let reattached_callback_count = self
            .core
            .replace_runtime_envelope_with_callback_reattachments(envelope, reattachments)?;
        self.clear_worker_boundary_certification_evidence();
        self.latest_worker_callback_capability_export_certification = retained_export;
        self.latest_worker_runtime_envelope_import_denial_report = retained_denial;
        let import_outcome = if reattached_callback_count == 0 {
            "Admitted"
        } else {
            "AdmittedWithReattachments"
        };
        let report = WorkerRuntimeEnvelopeImportReport::admitted(
            import_outcome,
            reattached_callback_ids,
            reattached_callback_count,
            host_capability_transport_count,
            committed_truth_digest_for_runtime(&self.core)?,
        )?;
        self.latest_worker_runtime_envelope_import_report = Some(report.clone());
        self.latest_worker_runtime_envelope_import_reattachment_report = Some(report.clone());
        Ok(report)
    }
}

fn host_capability_transport_count(artifacts: &[UnavailableCallbackArtifact]) -> u64 {
    artifacts
        .iter()
        .map(|artifact| artifact.host_capability_transports.len() as u64)
        .sum()
}
