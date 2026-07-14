use serde::Serialize;

use crate::boundary::errors::WorthSignalJsError;
use crate::runtime::core::certification_digest::canonical_certification_digest;

pub(crate) fn canonical_worker_certification_digest<T: Serialize>(
    value: &T,
) -> Result<String, WorthSignalJsError> {
    canonical_certification_digest(value)
}
