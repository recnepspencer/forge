use std::collections::BTreeSet;
use std::path::Path;

use syn::{Fields, Item, Visibility};

use crate::topology::WorkspaceSourceInventory;

const SOURCE_ROOT: &str = "apps/platform-pulse/src";

pub(super) fn audit(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    super::source_topology::audit(inventory)?;
    let source = |path: &str| inventory.text(Path::new(SOURCE_ROOT).join(path));
    audit_library_surface(source("lib.rs"))?;
    audit_product_imports_and_features(inventory)?;
    audit_launch_and_application(source("launch_configuration.rs"), source("application.rs"))?;
    audit_protocol(
        source("observation_contract/envelope.rs"),
        source("observation_contract/lifecycle.rs"),
    )?;
    let live_projection = format!(
        "{}\n{}",
        source("observation_contract/projection.rs"),
        source("observation_contract/projection/replacement_projection.rs"),
    );
    audit_projection_contract(
        &live_projection,
        source("observation_contract/terminal_projection.rs"),
        source("observation_contract/lifecycle.rs"),
    )?;
    audit_publication_bounds(source("lifecycle_observation_publication.rs"))?;
    audit_unchanged_frame(source("native_frame.rs"))?;
    audit_product_entry(source("main.rs"))
}

pub(super) fn audit_library_surface(source: &str) -> Result<(), String> {
    let syntax =
        syn::parse_file(source).map_err(|error| format!("pulse library should parse: {error}"))?;
    let public_modules = syntax
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Mod(module)
                if matches!(module.vis, Visibility::Public(_)) && module.content.is_none() =>
            {
                Some(module.ident.to_string())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        "intent".to_owned(),
        "observation_contract".to_owned(),
        "visual_identity_pulse".to_owned(),
    ]);
    let public_constants = syntax
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Const(item) if matches!(item.vis, Visibility::Public(_)) => {
                Some(item.ident.to_string())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let expected_constants = BTreeSet::from(["PLATFORM_PULSE_STATUS_QUERY_VIEW".to_owned()]);
    let public_item_count = syntax
        .items
        .iter()
        .filter(|item| match item {
            Item::Const(item) => matches!(item.vis, Visibility::Public(_)),
            Item::Mod(item) => matches!(item.vis, Visibility::Public(_)),
            _ => false,
        })
        .count();
    if public_modules != expected
        || public_constants != expected_constants
        || public_item_count != expected.len() + expected_constants.len()
        || !source.contains(
            "pub const PLATFORM_PULSE_STATUS_QUERY_VIEW: &str = \"platform.pulse.status\";",
        )
    {
        return Err(
            "Pulse public modules or constants escaped the adjudicated lifecycle, visual, Query-view, and intent surfaces"
                .to_owned(),
        );
    }
    Ok(())
}

fn audit_product_imports_and_features(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    for source in inventory.rust_files_under(SOURCE_ROOT) {
        audit_source_posture(source.relative_path(), source.text())?;
    }
    Ok(())
}

pub(super) fn audit_source_posture(path: &Path, text: &str) -> Result<(), String> {
    let ungoverned_features = text.replace("#[cfg(feature = \"executable-world\")]", "");
    if ungoverned_features.contains("executable_world")
        || ungoverned_features.contains("cfg(feature")
    {
        return Err(format!(
            "{} cannot branch on ungoverned executable-world membership",
            path.display()
        ));
    }
    for forbidden in [
        "worth_ui_runtime",
        "worth_ui_dsl",
        "worth_ui_certification",
        "worth_ui_test_support",
        "with_certification_worker_event_loop",
        "CertificationWorker",
    ] {
        if text.contains(forbidden) {
            return Err(format!("{} cannot import `{forbidden}`", path.display()));
        }
    }
    for marker in [
        "WORTH_UI_PLATFORM_PULSE_PUBLISHED",
        "WORTH_UI_PLATFORM_PULSE_REPLACED",
    ] {
        if text.contains(marker) {
            return Err(format!(
                "{} retained obsolete string marker `{marker}`",
                path.display()
            ));
        }
    }
    Ok(())
}

