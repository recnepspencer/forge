use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticMismatchCategory {
    GraphState,
    GraphStructure,
    Metrics,
    PlanShape,
    TaskOutcome,
    Provenance,
    ExecutionRecord,
    Flow,
    FailureState,
    History,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticMismatch {
    pub category: DiagnosticMismatchCategory,
    pub field: String,
    pub left: String,
    pub right: String,
}

macro_rules! define_diff {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
        pub struct $name {
            pub mismatches: Vec<DiagnosticMismatch>,
        }

        impl $name {
            pub fn is_empty(&self) -> bool {
                self.mismatches.is_empty()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.mismatches.is_empty() {
                    return write!(f, "{}(no mismatches)", stringify!($name));
                }
                writeln!(
                    f,
                    "{} mismatches={}",
                    stringify!($name),
                    self.mismatches.len()
                )?;
                for mismatch in &self.mismatches {
                    writeln!(
                        f,
                        "  {:?} {}: left={} right={}",
                        mismatch.category, mismatch.field, mismatch.left, mismatch.right
                    )?;
                }
                Ok(())
            }
        }
    };
}

define_diff!(GraphDiff);
define_diff!(PlanDiff);
define_diff!(ExecutionReportDiff);
define_diff!(ExplanationDiff);
define_diff!(HistoryDiff);
define_diff!(FlowDiff);
define_diff!(FailureDiff);

pub(crate) fn push_mismatch(
    mismatches: &mut Vec<DiagnosticMismatch>,
    category: DiagnosticMismatchCategory,
    field: &str,
    left: impl ToString,
    right: impl ToString,
) {
    mismatches.push(DiagnosticMismatch {
        category,
        field: field.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    });
}

pub(crate) fn compare_value<T>(
    mismatches: &mut Vec<DiagnosticMismatch>,
    category: DiagnosticMismatchCategory,
    field: &str,
    left: T,
    right: T,
) where
    T: PartialEq + ToString,
{
    if left != right {
        push_mismatch(mismatches, category, field, left, right);
    }
}
