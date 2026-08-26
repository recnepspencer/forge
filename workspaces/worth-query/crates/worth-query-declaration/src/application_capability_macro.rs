#[macro_export]
macro_rules! worth_query_capability {
    ($vis:vis $Capability:ident in $Schema:ty) => {
        $crate::worth_query_capability!(
            $vis $Capability in $Schema,
            identity stringify!($Capability)
        );
    };
    ($vis:vis $Capability:ident in $Schema:ty, identity $identity:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Capability;

        impl $crate::facade::portable_identity::WorthQueryPortableType for $Capability {
            const PORTABLE_TYPE_IDENTITY:
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity =
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity::declared(
                    $identity,
                );
        }

        impl $crate::facade::application_capability::ApplicationCapabilityMarkerIdentity
            for $Capability
        {
            type Schema = $Schema;
            const IDENTIFIER: &'static str = stringify!($Capability);
        }

        impl $Capability {
            pub const fn reference(
            ) -> $crate::facade::application_capability::ApplicationCapabilityRef<$Schema, Self> {
                $crate::facade::application_capability::ApplicationCapabilityRef::from_declaration()
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_capability_context {
    ($vis:vis $Context:ident in $Schema:ty) => {
        $crate::worth_query_capability_context!(
            $vis $Context in $Schema,
            identity stringify!($Context)
        );
    };
    ($vis:vis $Context:ident in $Schema:ty, identity $identity:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Context;

        impl $crate::facade::portable_identity::WorthQueryPortableType for $Context {
            const PORTABLE_TYPE_IDENTITY:
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity =
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity::declared(
                    $identity,
                );
        }

        impl $crate::facade::application_capability::ApplicationCapabilityContextMarkerIdentity
            for $Context
        {
            type Schema = $Schema;
            const IDENTIFIER: &'static str = stringify!($Context);
        }

        impl $Context {
            pub const fn reference(
            ) -> $crate::facade::application_capability::ApplicationCapabilityContextRef<$Schema, Self>
            {
                $crate::facade::application_capability::ApplicationCapabilityContextRef::from_declaration()
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_capability_context_entity_slot {
    (
        $vis:vis $Slot:ident in $Schema:ty,
        $Context:ty => $Entity:ty
    ) => {
        $crate::worth_query_capability_context_entity_slot!(
            $vis $Slot in $Schema,
            $Context => $Entity,
            identity stringify!($Slot)
        );
    };
    (
        $vis:vis $Slot:ident in $Schema:ty,
        $Context:ty => $Entity:ty,
        identity $identity:expr
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Slot;

        impl $crate::facade::portable_identity::WorthQueryPortableType for $Slot {
            const PORTABLE_TYPE_IDENTITY:
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity =
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity::declared(
                    $identity,
                );
        }

        impl $crate::facade::application_capability::ApplicationCapabilityContextEntitySlotMarkerIdentity
            for $Slot
        {
            type Schema = $Schema;
            type Context = $Context;
            type Entity = $Entity;
            const IDENTIFIER: &'static str = stringify!($Slot);
        }

        impl $Slot {
            pub const fn reference(
            ) -> $crate::facade::application_capability::ApplicationCapabilityContextEntitySlotRef<
                $Schema,
                $Context,
                Self,
                $Entity,
            > {
                $crate::facade::application_capability::ApplicationCapabilityContextEntitySlotRef::from_declaration()
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_capability_provenance {
    ($vis:vis $Provenance:ident in $Schema:ty) => {
        $crate::worth_query_capability_provenance!(
            $vis $Provenance in $Schema,
            identity stringify!($Provenance)
        );
    };
    ($vis:vis $Provenance:ident in $Schema:ty, identity $identity:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Provenance;

        impl $crate::facade::portable_identity::WorthQueryPortableType for $Provenance {
            const PORTABLE_TYPE_IDENTITY:
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity =
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity::declared(
                    $identity,
                );
        }

        impl $crate::facade::application_capability::ApplicationCapabilityProvenanceMarkerIdentity
            for $Provenance
        {
            type Schema = $Schema;
            const IDENTIFIER: &'static str = stringify!($Provenance);
        }

        impl $Provenance {
            pub const fn reference(
            ) -> $crate::facade::application_capability::ApplicationCapabilityProvenanceRef<$Schema, Self>
            {
                $crate::facade::application_capability::ApplicationCapabilityProvenanceRef::from_declaration()
            }
        }
    };
}
