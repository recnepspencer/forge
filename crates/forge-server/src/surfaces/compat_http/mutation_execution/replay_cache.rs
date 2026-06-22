use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode};

use super::{
    idempotency::{
        ForgeServerIdempotencyKey, ForgeServerIdempotentReplayReceipt,
        ForgeServerStoredCompatibilityMutation,
    },
    response::ForgeServerCompatibilityMutation,
};

pub(super) fn try_replay(
    idempotency_store: &Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
    idempotency_key: &Option<ForgeServerIdempotencyKey>,
    request_digest: &str,
) -> Result<Option<ForgeServerCompatibilityMutation>, ForgeServerQueryHandoffDenial> {
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
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityIdempotencyConflict,
            prepared_request.admission().request_context().diagnostics_profile(),
            format!(
                "compatibility mutation idempotency key `{}` was already bound to request digest `{}` and cannot be reused for `{request_digest}`",
                key.value(),
                stored.request_digest(),
            ),
        )
        .with_facts(
            crate::ForgeServerQueryHandoffDenialFacts::default().with_idempotency_conflict(
                key.value(),
                request_digest,
                stored.request_digest(),
            ),
        ));
    }
    Ok(Some(stored.mutation().to_replayed(
        ForgeServerIdempotentReplayReceipt::replayed(
            key,
            request_digest,
            stored.mutation().canonical_digest(),
        ),
    )))
}

pub(super) fn record_replay(
    idempotency_store: &Arc<Mutex<HashMap<String, ForgeServerStoredCompatibilityMutation>>>,
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
    idempotency_key: Option<ForgeServerIdempotencyKey>,
    request_digest: String,
    mutation: ForgeServerCompatibilityMutation,
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
            ForgeServerStoredCompatibilityMutation::new(request_digest, mutation),
        );
}
