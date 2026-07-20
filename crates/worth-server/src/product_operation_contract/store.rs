use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    WorthServerCompletedProductOperation, WorthServerProductIdempotencyConflict,
    WorthServerProductIdempotencyKey, WorthServerProductIdempotencyRecord,
    WorthServerProductOperationRetryReceipt, WorthServerProductOperationSurfaceDenial,
    WorthServerProductOperationSurfaceDenialCode, WorthServerProductOperationSurfaceDenialFacts,
};

use super::WorthServerProductIdempotencyBinding;

#[derive(Clone, Debug)]
pub(crate) struct WorthServerStoredProductOperation {
    record: WorthServerProductIdempotencyRecord,
}

impl WorthServerStoredProductOperation {
    pub(crate) fn new(record: WorthServerProductIdempotencyRecord) -> Self {
        Self { record }
    }

    fn record(&self) -> &WorthServerProductIdempotencyRecord {
        &self.record
    }
}

pub(crate) fn build_storage_key(binding: &WorthServerProductIdempotencyBinding) -> String {
    binding.storage_key().to_string()
}

pub(crate) fn admit_retry(
    retry_store: &Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    storage_key: &str,
    idempotency_key: &WorthServerProductIdempotencyKey,
    request_digest: &str,
) -> Result<Option<WorthServerCompletedProductOperation>, WorthServerProductOperationSurfaceDenial>
{
    let store = retry_store
        .lock()
        .expect("product idempotency retry store mutex should not be poisoned");
    let Some(stored) = store.get(storage_key) else {
        return Ok(None);
    };
    if stored.record().request_digest() != request_digest {
        let conflict = WorthServerProductIdempotencyConflict::new(
            idempotency_key.value(),
            request_digest,
            stored.record().request_digest(),
        );
        return Err(
            WorthServerProductOperationSurfaceDenial::new(
                WorthServerProductOperationSurfaceDenialCode::IdempotencyConflict,
                format!(
                    "product idempotency key `{}` was already bound to request digest `{}` and cannot be reused for `{request_digest}`",
                    idempotency_key.value(),
                    stored.record().request_digest(),
                ),
            )
            .with_facts(
                WorthServerProductOperationSurfaceDenialFacts::default()
                    .with_idempotency_conflict(conflict),
            ),
        );
    }
    Ok(Some(
        stored
            .record()
            .completed_operation()
            .as_previously_committed(
                WorthServerProductOperationRetryReceipt::previously_committed(
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

pub(crate) fn record_retry(
    retry_store: &Arc<Mutex<HashMap<String, WorthServerStoredProductOperation>>>,
    storage_key: String,
    request_digest: String,
    completed_operation: WorthServerCompletedProductOperation,
) {
    retry_store
        .lock()
        .expect("product idempotency retry store mutex should not be poisoned")
        .insert(
            storage_key,
            WorthServerStoredProductOperation::new(WorthServerProductIdempotencyRecord::new(
                request_digest,
                completed_operation,
            )),
        );
}