fn audit_launch_and_application(launch: &str, application: &str) -> Result<(), String> {
    let launch = compact(launch);
    require_contains(
        &launch,
        "pub(crate)structAdmittedPlatformPulseLaunchConfiguration{source_root:PathBuf,query_source_root:PathBuf,intent_source_root:PathBuf,}",
        "private admitted launch type",
    )?;
    for (edge, owner) in [
        (
            "option==OsStr::new(\"--source-root\")&&source_root.is_none()",
            "explicit source-root argument",
        ),
        (
            "option==OsStr::new(\"--query-source-root\")&&query_source_root.is_none()",
            "explicit Query source-root argument",
        ),
        (
            "option==OsStr::new(\"--intent-source-root\")&&intent_source_root.is_none()",
            "explicit intent source-root argument",
        ),
    ] {
        require_contains(&launch, edge, owner)?;
    }
    if launch.matches("if!root.is_absolute()").count() != 3 {
        return Err("all three launch roots require absolute-path admission".to_owned());
    }
    require_contains(
        &launch,
        "constENTRY_SOURCE:&str=\"main.wui\"",
        "canonical entry source",
    )?;
    require_contains(
        &launch,
        "constINTENT_SOURCE:&str=\"platform-pulse-intent.json\"",
        "canonical intent source",
    )?;
    require_contains(
        &launch,
        "letentry=root.join(ENTRY_SOURCE)",
        "entry admission",
    )?;
    require_contains(
        &launch,
        "letinput=root.join(INTENT_SOURCE)",
        "intent admission",
    )?;
    let application = compact(application);
    require_contains(
        &application,
        "pub(crate)fnprepare(context:egui::Context,launch:&AdmittedPlatformPulseLaunchConfiguration,)",
        "typed application preparation",
    )?;
    require_contains(
        &application,
        "WorthUiFilesystemSourceProvider::new(launch.source_root())",
        "production filesystem provider",
    )?;
    require_contains(
        &application,
        "WorthUiFilesystemSourceWatcher::start(provider)",
        "production filesystem watcher",
    )?;
    require_contains(
        &application,
        "crate::query_source::install(launch.query_source_root())",
        "production Query source installation",
    )?;
    require_contains(
        &application,
        "PlatformPulseIntentInputInstallation::open(launch.intent_source_root())",
        "production intent source installation",
    )?;
    require_contains(
        &application,
        "attempt_source_rebind(capability_app.capabilities())",
        "source lowering",
    )?;
    Ok(())
}

