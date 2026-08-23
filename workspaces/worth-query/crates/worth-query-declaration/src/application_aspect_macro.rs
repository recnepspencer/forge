#[macro_export]
macro_rules! worth_query_aspect {
    (
        $vis:vis $Aspect:ident in $Schema:ty, $Entity:ty;
        identity = AspectIdentity($identity:expr),
        revision = AspectContractRevision($revision:expr) $(,)?
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Aspect;

        impl $crate::facade::application_schema::ApplicationAspectMarkerIdentity for $Aspect {
            type Schema = $Schema;
            type Entity = $Entity;
            const IDENTIFIER: &'static str = stringify!($Aspect);
            const ASPECT_IDENTITY: $crate::facade::application_schema::AspectIdentity =
                $crate::facade::application_schema::AspectIdentity($identity);
            const CONTRACT_REVISION: $crate::facade::application_schema::AspectContractRevision =
                $crate::facade::application_schema::AspectContractRevision($revision);
        }

        impl $Aspect {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationAspectRef<$Schema, $Entity, Self> {
                $crate::facade::application_schema::ApplicationAspectRef::from_schema_identifier(
                    stringify!($Aspect),
                )
            }
        }
    };
}
