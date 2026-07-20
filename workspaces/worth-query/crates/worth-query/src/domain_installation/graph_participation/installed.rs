use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    registry::WorthQueryInstalledGraphParticipationRecord, WorthQueryGraphParticipationContract,
};

pub struct WorthQueryInstalledGraphParticipation<G> {
    pub(crate) record: Arc<WorthQueryInstalledGraphParticipationRecord>,
    _marker: PhantomData<fn() -> G>,
}

impl<G> WorthQueryInstalledGraphParticipation<G> {
    pub(crate) fn new(record: Arc<WorthQueryInstalledGraphParticipationRecord>) -> Self {
        Self {
            record,
            _marker: PhantomData,
        }
    }

    pub fn role(&self) -> &str {
        &self.record.definition.role
    }

    pub fn contract(&self) -> &WorthQueryGraphParticipationContract {
        &self.record.definition.contract
    }
}
