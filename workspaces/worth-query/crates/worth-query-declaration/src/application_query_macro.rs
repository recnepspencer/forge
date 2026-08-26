/// Declares a typed application query and its stable protocol identity axes.
#[macro_export]
macro_rules! worth_query_application_query {
    (
        $vis:vis $Query:ident in $Schema:ty,
        parameters $Parameters:ident,
        result $Result:ident,
        scope $Scope:ident,
        name $name:literal
    ) => {
        $crate::worth_query_application_query!(
            $vis $Query in $Schema,
            identity stringify!($Query),
            parameters $Parameters => stringify!($Parameters),
            result $Result => <$Result as $crate::facade::portable_identity::WorthQueryPortableType>::PORTABLE_TYPE_IDENTITY.as_str(),
            scope $Scope => stringify!($Scope),
            name $name
        );
    };
    (
        $vis:vis $Query:ident in $Schema:ty,
        identity $query_identity:expr,
        parameters $Parameters:ty => $parameter_identity:expr,
        result $Result:ty => $result_identity:expr,
        scope $Scope:ty => $scope_identity:expr,
        name $name:literal
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis struct $Query;

        impl $crate::facade::application_query::ApplicationQueryMarkerIdentity for $Query {
            type Schema = $Schema;
            type Parameters = $Parameters;
            type QueryResult = $Result;
            type Scope = $Scope;

            const IDENTIFIER: &'static str = $name;
            const QUERY_TYPE_IDENTITY: $crate::facade::portable_identity::WorthQueryPortableTypeIdentity =
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity::declared($query_identity);
            const PARAMETER_TYPE_IDENTITY: $crate::facade::portable_identity::WorthQueryPortableTypeIdentity =
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity::declared($parameter_identity);
            const RESULT_TYPE_IDENTITY: $crate::facade::portable_identity::WorthQueryPortableTypeIdentity =
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity::declared($result_identity);
            const SCOPE_TYPE_IDENTITY: $crate::facade::portable_identity::WorthQueryPortableTypeIdentity =
                $crate::facade::portable_identity::WorthQueryPortableTypeIdentity::declared($scope_identity);
        }

        impl $Query {
            #[allow(dead_code)]
            pub const fn reference() -> $crate::facade::application_query::ApplicationQueryReference<
                $Schema,
                Self,
                $Parameters,
                $Result,
                $Scope,
            > {
                $crate::facade::application_query::ApplicationQueryReference::from_declaration()
            }
        }
    };
}
