#[macro_export]
macro_rules! worth_query_application_schema {
    (
        $vis:vis schema $Schema:ident {
            owner: $owner:literal,
            version: ($major:expr, $minor:expr),
            members: |$builder:ident| $body:block
        }
    ) => {
        $crate::worth_query_application_schema!(
            @define
            $vis schema $Schema {
                owner: $owner,
                version: ($major, $minor),
                members: |$builder| $body
            }
        );
    };
    (
        $vis:vis schema $Schema:ident {
            owner: $owner:ident,
            version: ($major:expr, $minor:expr),
            members: |$builder:ident| $body:block
        }
    ) => {
        $crate::worth_query_application_schema!(
            @define
            $vis schema $Schema {
                owner: stringify!($owner),
                version: ($major, $minor),
                members: |$builder| $body
            }
        );
    };
    (
        @define
        $vis:vis schema $Schema:ident {
            owner: $owner:expr,
            version: ($major:expr, $minor:expr),
            members: |$builder:ident| $body:block
        }
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Schema;

        impl $crate::facade::application_schema::ApplicationSchema for $Schema {
            const OWNER: &'static str = $owner;
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

        impl $crate::facade::application_schema::DeclaredApplicationFieldValue for $Field {
            type Value = $Value;
        }

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

        impl $crate::facade::application_schema::DeclaredApplicationFieldValue for $Field {
            type Value = $Value;
        }

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
macro_rules! worth_query_application_query {
    (
        $vis:vis $Query:ident in $Schema:ty,
        parameters $Parameters:ty,
        result $Result:ty,
        scope $Scope:ty,
        name $name:literal
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Query;

        impl $Query {
            pub const fn reference() -> $crate::facade::application_query::ApplicationQueryReference<
                $Schema,
                Self,
                $Parameters,
                $Result,
                $Scope,
            > {
                $crate::facade::application_query::ApplicationQueryReference::from_schema_identifier(
                    $name,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! worth_query_principal_binding {
    (
        $vis:vis $Binding:ident in $Schema:ty,
        mapping $Mapping:ty {
            identity: $IdentityField:ty,
            status: $StatusField:ty,
            target: $TargetRelation:ty => $Principal:ty,
            principal_identity: $PrincipalIdentityField:ty
        }
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Binding;

        impl $Binding {
            pub fn reference() -> $crate::facade::application_schema::ApplicationPrincipalBindingRef<
                $Schema,
                Self,
                $Mapping,
                $Principal,
                <$PrincipalIdentityField as $crate::facade::application_schema::DeclaredApplicationFieldValue>::Value,
            > {
                let identity: $crate::facade::application_schema::ApplicationFieldRef<
                    $Schema,
                    $Mapping,
                    _,
                    $IdentityField,
                    $crate::facade::authentication::WorthQueryExternalPrincipalIdentity,
                    _,
                    $crate::facade::application_schema::EqualityPredicate,
                    _,
                > = <$IdentityField>::reference();
                let status: $crate::facade::application_schema::ApplicationFieldRef<
                    $Schema,
                    $Mapping,
                    _,
                    $StatusField,
                    $crate::facade::authentication::WorthQueryPrincipalMappingStatus,
                    _,
                    _,
                    _,
                > = <$StatusField>::reference();
                let target: $crate::facade::application_schema::ApplicationRelationRef<
                    $Schema,
                    $TargetRelation,
                    $Mapping,
                    $Principal,
                > = <$TargetRelation>::reference();
                let principal_identity: $crate::facade::application_schema::ApplicationFieldRef<
                    $Schema,
                    $Principal,
                    _,
                    $PrincipalIdentityField,
                    _,
                    $crate::facade::application_schema::ReadOnly,
                    $crate::facade::application_schema::EqualityPredicate,
                    _,
                > = <$PrincipalIdentityField>::reference();
                $crate::facade::application_schema::ApplicationPrincipalBindingRef::<
                    $Schema,
                    Self,
                    $Mapping,
                    $Principal,
                    <$PrincipalIdentityField as $crate::facade::application_schema::DeclaredApplicationFieldValue>::Value,
                >::from_schema_identifiers(
                    stringify!($Binding),
                    identity.entity(),
                    identity.aspect(),
                    identity.field(),
                    status.aspect(),
                    status.field(),
                    target.name(),
                    target.to(),
                    principal_identity.aspect(),
                    principal_identity.field(),
                )
            }
        }
    };
}
