use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::boundary::errors::ForgeSignalJsError;

pub(crate) fn canonical_certification_digest<T: Serialize>(
    value: &T,
) -> Result<String, ForgeSignalJsError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|err| ForgeSignalJsError::invalid_input(format!("serialization failed: {err}")))?;
    let digest = Sha256::digest(bytes);
    Ok(format!("{digest:x}"))
}
