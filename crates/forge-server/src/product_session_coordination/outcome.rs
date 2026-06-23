use crate::ForgeServerProductSession;

use super::{
    ForgeServerLoweredProductSessionCoordinationPlan, ForgeServerProductSessionSchedulerAdmission,
};

#[derive(Clone, Debug)]
pub struct ForgeServerCompletedProductSessionCoordination {
    session: ForgeServerProductSession,
    plan: ForgeServerLoweredProductSessionCoordinationPlan,
    scheduler_admission: ForgeServerProductSessionSchedulerAdmission,
}

impl ForgeServerCompletedProductSessionCoordination {
    pub(crate) fn new(
        session: ForgeServerProductSession,
        plan: ForgeServerLoweredProductSessionCoordinationPlan,
        scheduler_admission: ForgeServerProductSessionSchedulerAdmission,
    ) -> Self {
        Self {
            session,
            plan,
            scheduler_admission,
        }
    }

    pub fn session(&self) -> &ForgeServerProductSession {
        &self.session
    }

    pub fn into_session(self) -> ForgeServerProductSession {
        self.session
    }

    pub fn plan(&self) -> &ForgeServerLoweredProductSessionCoordinationPlan {
        &self.plan
    }

    pub fn scheduler_admission(&self) -> &ForgeServerProductSessionSchedulerAdmission {
        &self.scheduler_admission
    }
}
