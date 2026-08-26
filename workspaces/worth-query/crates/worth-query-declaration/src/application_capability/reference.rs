use std::marker::PhantomData;

use crate::portable_identity::WorthQueryPortableTypeIdentity;

macro_rules! named_reference {
    ($name:ident, $($marker:ident),+) => {
        pub struct $name<$($marker),+> {
            name: &'static str,
            marker_identity: WorthQueryPortableTypeIdentity,
            _marker: PhantomData<fn() -> ($($marker),+)>,
        }

        impl<$($marker),+> $name<$($marker),+> {
            #[cfg(test)]
            pub(crate) const fn from_schema_identifier(name: &'static str) -> Self {
                Self::from_test_declaration(
                    name,
                    WorthQueryPortableTypeIdentity::declared(name),
                )
            }

            #[cfg(test)]
            pub(crate) const fn from_test_declaration(
                name: &'static str,
                marker_identity: WorthQueryPortableTypeIdentity,
            ) -> Self {
                Self {
                    name,
                    marker_identity,
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

impl<Schema, Capability> ApplicationCapabilityRef<Schema, Capability>
where
    Capability: super::ApplicationCapabilityMarkerIdentity<Schema = Schema>,
{
    #[doc(hidden)]
    pub const fn from_declaration() -> Self {
        Self {
            name: Capability::IDENTIFIER,
            marker_identity: Capability::PORTABLE_TYPE_IDENTITY,
            _marker: PhantomData,
        }
    }
}

impl<Schema, Context> ApplicationCapabilityContextRef<Schema, Context>
where
    Context: super::ApplicationCapabilityContextMarkerIdentity<Schema = Schema>,
{
    #[doc(hidden)]
    pub const fn from_declaration() -> Self {
        Self {
            name: Context::IDENTIFIER,
            marker_identity: Context::PORTABLE_TYPE_IDENTITY,
            _marker: PhantomData,
        }
    }
}

impl<Schema, Provenance> ApplicationCapabilityProvenanceRef<Schema, Provenance>
where
    Provenance: super::ApplicationCapabilityProvenanceMarkerIdentity<Schema = Schema>,
{
    #[doc(hidden)]
    pub const fn from_declaration() -> Self {
        Self {
            name: Provenance::IDENTIFIER,
            marker_identity: Provenance::PORTABLE_TYPE_IDENTITY,
            _marker: PhantomData,
        }
    }
}

macro_rules! marker_identity_accessor {
    ($name:ident, $($marker:ident),+) => {
        impl<$($marker),+> $name<$($marker),+> {
            pub const fn marker_identity(self) -> WorthQueryPortableTypeIdentity {
                self.marker_identity
            }
        }
    };
}

marker_identity_accessor!(ApplicationCapabilityRef, Schema, Capability);
marker_identity_accessor!(ApplicationCapabilityContextRef, Schema, Context);
marker_identity_accessor!(ApplicationCapabilityProvenanceRef, Schema, Provenance);
