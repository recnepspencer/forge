use crate::facade::entry::CapabilityRegistrationBuilder;
use crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use crate::facade::lifecycle::{
    prepare_application_authority, WorthUiApplicationPreparationDenial,
    WorthUiApplicationPreparationSource,
};
use crate::facade::measurement_exchange::WorthUiOperationalHostAdapter;
use crate::facade::prepared_application_authority::WorthUiHostSessionPlan;
use crate::facade::registry::descriptor::{
    CommandDescriptor, CommandProjectionDescriptor, ComponentDescriptor, IconDescriptor,
    MosaicPlacementPolicyDescriptor, MosaicRegionKindDescriptor, MosaicSizingContractDescriptor,
    MosaicStateSlotDescriptor, NativeCapabilityDescriptor, PluginSlotDescriptor,
    RuntimeOutcomeProjectionDescriptor, SettingDescriptor, SurfaceDescriptor,
    TaskPresentationDescriptor, ThemeTokenDescriptor, WorthUiQueryViewRegistration,
};
use crate::facade::registry::diagnostics::CapabilityRegistrationReport;
use crate::facade::{WorthUiApp, WorthUiDslPackage};
use crate::graph::UiGraphWorldProfile;
use crate::runtime::WorthUiWatchedCandidateSubmission;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewRegistrationError {
    Binding(worth_ui_query_binding::WorthUiQueryBindingRegistrationDenial),
    InvalidIdentity(crate::capability::CapabilityIdError),
}

/// Builder for a Worth UI application definition.
pub struct WorthUiBuilder {
    inner: CapabilityRegistrationBuilder,
    preparation_source: WorthUiBuilderPreparationSource,
    host_session_plan: WorthUiHostSessionPlan,
    graph_world_profile: UiGraphWorldProfile,
    runtime_instance_basis_admissions: Vec<crate::graph::UiRuntimeInstanceBasisAdmission>,
    measurement_inspection_evidence: Vec<UiMeasurementInspectionEvidenceBundle>,
    query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
}

pub type WorthUiAppBuilder = WorthUiBuilder;

impl WorthUiBuilder {
    pub(crate) fn new() -> Self {
        Self {
            inner: CapabilityRegistrationBuilder::new(),
            preparation_source: WorthUiBuilderPreparationSource::Declared(
                WorthUiDslPackage::empty(),
            ),
            host_session_plan: WorthUiHostSessionPlan::prepare(
                crate::host::adapter::WorthUiHeadlessHost::default(),
            ),
            graph_world_profile: UiGraphWorldProfile::authoritative(),
            runtime_instance_basis_admissions: Vec::new(),
            measurement_inspection_evidence: Vec::new(),
            query_binding_plan: Default::default(),
        }
    }

    pub fn with_dsl_package(mut self, dsl_package: WorthUiDslPackage) -> Self {
        self.preparation_source = WorthUiBuilderPreparationSource::Declared(dsl_package);
        self
    }

    /// Prepare the exact artifact/declaration composition admitted by watched
    /// source ingress. The submission is consumed whole and cannot be split.
    pub fn with_candidate_submission(
        mut self,
        submission: WorthUiWatchedCandidateSubmission,
    ) -> Self {
        self.preparation_source = WorthUiBuilderPreparationSource::Watched(Box::new(submission));
        self
    }

