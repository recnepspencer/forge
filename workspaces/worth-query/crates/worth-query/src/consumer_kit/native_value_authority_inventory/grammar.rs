use std::collections::BTreeSet;

use worth_foundational::facade::ScalarAspectType;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryNativeValueFamily {
    Null,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Decimal,
    BigInt,
    Rational,
    StringRaw,
    StringSymbol,
    Bytes,
    Uuid,
    Date,
    Time,
    Timestamp,
    TimestampTz,
    EntityRef,
    ContentRef,
    Struct,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryNativePredicateCapability {
    Equality,
    Membership,
    Presence,
    Ordering,
    Numeric,
    String,
    Reference,
    Temporal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeValueGrammarRow {
    family: WorthQueryNativeValueFamily,
    scalar_type: Option<ScalarAspectType>,
    semantic_carrier: &'static str,
    authoring_path: &'static str,
    predicate_capabilities: &'static [WorthQueryNativePredicateCapability],
    projection_form: &'static str,
    refinement_form: &'static str,
    certification_owner: &'static str,
}

impl WorthQueryNativeValueGrammarRow {
    pub fn family(&self) -> WorthQueryNativeValueFamily {
        self.family
    }

    pub fn scalar_type(&self) -> Option<ScalarAspectType> {
        self.scalar_type
    }

    pub fn semantic_carrier(&self) -> &'static str {
        self.semantic_carrier
    }

    pub fn authoring_path(&self) -> &'static str {
        self.authoring_path
    }

    pub fn predicate_capabilities(&self) -> &'static [WorthQueryNativePredicateCapability] {
        self.predicate_capabilities
    }

    pub fn projection_form(&self) -> &'static str {
        self.projection_form
    }

    pub fn refinement_form(&self) -> &'static str {
        self.refinement_form
    }

    pub fn certification_owner(&self) -> &'static str {
        self.certification_owner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeValueGrammarAudit {
    missing_scalar_types: Vec<ScalarAspectType>,
    duplicate_families: Vec<WorthQueryNativeValueFamily>,
    missing_cell_families: Vec<WorthQueryNativeValueFamily>,
    struct_row_count: usize,
}

impl WorthQueryNativeValueGrammarAudit {
    pub fn missing_scalar_types(&self) -> &[ScalarAspectType] {
        &self.missing_scalar_types
    }

    pub fn duplicate_families(&self) -> &[WorthQueryNativeValueFamily] {
        &self.duplicate_families
    }

    pub fn missing_cell_families(&self) -> &[WorthQueryNativeValueFamily] {
        &self.missing_cell_families
    }

    pub fn struct_row_count(&self) -> usize {
        self.struct_row_count
    }
}

pub fn worth_query_native_value_grammar() -> &'static [WorthQueryNativeValueGrammarRow] {
    GRAMMAR
}

pub fn audit_native_value_grammar(
    rows: &[WorthQueryNativeValueGrammarRow],
) -> WorthQueryNativeValueGrammarAudit {
    let mut observed_families = BTreeSet::new();
    let mut duplicate_families = Vec::new();
    let mut observed_scalar_types = BTreeSet::new();
    let mut missing_cell_families = Vec::new();
    let mut struct_row_count = 0;

    for row in rows {
        if !observed_families.insert(row.family) {
            duplicate_families.push(row.family);
        }
        if let Some(scalar_type) = row.scalar_type {
            observed_scalar_types.insert(scalar_type);
        } else if row.family == WorthQueryNativeValueFamily::Struct {
            struct_row_count += 1;
        }
        if row.semantic_carrier.is_empty()
            || row.authoring_path.is_empty()
            || row.predicate_capabilities.is_empty()
            || row.projection_form.is_empty()
            || row.refinement_form.is_empty()
            || row.certification_owner.is_empty()
        {
            missing_cell_families.push(row.family);
        }
    }

    WorthQueryNativeValueGrammarAudit {
        missing_scalar_types: ALL_SCALAR_TYPES
            .iter()
            .copied()
            .filter(|scalar_type| !observed_scalar_types.contains(scalar_type))
            .collect(),
        duplicate_families,
        missing_cell_families,
        struct_row_count,
    }
}

