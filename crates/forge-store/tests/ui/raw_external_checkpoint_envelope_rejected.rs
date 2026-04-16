use forge_store::{
    EmbeddedCheckpointClassification, EmbeddedStoreHandle, ExternalRuntimeCheckpointEnvelope,
};

fn misuse(mut embedded: EmbeddedStoreHandle) {
    let checkpoint = ExternalRuntimeCheckpointEnvelope::new(
        "checkpoint-raw",
        "embedded-runtime",
        EmbeddedCheckpointClassification::DerivedDurable,
    );
    let _ = embedded.persist_external_checkpoint(checkpoint);
}

fn main() {}
