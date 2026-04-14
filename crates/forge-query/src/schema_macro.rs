#[macro_export]
macro_rules! forge_query_schema {
    (
        $vis:vis schema $Schema:ident($root:literal) {
            fields {
                $(
                    $field_vis:vis field $Field:ident($aspect:literal, $field:literal, $kind:ident)
                        => [ $($caps:tt)* ];
                )*
            }
            relations {
                $(
                    $relation_vis:vis relation $Relation:ident($relation:literal, $max_depth:expr);
                )*
            }
        }
    ) => {
        $vis struct $Schema;

        impl $crate::facade::TypedSchemaRoot for $Schema {
            const ROOT_ENTITY: &'static str = $root;
        }

        impl $Schema {
            pub fn schema_view() -> $crate::facade::QuerySchemaView {
                $crate::facade::QuerySchemaView::new(
                    stringify!($Schema),
                    [
                        $(
                            $crate::forge_query_schema!(
                                @schema_field
                                $aspect,
                                $field,
                                $kind,
                                [ $($caps)* ]
                            )
                        ),*
                    ],
                    [
                        $(
                            $crate::facade::SchemaRelationView::new($relation, $max_depth)
                        ),*
                    ],
                )
            }
        }

        $(
            $field_vis struct $Field;

            impl $crate::facade::TypedSchemaField for $Field {
                type Schema = $Schema;
                const ASPECT: &'static str = $aspect;
                const FIELD: &'static str = $field;
            }

            $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($caps)* ]);
        )*

        $(
            $relation_vis struct $Relation;

            impl $crate::facade::TypedTraversalRelation for $Relation {
                type Schema = $Schema;
                const RELATION: &'static str = $relation;
            }
        )*
    };

    (@schema_field $aspect:literal, $field:literal, $kind:ident, [ $($caps:tt)* ]) => {
        $crate::forge_query_schema!(
            @apply_schema_caps
            $crate::facade::SchemaFieldView::new(
                $aspect,
                $field,
                $crate::facade::SchemaFieldKind::$kind
            ),
            [ $($caps)* ]
        )
    };

    (@apply_schema_caps $expr:expr, [ ]) => { $expr };
    (@apply_schema_caps $expr:expr, [ projectable $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(@apply_schema_caps $expr, [ $($($rest)*)? ])
    };
    (@apply_schema_caps $expr:expr, [ equality($ty:tt) $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(@apply_schema_caps $expr, [ $($($rest)*)? ])
    };
    (@apply_schema_caps $expr:expr, [ integer_comparable $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(@apply_schema_caps $expr, [ $($($rest)*)? ])
    };
    (@apply_schema_caps $expr:expr, [ contains $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(
            @apply_schema_caps
            $expr.text_predicate_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ membership $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(
            @apply_schema_caps
            $expr.membership_predicate_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ presence $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(
            @apply_schema_caps
            $expr.presence_predicate_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ workflow $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(
            @apply_schema_caps
            $expr.workflow_predicate_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ orderable $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(@apply_schema_caps $expr, [ $($($rest)*)? ])
    };
    (@apply_schema_caps $expr:expr, [ ordering_only $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(
            @apply_schema_caps
            $expr.ordering_only(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ non_queryable $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(
            @apply_schema_caps
            $expr.non_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ non_orderable $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(
            @apply_schema_caps
            $expr.non_orderable(),
            [ $($($rest)*)? ]
        )
    };

    (@impl_field_caps $Schema:ident, $Field:ident, [ ]) => {};
    (@impl_field_caps $Schema:ident, $Field:ident, [ projectable $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::TypedProjectableField for $Field {}
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ equality($ty:tt) $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::TypedEqualityField for $Field {
            type Value = $ty;

            fn into_scalar(value: Self::Value) -> $crate::facade::ScalarPredicateValue {
                $crate::forge_query_schema!(@into_scalar $ty, value)
            }
        }
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ integer_comparable $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::TypedIntegerComparableField for $Field {}
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ contains $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::TypedStringContainsField for $Field {}
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ membership $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::TypedMembershipField for $Field {}
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ presence $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::TypedPresenceField for $Field {}
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ workflow $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ orderable $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::TypedOrderableField for $Field {}
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ ordering_only $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::TypedOrderableField for $Field {}
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ non_queryable $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ non_orderable $(, $($rest:tt)*)? ]) => {
        $crate::forge_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };

    (@into_scalar String, $value:expr) => {
        $crate::facade::ScalarPredicateValue::String($value)
    };
    (@into_scalar i64, $value:expr) => {
        $crate::facade::ScalarPredicateValue::Integer($value)
    };
    (@into_scalar bool, $value:expr) => {
        $crate::facade::ScalarPredicateValue::Boolean($value)
    };
}
