use crate::evidence::HarnessFailureLocation;
use crate::honesty::HarnessHonestyDenial;

use super::HarnessReplayDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HarnessRunDenial {
    EmptyScenario {
        scenario_id: crate::scenario::HarnessScenarioId,
    },
    RuntimeLaunchDenied {
        location: HarnessFailureLocation,
    },
    Honesty {
        location: HarnessFailureLocation,
        denial: HarnessHonestyDenial,
    },
    ReplayMismatch {
        denial: HarnessReplayDenial,
    },
}

impl HarnessRunDenial {
    pub(crate) fn localized_honesty(
        location: HarnessFailureLocation,
        denial: HarnessHonestyDenial,
    ) -> Self {
        Self::Honesty { location, denial }
    }
}
