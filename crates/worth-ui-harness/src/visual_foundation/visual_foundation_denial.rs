use crate::theme::HarnessVisualTokenRole;

use super::{HarnessCommandProjectionVisualRole, HarnessRuntimeOutcomeVisualRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HarnessVisualFoundationDenial {
    MissingTokenRole {
        role: HarnessVisualTokenRole,
    },
    DuplicateTokenRole {
        role: HarnessVisualTokenRole,
    },
    MissingDensityMeasurements,
    MissingIconDescriptors,
    MissingCommandProjection {
        role: HarnessCommandProjectionVisualRole,
    },
    MissingRuntimeOutcomeProjection {
        role: HarnessRuntimeOutcomeVisualRole,
    },
}
