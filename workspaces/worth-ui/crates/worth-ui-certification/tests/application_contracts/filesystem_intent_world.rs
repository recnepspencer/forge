//! Intent-bearing variants of the canonical filesystem-mounted world.

use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_headless::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_runtime::facade::measurement_exchange::UiViewportExtentObservation;
use worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode;

use super::{component_graph_nodes, launch_mounted_components, FilesystemContractWorkspace};

pub(crate) const INTENT_WORLD_OPERABILITY_FACT: &str = "phase3.intent.world.operable";

struct DecoyPayload;

impl worth_ui::facade::intent::UiIntentPayload for DecoyPayload {
    const SCHEMA: worth_ui::facade::intent::UiIntentSchema =
        worth_ui::facade::intent::UiIntentSchema::stable("aaa.phase3.decoy.payload", 1);
    const FIELDS: worth_ui::facade::intent::UiIntentPayloadFieldSet =
        worth_ui::facade::intent::UiIntentPayloadFieldSet::EMPTY;

    fn project(
        _fields: &mut worth_ui::facade::intent::UiIntentPayloadProjection<Self>,
    ) -> Result<Self, worth_ui::facade::intent::UiIntentPayloadProjectionViolation> {
        Ok(Self)
    }
}

struct DecoyOutcome;

impl worth_ui::facade::intent::UiIntentProductOutcome for DecoyOutcome {
    const SCHEMA: worth_ui::facade::intent::UiIntentSchema =
        worth_ui::facade::intent::UiIntentSchema::stable("aaa.phase3.decoy.outcome", 1);
    const CONSEQUENCE_FAMILIES: worth_ui::facade::intent::UiIntentProductConsequenceFamilies =
        worth_ui::facade::intent::UiIntentProductConsequenceFamilies::NONE;

    fn into_consequences(self) -> worth_ui::facade::intent::UiIntentProductConsequences {
        worth_ui::facade::intent::UiIntentProductConsequences::none()
    }
}

struct DecoyIntent;

impl worth_ui::facade::intent::UiIntent for DecoyIntent {
    type Payload = DecoyPayload;
    type ProductOutcome = DecoyOutcome;

    const ID: worth_ui::facade::intent::UiIntentId =
        worth_ui::facade::intent::UiIntentId::stable("aaa.phase3.decoy");
    const ACCEPTED_INTERACTIONS: worth_ui::facade::intent::UiIntentAcceptedInteractions =
        worth_ui::facade::intent::UiIntentAcceptedInteractions::new(&[
            worth_ui::facade::intent::UiSemanticInteractionFamily::Activate,
        ]);
}

pub(crate) fn launch_file_intent_world<I: worth_ui::facade::intent::UiIntent>(
    source: &str,
) -> worth_ui::facade::app::WorthUiActiveApplicationSession {
    let responsibility = "phase-3-intent-file-route";
    let host = intent_host();
    let scenario = FilesystemApplicationLifecycleScenario::new(responsibility);
    let workspace = FilesystemContractWorkspace::new(responsibility);
    workspace.write("app/main.wui", source);
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production filesystem provider reads routed source");
    let capabilities = scenario
        .visual_identity_application_builder(host.clone())
        .register_intent_boolean_fact(intent_world_operability_fact(), true)
        .expect("intent-world operability fact registers")
        .register_intent_definition(
            worth_ui::facade::intent::UiIntentDefinition::<DecoyIntent>::application_effect(),
        )
        .expect("decoy intent definition registers")
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<DecoyIntent>::new(),
        )
        .expect("decoy intent provider registers")
        .register_intent_definition(
            worth_ui::facade::intent::UiIntentDefinition::<I>::application_effect(),
        )
        .expect("scenario intent definition registers")
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<I>::new(),
        )
        .expect("scenario intent provider registers")
        .freeze()
        .expect("intent-world capabilities prepare");
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        snapshot,
        capabilities.capabilities(),
    );
    let application = scenario
        .visual_identity_application_builder(host)
        .register_intent_boolean_fact(intent_world_operability_fact(), true)
        .expect("intent-world operability fact registers")
        .register_intent_definition(
            worth_ui::facade::intent::UiIntentDefinition::<DecoyIntent>::application_effect(),
        )
        .expect("decoy intent definition registers")
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<DecoyIntent>::new(),
        )
        .expect("decoy intent provider registers")
        .register_intent_definition(
            worth_ui::facade::intent::UiIntentDefinition::<I>::application_effect(),
        )
        .expect("scenario intent definition registers")
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<I>::new(),
        )
        .expect("scenario intent provider registers")
        .with_candidate_submission(submission)
        .freeze()
        .expect("filesystem-authored intent world prepares");
    workspace.close();
    launch_intent_application(application)
}

