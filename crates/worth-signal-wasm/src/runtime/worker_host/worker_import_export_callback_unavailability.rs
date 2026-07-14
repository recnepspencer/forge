use serde::Serialize;

use crate::boundary::errors::WorthSignalJsError;
use crate::runtime::adapters::UnavailableCallbackArtifact;

use super::{
    canonical_worker_certification_digest, WorkerCallbackCapabilityExportCertificationPackage,
    WorkerRuntimeEnvelopeImportReport, WorkerRuntimeShell,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerImportExportCallbackUnavailabilityCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub exported_callback_count: u64,
    pub unavailable_callback_count: u64,
    pub rejected_callback_count: u64,
    pub reattached_callback_count: u64,
    pub host_capability_transport_count: u64,
    pub fallback_count: u64,
    pub export_digest: String,
    pub import_digest: String,
    pub capability_reattachment_digest: String,
    pub callback_unavailability_artifact: &'static str,
    pub callback_unavailability_digest: String,
    pub certification_digest: String,
    pub unavailable_callbacks: Vec<UnavailableCallbackArtifact>,
}

impl WorkerImportExportCallbackUnavailabilityCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_retained_evidence(
        shell: &WorkerRuntimeShell,
    ) -> Result<Self, WorthSignalJsError> {
        let export = retained_export(shell)?;
        ensure_callback_unavailability_export(export)?;
        let portable_denial = retained_portable_denial(shell)?;
        let reattachment_import = retained_reattachment_import(shell)?;
        ensure_portable_denial_matches_export(export, portable_denial)?;
        ensure_reattachment_import_matches_export(export, reattachment_import)?;

        let unavailable_callback_ids = unavailable_callback_ids(export);
        let capability_reattachment_digest = canonical_worker_certification_digest(&(
            "workerImportExportCallbackReattachment",
            &reattachment_import.reattached_callback_ids,
            reattachment_import.reattached_callback_count,
            reattachment_import.host_capability_transport_count,
            reattachment_import.worker_first_truth_digest.as_str(),
        ))?;
        let callback_unavailability_digest = canonical_worker_certification_digest(&(
            "workerImportExportCallbackUnavailability",
            &unavailable_callback_ids,
            &export.unavailable_callbacks,
            portable_denial.import_digest.as_str(),
        ))?;
        let import_digest = canonical_worker_certification_digest(&(
            "workerImportExportCallbackImport",
            portable_denial.import_digest.as_str(),
            reattachment_import.import_digest.as_str(),
        ))?;
        let fallback_count = export
            .fallback_count
            .saturating_add(portable_denial.fallback_count)
            .saturating_add(reattachment_import.fallback_count);
        let certification_digest = canonical_worker_certification_digest(&(
            "workerImportExportCallbackUnavailabilityCertification",
            export.certification_digest.as_str(),
            import_digest.as_str(),
            capability_reattachment_digest.as_str(),
            callback_unavailability_digest.as_str(),
            fallback_count,
        ))?;

        Ok(Self {
            certification_family: "workerImportExportCallbackUnavailabilityCertification",
            covered_suite_count: 1,
            exported_callback_count: export.exported_callback_count,
            unavailable_callback_count: export.unavailable_callback_count,
            rejected_callback_count: portable_denial.rejected_callback_count,
            reattached_callback_count: reattachment_import.reattached_callback_count,
            host_capability_transport_count: export.host_capability_transport_count,
            fallback_count,
            export_digest: export.certification_digest.clone(),
            import_digest,
            capability_reattachment_digest,
            callback_unavailability_artifact: "computeCallbackUnavailableForPortableExport",
            callback_unavailability_digest,
            certification_digest,
            unavailable_callbacks: export.unavailable_callbacks.clone(),
        })
    }
}

