use crate::scenario::application_authority_closure::fixed_host::FixedCertificationHostBinding;
use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::intent::{UiIntent, UiIntentDefinition};
use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;

type BoundApplicationBuilder =
    crate::scenario::application_authority_closure::FixedCertificationApplicationBuilder;

use super::FilesystemApplicationLifecycleScenario;
use crate::scenario::application_authority_closure::visual_identity_application::{
    clipped_semantic_text_action_application_builder_with_host,
    clipped_semantic_text_action_application_builder_with_host_and_profile,
    clipped_visual_identity_application_builder_with_host,
    duplicate_hit_order_application_builder_with_host,
    focusable_semantic_text_action_application_builder_with_host,
    portal_semantic_text_action_application_builder_with_host,
    region_identity_application_builder_with_host,
    single_semantic_text_application_builder_with_host,
    visual_identity_application_builder_with_host, PHASE5_CANCELLATION_BACKGROUND,
    PHASE5_CANCELLATION_COLOR_TOKEN, PHASE5_CANCELLATION_COMPONENT, PHASE5_CANCELLATION_SURFACE,
    PHASE5_CANCELLATION_TOKEN, VISUAL_HIT_ONLY_COMPONENT, VISUAL_IDENTITY_SURFACE,
    VISUAL_NEITHER_COMPONENT, VISUAL_PAINT_AND_HIT_COMPONENT, VISUAL_PAINT_AND_HIT_TOKEN,
    VISUAL_PAINT_ONLY_COMPONENT, VISUAL_PAINT_ONLY_TOKEN, VISUAL_PURPLE_TOKEN, VISUAL_RED_TOKEN,
};

impl FilesystemApplicationLifecycleScenario {
    pub fn phase5_cancellation_source_text() -> String {
        format!(
            "component {PHASE5_CANCELLATION_BACKGROUND} {{}}\n\
             component {PHASE5_CANCELLATION_COMPONENT} {{}}\n\
             surface {PHASE5_CANCELLATION_SURFACE} {{}}\n\
             token {PHASE5_CANCELLATION_TOKEN} = \"{PHASE5_CANCELLATION_COLOR_TOKEN}\";\n"
        )
    }

