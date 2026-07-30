use crate::facade::entry::CapabilityRegistrationBuilder;
use crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use crate::facade::lifecycle::{
    prepare_application_authority, WorthUiApplicationPreparationDenial,
    WorthUiApplicationPreparationInput, WorthUiApplicationPreparationSource,
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
use crate::facade::WorthUiApp;
use crate::graph::UiGraphWorldProfile;
use crate::runtime::WorthUiWatchedCandidateSubmission;

mod intent_registration;
mod registration_error;

pub use registration_error::{
    WorthUiProjectionRegistrationError, WorthUiQueryViewRegistrationError,
};

/// Builder for a Worth UI application definition.
pub struct WorthUiApplicationBuilder<ChangeProfileState = UiChangeProfileInstalled> {
    inner: CapabilityRegistrationBuilder,
    preparation_source: WorthUiApplicationBuilderPreparationSource,
    host_session_plan: WorthUiHostSessionPlan,
    visual_inspection_policy: worth_ui_inspection::UiVisualInspectionPolicy,
    graph_world_profile: UiGraphWorldProfile,
    runtime_instance_basis_admissions: Vec<crate::graph::UiRuntimeInstanceBasisAdmission>,
    measurement_inspection_evidence: Vec<UiMeasurementInspectionEvidenceBundle>,
    query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
    change_profile: ChangeProfileState,
}

pub struct UiChangeProfileMissing {
    _sealed: (),
}

pub struct UiChangeProfileInstalled {
    profile: crate::runtime::rebind::UiChangeProfile,
}

impl WorthUiApplicationBuilder<UiChangeProfileMissing> {
    pub(crate) fn new() -> Self {
        Self {
            inner: CapabilityRegistrationBuilder::new(),
            preparation_source: WorthUiApplicationBuilderPreparationSource::RustAuthored(
                worth_ui_dsl::WorthUiRustAuthoredArtifactInput::default(),
            ),
            host_session_plan: WorthUiHostSessionPlan::prepare(
                crate::host::adapter::WorthUiHeadlessHost,
            ),
            visual_inspection_policy:
                worth_ui_inspection::UiVisualInspectionPolicy::production_default(
                    worth_ui_inspection::UiVisualInspectionDisclosure::local_development_unredacted(
                    ),
                )
                .expect("the governed default visual inspection policy is valid"),
            graph_world_profile: UiGraphWorldProfile::authoritative(),
            runtime_instance_basis_admissions: Vec::new(),
            measurement_inspection_evidence: Vec::new(),
            query_binding_plan: Default::default(),
            change_profile: UiChangeProfileMissing { _sealed: () },
        }
    }

    pub fn with_change_profile(
        self,
        profile: crate::runtime::rebind::UiChangeProfile,
    ) -> WorthUiApplicationBuilder<UiChangeProfileInstalled> {
        self.transition_change_profile(UiChangeProfileInstalled { profile })
    }
}

impl<ChangeProfileState> WorthUiApplicationBuilder<ChangeProfileState> {
    pub fn with_rust_authored_input(
        mut self,
        input: worth_ui_dsl::WorthUiRustAuthoredArtifactInput,
    ) -> Self {
        self.preparation_source = WorthUiApplicationBuilderPreparationSource::RustAuthored(input);
        self
    }

    #[cfg(test)]
    pub(crate) fn with_rust_authored_declaration_fixture(
        self,
        fixture: crate::facade::WorthUiRustAuthoredDeclarationFixture,
    ) -> Self {
        self.with_rust_authored_input(fixture.into_input())
    }

    /// Prepare the exact artifact/declaration composition admitted by watched
    /// source ingress. The submission is consumed whole and cannot be split.
    pub fn with_candidate_submission(
        mut self,
        submission: WorthUiWatchedCandidateSubmission,
    ) -> Self {
        self.preparation_source =
            WorthUiApplicationBuilderPreparationSource::Watched(Box::new(submission));
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

    /// Declare the immutable visual-inspection disclosure and resource policy
    /// that launch will seal into this application's session authority.
    pub fn with_visual_inspection_policy(
        mut self,
        policy: worth_ui_inspection::UiVisualInspectionPolicy,
    ) -> Self {
        self.visual_inspection_policy = policy;
        self
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn with_graph_world_profile(
        mut self,
        graph_world_profile: UiGraphWorldProfile,
    ) -> Self {
        self.graph_world_profile = graph_world_profile;
        self
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn with_runtime_instance_basis_admissions(
        mut self,
        admissions: impl IntoIterator<Item = crate::graph::UiRuntimeInstanceBasisAdmission>,
    ) -> Self {
        self.runtime_instance_basis_admissions = admissions.into_iter().collect();
        self
    }

    #[cfg(test)]
    pub(crate) fn with_measurement_inspection_evidence(
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

    pub fn register_scalar_projection(
        mut self,
        registration: worth_ui_query_binding::UiScalarProjectionRegistration,
    ) -> Result<Self, WorthUiProjectionRegistrationError> {
        self.query_binding_plan = self
            .query_binding_plan
            .register_scalar_projection(registration)
            .map_err(WorthUiProjectionRegistrationError)?;
        Ok(self)
    }

    pub fn register_collection_projection(
        mut self,
        registration: worth_ui_query_binding::UiCollectionProjectionRegistration,
    ) -> Result<Self, WorthUiProjectionRegistrationError> {
        self.query_binding_plan = self
            .query_binding_plan
            .register_collection_projection(registration)
            .map_err(WorthUiProjectionRegistrationError)?;
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

    fn transition_change_profile<NextProfileState>(
        self,
        change_profile: NextProfileState,
    ) -> WorthUiApplicationBuilder<NextProfileState> {
        WorthUiApplicationBuilder {
            inner: self.inner,
            preparation_source: self.preparation_source,
            host_session_plan: self.host_session_plan,
            visual_inspection_policy: self.visual_inspection_policy,
            graph_world_profile: self.graph_world_profile,
            runtime_instance_basis_admissions: self.runtime_instance_basis_admissions,
            measurement_inspection_evidence: self.measurement_inspection_evidence,
            query_binding_plan: self.query_binding_plan,
            change_profile,
        }
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

impl WorthUiApplicationBuilder<UiChangeProfileInstalled> {
    pub fn freeze(self) -> Result<WorthUiApp, WorthUiApplicationPreparationDenial> {
        let capability_snapshot = self
            .inner
            .freeze_with_registration_report()
            .into_accepted_snapshot();
        let preparation_source = match self.preparation_source {
            WorthUiApplicationBuilderPreparationSource::RustAuthored(input) => {
                WorthUiApplicationPreparationSource::rust_authored(&input, &capability_snapshot)?
            }
            WorthUiApplicationBuilderPreparationSource::Watched(submission) => {
                WorthUiApplicationPreparationSource::watched_submission(
                    *submission,
                    capability_snapshot.digest(),
                )?
            }
        };
        Ok(WorthUiApp::from_prepared_authority(
            prepare_application_authority(WorthUiApplicationPreparationInput {
                capability_snapshot,
                preparation_source,
                host_session_plan: self.host_session_plan,
                visual_inspection_policy: self.visual_inspection_policy,
                graph_world_profile: self.graph_world_profile,
                runtime_instance_basis_admissions: self
                    .runtime_instance_basis_admissions
                    .into_boxed_slice(),
                measurement_inspection_evidence: self
                    .measurement_inspection_evidence
                    .into_boxed_slice(),
                query_binding_plan: self.query_binding_plan,
                change_profile: self.change_profile.profile,
            })?,
        ))
    }
}

enum WorthUiApplicationBuilderPreparationSource {
    RustAuthored(worth_ui_dsl::WorthUiRustAuthoredArtifactInput),
    Watched(Box<WorthUiWatchedCandidateSubmission>),
}
