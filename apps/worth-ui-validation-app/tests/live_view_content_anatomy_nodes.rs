use worth_ui::facade::{
    WorthUiCompositionNodeKind, WorthUiLiveViewProjectionAdmissionDenial,
    WorthUiLiveViewProjectionAdmissionReceipt, WorthUiMountedNodeReceipt,
    WorthUiPrimitiveContentItemKind, WorthUiPrimitiveContentReceipt, WorthUiPrimitiveContentRole,
    WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::{
    ValidationLiveViewSource, VALIDATION_SAMPLE_LIVE_VIEW_SOURCE,
};
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

#[test]
fn authored_content_block_mounts_primitive_content_receipt() {
    let app = prepared_app_with_live_view_source(source_with_submit_content("Submit"));
    let proof = app
        .live_view_projection_proof()
        .expect("content anatomy should admit");
    let content = proof
        .projection()
        .content_receipts()
        .iter()
        .find(|receipt| receipt.dependency_fact().identity() == "submit_label")
        .expect("projection carries admitted content receipt");
    assert_eq!(content.text(), "Submit");
    assert!(content
        .items()
        .iter()
        .any(|item| item.kind() == WorthUiPrimitiveContentItemKind::Icon));
    assert_eq!(
        content.dependency_fact().family(),
        WorthUiRuntimeFactFamily::PrimitiveContent
    );

    let action_children = proof
        .mounted_product_view()
        .composition_tree()
        .ordered_children("action_row");
    assert!(action_children.iter().any(|child| {
        child.composition_node().kind() == WorthUiCompositionNodeKind::Content
            && matches!(child.mounted_node(), WorthUiMountedNodeReceipt::Content(node)
                if node.content().receipt_digest() == content.receipt_digest())
    }));
}

#[test]
fn invalid_content_icon_joins_projection_denials() {
    let app = prepared_app_with_live_view_source(
        source_with_submit_content("Submit")
            .replace("worth.icon.action.check", "worth.icon.action.missing"),
    );
    let report = app
        .live_view_projection_proof_typed()
        .expect_err("unknown content icon must deny projection admission");
    assert!(report.denials().iter().any(|denial| matches!(
        denial,
        WorthUiLiveViewProjectionAdmissionDenial::PrimitiveContent(content)
            if content.prop_key() == "content_icon"
                && content.raw_value() == "worth.icon.action.missing"
                && content.source_span().is_some()
    )));
    assert_ne!(report.denial_set_digest(), 0);
}

#[test]
fn content_text_hot_reload_preserves_composition_graph_identity() {
    let mut app = prepared_app_with_live_view_source(source_with_submit_content("Submit"));
    let first = app
        .live_view_projection_proof()
        .expect("initial content source admits");
    let first_graph = first.mounted_product_view().composition_graph_digest();
    let first_content = content_receipt(first.projection(), "submit_label")
        .expect("initial content receipt")
        .receipt_digest();

    let next = app
        .hot_reload_live_view_source(source_with_submit_content("Send it"))
        .expect("content text edit should hot reload");
    let next_content = content_receipt(next.projection(), "submit_label")
        .expect("next content receipt")
        .receipt_digest();
    assert_eq!(
        first_graph,
        next.mounted_product_view().composition_graph_digest(),
        "content prop edits are content facts, not topology edits"
    );
    assert_ne!(first_content, next_content);
}

#[test]
fn content_role_hot_reload_preserves_composition_graph_identity() {
    let mut app = prepared_app_with_live_view_source(source_with_helper_content("helper_text"));
    let first = app
        .live_view_projection_proof()
        .expect("initial role source admits");
    let first_graph = first.mounted_product_view().composition_graph_digest();
    assert_eq!(
        content_receipt(first.projection(), "submit_label")
            .expect("initial content receipt")
            .role(),
        WorthUiPrimitiveContentRole::HelperText
    );

    let next = app
        .hot_reload_live_view_source(source_with_helper_content("error_text"))
        .expect("content role edit should hot reload");
    assert_eq!(
        first_graph,
        next.mounted_product_view().composition_graph_digest(),
        "content role edits are content facts, not topology edits"
    );
    assert_eq!(
        content_receipt(next.projection(), "submit_label")
            .expect("next content receipt")
            .role(),
        WorthUiPrimitiveContentRole::ErrorText
    );
}

#[test]
fn content_image_mounts_as_local_static_content_item() {
    let app = prepared_app_with_live_view_source(source_with_image_content());
    let proof = app
        .live_view_projection_proof()
        .expect("local static image content should admit");
    let content = content_receipt(proof.projection(), "submit_label").expect("content receipt");
    let image = content
        .items()
        .iter()
        .find_map(|item| item.as_image())
        .expect("content contains image item");

    assert_eq!(image.asset_id(), "worth.image.logo");
    assert_eq!(image.source_kind(), "local_static");
    assert_eq!(image.width_points(), 64.0);
    assert_eq!(image.height_points(), 40.0);
}

#[test]
fn invalid_content_role_and_image_join_projection_denials() {
    let app = prepared_app_with_live_view_source(
        source_with_image_content()
            .replace("worth.image.logo", "worth.image.missing")
            .replace("content_role body", "content_role headline"),
    );
    let report = app
        .live_view_projection_proof_typed()
        .expect_err("invalid content role and image must deny projection admission");

    assert!(report.denials().iter().any(|denial| matches!(
        denial,
        WorthUiLiveViewProjectionAdmissionDenial::PrimitiveContent(content)
            if content.prop_key() == "content_role"
                && content.raw_value() == "headline"
                && content.source_span().is_some()
    )));
    assert!(report.denials().iter().any(|denial| matches!(
        denial,
        WorthUiLiveViewProjectionAdmissionDenial::PrimitiveContent(content)
            if content.prop_key() == "content_image"
                && content.raw_value() == "worth.image.missing"
                && content.source_span().is_some()
    )));
}

fn prepared_app_with_live_view_source(
    source: impl Into<String>,
) -> worth_ui_validation_app::ValidationWorkbenchApp {
    let authored_inputs = ValidationWorkbenchAuthoredInputs::sample()
        .with_live_view_source(ValidationLiveViewSource::new(source.into()));
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(authored_inputs)
        .expect("validation app should prepare");
    worth_ui_validation_app::ValidationWorkbenchApp::new(launch)
}

fn content_receipt<'a>(
    projection: &'a WorthUiLiveViewProjectionAdmissionReceipt,
    subject_id: &str,
) -> Option<&'a WorthUiPrimitiveContentReceipt> {
    projection
        .content_receipts()
        .iter()
        .find(|receipt| receipt.dependency_fact().identity() == subject_id)
}

