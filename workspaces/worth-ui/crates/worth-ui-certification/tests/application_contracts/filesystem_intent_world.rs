//! Intent-bearing variants of the canonical filesystem-mounted world.

use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_runtime::facade::host::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_runtime::facade::measurement_exchange::UiViewportExtentObservation;
use worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode;

use super::{component_graph_nodes, launch_mounted_components, FilesystemContractWorkspace};

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
    let capabilities = scenario.visual_identity_capability_application_with_intents(
        host.clone(),
        worth_ui::facade::intent::UiIntentDefinition::<DecoyIntent>::application_effect(),
        worth_ui::facade::intent::UiIntentDefinition::<I>::application_effect(),
    );
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        snapshot,
        capabilities.capabilities(),
    );
    let application = scenario.prepare_visual_identity_application_with_intents_and_host(
        submission,
        worth_ui::facade::intent::UiIntentDefinition::<DecoyIntent>::application_effect(),
        worth_ui::facade::intent::UiIntentDefinition::<I>::application_effect(),
        host,
    );
    workspace.close();
    launch_intent_application(application)
}

pub(crate) fn launch_rust_intent_world<I: worth_ui::facade::intent::UiIntent>(
    input: worth_ui_dsl::WorthUiRustAuthoredArtifactInput,
) -> worth_ui::facade::app::WorthUiActiveApplicationSession {
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-3-intent-rust-route");
    let application = scenario.prepare_visual_identity_rust_application_with_intents_and_host(
        input,
        worth_ui::facade::intent::UiIntentDefinition::<DecoyIntent>::application_effect(),
        worth_ui::facade::intent::UiIntentDefinition::<I>::application_effect(),
        intent_host(),
    );
    launch_intent_application(application)
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
