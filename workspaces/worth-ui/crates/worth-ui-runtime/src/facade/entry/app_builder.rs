use crate::facade::entry::CapabilityRegistrationBuilder;
use crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use crate::facade::lifecycle::{
    prepare_application_authority, WorthUiApplicationPreparationDenial,
    WorthUiApplicationPreparationInput, WorthUiApplicationPreparationSource,
};
use crate::facade::measurement_exchange::WorthUiOperationalHostAdapter;
use crate::facade::prepared_application_authority::WorthUiHostSessionPlan;
use crate::facade::registry::diagnostics::CapabilityRegistrationReport;
use crate::facade::WorthUiApp;
use crate::graph::UiGraphWorldProfile;
use crate::runtime::WorthUiWatchedCandidateSubmission;

mod application_fact_registration;
mod capability_registration;
mod intent_registration;
mod query_registration;
mod registration_error;

pub use registration_error::{
    WorthUiProjectionRegistrationError, WorthUiQueryViewRegistrationError,
};

/// Builder for a Worth UI application definition.
pub struct WorthUiApplicationBuilder<
    ChangeProfileState = UiChangeProfileInstalled,
    IntentWiringState = UiIntentWiringSatisfied,
> {
    inner: CapabilityRegistrationBuilder,
    preparation_source: WorthUiApplicationBuilderPreparationSource,
    host_session_plan: WorthUiHostSessionPlan,
    visual_inspection_policy: worth_ui_inspection::UiVisualInspectionPolicy,
    graph_world_profile: UiGraphWorldProfile,
    runtime_instance_basis_admissions: Vec<crate::graph::UiRuntimeInstanceBasisAdmission>,
    measurement_inspection_evidence: Vec<UiMeasurementInspectionEvidenceBundle>,
    query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
    intent_application_facts: crate::declaration::UiIntentApplicationFactPlan,
    intent_execution_bindings: crate::runtime::intent_execution::UiIntentExecutionBindingPlan,
    change_profile: ChangeProfileState,
    intent_wiring: IntentWiringState,
}

pub struct UiChangeProfileMissing {
    _sealed: (),
}

pub struct UiChangeProfileInstalled {
    profile: crate::runtime::rebind::UiChangeProfile,
}

pub struct UiIntentWiringSatisfied {
    _sealed: (),
}

pub struct UiIntentProviderRequired<I: crate::capability::UiIntent> {
    definition:
        crate::capability::UiIntentDefinition<I, crate::capability::UiApplicationEffectDestination>,
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
            intent_application_facts: Default::default(),
            intent_execution_bindings:
                crate::runtime::intent_execution::UiIntentExecutionBindingPlan::new(),
            change_profile: UiChangeProfileMissing { _sealed: () },
            intent_wiring: UiIntentWiringSatisfied { _sealed: () },
        }
    }

    pub fn with_change_profile(
        self,
        profile: crate::runtime::rebind::UiChangeProfile,
    ) -> WorthUiApplicationBuilder<UiChangeProfileInstalled, UiIntentWiringSatisfied> {
        self.transition_change_profile(UiChangeProfileInstalled { profile })
    }
}

impl<ChangeProfileState, IntentWiringState>
    WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState>
{
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

    fn transition_change_profile<NextProfileState>(
        self,
        change_profile: NextProfileState,
    ) -> WorthUiApplicationBuilder<NextProfileState, IntentWiringState> {
        WorthUiApplicationBuilder {
            inner: self.inner,
            preparation_source: self.preparation_source,
            host_session_plan: self.host_session_plan,
            visual_inspection_policy: self.visual_inspection_policy,
            graph_world_profile: self.graph_world_profile,
            runtime_instance_basis_admissions: self.runtime_instance_basis_admissions,
            measurement_inspection_evidence: self.measurement_inspection_evidence,
            query_binding_plan: self.query_binding_plan,
            intent_application_facts: self.intent_application_facts,
            intent_execution_bindings: self.intent_execution_bindings,
            change_profile,
            intent_wiring: self.intent_wiring,
        }
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

impl<ChangeProfileState> WorthUiApplicationBuilder<ChangeProfileState, UiIntentWiringSatisfied> {
    pub fn freeze_with_registration_report(self) -> CapabilityRegistrationReport {
        self.inner.freeze_with_registration_report()
    }
}

impl WorthUiApplicationBuilder<UiChangeProfileInstalled, UiIntentWiringSatisfied> {
    pub fn freeze(self) -> Result<WorthUiApp, WorthUiApplicationPreparationDenial> {
        let capability_snapshot = self
            .inner
            .freeze_with_registration_report()
            .into_accepted_snapshot();
        let intent_execution_bindings = self
            .intent_execution_bindings
            .freeze(capability_snapshot.intent_definitions())
            .map_err(WorthUiApplicationPreparationDenial::IntentExecutionBinding)?;
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
                intent_application_facts: self.intent_application_facts,
                intent_execution_bindings,
                change_profile: self.change_profile.profile,
            })?,
        ))
    }
}

enum WorthUiApplicationBuilderPreparationSource {
    RustAuthored(worth_ui_dsl::WorthUiRustAuthoredArtifactInput),
    Watched(Box<WorthUiWatchedCandidateSubmission>),
}
