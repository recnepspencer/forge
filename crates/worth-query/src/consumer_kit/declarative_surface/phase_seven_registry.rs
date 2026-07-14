use super::model::{
    WorthQueryDeclarativeCapabilityFamily as Family,
    WorthQueryDeclarativePhaseResponsibility as Phase, WorthQueryDeclarativeSurfaceClass as Class,
    WorthQueryDeclarativeSurfaceRow as Row,
};

const HISTORY_MOD: &str = "src/ordinary/history/mod.rs";
const HISTORY_DECLARATION: &str = "src/ordinary/history/declaration.rs";
const HISTORY_EXECUTION: &str = "src/ordinary/history/execution.rs";
const COMPARISON_MOD: &str = "src/ordinary/comparison/mod.rs";
const COMPARISON_DECLARATION: &str = "src/ordinary/comparison/declaration.rs";
const COMPARISON_EXECUTION: &str = "src/ordinary/comparison/execution.rs";

pub(super) fn phase_seven_surface_rows() -> &'static [Row] {
    ROWS
}

const ROWS: &[Row] = &[
    declaration(HISTORY_MOD, Family::Historical, "facade::history::declare"),
    declaration(
        HISTORY_DECLARATION,
        Family::Historical,
        "facade::history::declare",
    ),
    Row::method(
        HISTORY_DECLARATION,
        "WorthQueryHistoricalPathDeclaration",
        "using",
        Family::Historical,
        Phase::Refine,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary historical consumer",
        "WorthQueryHistoricalPathDeclaration::using",
    ),
    Row::method(
        HISTORY_EXECUTION,
        "WorthQueryHistoricalRequest",
        "run",
        Family::Historical,
        Phase::Execute,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary historical consumer",
        "WorthQueryHistoricalRequest::run",
    ),
    declaration(
        COMPARISON_MOD,
        Family::Comparison,
        "facade::comparison::declare",
    ),
    declaration(
        COMPARISON_DECLARATION,
        Family::Comparison,
        "facade::comparison::declare",
    ),
    Row::method(
        COMPARISON_DECLARATION,
        "WorthQueryComparisonRefinement",
        "using",
        Family::Comparison,
        Phase::Refine,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary comparison consumer",
        "WorthQueryComparisonRefinement::using",
    ),
    Row::method(
        COMPARISON_EXECUTION,
        "WorthQueryComparisonRequest",
        "run",
        Family::Comparison,
        Phase::Execute,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary comparison consumer",
        "WorthQueryComparisonRequest::run",
    ),
];

const fn declaration(path: &'static str, family: Family, replacement: &'static str) -> Row {
    Row::new(
        path,
        "declare",
        family,
        Phase::Declare,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary capability consumer",
        replacement,
    )
}
