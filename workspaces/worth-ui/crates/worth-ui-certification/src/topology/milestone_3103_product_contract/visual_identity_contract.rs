use std::path::Path;

use syn::{Fields, FnArg, ImplItem, Item, Type, Visibility};

use crate::topology::WorkspaceSourceInventory;

#[derive(Clone)]
pub(super) struct ProductVisualIdentitySources {
    pub(super) execution: String,
    pub(super) adjudication: String,
    pub(super) publication: String,
    pub(super) wire: String,
    pub(super) projection: String,
    pub(super) native_frame: String,
    pub(super) main: String,
}

pub(super) fn audit(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    audit_sources(&ProductVisualIdentitySources::capture(inventory))
}

pub(super) fn audit_sources(sources: &ProductVisualIdentitySources) -> Result<(), String> {
    audit_execution(&sources.execution)?;
    audit_adjudication(&sources.adjudication)?;
    audit_publication(&sources.publication, &sources.projection)?;
    audit_output_only_wire(&sources.wire)?;
    audit_native_wiring(&sources.native_frame, &sources.main)
}

fn audit_execution(source: &str) -> Result<(), String> {
    for edge in [
        "enum PlatformPulseVisualIdentityState",
        "AwaitingFirstFrame",
        "Capturing(PlatformPulseVisualCapture)",
        "OverlayVisible(PlatformPulseVisibleOverlay)",
        "OverlayCleared(PlatformPulseRetainedSnapshot)",
        "issue_pixel_grant()",
        "begin_visual_pixel_snapshot(&grant, request)",
        "poll_visual_snapshot(capture.pending, *tick)",
        "adjudicate_points(&receipt)",
        "overlay_target(&points.selected_target)",
        "issue_overlay_grant()",
        "show_identity_overlay(&grant, target)",
        "present_visual_overlay(pending, deadline, current)",
        "clear_visual_overlay(overlay.published, deadline, current)",
        "dispose_visual_snapshot(retained.snapshot)",
    ] {
        require(source, edge, "ordinary product visual execution")?;
    }
    for shortcut in [
        "#[cfg(test)]",
        "cfg!(test)",
        "executable_world",
        "egui::",
        "eframe::",
        ".layer_painter(",
        "rect_filled(",
    ] {
        forbid(source, shortcut, "ordinary product visual execution")?;
    }
    Ok(())
}

fn audit_adjudication(source: &str) -> Result<(), String> {
    let source = compact(source);
    for edge in [
        "PLATFORM_PULSE_TARGET_LOGICAL_POINT",
        "PLATFORM_PULSE_BACKGROUND_LOGICAL_POINT",
        "scope.adjudicate_point(point)",
        "target_visible.identity_trace().mounted_node()!=target_node",
        "background_visible.identity_trace().mounted_node()",
        "background_hit.identity_trace().mounted_node()==target_node",
        "!=PLATFORM_PULSE_IDENTITY_TARGET_AUTHORED_NAME",
    ] {
        require(&source, edge, "nondegenerate product point adjudication")?;
    }
    Ok(())
}

fn audit_publication(publication: &str, projection: &str) -> Result<(), String> {
    for edge in [
        "&UiVisualSnapshotReceipt<UiPixelsRequired>",
        "&UiPublishedVisualOverlay",
        "UiClearedVisualOverlayReceipt",
        "UiVisualSnapshotDisposalReceipt",
    ] {
        require(publication, edge, "receipt-derived visual publication")?;
        require(projection, edge, "receipt-derived visual projection")?;
    }
    let compact_publication = compact(publication);
    let compact_projection = compact(projection);
    require_typed_trace_input(
        publication,
        "visual_point_trace",
        "typed visual trace publication",
    )?;
    require_typed_trace_input(
        projection,
        "project_visual_point_trace",
        "typed visual trace projection",
    )?;
    for edge in [
        "pubstructPlatformPulseVisualPointObservation<'a>",
        "point:UiClientPhysicalPixel",
        "adjudication:&'aUiVisualPointAdjudication",
        "pubstructPlatformPulseVisualPointTraceInput<'a>",
        "target:PlatformPulseVisualPointObservation<'a>",
        "background:PlatformPulseVisualPointObservation<'a>",
    ] {
        require(&compact_projection, edge, "typed visual trace projection")?;
    }
    for positional in [
        "target_point:UiClientPhysicalPixel,target:&UiVisualPointAdjudication",
        "background_point:UiClientPhysicalPixel,background:&UiVisualPointAdjudication",
    ] {
        forbid(
            &compact_publication,
            positional,
            "typed visual trace publication",
        )?;
        forbid(
            &compact_projection,
            positional,
            "typed visual trace projection",
        )?;
    }
    for raw_reentry in [
        "UiVisualOverlayTarget::",
        "from_diagnostic",
        "from_wire",
        "reenter",
    ] {
        forbid(publication, raw_reentry, "visual publication")?;
        forbid(projection, raw_reentry, "visual projection")?;
    }
    Ok(())
}

