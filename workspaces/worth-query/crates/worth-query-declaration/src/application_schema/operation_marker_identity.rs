//! Declaration-owned identity for an application operation marker.

use crate::portable_identity::WorthQueryPortableType;

/// Exact schema membership and stable input meaning declared for an operation.
pub trait ApplicationOperationMarkerIdentity {
    type Schema;
    type Input: WorthQueryPortableType;

    const IDENTIFIER: &'static str;
}