    pub fn with_host<Host>(mut self, host: Host) -> Self
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        let retention_budget = self.host_session_plan.mounted_frame_retention_budget();
        let observation_capacity = self.host_session_plan.host_observation_capacity();
        self.host_session_plan = WorthUiHostSessionPlan::prepare(host);
        self.host_session_plan
            .set_mounted_frame_retention_budget(retention_budget);
        self.host_session_plan
            .set_host_observation_capacity(observation_capacity);
        self
    }

    pub fn with_host_observation_capacity(
        mut self,
        capacity: crate::facade::observation_report::UiHostObservationCapacity,
    ) -> Self {
        self.host_session_plan
            .set_host_observation_capacity(capacity);
        self
    }

    pub fn with_mounted_frame_retention_budget(
        mut self,
        budget: crate::mounting::UiMountedFrameRetentionBudget,
    ) -> Self {
        self.host_session_plan
            .set_mounted_frame_retention_budget(budget);
        self
    }

    pub fn with_graph_world_profile(mut self, graph_world_profile: UiGraphWorldProfile) -> Self {
        self.graph_world_profile = graph_world_profile;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_runtime_instance_basis_admissions(
        mut self,
        admissions: impl IntoIterator<Item = crate::graph::UiRuntimeInstanceBasisAdmission>,
    ) -> Self {
        self.runtime_instance_basis_admissions = admissions.into_iter().collect();
        self
    }

    #[cfg(test)]
    pub fn with_measurement_inspection_evidence(
        mut self,
        evidence: UiMeasurementInspectionEvidenceBundle,
    ) -> Self {
        self.measurement_inspection_evidence.push(evidence);
        self
    }

    pub fn register_command(mut self, descriptor: CommandDescriptor) -> Self {
        self.inner = self.inner.register_command(descriptor);
        self
    }

    pub fn register_command_projection(mut self, descriptor: CommandProjectionDescriptor) -> Self {
        self.inner = self.inner.register_command_projection(descriptor);
        self
    }

    pub fn register_component(mut self, descriptor: ComponentDescriptor) -> Self {
        self.inner = self.inner.register_component(descriptor);
        self
    }

    pub fn register_icon(mut self, descriptor: IconDescriptor) -> Self {
        self.inner = self.inner.register_icon(descriptor);
        self
    }

    pub fn register_surface(mut self, descriptor: SurfaceDescriptor) -> Self {
        self.inner = self.inner.register_surface(descriptor);
        self
    }

    pub fn register_mosaic_region_kind(mut self, descriptor: MosaicRegionKindDescriptor) -> Self {
        self.inner = self.inner.register_mosaic_region_kind(descriptor);
        self
    }

    pub fn register_mosaic_placement_policy(
        mut self,
        descriptor: MosaicPlacementPolicyDescriptor,
    ) -> Self {
        self.inner = self.inner.register_mosaic_placement_policy(descriptor);
        self
    }

    pub fn register_mosaic_sizing_contract(
        mut self,
        descriptor: MosaicSizingContractDescriptor,
    ) -> Self {
        self.inner = self.inner.register_mosaic_sizing_contract(descriptor);
        self
    }

    pub fn register_mosaic_state_slot(mut self, descriptor: MosaicStateSlotDescriptor) -> Self {
        self.inner = self.inner.register_mosaic_state_slot(descriptor);
        self
    }

    pub fn register_native_capability(mut self, descriptor: NativeCapabilityDescriptor) -> Self {
        self.inner = self.inner.register_native_capability(descriptor);
        self
    }

    pub fn register_plugin_slot(mut self, descriptor: PluginSlotDescriptor) -> Self {
        self.inner = self.inner.register_plugin_slot(descriptor);
        self
    }

    #[cfg(test)]
    pub(crate) fn register_view_binding(
        mut self,
        descriptor: crate::facade::registry::descriptor::ViewBindingDescriptor,
    ) -> Self {
        self.inner = self.inner.register_view_binding(descriptor);
        self
    }

    /// Register an installed Query view as one coherent definition and
    /// runtime-affine authority. Query posture cannot be assembled piecemeal.
    pub fn register_query_view(
        mut self,
        registration: impl Into<WorthUiQueryViewRegistration>,
    ) -> Result<Self, WorthUiQueryViewRegistrationError> {
        let (view, visible_state_bindings, denial_presentation) = registration.into().into_parts();
        let definition = view.definition().clone();
        let id = crate::capability::ViewBindingId::new(definition.identity().as_str())
            .map_err(WorthUiQueryViewRegistrationError::InvalidIdentity)?;
        let family = match definition.shape() {
            worth_ui_query_binding::WorthUiQueryViewShape::Collection => {
                crate::capability::ViewBindingFamily::collection()
            }
            worth_ui_query_binding::WorthUiQueryViewShape::Detail => {
                crate::capability::ViewBindingFamily::detail()
            }
        };
        self.query_binding_plan = self
            .query_binding_plan
            .register_view(view)
            .map_err(WorthUiQueryViewRegistrationError::Binding)?;
        let descriptor = visible_state_bindings.into_iter().fold(
            crate::capability::ViewBindingDescriptor::from_definition(id, family, definition)
                .with_denial_presentation(denial_presentation),
            crate::capability::ViewBindingDescriptor::with_visible_state_binding,
        );
        self.inner = self.inner.register_view_binding(descriptor);
        Ok(self)
    }

    pub fn register_runtime_outcome_projection(
        mut self,
        descriptor: RuntimeOutcomeProjectionDescriptor,
    ) -> Self {
        self.inner = self.inner.register_runtime_outcome_projection(descriptor);
        self
    }

    pub fn register_setting(mut self, descriptor: SettingDescriptor) -> Self {
        self.inner = self.inner.register_setting(descriptor);
        self
    }

    pub fn register_task_presentation(mut self, descriptor: TaskPresentationDescriptor) -> Self {
        self.inner = self.inner.register_task_presentation(descriptor);
        self
    }

    pub fn register_theme_token(mut self, descriptor: ThemeTokenDescriptor) -> Self {
        self.inner = self.inner.register_theme_token(descriptor);
        self
    }

    pub fn freeze(self) -> Result<WorthUiApp, WorthUiApplicationPreparationDenial> {
        let capability_snapshot = self
            .inner
            .freeze_with_registration_report()
            .into_accepted_snapshot();
        let preparation_source = match self.preparation_source {
            WorthUiBuilderPreparationSource::Declared(dsl_package) => {
                WorthUiApplicationPreparationSource::declared(dsl_package)
            }
            WorthUiBuilderPreparationSource::Watched(submission) => {
                WorthUiApplicationPreparationSource::watched_submission(
                    *submission,
                    capability_snapshot.digest(),
                )?
            }
        };
        Ok(WorthUiApp::from_prepared_authority(
            prepare_application_authority(
                capability_snapshot,
                preparation_source,
                self.host_session_plan,
                self.graph_world_profile,
                self.runtime_instance_basis_admissions.into_boxed_slice(),
                self.measurement_inspection_evidence.into_boxed_slice(),
                self.query_binding_plan,
            )?,
        ))
    }

    pub fn freeze_with_registration_report(self) -> CapabilityRegistrationReport {
        self.inner.freeze_with_registration_report()
    }

    pub fn with_minimal_registration_diagnostics(mut self) -> Self {
        self.inner = self.inner.with_minimal_registration_diagnostics();
        self
    }

    pub fn with_rich_registration_diagnostics(mut self) -> Self {
        self.inner = self.inner.with_rich_registration_diagnostics();
        self
    }
}

enum WorthUiBuilderPreparationSource {
    Declared(WorthUiDslPackage),
    Watched(Box<WorthUiWatchedCandidateSubmission>),
}
