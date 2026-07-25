use crate::physical_runtime::{
    PhysicalSignalAspectBindingDigest, PhysicalWorkConsumerHandle, PhysicalWorkIdentity,
};

use super::PhysicalSignalGraph;

pub(in crate::physical_runtime::instance::signal_owner) enum PhysicalSignalAbandonmentFailure {
    RouteAbsent,
    ConsumerMismatch,
    CancellationRejected,
    #[cfg(feature = "certification-test-authority")]
    InjectedWorkerFailure,
}

impl PhysicalSignalGraph {
    pub(in crate::physical_runtime) fn abandon_work(
        &mut self,
        identity: PhysicalWorkIdentity,
        route: PhysicalSignalAspectBindingDigest,
        consumer: Option<PhysicalWorkConsumerHandle>,
    ) -> Result<(), PhysicalSignalAbandonmentFailure> {
        let route_is_installed = self
            .bindings
            .bindings()
            .iter()
            .any(|binding| binding.digest() == route);
        if !route_is_installed {
            return Err(PhysicalSignalAbandonmentFailure::RouteAbsent);
        }
        if let Some(consumer) = consumer {
            if consumer.identity() != identity || consumer.route() != route {
                return Err(PhysicalSignalAbandonmentFailure::ConsumerMismatch);
            }
            self.runtime
                .cancel_resource_request(
                    consumer.signal_request(),
                    worth_signal::facade::ResourceCancellationReason::HostRequested,
                )
                .map_err(|_| PhysicalSignalAbandonmentFailure::CancellationRejected)?;
        }
        self.release_identity(identity);
        Ok(())
    }
}
