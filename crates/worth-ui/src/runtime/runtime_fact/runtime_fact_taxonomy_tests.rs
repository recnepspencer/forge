use crate::capability::{
    AppearanceTokenId, CommandProjectionId, ComponentId, DensityTokenId, SurfaceId, ThemeTokenId,
    ViewBindingId,
};
use crate::runtime::{
    WorthUiCapabilityDeltaRuntimeFactLowering, WorthUiCapabilityReloadEvidence,
    WorthUiCapabilityReloadStatus, WorthUiHandlePlanGeneration, WorthUiPageInstanceId,
    WorthUiPageTemplateId, WorthUiProjectionDependencySet, WorthUiQueryBindingComparison,
    WorthUiQueryBindingComparisonCounters, WorthUiQueryBindingComparisonEntry,
    WorthUiQueryBindingComparisonOutcome, WorthUiQueryBindingIdentity,
    WorthUiQueryBindingRuntimeFactLowering, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
    WorthUiRuntimeFactSet, WorthUiRuntimeFactSetDigest, WorthUiViewBindingHandle,
    WorthUiVirtualizedDataFrameTarget, WorthUiVisibleRange,
};

use super::WorthUiContentSlotId;

#[test]
fn page_template_instance_and_binding_facts_do_not_collapse() {
    let template = page_template("ProductDetailPage");
    let instance = page_instance("product-detail:P-1001");

    let template_fact = WorthUiRuntimeFactId::page_template(&template);
    let instance_fact = WorthUiRuntimeFactId::page_instance(&instance);
    let binding_fact = WorthUiRuntimeFactId::page_instance_template_binding(&instance, &template);

    assert_ne!(template_fact, instance_fact);
    assert_ne!(template_fact, binding_fact);
    assert_ne!(instance_fact, binding_fact);
    assert_eq!(
        template_fact.family(),
        WorthUiRuntimeFactFamily::PageTemplate
    );
    assert_eq!(
        instance_fact.family(),
        WorthUiRuntimeFactFamily::PageInstance
    );
    assert_eq!(
        binding_fact.family(),
        WorthUiRuntimeFactFamily::PageInstanceTemplateBinding
    );
}

#[test]
fn page_content_slot_fact_does_not_invalidate_header_theme_dependencies() {
    let template = page_template("ProductDetailPage");
    let slot = WorthUiContentSlotId::new("summary").expect("valid content slot id");
    let theme = ThemeTokenId::new("validation.theme.header.panel").expect("valid theme token id");

    let changed =
        WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::page_content_slot(&template, &slot));
    let header_theme_dependencies = WorthUiProjectionDependencySet::empty()
        .depends_on(WorthUiRuntimeFactId::theme_token(&theme));

    assert!(!header_theme_dependencies.intersects(&changed));
}

#[test]
fn theme_token_fact_does_not_invalidate_layout_content_or_density_dependencies() {
    let theme = ThemeTokenId::new("validation.theme.header.panel").expect("valid theme token id");
    let density =
        DensityTokenId::new("validation.density.compact").expect("valid density token id");
    let changed = WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::theme_token(&theme));

    let unrelated_dependencies = WorthUiProjectionDependencySet::empty()
        .depends_on(WorthUiRuntimeFactId::layout_topology("workspace/root"))
        .depends_on(WorthUiRuntimeFactId::content_mount("HeaderProofPage/proof"))
        .depends_on(WorthUiRuntimeFactId::density_token(&density));

    assert!(!unrelated_dependencies.intersects(&changed));
}

#[test]
fn appearance_and_density_facts_do_not_collapse_when_raw_identity_matches() {
    let appearance =
        AppearanceTokenId::new("validation.header.shared").expect("valid appearance token id");
    let density = DensityTokenId::new("validation.header.shared").expect("valid density token id");

    let appearance_fact = WorthUiRuntimeFactId::appearance_token(&appearance);
    let density_fact = WorthUiRuntimeFactId::density_token(&density);

    assert_ne!(appearance_fact, density_fact);
    assert_eq!(
        appearance_fact.family(),
        WorthUiRuntimeFactFamily::Appearance
    );
    assert_eq!(
        density_fact.family(),
        WorthUiRuntimeFactFamily::DensityToken
    );
}

#[test]
fn authored_mount_component_selection_is_distinct_from_component_capability_family() {
    let component =
        ComponentId::new("validation.surface.header.proof").expect("valid component id");
    let authored =
        WorthUiRuntimeFactId::authored_mount_component_selection("validation.surface.header.proof");
    let capability = WorthUiRuntimeFactId::component(&component);

    assert_ne!(authored, capability);
    assert_eq!(
        authored.family(),
        WorthUiRuntimeFactFamily::AuthoredMountComponentSelection
    );
    assert_eq!(capability.family(), WorthUiRuntimeFactFamily::Component);
}

#[test]
fn fact_set_digest_is_stable_across_insertion_order() {
    let left = WorthUiRuntimeFactSet::empty()
        .with(theme_fact("validation.theme.header.panel"))
        .with(command_projection_fact("validation.header.menu.file"));
    let right = WorthUiRuntimeFactSet::empty()
        .with(command_projection_fact("validation.header.menu.file"))
        .with(theme_fact("validation.theme.header.panel"));

    assert_eq!(left.digest(), right.digest());
}

