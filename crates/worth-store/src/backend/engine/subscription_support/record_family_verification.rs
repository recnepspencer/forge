use crate::failure::StoreError;

use super::super::{StateBackedStoreBackend, StatePersistence};

pub(super) fn verify_subscription_support_record_family<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Result<(), StoreError> {
    backend.state.verify_subscription_support_record_family()
}
