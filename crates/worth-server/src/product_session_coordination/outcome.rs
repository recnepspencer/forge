use crate::WorthServerProductSession;

use super::{
    WorthServerLoweredProductSessionCoordinationPlan, WorthServerProductSessionSchedulerAdmission,
};

#[derive(Clone, Debug)]
pub struct WorthServerCompletedProductSessionCoordination {
    session: WorthServerProductSession,
    plan: WorthServerLoweredProductSessionCoordinationPlan,
    scheduler_admission: WorthServerProductSessionSchedulerAdmission,
}

impl WorthServerCompletedProductSessionCoordination {
    pub(crate) fn new(
        session: WorthServerProductSession,
        plan: WorthServerLoweredProductSessionCoordinationPlan,
        scheduler_admission: WorthServerProductSessionSchedulerAdmission,
    ) -> Self {
        Self {
            session,
            plan,
            scheduler_admission,
        }
    }

    pub fn session(&self) -> &WorthServerProductSession {
        &self.session
    }

    pub fn into_session(self) -> WorthServerProductSession {
        self.session
    }

    pub fn plan(&self) -> &WorthServerLoweredProductSessionCoordinationPlan {
        &self.plan
    }

    pub fn scheduler_admission(&self) -> &WorthServerProductSessionSchedulerAdmission {
        &self.scheduler_admission
    }
}
