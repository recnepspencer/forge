use std::marker::PhantomData;

macro_rules! named_reference {
    ($name:ident, $($marker:ident),+) => {
        pub struct $name<$($marker),+> {
            name: &'static str,
            _marker: PhantomData<fn() -> ($($marker),+)>,
        }

        impl<$($marker),+> $name<$($marker),+> {
            #[doc(hidden)]
            pub const fn from_schema_identifier(name: &'static str) -> Self {
                Self {
                    name,
                    _marker: PhantomData,
                }
            }

            pub const fn name(self) -> &'static str {
                self.name
            }
        }

        impl<$($marker),+> Copy for $name<$($marker),+> {}

        impl<$($marker),+> Clone for $name<$($marker),+> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<$($marker),+> std::fmt::Debug for $name<$($marker),+> {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field("name", &self.name)
                    .finish_non_exhaustive()
            }
        }
    };
}

named_reference!(ApplicationCapabilityRef, Schema, Capability);
named_reference!(ApplicationCapabilityContextRef, Schema, Context);
named_reference!(ApplicationCapabilityProvenanceRef, Schema, Provenance);
