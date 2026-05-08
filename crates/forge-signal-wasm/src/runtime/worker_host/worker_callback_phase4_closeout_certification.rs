use serde::Serialize;

use crate::boundary::errors::ForgeSignalJsError;

use super::{
    canonical_worker_certification_digest, committed_truth_digest_for_runtime,
    WorkerDefinitionEnvelopePublicationReport, WorkerRuntimeEnvelopeImportReport,
    WorkerRuntimeShell,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCallbackPhase4CloseoutCertificationPackage {
    pub certification_family: &'static str,
    pub closeout_gate_mode: &'static str,
    pub covered_suite_count: u64,
    pub runtime_envelope_import_outcome: &'static str,
    pub definition_publication_outcome: &'static str,
    pub placement_digest: String,
    pub denial_digest: String,
    pub fallback_digest: String,
    pub capability_availability_digest: String,
    pub replay_import_compatibility_digest: String,
    pub placement_identity_digest: String,
    pub runtime_envelope_import_digest: String,
    pub definition_publication_digest: String,
    pub worker_first_truth_digest: String,
    pub placement_row_count: u64,
    pub imported_reattached_callback_count: u64,
    pub published_reattached_callback_count: u64,
    pub host_capability_transport_count: u64,
    pub fallback_count: u64,
    pub certification_digest: String,
}

impl WorkerCallbackPhase4CloseoutCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_worker_retained_evidence(
        shell: &WorkerRuntimeShell,
    ) -> Result<Self, ForgeSignalJsError> {
        let import_report = shell.latest_worker_runtime_envelope_import_report()?;
        let publication_report = shell.latest_worker_definition_publication_report()?;
        validate_retained_import_report(import_report)?;
        validate_retained_publication_report(publication_report)?;

        let placement = shell.core.worker_callback_placement_eligibility()?;
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&shell.core)?;
        if import_report.worker_first_truth_digest != worker_first_truth_digest
            || publication_report.worker_first_truth_digest != worker_first_truth_digest
        {
            return Err(ForgeSignalJsError::invalid_input(
                "worker callback Phase 4 closeout certification requires current import and publication evidence",
            ));
        }

        let host_capability_transport_count = import_report
            .host_capability_transport_count
            .saturating_add(publication_report.host_capability_transport_count);
        let fallback_count = import_report
            .fallback_count
            .saturating_add(publication_report.fallback_count);
        let certification_digest = canonical_worker_certification_digest(&(
            "workerCallbackPhase4CloseoutCertification",
            "PublicationReattachmentWithPortableImportDenial",
            placement.placement_digest.as_str(),
            placement.denial_digest.as_str(),
            placement.fallback_digest.as_str(),
            placement.capability_availability_digest.as_str(),
            placement.replay_import_compatibility_digest.as_str(),
            placement.placement_identity_digest.as_str(),
            import_report.import_outcome,
            import_report.import_digest.as_str(),
            publication_report.publication_outcome,
            publication_report.publication_digest.as_str(),
            worker_first_truth_digest.as_str(),
            fallback_count,
        ))?;

        Ok(Self {
            certification_family: "workerCallbackPhase4CloseoutCertification",
            closeout_gate_mode: "PublicationReattachmentWithPortableImportDenial",
            covered_suite_count: 3,
            runtime_envelope_import_outcome: import_report.import_outcome,
            definition_publication_outcome: publication_report.publication_outcome,
            placement_digest: placement.placement_digest,
            denial_digest: placement.denial_digest,
            fallback_digest: placement.fallback_digest,
            capability_availability_digest: placement.capability_availability_digest,
            replay_import_compatibility_digest: placement.replay_import_compatibility_digest,
            placement_identity_digest: placement.placement_identity_digest,
            runtime_envelope_import_digest: import_report.import_digest.clone(),
            definition_publication_digest: publication_report.publication_digest.clone(),
            worker_first_truth_digest,
            placement_row_count: placement.rows.len() as u64,
            imported_reattached_callback_count: import_report.reattached_callback_count,
            published_reattached_callback_count: publication_report.reattached_callback_count,
            host_capability_transport_count,
            fallback_count,
            certification_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn certify_worker_callback_phase4_closeout(
        &self,
    ) -> Result<WorkerCallbackPhase4CloseoutCertificationPackage, ForgeSignalJsError> {
        WorkerCallbackPhase4CloseoutCertificationPackage::from_worker_retained_evidence(self)
    }

    pub(in crate::runtime::worker_host) fn latest_worker_runtime_envelope_import_report(
        &self,
    ) -> Result<&WorkerRuntimeEnvelopeImportReport, ForgeSignalJsError> {
        self.latest_worker_runtime_envelope_import_report
            .as_ref()
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(
                    "worker callback Phase 4 closeout certification requires runtime-envelope import evidence",
                )
            })
    }

    pub(in crate::runtime::worker_host) fn latest_worker_definition_publication_report(
        &self,
    ) -> Result<&WorkerDefinitionEnvelopePublicationReport, ForgeSignalJsError> {
        self.latest_worker_definition_publication_report
            .as_ref()
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(
                    "worker callback Phase 4 closeout certification requires definition publication evidence",
                )
            })
    }
}

fn validate_retained_import_report(
    report: &WorkerRuntimeEnvelopeImportReport,
) -> Result<(), ForgeSignalJsError> {
    let denied_callback_identity_is_bound = report.rejected_callback_count > 0
        && report.reattached_callback_count == 0
        && report.rejected_callback_ids.len() as u64 == report.rejected_callback_count
        && report.reattached_callback_ids.is_empty();
    if report.envelope_family != "workerRuntimeEnvelopeImport"
        || report.import_outcome != "Denied"
        || report.fallback_count != 0
        || !denied_callback_identity_is_bound
    {
        return Err(ForgeSignalJsError::invalid_input(
            "worker callback Phase 4 closeout certification requires callback-bearing portable import denial evidence",
        ));
    }
    Ok(())
}

fn validate_retained_publication_report(
    report: &WorkerDefinitionEnvelopePublicationReport,
) -> Result<(), ForgeSignalJsError> {
    let reattached_callback_identity_is_bound = report.reattached_callback_count > 0
        && report.rejected_callback_ids.is_empty()
        && report.reattached_callback_ids.len() as u64 == report.reattached_callback_count;
    if report.publication_family != "workerDefinitionEnvelopePublication"
        || report.publication_outcome != "AdmittedWithReattachments"
        || report.fallback_count != 0
        || !reattached_callback_identity_is_bound
    {
        return Err(ForgeSignalJsError::invalid_input(
            "worker callback Phase 4 closeout certification requires callback reattachment publication evidence",
        ));
    }
    Ok(())
}
