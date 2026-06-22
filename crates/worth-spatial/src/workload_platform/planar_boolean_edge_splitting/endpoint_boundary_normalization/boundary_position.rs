use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlanarBooleanSplitBoundaryPosition {
    Start,
    Interior,
    End,
}

impl PlanarBooleanSplitBoundaryPosition {
    pub(super) fn from_parameter(parameter: f64) -> Self {
        if canonical_parameter_bits(parameter) == canonical_parameter_bits(0.0) {
            Self::Start
        } else if canonical_parameter_bits(parameter) == canonical_parameter_bits(1.0) {
            Self::End
        } else {
            Self::Interior
        }
    }

    pub(super) fn is_boundary(self) -> bool {
        matches!(self, Self::Start | Self::End)
    }

    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Interior => "interior",
            Self::End => "end",
        }
    }
}

impl Default for PlanarBooleanSplitBoundaryPosition {
    fn default() -> Self {
        Self::Interior
    }
}
