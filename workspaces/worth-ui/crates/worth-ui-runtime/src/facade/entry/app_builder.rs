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
mod freeze;
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
    HostBindingState = UiApplicationHostUnbound,
> {
    inner: CapabilityRegistrationBuilder,
    preparation_source: WorthUiApplicationBuilderPreparationSource,
    host_binding: HostBindingState,
    mounted_frame_retention_budget: crate::mounting::UiMountedFrameRetentionBudget,
    host_observation_capacity: crate::facade::observation_report::UiHostObservationCapacity,
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

/// Compiler-visible posture of an application definition that has not been
/// admitted to a framework-owned host binding.
pub struct UiApplicationHostUnbound {
    _sealed: (),
}

/// Compiler-visible posture of an application definition admitted to exactly
/// one framework-owned host binding.
pub struct UiApplicationHostBound {
    plan: WorthUiHostSessionPlan,
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

#[cfg(test)]
pub(crate) type WorthUiCertificationApplicationBuilder = WorthUiApplicationBuilder<
    UiChangeProfileInstalled,
    UiIntentWiringSatisfied,
    UiApplicationHostBound,
>;

impl
    WorthUiApplicationBuilder<
        UiChangeProfileMissing,
        UiIntentWiringSatisfied,
        UiApplicationHostUnbound,
    >
{
    pub(crate) fn new() -> Self {
        Self {
            inner: CapabilityRegistrationBuilder::new(),
            preparation_source: WorthUiApplicationBuilderPreparationSource::RustAuthored(
                worth_ui_dsl::WorthUiRustAuthoredArtifactInput::default(),
            ),
            host_binding: UiApplicationHostUnbound { _sealed: () },
            mounted_frame_retention_budget: Default::default(),
            host_observation_capacity: Default::default(),
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
}

impl<HostBindingState>
    WorthUiApplicationBuilder<UiChangeProfileMissing, UiIntentWiringSatisfied, HostBindingState>
{
    pub fn with_change_profile(
        self,
        profile: crate::runtime::rebind::UiChangeProfile,
    ) -> WorthUiApplicationBuilder<
        UiChangeProfileInstalled,
        UiIntentWiringSatisfied,
        HostBindingState,
    > {
        self.transition_change_profile(UiChangeProfileInstalled { profile })
    }
}

impl<ChangeProfileState, IntentWiringState, HostBindingState>
    WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState, HostBindingState>
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

    pub fn with_host_observation_capacity(
        mut self,
        capacity: crate::facade::observation_report::UiHostObservationCapacity,
    ) -> Self {
        self.host_observation_capacity = capacity;
        self
    }

    pub fn with_mounted_frame_retention_budget(
        mut self,
        budget: crate::mounting::UiMountedFrameRetentionBudget,
    ) -> Self {
        self.mounted_frame_retention_budget = budget;
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
    ) -> WorthUiApplicationBuilder<NextProfileState, IntentWiringState, HostBindingState> {
        WorthUiApplicationBuilder {
            inner: self.inner,
            preparation_source: self.preparation_source,
            host_binding: self.host_binding,
            mounted_frame_retention_budget: self.mounted_frame_retention_budget,
            host_observation_capacity: self.host_observation_capacity,
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

impl<ChangeProfileState, IntentWiringState>
    WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState, UiApplicationHostUnbound>
{
    #[doc(hidden)]
    pub fn bind_certification_host_adapter<Host>(
        self,
        _grant: worth_ui_host_contract::UiCertificationHostBindingGrant,
        host: Host,
    ) -> WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState, UiApplicationHostBound>
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        self.bind_with_plan(WorthUiHostSessionPlan::prepare(host))
    }

    #[doc(hidden)]
    pub fn bind_legacy_egui_migration_host<Host>(
        self,
        _grant: worth_ui_host_contract::UiLegacyEguiHostBindingGrant,
        host: Host,
    ) -> WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState, UiApplicationHostBound>
    where
        Host: WorthUiOperationalHostAdapter + 'static,
    {
        self.bind_with_plan(WorthUiHostSessionPlan::prepare(host))
    }

    fn bind_with_plan(
        self,
        plan: WorthUiHostSessionPlan,
    ) -> WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState, UiApplicationHostBound>
    {
        WorthUiApplicationBuilder {
            inner: self.inner,
            preparation_source: self.preparation_source,
            host_binding: UiApplicationHostBound { plan },
            mounted_frame_retention_budget: self.mounted_frame_retention_budget,
            host_observation_capacity: self.host_observation_capacity,
            visual_inspection_policy: self.visual_inspection_policy,
            graph_world_profile: self.graph_world_profile,
            runtime_instance_basis_admissions: self.runtime_instance_basis_admissions,
            measurement_inspection_evidence: self.measurement_inspection_evidence,
            query_binding_plan: self.query_binding_plan,
            intent_application_facts: self.intent_application_facts,
            intent_execution_bindings: self.intent_execution_bindings,
            change_profile: self.change_profile,
            intent_wiring: self.intent_wiring,
        }
    }
}

impl<ChangeProfileState, HostBindingState>
    WorthUiApplicationBuilder<ChangeProfileState, UiIntentWiringSatisfied, HostBindingState>
{
    pub fn freeze_with_registration_report(self) -> CapabilityRegistrationReport {
        self.inner.freeze_with_registration_report()
    }
}

enum WorthUiApplicationBuilderPreparationSource {
    RustAuthored(worth_ui_dsl::WorthUiRustAuthoredArtifactInput),
    Watched(Box<WorthUiWatchedCandidateSubmission>),
}
