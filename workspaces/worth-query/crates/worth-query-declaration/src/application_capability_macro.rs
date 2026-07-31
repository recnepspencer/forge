#[macro_export]
macro_rules! worth_query_capability {
    ($vis:vis $Capability:ident in $Schema:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Capability;

        impl $Capability {
            pub const fn reference(
            ) -> $crate::facade::application_capability::ApplicationCapabilityRef<$Schema, Self> {
                $crate::facade::application_capability::ApplicationCapabilityRef::from_schema_identifier(
                    stringify!($Capability),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_capability_context {
    ($vis:vis $Context:ident in $Schema:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Context;

        impl $Context {
            pub const fn reference(
            ) -> $crate::facade::application_capability::ApplicationCapabilityContextRef<$Schema, Self>
            {
                $crate::facade::application_capability::ApplicationCapabilityContextRef::from_schema_identifier(
                    stringify!($Context),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_capability_provenance {
    ($vis:vis $Provenance:ident in $Schema:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Provenance;

        impl $Provenance {
            pub const fn reference(
            ) -> $crate::facade::application_capability::ApplicationCapabilityProvenanceRef<$Schema, Self>
            {
                $crate::facade::application_capability::ApplicationCapabilityProvenanceRef::from_schema_identifier(
                    stringify!($Provenance),
                )
            }
        }
    };
}
