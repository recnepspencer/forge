use super::delivery::unregister_live_subscription_index;
use super::*;
use crate::subscription::{validate_subscription_lifecycle_close, SubscriptionLifecycleCloseout};

impl WorthQueryRuntime {
    pub(crate) fn close_managed_live_view<T>(
        &mut self,
        view: &WorthQueryLiveView<T>,
    ) -> Result<SubscriptionLifecycleCloseout, WorthQueryRuntimeError> {
        let target = WorthQueryLiveArtifactTarget::from_subscription_installation(
            view.subscription_installation(),
        );
        let state = self.live_subscriptions.get(&target).ok_or_else(|| {
            WorthQueryRuntimeError::MissingLiveSubscription(view.name().to_string())
        })?;
        if state.installation != *view.subscription_installation() {
            return Err(lifecycle_close_error(
                view.name(),
                "managed handle installation does not match Query's registered live resource",
            ));
        }
        if state.read_authority_binding.is_none() {
            return Err(lifecycle_close_error(
                view.name(),
                "managed live resource is missing its admitted read-authority binding",
            ));
        }
        if self.derived_dependency_index.has_live_dependents(&target)
            || self.effect_index.has_live_dependents(&target)
        {
            return Err(lifecycle_close_error(
                view.name(),
                "managed live resource still has registered computed or effect dependents",
            ));
        }

        let close_request =
            SubscriptionLifecycleCloseRequest::DetachConsumer(state.consumer_attachment.clone());
        validate_subscription_lifecycle_close(
            &self.active_subscriptions,
            &state.active_lane_handle,
            &close_request,
        )
        .map_err(|error| {
            lifecycle_close_error(
                view.name(),
                format!("{}: {}", error.denial_kind().as_str(), error.message()),
            )
        })?;
        self.backend
            .close_live_view(view.name())
            .map_err(|error| lifecycle_close_error(view.name(), error.to_string()))?;
        let closeout = close_subscription_lifecycle(
            &mut self.active_subscriptions,
            &state.active_lane_handle,
            close_request,
        )
        .map_err(|error| {
            lifecycle_close_error(
                view.name(),
                format!("{}: {}", error.denial_kind().as_str(), error.message()),
            )
        })?;

        self.live_subscriptions.remove(&target);
        self.materialized_read_views.remove(&target);
        unregister_live_subscription_index(&mut self.live_subscription_index, view.name());
        Ok(closeout)
    }
}

fn lifecycle_close_error(view_name: &str, message: impl Into<String>) -> WorthQueryRuntimeError {
    WorthQueryRuntimeError::LiveSubscriptionInstallation {
        view_name: view_name.to_string(),
        stage: "managed-lifecycle-close",
        message: message.into(),
    }
}
