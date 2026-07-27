#[macro_export]
macro_rules! worth_query_application_schema {
    (
        $vis:vis schema $Schema:ident {
            owner: $owner:ident,
            version: ($major:expr, $minor:expr),
            members: |$builder:ident| $body:block
        }
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Schema;

        impl $crate::facade::application_schema::ApplicationSchema for $Schema {
            const OWNER: &'static str = stringify!($owner);
            const NAME: &'static str = stringify!($Schema);
            const MAJOR: u32 = $major;
            const MINOR: u32 = $minor;

            fn declaration() -> Result<
                $crate::facade::application_schema::ApplicationSchemaDeclaration<Self>,
                $crate::facade::application_schema::ApplicationSchemaDeclarationDenial,
            > {
                let $builder =
                    $crate::facade::application_schema::ApplicationSchemaDeclarationBuilder::<Self>::for_schema();
                let $builder = $body;
                $builder.build()
            }
        }

        impl $Schema {
            pub fn declaration() -> Result<
                $crate::facade::application_schema::ApplicationSchemaDeclaration<Self>,
                $crate::facade::application_schema::ApplicationSchemaDeclarationDenial,
            > {
                <Self as $crate::facade::application_schema::ApplicationSchema>::declaration()
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_entity {
    ($vis:vis $Entity:ident in $Schema:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Entity;

        impl $Entity {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationEntityRef<$Schema, Self> {
                $crate::facade::application_schema::ApplicationEntityRef::from_schema_identifier(
                    stringify!($Entity),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_aspect {
    ($vis:vis $Aspect:ident in $Schema:ty, $Entity:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Aspect;

        impl $Aspect {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationAspectRef<$Schema, $Entity, Self> {
                $crate::facade::application_schema::ApplicationAspectRef::from_schema_identifier(
                    stringify!($Aspect),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_field {
    (
        $vis:vis $Field:ident in $Schema:ty, $Entity:ty, $Aspect:ty:
        $Value:ty, currency $Currency:ty, $write:ident, $equality:ident
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Field;

        impl $Field {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationFieldRef<
                $Schema,
                $Entity,
                $Aspect,
                Self,
                $Value,
                $crate::worth_query_field!(@write $write),
                $crate::worth_query_field!(@equality $equality),
                $crate::facade::application_schema::DeclaredApplicationCurrency<
                    $Currency,
                    <$Value as $crate::facade::application_schema::TypedCurrencyApplicationValue>::Currency,
                >,
            > {
                $crate::facade::application_schema::ApplicationFieldRef::from_schema_identifiers(
                    stringify!($Entity),
                    stringify!($Aspect),
                    stringify!($Field),
                )
            }
        }
    };
    (
        $vis:vis $Field:ident in $Schema:ty, $Entity:ty, $Aspect:ty:
        $Value:ty, $write:ident, $equality:ident
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Field;

        impl $Field {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationFieldRef<
                $Schema,
                $Entity,
                $Aspect,
                Self,
                $Value,
                $crate::worth_query_field!(@write $write),
                $crate::worth_query_field!(@equality $equality),
            > {
                $crate::facade::application_schema::ApplicationFieldRef::from_schema_identifiers(
                    stringify!($Entity),
                    stringify!($Aspect),
                    stringify!($Field),
                )
            }

        }
    };
    (@write read_only) => { $crate::facade::application_schema::ReadOnly };
    (@write read_write) => { $crate::facade::application_schema::ReadWrite };
    (@equality no_equality) => { $crate::facade::application_schema::NoEqualityPredicate };
    (@equality equality) => { $crate::facade::application_schema::EqualityPredicate };
}

#[macro_export]
macro_rules! worth_query_relation {
    ($vis:vis $Relation:ident in $Schema:ty, $From:ty => $To:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Relation;

        impl $Relation {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationRelationRef<$Schema, Self, $From, $To> {
                $crate::facade::application_schema::ApplicationRelationRef::from_schema_identifiers(
                    stringify!($Relation),
                    stringify!($From),
                    stringify!($To),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_operation {
    ($vis:vis $Operation:ident($Input:ty) in $Schema:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Operation;

        impl $Operation {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationOperationRef<$Schema, Self, $Input> {
                $crate::facade::application_schema::ApplicationOperationRef::from_schema_identifier(
                    stringify!($Operation),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_policy {
    ($vis:vis $Policy:ident in $Schema:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Policy;

        impl $Policy {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationPolicyRef<$Schema, Self> {
                $crate::facade::application_schema::ApplicationPolicyRef::from_schema_identifier(
                    stringify!($Policy),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_currency {
    ($vis:vis $Currency:ident($DomainCurrency:ty) in $Schema:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Currency;

        impl $crate::facade::application_schema::ApplicationCurrencyMarker<$DomainCurrency>
            for $Currency
        {
            const NAME: &'static str = stringify!($Currency);
        }

        impl $Currency {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationCurrencyRef<$Schema, Self> {
                $crate::facade::application_schema::ApplicationCurrencyRef::from_schema_identifier(
                    stringify!($Currency),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_effect {
    ($vis:vis $Effect:ident($Payload:ty) in $Schema:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Effect;

        impl $Effect {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationEffectRef<$Schema, Self, $Payload> {
                $crate::facade::application_schema::ApplicationEffectRef::from_schema_identifier(
                    stringify!($Effect),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_operation_writes {
    ($Operation:ty => [$($Field:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationWrites<$Operation> for $Field {}
        )+
    };
}

#[macro_export]
macro_rules! worth_query_operation_creates {
    ($Operation:ty => [$($Entity:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationCreates<$Operation> for $Entity {}
        )+
    };
}

#[macro_export]
macro_rules! worth_query_operation_deletes {
    ($Operation:ty => [$($Entity:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationDeletes<$Operation> for $Entity {}
        )+
    };
}

#[macro_export]
macro_rules! worth_query_operation_links {
    ($Operation:ty => [$($Relation:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationLinks<$Operation> for $Relation {}
        )+
    };
}

#[macro_export]
macro_rules! worth_query_operation_unlinks {
    ($Operation:ty => [$($Relation:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationUnlinks<$Operation> for $Relation {}
        )+
    };
}

#[macro_export]
macro_rules! worth_query_operation_emits {
    ($Operation:ty => [$($Effect:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationEmits<$Operation> for $Effect {}
        )+
    };
}
