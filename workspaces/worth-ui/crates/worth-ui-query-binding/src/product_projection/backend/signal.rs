use worth_query::facade::{foundation, runtime};

pub(super) struct WorthUiScalarProjectionSignalSink;

impl runtime::WorthQueryRuntimeSignalSinkAdapter for WorthUiScalarProjectionSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &foundation::WorthQueryMutationReceipt,
    ) -> Result<runtime::SignalInvalidationBoundaryReceipt, foundation::WorthQueryWorkspaceError>
    {
        let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
        self.build_signal_invalidation_boundary_receipt(receipt, routed)
    }
}
