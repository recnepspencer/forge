//! Declaration-owned identity for an application effect marker.

use crate::portable_identity::WorthQueryPortableType;

/// Exact schema membership and stable payload meaning declared for an effect.
pub trait ApplicationEffectMarkerIdentity {
    type Schema;
    type Payload: WorthQueryPortableType;

    const IDENTIFIER: &'static str;
}
