use std::collections::BTreeSet;
use std::path::Path;

use syn::{Fields, Item, Visibility};

use crate::topology::WorkspaceSourceInventory;

const SOURCE_ROOT: &str = "apps/platform-pulse/src";

pub(super) fn audit(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    audit_exact_source_topology(inventory)?;
    let source = |path: &str| inventory.text(Path::new(SOURCE_ROOT).join(path));
    audit_library_surface(source("lib.rs"))?;
    audit_product_imports_and_features(inventory)?;
    audit_launch_and_application(source("launch_configuration.rs"), source("application.rs"))?;
    audit_protocol(
        source("observation_contract/envelope.rs"),
        source("observation_contract/lifecycle.rs"),
    )?;
    audit_projection_contract(
        source("observation_contract/projection.rs"),
        source("observation_contract/terminal_projection.rs"),
        source("observation_contract/lifecycle.rs"),
    )?;
    audit_publication_bounds(source("lifecycle_observation_publication.rs"))?;
    audit_unchanged_frame(source("native_frame.rs"))?;
    audit_product_entry(source("main.rs"))
}

fn audit_exact_source_topology(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let expected = [
        "application.rs",
        "launch_configuration.rs",
        "lib.rs",
        "lifecycle_observation_publication.rs",
        "main.rs",
        "native_frame.rs",
        "observation_contract/envelope.rs",
        "observation_contract/lifecycle.rs",
        "observation_contract/mod.rs",
        "observation_contract/projection.rs",
        "observation_contract/terminal_projection.rs",
        "source_watch.rs",
    ]
    .into_iter()
    .map(|path| Path::new(SOURCE_ROOT).join(path))
    .collect::<BTreeSet<_>>();
    let observed = inventory
        .rust_files_under(SOURCE_ROOT)
        .map(|source| source.relative_path().to_path_buf())
        .collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(format!(
            "pulse product source topology drifted: {observed:?} != {expected:?}"
        ));
    }
    Ok(())
}

pub(super) fn audit_library_surface(source: &str) -> Result<(), String> {
    let syntax =
        syn::parse_file(source).map_err(|error| format!("pulse library should parse: {error}"))?;
    let only_observation_module = matches!(
        syntax.items.as_slice(),
        [Item::Mod(module)]
            if module.ident == "observation_contract"
                && matches!(module.vis, Visibility::Public(_))
                && module.content.is_none()
    );
    if !only_observation_module {
        return Err("pulse library may export only the lifecycle observation contract".to_owned());
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
    if text.contains("executable_world") || text.contains("cfg(feature") {
        return Err(format!(
            "{} cannot branch on executable-world membership",
            path.display()
        ));
    }
    for forbidden in [
        "worth_ui_runtime",
        "worth_ui_dsl",
        "worth_ui_certification",
        "worth_ui_test_support",
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
        "pub(crate)structAdmittedPlatformPulseLaunchConfiguration{source_root:PathBuf,}",
        "private admitted launch type",
    )?;
    require_contains(
        &launch,
        "Some(option)ifoption==OsStr::new(\"--source-root\")",
        "explicit source-root argument",
    )?;
    require_contains(
        &launch,
        "if!source_root.is_absolute()",
        "absolute source-root admission",
    )?;
    require_contains(
        &launch,
        "letentry_source=source_root.join(\"main.wui\")",
        "canonical entry source",
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
        "lower_to_candidate_submission(capability_app.capabilities())",
        "source lowering",
    )?;
    Ok(())
}

pub(super) fn audit_protocol(envelope: &str, lifecycle: &str) -> Result<(), String> {
    if !envelope.contains("\"worth-ui.platform-pulse.lifecycle-observation\"")
        || !envelope.contains("SCHEMA_VERSION: u16 = 1")
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
        "ReplacementPublished",
        "ReplacementDeniedPreserving",
        "ShutdownCompleted",
        "TerminalFailure",
    ];
    if variants != expected {
        return Err(format!(
            "lifecycle outcome variants drifted: {variants:?} != {expected:?}"
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
                return Err(format!("{} should use named payload fields", item.ident))
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
        "pubfnproject_replacement(&mutself,source:&WorthUiSourcePackageRevision,application:&WorthUiApplicationCutoverReceipt,mounted:&UiMountedFramePublicationReceipt,)",
        "pubfnproject_preserved_predecessor(&mutself,source:&WorthUiSourcePackageRevision,denial:&WorthUiWatchedCandidateSubmissionDenial,)",
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
        "pubfnproject_shutdown(&mutself,watcher:&WorthUiFilesystemWatcherShutdownReceipt,application:WorthUiNativeApplicationShutdownReceipt,)",
    ) {
        return Err("shutdown observation must consume both real shutdown receipts".to_owned());
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
