use serde::Serialize;

use crate::boundary::errors::WORTHSignalJsError;
use crate::runtime::adapters::{RuntimeDefinitionEnvelope, UnavailableCallbackArtifact};
pub(crate) use crate::runtime::core::DefinitionEnvelopeCallbackReattachment;

use super::{
    canonical_worker_certification_digest, committed_truth_digest_for_runtime, WorkerRuntimeShell,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerDefinitionEnvelopePublicationReport {
    pub publication_family: &'static str,
    pub publication_outcome: &'static str,
    pub published_source_count: u64,
    pub published_recipe_count: u64,
    pub reattached_callback_count: u64,
    pub host_capability_transport_count: u64,
    pub fallback_count: u64,
    pub rejected_callback_ids: Vec<String>,
    pub reattached_callback_ids: Vec<String>,
    pub worker_first_truth_digest: String,
    pub publication_digest: String,
}

impl WorkerDefinitionEnvelopePublicationReport {
    fn admitted(
        publication_outcome: &'static str,
        published_source_count: u64,
        published_recipe_count: u64,
        reattached_callback_ids: Vec<String>,
        host_capability_transport_count: u64,
        worker_first_truth_digest: String,
    ) -> Result<Self, WORTHSignalJsError> {
        let reattached_callback_count = reattached_callback_ids.len() as u64;
        let publication_digest = canonical_worker_certification_digest(&(
            "workerDefinitionEnvelopePublication",
            publication_outcome,
            published_source_count,
            published_recipe_count,
            Vec::<String>::new(),
            &reattached_callback_ids,
            reattached_callback_count,
            host_capability_transport_count,
            0_u64,
            worker_first_truth_digest.as_str(),
        ))?;
        Ok(Self {
            publication_family: "workerDefinitionEnvelopePublication",
            publication_outcome,
            published_source_count,
            published_recipe_count,
            reattached_callback_count,
            host_capability_transport_count,
            fallback_count: 0,
            rejected_callback_ids: Vec::new(),
            reattached_callback_ids,
            worker_first_truth_digest,
            publication_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn publish_definition_envelope_with_callback_reattachments(
        &mut self,
        envelope: RuntimeDefinitionEnvelope,
        reattachments: Vec<DefinitionEnvelopeCallbackReattachment>,
    ) -> Result<WorkerDefinitionEnvelopePublicationReport, WORTHSignalJsError> {
        let published_source_count = envelope.sources.len() as u64;
        let published_recipe_count = envelope.recipes.len() as u64;
        let host_capability_transport_count =
            host_capability_transport_count(&envelope.unavailable_callbacks);
        let reattached_callback_ids = envelope
            .unavailable_callbacks
            .iter()
            .map(|artifact| artifact.id.clone())
            .collect::<Vec<_>>();
        let reattached_callback_count = self
            .core
            .publish_definition_envelope_with_callback_reattachments(envelope, reattachments)?;
        self.clear_worker_boundary_certification_evidence();
        let publication_outcome = if reattached_callback_count == 0 {
            "Admitted"
        } else {
            "AdmittedWithReattachments"
        };
        let report = WorkerDefinitionEnvelopePublicationReport::admitted(
            publication_outcome,
            published_source_count,
            published_recipe_count,
            reattached_callback_ids,
            host_capability_transport_count,
            committed_truth_digest_for_runtime(&self.core)?,
        )?;
        self.latest_worker_definition_publication_report = Some(report.clone());
        Ok(report)
    }
}

fn host_capability_transport_count(artifacts: &[UnavailableCallbackArtifact]) -> u64 {
    artifacts
        .iter()
        .map(|artifact| artifact.host_capability_transports.len() as u64)
        .sum()
}