#[test]
fn fact_set_digest_changes_when_family_or_identity_changes() {
    let theme_digest = WorthUiRuntimeFactSet::single(theme_fact("validation.shared")).digest();
    let surface = SurfaceId::new("validation.shared").expect("valid surface id");
    let surface_digest =
        WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::surface_mount(&surface)).digest();
    let different_theme_digest =
        WorthUiRuntimeFactSet::single(theme_fact("validation.theme.header.menu")).digest();

    assert_digest_differs(theme_digest, surface_digest);
    assert_digest_differs(theme_digest, different_theme_digest);
}

#[test]
fn malformed_runtime_fact_identity_is_rejected_before_fact_construction() {
    assert!(WorthUiPageTemplateId::new("").is_err());
    assert!(WorthUiPageTemplateId::new(" ProductDetailPage").is_err());
    assert!(WorthUiPageTemplateId::new("Product Detail Page").is_err());
}

#[test]
fn query_binding_lowering_emits_only_changed_binding_facts() {
    let changed_binding = ViewBindingId::new("validation.query.changed").expect("valid id");
    let preserved_binding = ViewBindingId::new("validation.query.preserved").expect("valid id");
    let comparison = WorthUiQueryBindingComparison::new(
        10,
        11,
        vec![
            query_entry(
                &preserved_binding,
                WorthUiQueryBindingComparisonOutcome::PreserveMeaning,
            ),
            query_entry(
                &changed_binding,
                WorthUiQueryBindingComparisonOutcome::RebindRequired,
            ),
        ],
        WorthUiQueryBindingComparisonCounters::default(),
    );

    let changed_facts = WorthUiQueryBindingRuntimeFactLowering::from_comparison(&comparison);

    assert_eq!(changed_facts.active_artifact_digest_before(), 10);
    assert_eq!(changed_facts.candidate_artifact_digest_after(), 11);
    assert_eq!(changed_facts.changed_facts().len(), 1);
    assert!(changed_facts
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::query_binding(&changed_binding)));
    assert!(!changed_facts
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::query_binding(&preserved_binding)));
    assert!(!changed_facts
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::ActiveArtifact));
}

#[test]
fn capability_delta_lowering_reuses_runtime_evidence_facts_without_broadening() {
    let theme = ThemeTokenId::new("validation.theme.header.panel").expect("valid theme token id");
    let evidence = WorthUiCapabilityReloadEvidence::prepared(
        100,
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary,
        10,
        11,
        29,
        1,
        6,
        1,
        WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::theme_token(&theme)),
    );

    let changed_facts = WorthUiCapabilityDeltaRuntimeFactLowering::from_reload_evidence(&evidence);

    assert_eq!(changed_facts.active_snapshot_digest_before(), 10);
    assert_eq!(changed_facts.active_snapshot_digest_after(), 11);
    assert_eq!(changed_facts.changed_facts().len(), 1);
    assert!(changed_facts
        .changed_facts()
        .contains_exact(&WorthUiRuntimeFactId::theme_token(&theme)));
    assert!(!changed_facts
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::ActiveArtifact));
}

#[test]
fn virtualized_data_frame_fact_uses_frame_target_identity_not_query_binding_family() {
    let target = WorthUiVirtualizedDataFrameTarget::view_binding(
        WorthUiViewBindingHandle::new(3, WorthUiHandlePlanGeneration::new(9)),
        WorthUiVisibleRange::grid(10, 20, 2, 4).expect("valid visible range"),
    );
    let fact = WorthUiRuntimeFactId::virtualized_data_frame(target);

    assert_eq!(
        fact.family(),
        WorthUiRuntimeFactFamily::VirtualizedDataFrame
    );
    assert!(fact.identity().contains("view_binding:3:9:10:20:2:4"));
}

fn theme_fact(raw_id: &str) -> WorthUiRuntimeFactId {
    let token = ThemeTokenId::new(raw_id).expect("valid theme token id");
    WorthUiRuntimeFactId::theme_token(&token)
}

fn command_projection_fact(raw_id: &str) -> WorthUiRuntimeFactId {
    let projection = CommandProjectionId::new(raw_id).expect("valid projection id");
    WorthUiRuntimeFactId::command_projection(&projection)
}

fn page_template(raw_id: &str) -> WorthUiPageTemplateId {
    WorthUiPageTemplateId::new(raw_id).expect("valid page template id")
}

fn page_instance(raw_id: &str) -> WorthUiPageInstanceId {
    WorthUiPageInstanceId::new(raw_id).expect("valid page instance id")
}

fn query_entry(
    view_binding_id: &ViewBindingId,
    outcome: WorthUiQueryBindingComparisonOutcome,
) -> WorthUiQueryBindingComparisonEntry {
    WorthUiQueryBindingComparisonEntry::new(
        WorthUiQueryBindingIdentity::new(
            view_binding_id,
            "capability".to_string(),
            "composition".to_string(),
            "shape".to_string(),
        ),
        None,
        None,
        outcome,
        Vec::new(),
    )
}

fn assert_digest_differs(left: WorthUiRuntimeFactSetDigest, right: WorthUiRuntimeFactSetDigest) {
    assert_ne!(left, right);
}
