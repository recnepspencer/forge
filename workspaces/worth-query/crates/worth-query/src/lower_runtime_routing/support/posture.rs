use super::super::{
    WorthQueryLowerRuntimeCloseoutPosture, WorthQueryLowerRuntimeCrossingClassification,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimeSupportPosture {
    Admitted,
    CompatibilityDebt,
    SeamEliminated,
    Deferred,
    Forbidden,
}

impl WorthQueryLowerRuntimeSupportPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::CompatibilityDebt => "compatibility-debt",
            Self::SeamEliminated => "seam-eliminated",
            Self::Deferred => "deferred",
            Self::Forbidden => "forbidden",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimeSupportDetail {
    Crossing,
    Closeout {
        closeout_target: &'static str,
        required_closeout: &'static str,
        certification_row: &'static str,
    },
}

impl WorthQueryLowerRuntimeSupportDetail {
    pub fn closeout_target(&self) -> Option<&'static str> {
        match self {
            Self::Crossing => None,
            Self::Closeout {
                closeout_target, ..
            } => Some(*closeout_target),
        }
    }

    pub fn required_closeout(&self) -> Option<&'static str> {
        match self {
            Self::Crossing => None,
            Self::Closeout {
                required_closeout, ..
            } => Some(*required_closeout),
        }
    }

    pub fn certification_row(&self) -> Option<&'static str> {
        match self {
            Self::Crossing => None,
            Self::Closeout {
                certification_row, ..
            } => Some(*certification_row),
        }
    }
}

pub(crate) fn support_posture_for_classification(
    classification: WorthQueryLowerRuntimeCrossingClassification,
) -> WorthQueryLowerRuntimeSupportPosture {
    match classification {
        WorthQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse
        | WorthQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter => {
            WorthQueryLowerRuntimeSupportPosture::Admitted
        }
        WorthQueryLowerRuntimeCrossingClassification::CompatibilityDebtLane => {
            WorthQueryLowerRuntimeSupportPosture::CompatibilityDebt
        }
        WorthQueryLowerRuntimeCrossingClassification::DeferredNeighbor => {
            WorthQueryLowerRuntimeSupportPosture::Deferred
        }
        WorthQueryLowerRuntimeCrossingClassification::ForbiddenDuplicate => {
            WorthQueryLowerRuntimeSupportPosture::Forbidden
        }
    }
}

pub(crate) fn support_posture_for_closeout(
    posture: WorthQueryLowerRuntimeCloseoutPosture,
) -> WorthQueryLowerRuntimeSupportPosture {
    match posture {
        WorthQueryLowerRuntimeCloseoutPosture::SeamEliminated => {
            WorthQueryLowerRuntimeSupportPosture::SeamEliminated
        }
        WorthQueryLowerRuntimeCloseoutPosture::DeferredNeighbor => {
            WorthQueryLowerRuntimeSupportPosture::Deferred
        }
    }
}
