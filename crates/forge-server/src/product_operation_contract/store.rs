use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    ForgeServerCompletedProductOperation, ForgeServerProductIdempotencyConflict,
    ForgeServerProductIdempotencyKey, ForgeServerProductIdempotencyRecord,
    ForgeServerProductOperationReplayReceipt, ForgeServerProductOperationSurfaceDenial,
    ForgeServerProductOperationSurfaceDenialCode, ForgeServerProductOperationSurfaceDenialFacts,
};

use super::ForgeServerProductIdempotencyBinding;

#[derive(Clone, Debug)]
pub(crate) struct ForgeServerStoredProductOperation {
    record: ForgeServerProductIdempotencyRecord,
}

impl ForgeServerStoredProductOperation {
    pub(crate) fn new(record: ForgeServerProductIdempotencyRecord) -> Self {
        Self { record }
    }

    fn record(&self) -> &ForgeServerProductIdempotencyRecord {
        &self.record
    }
}

pub(crate) fn build_storage_key(binding: &ForgeServerProductIdempotencyBinding) -> String {
    binding.storage_key().to_string()
}

pub(crate) fn admit_replay(
    replay_store: &Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>>,
    storage_key: &str,
    idempotency_key: &ForgeServerProductIdempotencyKey,
    request_digest: &str,
) -> Result<Option<ForgeServerCompletedProductOperation>, ForgeServerProductOperationSurfaceDenial>
{
    let store = replay_store
        .lock()
        .expect("product idempotency replay store mutex should not be poisoned");
    let Some(stored) = store.get(storage_key) else {
        return Ok(None);
    };
    if stored.record().request_digest() != request_digest {
        let conflict = ForgeServerProductIdempotencyConflict::new(
            idempotency_key.value(),
            request_digest,
            stored.record().request_digest(),
        );
        return Err(
            ForgeServerProductOperationSurfaceDenial::new(
                ForgeServerProductOperationSurfaceDenialCode::IdempotencyConflict,
                format!(
                    "product idempotency key `{}` was already bound to request digest `{}` and cannot be reused for `{request_digest}`",
                    idempotency_key.value(),
                    stored.record().request_digest(),
                ),
            )
            .with_facts(
                ForgeServerProductOperationSurfaceDenialFacts::default()
                    .with_idempotency_conflict(conflict),
            ),
        );
    }
    Ok(Some(
        stored.record().completed_operation().to_replayed(
            ForgeServerProductOperationReplayReceipt::replayed(
                idempotency_key.value(),
                request_digest,
                stored
                    .record()
                    .completed_operation()
                    .envelope()
                    .canonical_digest(),
            ),
        ),
    ))
}

pub(crate) fn record_replay(
    replay_store: &Arc<Mutex<HashMap<String, ForgeServerStoredProductOperation>>>,
    storage_key: String,
    request_digest: String,
    completed_operation: ForgeServerCompletedProductOperation,
) {
    replay_store
        .lock()
        .expect("product idempotency replay store mutex should not be poisoned")
        .insert(
            storage_key,
            ForgeServerStoredProductOperation::new(ForgeServerProductIdempotencyRecord::new(
                request_digest,
                completed_operation,
            )),
        );
}
