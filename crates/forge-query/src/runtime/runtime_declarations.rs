use super::*;

impl ForgeQueryRuntime {
    pub fn declare_live_view<T>(
        &mut self,
        name: impl Into<String>,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Live)?;
        let name = name.into();
        self.backend
            .admit_live_view_declaration(&name, &request, &schema_view)
            .map_err(
                |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: name.clone(),
                    stage: "backend-live-admission",
                    message: error.to_string(),
                },
            )?;
        let activation =
            self.install_live_subscription_for_request(&name, &request, schema_view.clone())?;
        let handle = match self
            .backend
            .declare_live_view(name.clone(), request, schema_view)
        {
            Ok(handle) => handle,
            Err(error) => {
                let closeout_result = close_subscription_lifecycle(
                    &mut self.active_subscriptions,
                    &activation.active_lane_handle,
                    SubscriptionLifecycleCloseRequest::DetachConsumer(
                        activation.consumer_attachment.clone(),
                    ),
                );
                let closeout_message = match closeout_result {
                    Ok(closeout) => format!(
                        "active subscription closeout:{}:terminal:{}",
                        closeout.closeout_digest(),
                        closeout.lane_terminal()
                    ),
                    Err(closeout_error) => format!(
                        "active subscription closeout failed:{}:{}",
                        closeout_error.denial_kind().as_str(),
                        closeout_error.message()
                    ),
                };
                return Err(ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                    view_name: name,
                    stage: "source-declaration",
                    message: format!("{error}; {closeout_message}"),
                });
            }
        };
        register_live_subscription_index(
            &mut self.live_subscription_index,
            &name,
            &activation.request,
        );
        self.live_subscriptions.insert(
            name,
            ForgeQueryRuntimeLiveSubscriptionState {
                installation: activation.installation.clone(),
                active_lane_handle: activation.active_lane_handle,
                consumer_attachment: activation.consumer_attachment,
                request: activation.request,
                delivery_batches: Vec::new(),
            },
        );
        Ok(ForgeQueryLiveView::new(handle, activation.installation))
    }

    pub fn declare_derived_view(
        &mut self,
        view: ForgeQueryDerivedView,
    ) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Computed)?;
        self.admit_derived_view_declaration(&view)?;
        insert_derived_runtime(
            &mut self.derived_views,
            &mut self.derived_dependency_index,
            view.clone(),
            None,
        );
        Ok(view)
    }

    pub fn declare_maintained_derived_view<T>(
        &mut self,
        view: ForgeQueryDerivedView,
        maintainer: impl ForgeQueryDerivedViewMaintainer + 'static,
    ) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Computed)?;
        self.admit_derived_view_declaration(&view)?;
        let name = view.name().to_string();
        insert_derived_runtime(
            &mut self.derived_views,
            &mut self.derived_dependency_index,
            view,
            Some(Box::new(maintainer)),
        );
        Ok(ForgeQueryDerivedViewHandle::new(name))
    }

    fn admit_derived_view_declaration(
        &self,
        view: &ForgeQueryDerivedView,
    ) -> Result<(), ForgeQueryRuntimeError> {
        let live_view_names = self.live_subscriptions.keys().cloned().collect();
        admit_derived_view_declaration(&self.derived_views, &live_view_names, view).map_err(
            |error| ForgeQueryRuntimeError::ComputedDeclaration {
                view_name: view.name().to_string(),
                stage: "dependency-admission",
                message: error.message(),
            },
        )
    }

    pub fn declare_effect<T>(
        &mut self,
        declaration: ForgeQueryEffectDeclaration,
    ) -> Result<ForgeQueryEffectHandle<T>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Effect)?;
        let live_view_names = self.live_subscriptions.keys().cloned().collect();
        let computed_view_names = self.derived_views.keys().cloned().collect();
        admit_effect_declaration(&live_view_names, &computed_view_names, &declaration)?;
        let name = declaration.name().to_string();
        let target_lane = declaration.target_lane();
        insert_effect_runtime(&mut self.effects, &mut self.effect_index, declaration);
        Ok(ForgeQueryEffectHandle::new(name, target_lane))
    }
}
