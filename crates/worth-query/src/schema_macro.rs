#[macro_export]
macro_rules! worth_query_schema {
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

        impl $crate::facade::runtime::TypedSchemaRoot for $Schema {
            const ROOT_ENTITY: &'static str = $root;
        }

        impl $Schema {
            pub fn schema_view() -> $crate::facade::runtime::QuerySchemaView {
                $crate::facade::runtime::QuerySchemaView::new(
                    stringify!($Schema),
                    [
                        $(
                            $crate::worth_query_schema!(
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
                            $crate::facade::runtime::SchemaRelationView::new(
                                $crate::facade::foundation::RelationName::new($relation)
                                    .expect("typed schema relation literal must be valid"),
                                $max_depth
                            )
                        ),*
                    ],
                )
            }
        }

        $(
            $field_vis struct $Field;

            impl $crate::facade::runtime::TypedSchemaField for $Field {
                type Schema = $Schema;
                const ASPECT: &'static str = $aspect;
                const FIELD: &'static str = $field;
            }

            $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($caps)* ]);
        )*

        $(
            $relation_vis struct $Relation;

            impl $crate::facade::runtime::TypedTraversalRelation for $Relation {
                type Schema = $Schema;
                const RELATION: &'static str = $relation;
            }
        )*
    };

    (@schema_field $aspect:literal, $field:literal, $kind:ident, [ $($caps:tt)* ]) => {
        $crate::worth_query_schema!(
            @apply_schema_caps
            $crate::facade::runtime::SchemaFieldView::new(
                $crate::facade::foundation::AspectName::new($aspect)
                    .expect("typed schema aspect literal must be valid"),
                $crate::facade::foundation::FieldName::new($field)
                    .expect("typed schema field literal must be valid"),
                $crate::facade::runtime::SchemaFieldKind::$kind
            ),
            [ $($caps)* ]
        )
    };

    (@apply_schema_caps $expr:expr, [ ]) => { $expr };
    (@apply_schema_caps $expr:expr, [ projectable $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@apply_schema_caps $expr, [ $($($rest)*)? ])
    };
    (@apply_schema_caps $expr:expr, [ equality($ty:tt) $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@apply_schema_caps $expr, [ $($($rest)*)? ])
    };
    (@apply_schema_caps $expr:expr, [ integer_comparable $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@apply_schema_caps $expr, [ $($($rest)*)? ])
    };
    (@apply_schema_caps $expr:expr, [ contains $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(
            @apply_schema_caps
            $expr.text_predicate_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ membership $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(
            @apply_schema_caps
            $expr.membership_predicate_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ presence $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(
            @apply_schema_caps
            $expr.presence_predicate_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ workflow $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(
            @apply_schema_caps
            $expr.workflow_predicate_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ orderable $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@apply_schema_caps $expr, [ $($($rest)*)? ])
    };
    (@apply_schema_caps $expr:expr, [ ordering_only $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(
            @apply_schema_caps
            $expr.ordering_only(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ non_queryable $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(
            @apply_schema_caps
            $expr.non_queryable(),
            [ $($($rest)*)? ]
        )
    };
    (@apply_schema_caps $expr:expr, [ non_orderable $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(
            @apply_schema_caps
            $expr.non_orderable(),
            [ $($($rest)*)? ]
        )
    };

    (@impl_field_caps $Schema:ident, $Field:ident, [ ]) => {};
    (@impl_field_caps $Schema:ident, $Field:ident, [ projectable $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedProjectableField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ equality($ty:tt) $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedEqualityField for $Field {
            type Value = $ty;

            fn into_scalar(value: Self::Value) -> $crate::facade::foundation::ScalarPredicateValue {
                $crate::worth_query_schema!(@into_scalar $ty, value)
            }
        }
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ integer_comparable $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedIntegerComparableField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ contains $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedStringContainsField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ membership $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedMembershipField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ presence $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedPresenceField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ workflow $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ orderable $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedOrderableField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ ordering_only $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedOrderableField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ non_queryable $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, [ non_orderable $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, [ $($($rest)*)? ]);
    };

    (@into_scalar String, $value:expr) => {
        $crate::facade::foundation::ScalarPredicateValue::String($value)
    };
    (@into_scalar i64, $value:expr) => {
        $crate::facade::foundation::ScalarPredicateValue::Integer($value)
    };
    (@into_scalar bool, $value:expr) => {
        $crate::facade::foundation::ScalarPredicateValue::Boolean($value)
    };
}
