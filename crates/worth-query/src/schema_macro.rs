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

            $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($caps)* ]);
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
                $crate::facade::runtime::ScalarAspectType::$kind
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
    (@apply_schema_caps $expr:expr, [ native_comparable $(, $($rest:tt)*)? ]) => {
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

    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ ]) => {};
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ projectable $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedProjectableField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ equality($ty:tt) $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedEqualityField for $Field {
            type Value = $ty;

            fn into_scalar(value: Self::Value) -> $crate::facade::foundation::WorthQueryPredicateOperand {
                $crate::worth_query_schema!(@native_operand $kind, value)
            }
        }
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ native_comparable $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedNativeComparableField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ contains $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedStringContainsField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ membership $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedMembershipField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ presence $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedPresenceField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ workflow $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ orderable $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedOrderableField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ ordering_only $(, $($rest:tt)*)? ]) => {
        impl $crate::facade::runtime::TypedOrderableField for $Field {}
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ non_queryable $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };
    (@impl_field_caps $Schema:ident, $Field:ident, $kind:ident, [ non_orderable $(, $($rest:tt)*)? ]) => {
        $crate::worth_query_schema!(@impl_field_caps $Schema, $Field, $kind, [ $($($rest)*)? ]);
    };

    (@native_operand Null, $value:expr) => {{ let _ = $value; $crate::facade::foundation::WorthQueryPredicateOperand::null() }};
    (@native_operand Bool, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::boolean($value) };
    (@native_operand Int8, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::int8($value) };
    (@native_operand Int16, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::int16($value) };
    (@native_operand Int32, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::int32($value) };
    (@native_operand Int64, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::int64($value) };
    (@native_operand UInt8, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::uint8($value) };
    (@native_operand UInt16, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::uint16($value) };
    (@native_operand UInt32, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::uint32($value) };
    (@native_operand UInt64, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::uint64($value) };
    (@native_operand Float32, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::float32($value) };
    (@native_operand Float64, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::float64($value) };
    (@native_operand Decimal, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::decimal($value) };
    (@native_operand BigInt, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::big_int($value) };
    (@native_operand Rational, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::rational($value) };
    (@native_operand String, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::interned_string($value) };
    (@native_operand Bytes, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::bytes($value) };
    (@native_operand Uuid, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::uuid($value) };
    (@native_operand Date, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::date($value) };
    (@native_operand Time, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::time($value) };
    (@native_operand Timestamp, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::timestamp($value) };
    (@native_operand TimestampTz, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::timestamp_tz($value) };
    (@native_operand EntityRef, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::entity_ref($value) };
    (@native_operand ContentRef, $value:expr) => { $crate::facade::foundation::WorthQueryPredicateOperand::content_ref($value) };
}
