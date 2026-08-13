use crate::history::data::CanonicalCommitEnvelope;
use crate::publication::cdc::data::SubscriberCheckpoint;
use crate::publication::{durable_canonical_envelopes, retained_canonical_envelope_at_position};
use crate::runtime::RelationalRuntime;

pub(crate) fn durable_checkpoint_envelope(
    runtime: &RelationalRuntime,
    checkpoint: &SubscriberCheckpoint,
    preloaded_envelopes: Option<&[CanonicalCommitEnvelope]>,
) -> Option<CanonicalCommitEnvelope> {
    retained_canonical_envelope_at_position(runtime, checkpoint.position(), preloaded_envelopes)
}

pub(crate) fn durable_envelopes(runtime: &RelationalRuntime) -> Vec<CanonicalCommitEnvelope> {
    durable_canonical_envelopes(runtime)
}
