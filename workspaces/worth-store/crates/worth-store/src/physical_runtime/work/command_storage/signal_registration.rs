use worth_signal::facade::ResourceRequestHandle;

use super::PhysicalCommandArena;
use crate::physical_runtime::{
    PhysicalSignalAspectBindingDigest, PhysicalWorkConsumerHandle, PhysicalWorkIdentity,
};

#[derive(Clone, Copy)]
pub(in crate::physical_runtime::work) struct PhysicalCommandSignalRegistration {
    pub(in crate::physical_runtime::work) route: PhysicalSignalAspectBindingDigest,
    pub(in crate::physical_runtime::work) consumer: Option<PhysicalWorkConsumerHandle>,
}

impl PhysicalCommandArena {
    pub(in crate::physical_runtime::work) fn bind_signal(
        &self,
        identity: PhysicalWorkIdentity,
        signal_request: ResourceRequestHandle,
        route: PhysicalSignalAspectBindingDigest,
        superseded: Option<ResourceRequestHandle>,
    ) -> bool {
        let shard = identity.operation().get() as usize % self.declared.len();
        let mut declared = self.declared[shard]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(entry) = declared.get_mut(&identity) else {
            return false;
        };
        let consumer = PhysicalWorkConsumerHandle::new(identity, signal_request, route);
        match entry.consumer {
            Some(bound) if superseded == Some(bound.signal_request()) && bound.route() == route => {
                entry.consumer = Some(consumer);
                entry.retry_pending = false;
                true
            }
            Some(bound) => bound == consumer,
            None => {
                entry.consumer = Some(consumer);
                true
            }
        }
    }

    pub(in crate::physical_runtime::work) fn register_signal_locality(
        &self,
        identity: PhysicalWorkIdentity,
        route: PhysicalSignalAspectBindingDigest,
    ) -> bool {
        let shard = identity.operation().get() as usize % self.declared.len();
        let mut declared = self.declared[shard]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let Some(entry) = declared.get_mut(&identity) else {
            return false;
        };
        match entry.signal_route {
            Some(bound) => bound == route,
            None => {
                entry.signal_route = Some(route);
                true
            }
        }
    }

    pub(in crate::physical_runtime::work) fn signal_registration(
        &self,
        identity: PhysicalWorkIdentity,
    ) -> Option<PhysicalCommandSignalRegistration> {
        let shard = identity.operation().get() as usize % self.declared.len();
        let declared = self.declared[shard]
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let entry = declared.get(&identity)?;
        Some(PhysicalCommandSignalRegistration {
            route: entry.signal_route?,
            consumer: entry.consumer,
        })
    }
}
