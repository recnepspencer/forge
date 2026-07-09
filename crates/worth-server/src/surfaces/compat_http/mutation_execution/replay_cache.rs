use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{WorthServerQueryHandoffDenial, WorthServerQueryHandoffDenialCode};

use super::{
    idempotency::{
        WorthServerIdempotencyKey, WorthServerIdempotentReplayReceipt,
        WorthServerStoredCompatibilityMutation,
    },
    response::WorthServerCompatibilityMutation,
};

pub(super) fn try_replay(
    idempotency_store: &Arc<Mutex<HashMap<String, WorthServerStoredCompatibilityMutation>>>,
    prepared_request: &crate::WorthServerCompatibilityPreparedRequest,
    idempotency_key: &Option<WorthServerIdempotencyKey>,
    request_digest: &str,
) -> Result<Option<WorthServerCompatibilityMutation>, WorthServerQueryHandoffDenial> {
    let Some(key) = idempotency_key.as_ref() else {
        return Ok(None);
    };
    let store = idempotency_store
        .lock()
        .expect("compatibility idempotency store mutex should not be poisoned");
    let storage_key = key.scoped_storage_key(prepared_request);
    let Some(stored) = store.get(&storage_key) else {
        return Ok(None);
    };
    if stored.request_digest() != request_digest {
        return Err(WorthServerQueryHandoffDenial::new(
            WorthServerQueryHandoffDenialCode::CompatibilityIdempotencyConflict,
            prepared_request.admission().request_context().diagnostics_profile(),
            format!(
                "compatibility mutation idempotency key `{}` was already bound to request digest `{}` and cannot be reused for `{request_digest}`",
                key.value(),
                stored.request_digest(),
            ),
        )
        .with_facts(
            crate::WorthServerQueryHandoffDenialFacts::default().with_idempotency_conflict(
                key.value(),
                request_digest,
                stored.request_digest(),
            ),
        ));
    }
    Ok(Some(stored.mutation().to_replayed(
        WorthServerIdempotentReplayReceipt::replayed(
            key,
            request_digest,
            stored.mutation().canonical_digest(),
        ),
    )))
}

pub(super) fn record_replay(
    idempotency_store: &Arc<Mutex<HashMap<String, WorthServerStoredCompatibilityMutation>>>,
    prepared_request: &crate::WorthServerCompatibilityPreparedRequest,
    idempotency_key: Option<WorthServerIdempotencyKey>,
    request_digest: String,
    mutation: WorthServerCompatibilityMutation,
) {
    let Some(key) = idempotency_key else {
        return;
    };
    let storage_key = key.scoped_storage_key(prepared_request);
    idempotency_store
        .lock()
        .expect("compatibility idempotency store mutex should not be poisoned")
        .insert(
            storage_key,
            WorthServerStoredCompatibilityMutation::new(request_digest, mutation),
        );
}