fn require_typed_trace_input(source: &str, name: &str, owner: &str) -> Result<(), String> {
    let syntax =
        syn::parse_file(source).map_err(|error| format!("{owner} should parse: {error}"))?;
    let method = syntax.items.iter().find_map(|item| {
        let Item::Impl(item) = item else {
            return None;
        };
        item.items.iter().find_map(|item| {
            let ImplItem::Fn(method) = item else {
                return None;
            };
            (method.sig.ident == name).then_some(method)
        })
    });
    let method = method.ok_or_else(|| format!("{owner} lost `{name}`"))?;
    let mut inputs = method.sig.inputs.iter();
    if !matches!(inputs.next(), Some(FnArg::Receiver(_))) {
        return Err(format!("{owner} must begin with a receiver"));
    }
    let Some(FnArg::Typed(input)) = inputs.next() else {
        return Err(format!("{owner} lost its typed trace input"));
    };
    let typed_trace = matches!(
        input.ty.as_ref(),
        Type::Path(path)
            if path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "PlatformPulseVisualPointTraceInput")
    );
    if !typed_trace || inputs.next().is_some() {
        return Err(format!(
            "{owner} must accept exactly one PlatformPulseVisualPointTraceInput"
        ));
    }
    Ok(())
}

fn audit_output_only_wire(source: &str) -> Result<(), String> {
    let syntax =
        syn::parse_file(source).map_err(|error| format!("visual wire should parse: {error}"))?;
    for item in &syntax.items {
        let Item::Struct(item) = item else {
            continue;
        };
        if !item.ident.to_string().starts_with("PlatformPulseVisual") {
            continue;
        }
        let Fields::Named(fields) = &item.fields else {
            return Err(format!("{} wire payload must use named fields", item.ident));
        };
        for field in &fields.named {
            let private_to_projection = match &field.vis {
                Visibility::Inherited => true,
                Visibility::Restricted(restricted) => restricted.path.is_ident("super"),
                Visibility::Public(_) => false,
            };
            if !private_to_projection {
                return Err(format!(
                    "{} wire fields must remain private to projection",
                    item.ident
                ));
            }
        }
    }
    for forbidden_payload in ["Vec<u8>", "Box<[u8]>", "RgbaImage", "ColorImage"] {
        forbid(source, forbidden_payload, "bounded visual wire")?;
    }
    for constructor in [
        "pub fn new(",
        "pub(crate) fn new(",
        "from_runtime_projection",
    ] {
        forbid(source, constructor, "output-only visual wire")?;
    }
    Ok(())
}

fn audit_native_wiring(native_frame: &str, main: &str) -> Result<(), String> {
    for edge in [
        "PlatformPulseVisualIdentityExecution",
        ".arm_after_first_frame(",
        ".advance(",
        ".retire_after_replacement(",
        "self.advance_visual_identity();",
    ] {
        require(native_frame, edge, "native product visual wiring")?;
    }
    for module in [
        "mod visual_identity_adjudication;",
        "mod visual_identity_execution;",
        "mod visual_observation_publication;",
    ] {
        require(main, module, "ordinary binary module wiring")?;
    }
    Ok(())
}

fn require(source: &str, edge: &str, owner: &str) -> Result<(), String> {
    source
        .contains(edge)
        .then_some(())
        .ok_or_else(|| format!("{owner} lost required edge `{edge}`"))
}

fn forbid(source: &str, shortcut: &str, owner: &str) -> Result<(), String> {
    (!source.contains(shortcut))
        .then_some(())
        .ok_or_else(|| format!("{owner} reopened shortcut `{shortcut}`"))
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

impl ProductVisualIdentitySources {
    pub(super) fn capture(inventory: &WorkspaceSourceInventory) -> Self {
        let source = |path| {
            inventory
                .text(Path::new("apps/platform-pulse/src").join(path))
                .to_owned()
        };
        Self {
            execution: source("visual_identity_execution.rs"),
            adjudication: source("visual_identity_adjudication.rs"),
            publication: source("visual_observation_publication.rs"),
            wire: source("observation_contract/visual.rs"),
            projection: source("observation_contract/visual_projection.rs"),
            native_frame: source("native_frame.rs"),
            main: source("main.rs"),
        }
    }
}