fn source_with_submit_content(label: &str) -> String {
    VALIDATION_SAMPLE_LIVE_VIEW_SOURCE.replace(
        "child interaction proof_submit sizing hug",
        &format!(
            "content submit_label {{\n                    content_kind inline\n                    content_order \"icon,text\"\n                    content_icon worth.icon.action.check\n                    content_text \"{label}\"\n                    content_icon_size validation.density.primitive.content.icon.default\n                    content_icon_stroke validation.density.primitive.content.icon.stroke.default\n                    content_text_size validation.density.primitive.content.text.default\n                }}\n                child interaction proof_submit sizing hug"
        ),
    )
}

fn source_with_helper_content(role: &str) -> String {
    VALIDATION_SAMPLE_LIVE_VIEW_SOURCE.replace(
        "child interaction proof_submit sizing hug",
        &format!(
            "content submit_label {{\n                    content_kind inline\n                    content_order \"text\"\n                    content_text \"Need a little help\"\n                    content_role {role}\n                    content_text_size validation.density.primitive.content.text.default\n                }}\n                child interaction proof_submit sizing hug"
        ),
    )
}

fn source_with_image_content() -> String {
    VALIDATION_SAMPLE_LIVE_VIEW_SOURCE.replace(
        "child interaction proof_submit sizing hug",
        "content submit_label {
                    content_kind stack
                    content_order \"image,text\"
                    content_image worth.image.logo
                    content_text \"Worth UI\"
                    content_role body
                    content_text_size validation.density.primitive.content.text.default
                }
                child interaction proof_submit sizing hug",
    )
}
