use std::fmt;

use super::{
    WorthUiAppearanceStateAdmissionReport, WorthUiEventGeometryAdmissionReport,
    WorthUiFlowLayoutAdmissionReport, WorthUiPrimitiveContentAdmissionReport,
    WorthUiPrimitivePropAdmissionReport,
};
use crate::runtime::WorthUiInteractionAdmissionReport;

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiPrimitiveProofDenial {
    MissingSurface {
        surface_id: String,
    },
    ComponentMismatch {
        surface_id: String,
        expected_component_id: String,
        actual_component_id: String,
    },
    InvalidAuthoredPrimitiveValues {
        report: WorthUiPrimitivePropAdmissionReport,
    },
    InvalidFlowLayoutValues {
        report: WorthUiFlowLayoutAdmissionReport,
    },
    InvalidContentValues {
        report: WorthUiPrimitiveContentAdmissionReport,
    },
    InvalidEventGeometryValues {
        report: WorthUiEventGeometryAdmissionReport,
    },
    InvalidAppearanceStateValues {
        report: WorthUiAppearanceStateAdmissionReport,
    },
    InvalidInteractionValues {
        report: WorthUiInteractionAdmissionReport,
    },
    MissingPrimitiveMeasurementToken {
        token: String,
    },
    WrongPrimitiveMeasurementKind {
        token: String,
        expected: String,
        actual: String,
    },
    EmptyDependencyContract {
        surface_id: String,
    },
}

impl fmt::Display for WorthUiPrimitiveProofDenial {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSurface { surface_id } => {
                write!(formatter, "Primitive surface `{surface_id}` was not found.")
            }
            Self::ComponentMismatch {
                surface_id,
                expected_component_id,
                actual_component_id,
            } => write!(
                formatter,
                "Primitive surface `{surface_id}` uses component `{actual_component_id}`, expected `{expected_component_id}`."
            ),
            Self::InvalidAuthoredPrimitiveValues { report } => {
                let denial_set = report
                    .status()
                    .denial_set()
                    .expect("invalid primitive values denial carries denial set");
                write!(
                    formatter,
                    "Primitive surface `{}` has {} invalid primitive value(s).",
                    report.surface_id(),
                    denial_set.denials().len()
                )
            }
            Self::InvalidFlowLayoutValues { report } => {
                let denial_set = report
                    .status()
                    .denial_set()
                    .expect("invalid flow layout values denial carries denial set");
                write!(
                    formatter,
                    "Flow layout for primitive surface `{}` has {} invalid value(s).",
                    report.surface_id(),
                    denial_set.denials().len()
                )
            }
            Self::InvalidContentValues { report } => {
                let denial_set = report
                    .status()
                    .denial_set()
                    .expect("invalid content values denial carries denial set");
                write!(
                    formatter,
                    "Content for primitive surface `{}` has {} invalid value(s).",
                    report.surface_id(),
                    denial_set.denials().len()
                )
            }
            Self::InvalidEventGeometryValues { report } => {
                let denial_set = report
                    .status()
                    .denial_set()
                    .expect("invalid event geometry values denial carries denial set");
                write!(
                    formatter,
                    "Event geometry for primitive surface `{}` has {} invalid value(s).",
                    report.surface_id(),
                    denial_set.denials().len()
                )
            }
            Self::InvalidAppearanceStateValues { report } => {
                let denial_set = report
                    .status()
                    .denial_set()
                    .expect("invalid appearance state values denial carries denial set");
                write!(
                    formatter,
                    "Appearance state for primitive surface `{}` has {} invalid value(s).",
                    report.surface_id(),
                    denial_set.denials().len()
                )
            }
            Self::InvalidInteractionValues { report } => {
                let denial_set = report
                    .status()
                    .denial_set()
                    .expect("invalid interaction values denial carries denial set");
                write!(
                    formatter,
                    "Interaction for primitive surface `{}` has {} invalid value(s).",
                    report.surface_id(),
                    denial_set.denials().len()
                )
            }
            Self::MissingPrimitiveMeasurementToken { token } => {
                write!(formatter, "Primitive measurement token `{token}` was not found.")
            }
            Self::WrongPrimitiveMeasurementKind {
                token,
                expected,
                actual,
            } => write!(
                formatter,
                "Primitive measurement token `{token}` resolved as `{actual}`, expected {expected}."
            ),
            Self::EmptyDependencyContract { surface_id } => write!(
                formatter,
                "Primitive surface `{surface_id}` produced an empty dependency contract."
            ),
        }
    }
}

impl std::error::Error for WorthUiPrimitiveProofDenial {}
