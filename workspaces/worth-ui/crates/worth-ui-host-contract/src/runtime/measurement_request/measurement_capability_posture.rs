use super::UiMeasurementRequestDenial;
use crate::runtime::{
    WorthUiHostCapability, WorthUiHostCapabilityPosture, WorthUiHostCapabilityReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMeasurementCapabilityGrant {
    required_capabilities: Box<[WorthUiHostCapability]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMeasurementCapabilityPosture {
    Available {
        required_capabilities: Box<[WorthUiHostCapability]>,
    },
    Missing {
        required_capabilities: Box<[WorthUiHostCapability]>,
    },
    Ambiguous {
        required_capabilities: Box<[WorthUiHostCapability]>,
    },
    DiagnosticOnly {
        required_capabilities: Box<[WorthUiHostCapability]>,
    },
}

impl UiMeasurementCapabilityGrant {
    pub(crate) fn new(
        report: &WorthUiHostCapabilityReport,
        required_capabilities: Vec<WorthUiHostCapability>,
    ) -> Result<Self, UiMeasurementRequestDenial> {
        match UiMeasurementCapabilityPosture::from_report(report, required_capabilities) {
            UiMeasurementCapabilityPosture::Available {
                required_capabilities,
            } => Ok(Self {
                required_capabilities,
            }),
            UiMeasurementCapabilityPosture::Missing {
                required_capabilities,
            } => Err(UiMeasurementRequestDenial::MissingCapability {
                required_capabilities,
            }),
            UiMeasurementCapabilityPosture::Ambiguous {
                required_capabilities,
            } => Err(UiMeasurementRequestDenial::AmbiguousCapability {
                required_capabilities,
            }),
            UiMeasurementCapabilityPosture::DiagnosticOnly {
                required_capabilities,
            } => Err(UiMeasurementRequestDenial::DiagnosticOnlyCapability {
                required_capabilities,
            }),
        }
    }

    pub fn required_capabilities(&self) -> &[WorthUiHostCapability] {
        &self.required_capabilities
    }

    pub fn posture(&self) -> UiMeasurementCapabilityPosture {
        UiMeasurementCapabilityPosture::Available {
            required_capabilities: self.required_capabilities.clone(),
        }
    }
}

impl UiMeasurementCapabilityPosture {
    pub fn from_report(
        report: &WorthUiHostCapabilityReport,
        mut required_capabilities: Vec<WorthUiHostCapability>,
    ) -> Self {
        required_capabilities.sort_by_key(|capability| capability.as_str());
        required_capabilities.dedup();
        let required_capabilities = required_capabilities.into_boxed_slice();

        match report.posture() {
            WorthUiHostCapabilityPosture::Available
                if required_capabilities
                    .iter()
                    .all(|capability| report.supports(*capability)) =>
            {
                Self::Available {
                    required_capabilities,
                }
            }
            WorthUiHostCapabilityPosture::Available | WorthUiHostCapabilityPosture::Missing => {
                Self::Missing {
                    required_capabilities,
                }
            }
            WorthUiHostCapabilityPosture::Ambiguous => Self::Ambiguous {
                required_capabilities,
            },
            WorthUiHostCapabilityPosture::DiagnosticOnly => Self::DiagnosticOnly {
                required_capabilities,
            },
        }
    }
}