    pub fn phase5_cancellation_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        single_semantic_text_application_builder_with_host(host)
            .freeze()
            .expect("Phase 5 cancellation application freezes")
    }

    pub fn prepare_phase5_cancellation_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        single_semantic_text_application_builder_with_host(host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored Phase 5 cancellation application freezes")
    }

    /// Start the canonical visual-identity world compiler while allowing a
    /// scenario to add the exact capability owners its claim requires.
    pub fn visual_identity_application_builder<Host>(&self, host: Host) -> BoundApplicationBuilder
    where
        Host: FixedCertificationHostBinding,
    {
        visual_identity_application_builder_with_host(host)
    }

    /// Start the visual-identity compiler with a clipped hit-only region so
    /// the separately allocated paint-and-hit control is independently targetable.
    pub fn clipped_visual_identity_application_builder<Host>(
        &self,
        host: Host,
    ) -> BoundApplicationBuilder
    where
        Host: FixedCertificationHostBinding,
    {
        clipped_visual_identity_application_builder_with_host(host)
    }

    /// Start a clipped visual world whose action control carries the existing
    /// host-neutral BodyDefault semantic-text contract.
    pub fn semantic_text_action_application_builder<Host>(
        &self,
        host: Host,
    ) -> BoundApplicationBuilder
    where
        Host: FixedCertificationHostBinding,
    {
        clipped_semantic_text_action_application_builder_with_host(host)
    }

    pub fn focusable_semantic_text_action_application_builder<Host>(
        &self,
        host: Host,
    ) -> BoundApplicationBuilder
    where
        Host: FixedCertificationHostBinding,
    {
        focusable_semantic_text_action_application_builder_with_host(host)
    }

    pub fn portal_semantic_text_action_application_builder<Host>(
        &self,
        host: Host,
    ) -> BoundApplicationBuilder
    where
        Host: FixedCertificationHostBinding,
    {
        portal_semantic_text_action_application_builder_with_host(host)
    }

    pub fn semantic_text_action_application_builder_with_change_profile<Host>(
        &self,
        host: Host,
        profile: worth_ui::facade::rebind::UiChangeProfile,
    ) -> BoundApplicationBuilder
    where
        Host: FixedCertificationHostBinding,
    {
        clipped_semantic_text_action_application_builder_with_host_and_profile(host, profile)
    }

    pub fn visual_identity_source_text() -> String {
        format!(
            "component {VISUAL_PAINT_ONLY_COMPONENT} {{}}\n\
             component {VISUAL_HIT_ONLY_COMPONENT} {{}}\n\
             component {VISUAL_PAINT_AND_HIT_COMPONENT} {{}}\n\
             component {VISUAL_NEITHER_COMPONENT} {{}}\n\
             surface {VISUAL_IDENTITY_SURFACE} {{}}\n\
             token {VISUAL_PAINT_ONLY_TOKEN} = \"{VISUAL_RED_TOKEN}\";\n\
             token {VISUAL_PAINT_AND_HIT_TOKEN} = \"{VISUAL_PURPLE_TOKEN}\";\n"
        )
    }

    pub fn visual_identity_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        visual_identity_application_builder_with_host(host)
            .freeze()
            .expect("visual identity capabilities should prepare")
    }

    pub fn visual_identity_capability_application_with_intent<Host, I>(
        &self,
        host: Host,
        definition: UiIntentDefinition<I>,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
        I: UiIntent,
    {
        visual_identity_application_builder_with_host(host)
            .register_intent_definition(definition)
            .expect("typed intent definition should register")
            .register_intent_provider(crate::WorthUiCertificationBeforeEffectProvider::<I>::new())
            .expect("typed certification provider should register")
            .freeze()
            .expect("visual identity and intent capabilities should prepare")
    }

    pub fn visual_identity_capability_application_with_intents<Host, I, J>(
        &self,
        host: Host,
        first: UiIntentDefinition<I>,
        second: UiIntentDefinition<J>,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
        I: UiIntent,
        J: UiIntent,
    {
        visual_identity_application_builder_with_host(host)
            .register_intent_definition(first)
            .expect("first typed intent definition should register")
            .register_intent_provider(crate::WorthUiCertificationBeforeEffectProvider::<I>::new())
            .expect("first typed certification provider should register")
            .register_intent_definition(second)
            .expect("second typed intent definition should register")
            .register_intent_provider(crate::WorthUiCertificationBeforeEffectProvider::<J>::new())
            .expect("second typed certification provider should register")
            .freeze()
            .expect("visual identity and intent capabilities should prepare")
    }

    pub fn duplicate_hit_order_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        duplicate_hit_order_application_builder_with_host(host)
            .freeze()
            .expect("duplicate-order capabilities should prepare")
    }

    pub fn region_identity_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        region_identity_application_builder_with_host(host)
            .freeze()
            .expect("region identity capabilities should prepare")
    }

    pub fn clipped_visual_identity_capability_application<Host>(&self, host: Host) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        clipped_visual_identity_application_builder_with_host(host)
            .freeze()
            .expect("clipped visual identity capabilities should prepare")
    }

    pub fn prepare_visual_identity_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        visual_identity_application_builder_with_host(host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored visual identity application should prepare")
    }

    pub fn prepare_visual_identity_application_with_intent_and_host<Host, I>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        definition: UiIntentDefinition<I>,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
        I: UiIntent,
    {
        visual_identity_application_builder_with_host(host)
            .register_intent_definition(definition)
            .expect("typed intent definition should register")
            .register_intent_provider(crate::WorthUiCertificationBeforeEffectProvider::<I>::new())
            .expect("typed certification provider should register")
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored routed application should prepare")
    }

    pub fn prepare_visual_identity_application_with_intents_and_host<Host, I, J>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        first: UiIntentDefinition<I>,
        second: UiIntentDefinition<J>,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
        I: UiIntent,
        J: UiIntent,
    {
        visual_identity_application_builder_with_host(host)
            .register_intent_definition(first)
            .expect("first typed intent definition should register")
            .register_intent_provider(crate::WorthUiCertificationBeforeEffectProvider::<I>::new())
            .expect("first typed certification provider should register")
            .register_intent_definition(second)
            .expect("second typed intent definition should register")
            .register_intent_provider(crate::WorthUiCertificationBeforeEffectProvider::<J>::new())
            .expect("second typed certification provider should register")
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored routed application should prepare")
    }

    pub fn prepare_visual_identity_rust_application_with_intent_and_host<Host, I>(
        &self,
        input: worth_ui_dsl::WorthUiRustAuthoredArtifactInput,
        definition: UiIntentDefinition<I>,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
        I: UiIntent,
    {
        visual_identity_application_builder_with_host(host)
            .register_intent_definition(definition)
            .expect("typed intent definition should register")
            .register_intent_provider(crate::WorthUiCertificationBeforeEffectProvider::<I>::new())
            .expect("typed certification provider should register")
            .with_rust_authored_input(input)
            .freeze()
            .expect("Rust-authored routed application should prepare")
    }

    pub fn prepare_visual_identity_rust_application_with_intents_and_host<Host, I, J>(
        &self,
        input: worth_ui_dsl::WorthUiRustAuthoredArtifactInput,
        first: UiIntentDefinition<I>,
        second: UiIntentDefinition<J>,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
        I: UiIntent,
        J: UiIntent,
    {
        visual_identity_application_builder_with_host(host)
            .register_intent_definition(first)
            .expect("first typed intent definition should register")
            .register_intent_provider(crate::WorthUiCertificationBeforeEffectProvider::<I>::new())
            .expect("first typed certification provider should register")
            .register_intent_definition(second)
            .expect("second typed intent definition should register")
            .register_intent_provider(crate::WorthUiCertificationBeforeEffectProvider::<J>::new())
            .expect("second typed certification provider should register")
            .with_rust_authored_input(input)
            .freeze()
            .expect("Rust-authored routed application should prepare")
    }

    pub fn prepare_visual_identity_application_with_policy_and_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        policy: worth_ui::facade::inspection::UiVisualInspectionPolicy,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        visual_identity_application_builder_with_host(host)
            .with_visual_inspection_policy(policy)
            .with_candidate_submission(submission)
            .freeze()
            .expect("policy-bounded filesystem visual identity application should prepare")
    }

    pub fn prepare_region_identity_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        region_identity_application_builder_with_host(host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored region identity application should prepare")
    }

    pub fn prepare_clipped_visual_identity_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        clipped_visual_identity_application_builder_with_host(host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("filesystem-authored clipped visual identity application should prepare")
    }

    pub fn prepare_region_identity_application_with_policy_and_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        policy: worth_ui::facade::inspection::UiVisualInspectionPolicy,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        region_identity_application_builder_with_host(host)
            .with_visual_inspection_policy(policy)
            .with_candidate_submission(submission)
            .freeze()
            .expect("policy-bounded filesystem region identity application should prepare")
    }

    pub fn prepare_duplicate_hit_order_application_with_host<Host>(
        &self,
        submission: WorthUiWatchedCandidateSubmission,
        host: Host,
    ) -> WorthUiApp
    where
        Host: FixedCertificationHostBinding,
    {
        duplicate_hit_order_application_builder_with_host(host)
            .with_candidate_submission(submission)
            .freeze()
            .expect("duplicate-order world should prepare before mounted projection")
    }
}
