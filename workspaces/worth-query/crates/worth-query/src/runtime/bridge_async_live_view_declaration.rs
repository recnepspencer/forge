use worth_runtime_bridge::facade::AdmittedBridgeAsyncRequestIdentity;

use super::async_source_binding::WorthQueryRuntimeAsyncSourceBinding;
use super::runtime_declarations::admit_live_view_declaration_receipt;
use super::{
    DeclarativeLiveQueryRequest, QuerySchemaView, WorthQueryLiveView, WorthQueryRuntime,
    WorthQueryRuntimeError, WorthQueryRuntimeFacadeFamily,
};

impl WorthQueryRuntime {
    pub fn declare_bridge_async_live_view<T>(
        &mut self,
        name: impl Into<String>,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
        bridge_request: &AdmittedBridgeAsyncRequestIdentity,
    ) -> Result<WorthQueryLiveView<T>, WorthQueryRuntimeError> {
        self.reap_abandoned_managed_live_resources()?;
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Live)?;
        let name = name.into();
        admit_live_view_declaration_receipt(&*self.backend, &name, &request, &schema_view)?;
        let future_selection =
            crate::subscription::QuerySubscriptionFutureSelection::async_resource_with_identity(
                true,
                vec![
                    crate::subscription::QuerySubscriptionAsyncRequestIdentityPart::new(
                        "bridge-declaration",
                        bridge_request
                            .lowered()
                            .declaration_identity_for_reporting(),
                    ),
                    crate::subscription::QuerySubscriptionAsyncRequestIdentityPart::new(
                        "bridge-request",
                        bridge_request.request_identity_for_reporting(),
                    ),
                ],
            );
        let activation = self.install_live_subscription_for_request(
            &name,
            &request,
            schema_view.clone(),
            future_selection,
        )?;
        let binding = WorthQueryRuntimeAsyncSourceBinding::admit(&name, bridge_request);
        self.finish_live_view_declaration(name, request, schema_view, activation, Some(binding))
    }
}