pub(crate) fn launch_rust_intent_world<I: worth_ui::facade::intent::UiIntent>(
    input: worth_ui_dsl::WorthUiRustAuthoredArtifactInput,
) -> worth_ui::facade::app::WorthUiActiveApplicationSession {
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-3-intent-rust-route");
    let application = scenario
        .visual_identity_application_builder(intent_host())
        .register_intent_boolean_fact(intent_world_operability_fact(), true)
        .expect("intent-world operability fact registers")
        .register_intent_definition(
            worth_ui::facade::intent::UiIntentDefinition::<DecoyIntent>::application_effect(),
        )
        .expect("decoy intent definition registers")
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<DecoyIntent>::new(),
        )
        .expect("decoy intent provider registers")
        .register_intent_definition(
            worth_ui::facade::intent::UiIntentDefinition::<I>::application_effect(),
        )
        .expect("scenario intent definition registers")
        .register_intent_provider(
            worth_ui_certification::WorthUiCertificationBeforeEffectProvider::<I>::new(),
        )
        .expect("scenario intent provider registers")
        .with_rust_authored_input(input)
        .freeze()
        .expect("Rust-authored intent world prepares");
    launch_intent_application(application)
}

pub(crate) fn launch_rust_command_intent_world<I: worth_ui::facade::intent::UiIntent>(
    input: worth_ui_dsl::WorthUiRustAuthoredArtifactInput,
    command: worth_ui::facade::declaration::CommandDescriptor,
) -> worth_ui::facade::app::WorthUiActiveApplicationSession {
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-6-command-intent-route");
    let application = scenario
        .visual_identity_application_builder(intent_host())
        .register_intent_boolean_fact(intent_world_operability_fact(), true)
        .expect("command intent operability fact registers")
        .register_runtime_service_intent_definition(
            worth_ui::facade::intent::UiIntentDefinition::<I>::runtime_service(
                worth_ui::facade::intent::UiIntentRuntimeServiceDestination::InvokeCommand,
            ),
        )
        .expect("command runtime-service definition registers")
        .register_command(command)
        .with_rust_authored_input(input)
        .freeze()
        .expect("Rust-authored command intent world prepares");
    launch_intent_application(application)
}

pub(crate) fn prepare_rust_command_intent_application<I: worth_ui::facade::intent::UiIntent>(
    input: worth_ui_dsl::WorthUiRustAuthoredArtifactInput,
    command: worth_ui::facade::declaration::CommandDescriptor,
    host: worth_ui_runtime::certification_support::ScriptedPresentationHost,
) -> worth_ui::facade::app::WorthUiApp {
    prepare_rust_command_intent_application_with_policy::<I>(
        input,
        command,
        host,
        worth_ui::facade::service::UiCommandRoutingPolicy::desktop(),
    )
}

pub(crate) fn prepare_rust_command_intent_application_with_policy<
    I: worth_ui::facade::intent::UiIntent,
>(
    input: worth_ui_dsl::WorthUiRustAuthoredArtifactInput,
    command: worth_ui::facade::declaration::CommandDescriptor,
    host: worth_ui_runtime::certification_support::ScriptedPresentationHost,
    policy: worth_ui::facade::service::UiCommandRoutingPolicy,
) -> worth_ui::facade::app::WorthUiApp {
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-6-native-command-route");
    scenario
        .visual_identity_application_builder(host)
        .with_command_routing_policy_defaults(policy)
        .register_intent_boolean_fact(intent_world_operability_fact(), true)
        .expect("command intent operability fact registers")
        .register_runtime_service_intent_definition(
            worth_ui::facade::intent::UiIntentDefinition::<I>::runtime_service(
                worth_ui::facade::intent::UiIntentRuntimeServiceDestination::InvokeCommand,
            ),
        )
        .expect("command runtime-service definition registers")
        .register_command(command)
        .with_rust_authored_input(input)
        .freeze()
        .expect("Rust-authored native command intent world prepares")
}

pub(crate) fn intent_world_operability_fact(
) -> worth_ui::facade::intent::UiIntentApplicationFact<worth_ui::facade::intent::UiIntentBoolean> {
    worth_ui::facade::intent::UiIntentApplicationFact::boolean(INTENT_WORLD_OPERABILITY_FACT)
        .expect("intent-world operability fact identity is valid")
}

fn intent_host() -> WorthUiHeadlessRecorder {
    WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        UiViewportExtentObservation {
            width: 160.0,
            height: 96.0,
        },
    )
}

fn launch_intent_application(
    application: worth_ui::facade::app::WorthUiApp,
) -> worth_ui::facade::app::WorthUiActiveApplicationSession {
    let component_nodes = component_graph_nodes(&application);
    launch_mounted_components(
        application,
        component_nodes,
        UiHostSurfacePresentationMode::RecordOnly,
    )
}
