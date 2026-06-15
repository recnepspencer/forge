use std::collections::BTreeSet;

use worth_ui_harness::facade::HarnessDensity;
use worth_ui_validation_app::pages::surface_atlas::{SurfaceAtlasFamily, SurfaceAtlasPage};
use worth_ui_validation_app::ValidationWorkbenchLaunch;

#[test]
fn surface_atlas_renders_all_required_surface_families() {
    let page = prepared_surface_atlas_page();
    let topology = page.model().topology();

    for family in SurfaceAtlasFamily::REQUIRED {
        assert!(
            topology.includes(family),
            "surface atlas is missing {}",
            family.label()
        );
        assert!(
            page.rendered_surface_families()
                .any(|rendered_family| rendered_family == family),
            "surface atlas does not visibly render {}",
            family.label()
        );
    }

    let stable_ids = topology
        .regions()
        .iter()
        .map(|region| region.stable_id())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        stable_ids.len(),
        SurfaceAtlasFamily::REQUIRED.len(),
        "surface atlas stable region ids must be unique"
    );

    let rendered_families = page.rendered_surface_families().collect::<BTreeSet<_>>();
    assert_eq!(
        rendered_families.len(),
        SurfaceAtlasFamily::REQUIRED.len(),
        "surface atlas render plan must not duplicate visible family claims"
    );
}

#[test]
fn surface_atlas_theme_controls_update_tokens_without_layout_drift() {
    let mut page = prepared_surface_atlas_page();
    let before_topology = page.model().topology().clone();
    let before_visual_foundation = page.model().visual_foundation().clone();

    page.model_mut().controls_mut().advance_theme_revision();
    page.model_mut()
        .controls_mut()
        .select_density(HarnessDensity::ComfortableWorkbench);

    assert_eq!(
        page.model().topology(),
        &before_topology,
        "theme and density controls must not rewrite the atlas topology"
    );
    assert_eq!(
        page.model().visual_foundation(),
        &before_visual_foundation,
        "theme controls must use the prepared visual foundation instead of inventing tokens"
    );
    assert_eq!(
        page.model().controls().density(),
        HarnessDensity::ComfortableWorkbench
    );
    assert_eq!(page.model().controls().theme_revision(), 1);
}

fn prepared_surface_atlas_page() -> SurfaceAtlasPage {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation launch should prepare");
    SurfaceAtlasPage::from_launch(&launch)
}
