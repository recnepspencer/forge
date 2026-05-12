use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::adapters::RuntimeDefinitionEnvelope;
use crate::runtime::core::RuntimeCore;

use super::WorkerGraphPublicationSummary;

pub(crate) fn publish_definition_envelope_into_worker_runtime(
    runtime: &mut RuntimeCore,
    envelope: RuntimeDefinitionEnvelope,
) -> Result<WorkerGraphPublicationSummary, ForgeSignalJsError> {
    deny_callback_backed_publication_before_lowering(&envelope)?;

    let (published_source_count, published_recipe_count) =
        runtime.publish_callback_free_definition_envelope(envelope)?;

    Ok(WorkerGraphPublicationSummary {
        published_source_count,
        published_recipe_count,
        admitted_callback_count: 0,
        denied_callback_count: 0,
    })
}

fn deny_callback_backed_publication_before_lowering(
    envelope: &RuntimeDefinitionEnvelope,
) -> Result<(), ForgeSignalJsError> {
    if envelope.unavailable_callbacks.is_empty() {
        return Ok(());
    }

    let denied_callback_ids = envelope
        .unavailable_callbacks
        .iter()
        .map(|artifact| artifact.id.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    Err(ForgeSignalJsError::callback_failure(
        "workerRuntimePublicationRequiresPortableDefinitions",
        format!(
            "worker runtime publication cannot admit callback-backed definitions before placement lowering closes: {denied_callback_ids}"
        ),
        Some(denied_callback_ids),
    ))
}
