use forge_store::{DurableStoreHandle, EmbeddedCheckpointClassification, ExternalRuntimeCheckpointEnvelope};

fn misuse(mut durable: DurableStoreHandle) {
    let checkpoint = ExternalRuntimeCheckpointEnvelope::new(
        "checkpoint-1",
        "external-runtime",
        EmbeddedCheckpointClassification::DerivedDurable,
    );
    let _ = durable.persist_external_checkpoint(checkpoint);
}

fn main() {}