impl WorkerRuntimeShell {
    pub fn certify_worker_import_export_callback_unavailability(
        &mut self,
    ) -> Result<WorkerImportExportCallbackUnavailabilityCertificationPackage, WorthSignalJsError>
    {
        let package =
            WorkerImportExportCallbackUnavailabilityCertificationPackage::from_retained_evidence(
                self,
            )?;
        self.latest_worker_import_export_callback_unavailability_certification =
            Some(package.clone());
        Ok(package)
    }
}

fn retained_export(
    shell: &WorkerRuntimeShell,
) -> Result<&WorkerCallbackCapabilityExportCertificationPackage, WorthSignalJsError> {
    shell
        .latest_worker_callback_capability_export_certification
        .as_ref()
        .ok_or_else(|| {
            WorthSignalJsError::invalid_input(
                "worker import/export callback certification requires export evidence",
            )
        })
}

fn retained_portable_denial(
    shell: &WorkerRuntimeShell,
) -> Result<&WorkerRuntimeEnvelopeImportReport, WorthSignalJsError> {
    shell
        .latest_worker_runtime_envelope_import_denial_report
        .as_ref()
        .ok_or_else(|| {
            WorthSignalJsError::invalid_input(
                "worker import/export callback certification requires portable import denial evidence",
            )
        })
}

fn retained_reattachment_import(
    shell: &WorkerRuntimeShell,
) -> Result<&WorkerRuntimeEnvelopeImportReport, WorthSignalJsError> {
    shell
        .latest_worker_runtime_envelope_import_reattachment_report
        .as_ref()
        .ok_or_else(|| {
            WorthSignalJsError::invalid_input(
                "worker import/export callback certification requires reattachment import evidence",
            )
        })
}

fn ensure_callback_unavailability_export(
    export: &WorkerCallbackCapabilityExportCertificationPackage,
) -> Result<(), WorthSignalJsError> {
    if export.unavailable_callback_count == 0 || export.unavailable_callbacks.is_empty() {
        return Err(WorthSignalJsError::invalid_input(
            "worker import/export callback certification requires callback-unavailability export artifacts",
        ));
    }
    if export.fallback_count != 0 {
        return Err(WorthSignalJsError::invalid_input(
            "worker import/export callback certification requires zero export fallback",
        ));
    }
    Ok(())
}

fn ensure_portable_denial_matches_export(
    export: &WorkerCallbackCapabilityExportCertificationPackage,
    portable_denial: &WorkerRuntimeEnvelopeImportReport,
) -> Result<(), WorthSignalJsError> {
    let exported_ids = unavailable_callback_ids(export);
    if portable_denial.import_outcome != "Denied"
        || portable_denial.rejected_callback_ids != exported_ids
        || portable_denial.rejected_callback_count != export.unavailable_callback_count
        || portable_denial.host_capability_transport_count != export.host_capability_transport_count
        || portable_denial.fallback_count != 0
    {
        return Err(WorthSignalJsError::invalid_input(
            "worker import/export callback certification requires matching portable denial evidence",
        ));
    }
    Ok(())
}

fn ensure_reattachment_import_matches_export(
    export: &WorkerCallbackCapabilityExportCertificationPackage,
    reattachment_import: &WorkerRuntimeEnvelopeImportReport,
) -> Result<(), WorthSignalJsError> {
    let exported_ids = unavailable_callback_ids(export);
    if reattachment_import.import_outcome != "AdmittedWithReattachments"
        || reattachment_import.reattached_callback_ids != exported_ids
        || reattachment_import.reattached_callback_count != export.unavailable_callback_count
        || reattachment_import.host_capability_transport_count
            != export.host_capability_transport_count
        || reattachment_import.fallback_count != 0
    {
        return Err(WorthSignalJsError::invalid_input(
            "worker import/export callback certification requires matching reattachment import evidence",
        ));
    }
    Ok(())
}

fn unavailable_callback_ids(
    export: &WorkerCallbackCapabilityExportCertificationPackage,
) -> Vec<String> {
    export
        .unavailable_callbacks
        .iter()
        .map(|artifact| artifact.id.clone())
        .collect()
}