pub(super) fn audit_protocol(envelope: &str, lifecycle: &str) -> Result<(), String> {
    let current = envelope
        .lines()
        .find(|line| line.contains("LIFECYCLE_OBSERVATION_SCHEMA_VERSION: u16 ="))
        .and_then(|line| line.rsplit_once('='))
        .and_then(|(_, value)| value.trim().trim_end_matches(';').parse::<u16>().ok())
        .ok_or_else(|| "lifecycle schema version should remain explicit".to_owned())?;
    if !envelope.contains("\"worth-ui.platform-pulse.lifecycle-observation\"")
        || current < 5
        || !envelope.contains(&format!("CompleteV{current}"))
        || !envelope.contains(&format!("schema_version @ 2..={}", current - 1))
        || !envelope.contains("\"WORTH_UI_PLATFORM_PULSE_EVENT \"")
        || !envelope.contains("MAXIMUM_ENCODED_OBSERVATION_BYTES: usize = 1_048_576")
    {
        return Err("lifecycle observation protocol identity or bounds drifted".to_owned());
    }
    let syntax = syn::parse_file(lifecycle)
        .map_err(|error| format!("lifecycle source should parse: {error}"))?;
    let variants = syntax
        .items
        .iter()
        .find_map(|item| match item {
            Item::Enum(item) if item.ident == "PlatformPulseLifecycleObservation" => Some(
                item.variants
                    .iter()
                    .map(|variant| variant.ident.to_string())
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .ok_or_else(|| "lifecycle outcome enum should exist".to_owned())?;
    let expected = [
        "ProcessStarted",
        "FirstFramePublished",
        "NativeInputReached",
        "QueryProjectionIssued",
        "QueryProjectionPublished",
        "VisualSnapshotCaptured",
        "VisualPointTrace",
        "VisualOverlayPublished",
        "VisualOverlayCleared",
        "VisualSnapshotRetired",
        "RebindPublished",
        "RebindDeniedPreserving",
        "VisualComparison",
        "ShutdownCompleted",
        "TerminalFailure",
    ];
    let mut required_index = 0;
    for variant in &variants {
        if expected
            .get(required_index)
            .is_some_and(|required| variant == required)
        {
            required_index += 1;
        }
    }
    if required_index != expected.len() {
        return Err(format!(
            "lifecycle outcomes lost or reordered a required 3.10.3 variant: {variants:?}"
        ));
    }
    audit_payload_privacy(&syntax)
}

fn audit_payload_privacy(syntax: &syn::File) -> Result<(), String> {
    for item in &syntax.items {
        let Item::Struct(item) = item else {
            continue;
        };
        if !item.ident.to_string().starts_with("PlatformPulse") {
            continue;
        }
        let fields = match &item.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unit => continue,
            Fields::Unnamed(_) => {
                return Err(format!("{} should use named payload fields", item.ident));
            }
        };
        for field in fields {
            let private_to_projection = match &field.vis {
                Visibility::Inherited => true,
                Visibility::Restricted(restricted) => restricted.path.is_ident("super"),
                Visibility::Public(_) => false,
            };
            if !private_to_projection {
                return Err(format!(
                    "{} payload fields must remain private to observation projection",
                    item.ident
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn audit_projection_contract(
    live: &str,
    terminal: &str,
    lifecycle: &str,
) -> Result<(), String> {
    let live = compact(live);
    for required in [
        "pubfnproject_first_frame(&mutself,source:&WorthUiSourcePackageRevision,publication:&UiMountedFramePublicationReceipt,)",
        "pubfnproject_replacement(&mutself,source:&WorthUiSourcePackageRevision,receipt:&UiRebindReceipt,)",
        "pubfnproject_preserved_predecessor(&mutself,source:&WorthUiSourcePackageRevision,denial:&UiSourceRebindAttemptFailure,)",
        "actual_native_effect_count:publication.cost_report().adapter().translated_rows()",
        "actual_native_effect_count:mounted.cost_report().adapter().translated_rows()",
    ] {
        if !live.contains(required) {
            return Err(format!(
                "receipt-derived live observation contract is missing `{required}`"
            ));
        }
    }
    let terminal = compact(terminal);
    if !terminal.contains(
        "pubfnproject_shutdown(&mutself,watcher:&WorthUiFilesystemWatcherShutdownReceipt,query:super::query::PlatformPulseQueryShutdownEvidence,intent:super::intent::PlatformPulseIntentWatcherShutdownEvidence,application:WorthUiNativeApplicationShutdownReceipt,)",
    ) || !terminal.contains("intent_watcher_joined:intent.worker_joined()")
        || !terminal.contains("pending_intent_input_count:intent.pending_input_count()")
    {
        return Err("shutdown observation must consume the complete real shutdown receipt set".to_owned());
    }
    if lifecycle.contains("pub fn new(") || lifecycle.contains("pub(crate) fn new(") {
        return Err("observation payload constructors cannot be public or crate-public".to_owned());
    }
    Ok(())
}

fn audit_publication_bounds(source: &str) -> Result<(), String> {
    let source = compact(source);
    for required in [
        "constMAXIMUM_EVENTS:usize=256;",
        "constMAXIMUM_ENCODED_BYTES:usize=1_048_576;",
        "ifevent_count>MAXIMUM_EVENTS",
        "iftotal_bytes>MAXIMUM_ENCODED_BYTES",
        "writeln!(stdout,\"{line}\")",
        "stdout.flush()",
    ] {
        if !source.contains(required) {
            return Err(format!(
                "bounded stdout observation publication is missing `{required}`"
            ));
        }
    }
    Ok(())
}

pub(super) fn audit_unchanged_frame(source: &str) -> Result<(), String> {
    let source = compact(source);
    if !source
        .contains("Ok(UiMountedFrameOutcome::Unchanged(_))ifself.initial_source.is_none()=>{}")
    {
        return Err("ordinary unchanged frames must perform zero observation work".to_owned());
    }
    Ok(())
}

fn audit_product_entry(source: &str) -> Result<(), String> {
    let source = compact(source);
    if !source.contains("AdmittedPlatformPulseLaunchConfiguration::from_process()")
        || !source.contains("eframe::run_native(")
        || !source.contains("PlatformPulseNativeFrame::new(creation,launch,frame_publisher,)")
    {
        return Err(
            "canonical binary entry no longer uses admitted launch and native frame".to_owned(),
        );
    }
    Ok(())
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn require_contains(source: &str, expected: &str, edge: &str) -> Result<(), String> {
    if source.contains(expected) {
        Ok(())
    } else {
        Err(format!(
            "admitted source-root product path lost its `{edge}` edge"
        ))
    }
}
