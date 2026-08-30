use crate::portable_identity::WorthQueryPortableTypeIdentity;

/// Declaration-owned stable identities for every Rust marker carried by a query.
pub trait ApplicationQueryMarkerIdentity {
    type Schema;
    type Parameters;
    type QueryResult;
    type Scope;

    const IDENTIFIER: &'static str;
    const QUERY_TYPE_NAME: &'static str;
    const PARAMETER_TYPE_NAME: &'static str;
    const RESULT_TYPE_NAME: &'static str;
    const SCOPE_TYPE_NAME: &'static str;
    const QUERY_TYPE_IDENTITY: WorthQueryPortableTypeIdentity =
        WorthQueryPortableTypeIdentity::declared(Self::QUERY_TYPE_NAME);
    const PARAMETER_TYPE_IDENTITY: WorthQueryPortableTypeIdentity =
        WorthQueryPortableTypeIdentity::declared(Self::PARAMETER_TYPE_NAME);
    const RESULT_TYPE_IDENTITY: WorthQueryPortableTypeIdentity =
        WorthQueryPortableTypeIdentity::declared(Self::RESULT_TYPE_NAME);
    const SCOPE_TYPE_IDENTITY: WorthQueryPortableTypeIdentity =
        WorthQueryPortableTypeIdentity::declared(Self::SCOPE_TYPE_NAME);
}
