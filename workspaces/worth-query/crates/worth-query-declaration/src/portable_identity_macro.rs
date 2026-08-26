/// Declares the stable semantic identity of a Rust type used in portable Query
/// meaning. Moving the type between modules does not alter this identity.
#[macro_export]
macro_rules! worth_query_portable_type {
    ($Type:ty => $identity:literal $(,)?) => {
        impl $crate::facade::portable_identity::WorthQueryPortableType for $Type {
            const PORTABLE_TYPE_IDENTITY:
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity =
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity::declared(
                    $identity,
                );
        }
    };
}