const BASE: &[WorthQueryNativePredicateCapability] = &[
    WorthQueryNativePredicateCapability::Equality,
    WorthQueryNativePredicateCapability::Membership,
    WorthQueryNativePredicateCapability::Presence,
];
const NUMERIC: &[WorthQueryNativePredicateCapability] = &[
    WorthQueryNativePredicateCapability::Equality,
    WorthQueryNativePredicateCapability::Membership,
    WorthQueryNativePredicateCapability::Presence,
    WorthQueryNativePredicateCapability::Ordering,
    WorthQueryNativePredicateCapability::Numeric,
];
const STRING: &[WorthQueryNativePredicateCapability] = &[
    WorthQueryNativePredicateCapability::Equality,
    WorthQueryNativePredicateCapability::Membership,
    WorthQueryNativePredicateCapability::Presence,
    WorthQueryNativePredicateCapability::Ordering,
    WorthQueryNativePredicateCapability::String,
];
const TEMPORAL: &[WorthQueryNativePredicateCapability] = &[
    WorthQueryNativePredicateCapability::Equality,
    WorthQueryNativePredicateCapability::Membership,
    WorthQueryNativePredicateCapability::Presence,
    WorthQueryNativePredicateCapability::Ordering,
    WorthQueryNativePredicateCapability::Temporal,
];
const REFERENCE: &[WorthQueryNativePredicateCapability] = &[
    WorthQueryNativePredicateCapability::Equality,
    WorthQueryNativePredicateCapability::Membership,
    WorthQueryNativePredicateCapability::Presence,
    WorthQueryNativePredicateCapability::Reference,
];
const STRUCT: &[WorthQueryNativePredicateCapability] =
    &[WorthQueryNativePredicateCapability::Presence];

macro_rules! scalar_row {
    ($family:ident, $scalar:ident, $predicates:ident) => {
        WorthQueryNativeValueGrammarRow {
            family: WorthQueryNativeValueFamily::$family,
            scalar_type: Some(ScalarAspectType::$scalar),
            semantic_carrier: "worth_foundational::AspectValue",
            authoring_path: "native mutation intent -> Foundational contract validation",
            predicate_capabilities: $predicates,
            projection_form: "ContractValidatedAspectValueView::Scalar",
            refinement_form: "borrowed native scalar refinement",
            certification_owner: "milestone-9.13-phase-26-native-value-closure",
        }
    };
}

const GRAMMAR: &[WorthQueryNativeValueGrammarRow] = &[
    scalar_row!(Null, Null, BASE),
    scalar_row!(Bool, Bool, BASE),
    scalar_row!(Int8, Int8, NUMERIC),
    scalar_row!(Int16, Int16, NUMERIC),
    scalar_row!(Int32, Int32, NUMERIC),
    scalar_row!(Int64, Int64, NUMERIC),
    scalar_row!(UInt8, UInt8, NUMERIC),
    scalar_row!(UInt16, UInt16, NUMERIC),
    scalar_row!(UInt32, UInt32, NUMERIC),
    scalar_row!(UInt64, UInt64, NUMERIC),
    scalar_row!(Float32, Float32, NUMERIC),
    scalar_row!(Float64, Float64, NUMERIC),
    scalar_row!(Decimal, Decimal, NUMERIC),
    scalar_row!(BigInt, BigInt, NUMERIC),
    scalar_row!(Rational, Rational, NUMERIC),
    scalar_row!(StringRaw, String, STRING),
    scalar_row!(StringSymbol, String, STRING),
    scalar_row!(Bytes, Bytes, BASE),
    scalar_row!(Uuid, Uuid, BASE),
    scalar_row!(Date, Date, TEMPORAL),
    scalar_row!(Time, Time, TEMPORAL),
    scalar_row!(Timestamp, Timestamp, TEMPORAL),
    scalar_row!(TimestampTz, TimestampTz, TEMPORAL),
    scalar_row!(EntityRef, EntityRef, REFERENCE),
    scalar_row!(ContentRef, ContentRef, REFERENCE),
    WorthQueryNativeValueGrammarRow {
        family: WorthQueryNativeValueFamily::Struct,
        scalar_type: None,
        semantic_carrier: "worth_foundational::StructAspectValue",
        authoring_path: "native struct intent -> Foundational contract and mask validation",
        predicate_capabilities: STRUCT,
        projection_form: "ContractValidatedAspectValueView::Struct",
        refinement_form: "borrowed native struct and field refinement",
        certification_owner: "milestone-9.13-phase-26-native-value-closure",
    },
];

const ALL_SCALAR_TYPES: &[ScalarAspectType] = &[
    ScalarAspectType::Null,
    ScalarAspectType::Bool,
    ScalarAspectType::Int8,
    ScalarAspectType::Int16,
    ScalarAspectType::Int32,
    ScalarAspectType::Int64,
    ScalarAspectType::UInt8,
    ScalarAspectType::UInt16,
    ScalarAspectType::UInt32,
    ScalarAspectType::UInt64,
    ScalarAspectType::Float32,
    ScalarAspectType::Float64,
    ScalarAspectType::Decimal,
    ScalarAspectType::BigInt,
    ScalarAspectType::Rational,
    ScalarAspectType::String,
    ScalarAspectType::Bytes,
    ScalarAspectType::Uuid,
    ScalarAspectType::Date,
    ScalarAspectType::Time,
    ScalarAspectType::Timestamp,
    ScalarAspectType::TimestampTz,
    ScalarAspectType::EntityRef,
    ScalarAspectType::ContentRef,
];
