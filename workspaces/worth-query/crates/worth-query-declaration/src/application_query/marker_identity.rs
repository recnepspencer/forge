use crate::portable_identity::WorthQueryPortableTypeIdentity;

/// Declaration-owned stable identities for every Rust marker carried by a query.
pub trait ApplicationQueryMarkerIdentity {
    type Schema;
    type Parameters;
    type QueryResult;
    type Scope;

    const IDENTIFIER: &'static str;
    const QUERY_TYPE_IDENTITY: WorthQueryPortableTypeIdentity;
    const PARAMETER_TYPE_IDENTITY: WorthQueryPortableTypeIdentity;
    const RESULT_TYPE_IDENTITY: WorthQueryPortableTypeIdentity;
    const SCOPE_TYPE_IDENTITY: WorthQueryPortableTypeIdentity;
}
