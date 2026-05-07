use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::adapters::RuntimeDefinitionEnvelope;
use crate::runtime::core::RuntimeCore;

use super::WorkerGraphPublicationSummary;

pub(crate) fn publish_definition_envelope_into_worker_runtime(
    runtime: &mut RuntimeCore,
    envelope: RuntimeDefinitionEnvelope,
) -> Result<WorkerGraphPublicationSummary, ForgeSignalJsError> {
    deny_callback_backed_publication_before_lowering(&envelope)?;

    let mut published_source_count = 0_u64;
    let mut published_recipe_count = 0_u64;

    for family in envelope.source_families {
        runtime.define_source_family(family)?;
    }
    for family in envelope.recipe_families {
        runtime.define_keyed_recipe_family(family)?;
    }
    for source in envelope.sources {
        runtime.define_source(source)?;
        published_source_count = published_source_count.saturating_add(1);
    }
    for recipe in envelope.recipes {
        runtime.define_recipe(recipe)?;
        published_recipe_count = published_recipe_count.saturating_add(1);
    }

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
