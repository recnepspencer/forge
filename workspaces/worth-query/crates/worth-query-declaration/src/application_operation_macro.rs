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
macro_rules! worth_query_ability {
    ($vis:vis $Ability:ident scoped_to $Scope:ty, in $Schema:ty) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Ability;

        impl $Ability {
            pub const fn reference() -> $crate::facade::application_schema::ApplicationAbilityRef<
                $Schema,
                Self,
                $Scope,
            > {
                $crate::facade::application_schema::ApplicationAbilityRef::from_schema_identifiers(
                    stringify!($Ability),
                    stringify!($Scope),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_operation_requires {
    ($Operation:ty => [$($Ability:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationRequiresAbility<$Operation>
                for $Ability
            {}
        )+
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

        const _: () = {
            fn assert_payload_contract<
                Payload: $crate::facade::application_schema::ApplicationEffectPayload,
            >() {
            }
            let _ = assert_payload_contract::<$Payload>;
        };

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
macro_rules! worth_query_operation_reads {
    ($Operation:ty => [$($Member:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationReads<$Operation> for $Member {}
        )+
    };
}

#[macro_export]
macro_rules! worth_query_operation_expects_version {
    ($Operation:ty => [$($Field:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationExpectsVersion<$Operation>
                for $Field
            {}
        )+
    };
}

#[macro_export]
macro_rules! worth_query_operation_expects_fact {
    ($Operation:ty => [$($Field:ty),+ $(,)?]) => {
        $(
            impl $crate::facade::application_schema::OperationExpectsFact<$Operation>
                for $Field
            {}
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
