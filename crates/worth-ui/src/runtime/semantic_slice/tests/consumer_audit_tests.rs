use crate::facade::{
    AppearanceTokenId, CommandDescriptor, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSelectionMode,
    CommandProjectionSurface, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, DensityTokenId, MosaicChildRule,
    MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture, MosaicRegionKindDescriptor,
    MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole, MosaicScrollOwnership,
    MosaicSizingBehavior, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass,
    SurfaceStateClass, ThemeColorValue, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource,
    ThemeTokenValue, WorthUi, WorthUiApp, WorthUiAppearanceFamily,
    WorthUiAppearanceTokenDescriptor, WorthUiAppearanceTokenSource, WorthUiAppearanceValue,
    WorthUiDensityFamily, WorthUiDensityTokenDescriptor, WorthUiDensityValue, WorthUiLengthValue,
    WorthUiPaddingValue, WorthUiRuntimeSourceModule, WorthUiSpacingValue,
};
use crate::runtime::{
    WorthUiDropdownAppearanceRequest, WorthUiDropdownProjectionPlan,
    WorthUiDropdownProjectionRequest, WorthUiHeaderThemePlan, WorthUiHeaderThemeTokenRequest,
    WorthUiPageHostPlan, WorthUiPageHostRequest, WorthUiRuntimeHost,
    WorthUiSemanticConsumerAuditReceipt, WorthUiSemanticSliceId, WorthUiSemanticSliceInventory,
};

#[test]
fn consumer_audit_uses_real_dropdown_projection_dependencies() {
    let app = semantic_projection_app();
    let plan = WorthUiDropdownProjectionPlan::from_snapshot(
        app.capabilities(),
        WorthUiDropdownProjectionRequest::for_command_projection(
            projection_id(),
            single_component_id(),
            multi_component_id(),
            WorthUiDropdownAppearanceRequest::new(
                AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
                DensityTokenId::new("density.header.row_padding").unwrap(),
                DensityTokenId::new("density.header.control_spacing").unwrap(),
            ),
        ),
    )
    .expect("dropdown plan should build from real capability snapshot");

    let audit = WorthUiSemanticConsumerAuditReceipt::audit_projection_plan(
        &WorthUiSemanticSliceInventory::current(),
        &plan,
    );

    assert!(
        audit.is_consistent(),
        "unexpected findings: {:?}",
        audit.findings()
    );
    assert!(audit
        .consumed_slice_ids()
        .contains(&WorthUiSemanticSliceId::RegisteredComponentCapability));
    assert!(audit
        .consumed_slice_ids()
        .contains(&WorthUiSemanticSliceId::CommandDeclaration));
    assert!(audit
        .consumed_slice_ids()
        .contains(&WorthUiSemanticSliceId::CommandProjectionSelectionMode));
    assert!(audit
        .consumed_slice_ids()
        .contains(&WorthUiSemanticSliceId::DropdownSelectedState));
}

#[test]
fn consumer_audit_uses_real_header_theme_dependencies() {
    let app = semantic_projection_app();
    let plan = WorthUiHeaderThemePlan::from_snapshot(
        app.capabilities(),
        WorthUiHeaderThemeTokenRequest::new(
            ThemeTokenId::new("theme.header.panel_fill").unwrap(),
            ThemeTokenId::new("theme.header.menu_fill").unwrap(),
            ThemeTokenId::new("theme.header.menu_hover_fill").unwrap(),
            ThemeTokenId::new("theme.header.menu_active_fill").unwrap(),
            ThemeTokenId::new("theme.header.text").unwrap(),
            ThemeTokenId::new("theme.header.border").unwrap(),
        ),
    )
    .expect("header theme plan should build from real capability snapshot");

    let audit = WorthUiSemanticConsumerAuditReceipt::audit_projection_plan(
        &WorthUiSemanticSliceInventory::current(),
        &plan,
    );

    assert!(
        audit.is_consistent(),
        "unexpected findings: {:?}",
        audit.findings()
    );
    assert_eq!(
        audit.consumed_slice_ids(),
        &[WorthUiSemanticSliceId::ThemeTokenValue]
    );
}

#[test]
fn consumer_audit_uses_real_page_host_dependencies() {
    let app = semantic_projection_app();
    let runtime = runtime_for_page_host(&app);
    let plan =
        WorthUiPageHostPlan::from_runtime(&runtime, WorthUiPageHostRequest::new("ProductsPage"))
            .expect("page host plan should build from real runtime authoring");

    let audit = WorthUiSemanticConsumerAuditReceipt::audit_projection_plan(
        &WorthUiSemanticSliceInventory::current(),
        &plan,
    );

    assert!(
        audit.is_consistent(),
        "unexpected findings: {:?}",
        audit.findings()
    );
    assert_eq!(
        audit.consumed_slice_ids(),
        &[
            WorthUiSemanticSliceId::LayoutTopology,
            WorthUiSemanticSliceId::LayoutGapRule,
            WorthUiSemanticSliceId::LayoutPaddingRule,
            WorthUiSemanticSliceId::ContentSlotAssignment,
            WorthUiSemanticSliceId::PageTemplateDeclaration,
            WorthUiSemanticSliceId::PageInstanceDeclaration,
            WorthUiSemanticSliceId::PageTemplateBinding,
            WorthUiSemanticSliceId::SurfaceMountTarget,
            WorthUiSemanticSliceId::AuthoredMountComponentSelection,
            WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
        ]
    );
}

fn semantic_projection_app() -> crate::facade::WorthUiApp {
    let command_new = CommandId::new("workspace.command.new").unwrap();
    let command_open = CommandId::new("workspace.command.open").unwrap();
    WorthUi::app()
        .register_command(CommandDescriptor::new(command_new.clone(), "New"))
        .register_command(CommandDescriptor::new(command_open.clone(), "Open"))
        .register_command_projection(
            CommandProjectionDescriptor::new(projection_id(), CommandProjectionSurface::menu_bar())
                .with_selection_mode(CommandProjectionSelectionMode::SingleSelect)
                .with_command_reference(CommandProjectionCommandReference::command(command_new))
                .with_command_reference(CommandProjectionCommandReference::command(command_open)),
        )
        .register_appearance_token(WorthUiAppearanceTokenDescriptor::define(
            AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
            WorthUiAppearanceFamily::Layout,
            WorthUiAppearanceTokenSource::Application,
            WorthUiAppearanceValue::Length(WorthUiLengthValue::from_px("220px").unwrap()),
        ))
        .register_density_token(WorthUiDensityTokenDescriptor::define(
            DensityTokenId::new("density.header.row_padding").unwrap(),
            WorthUiDensityFamily::RowPadding,
            WorthUiDensityValue::Padding(
                WorthUiPaddingValue::from_shorthand_px("1px 6px").unwrap(),
            ),
        ))
        .register_density_token(WorthUiDensityTokenDescriptor::define(
            DensityTokenId::new("density.header.control_spacing").unwrap(),
            WorthUiDensityFamily::ControlSpacing,
            WorthUiDensityValue::Spacing(WorthUiSpacingValue::from_px("8px").unwrap()),
        ))
        .register_theme_token(crate::facade::ThemeTokenDescriptor::define(
            ThemeTokenId::new("theme.header.panel_fill").unwrap(),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::Color(ThemeColorValue::hex("#111111").unwrap()),
        ))
        .register_theme_token(crate::facade::ThemeTokenDescriptor::define(
            ThemeTokenId::new("theme.header.menu_fill").unwrap(),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::Color(ThemeColorValue::hex("#222222").unwrap()),
        ))
        .register_theme_token(crate::facade::ThemeTokenDescriptor::define(
            ThemeTokenId::new("theme.header.menu_hover_fill").unwrap(),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::Color(ThemeColorValue::hex("#333333").unwrap()),
        ))
        .register_theme_token(crate::facade::ThemeTokenDescriptor::define(
            ThemeTokenId::new("theme.header.menu_active_fill").unwrap(),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::Color(ThemeColorValue::hex("#444444").unwrap()),
        ))
        .register_theme_token(crate::facade::ThemeTokenDescriptor::define(
            ThemeTokenId::new("theme.header.text").unwrap(),
            ThemeTokenFamily::text(),
            ThemeTokenSource::application(),
            ThemeTokenValue::Color(ThemeColorValue::hex("#eeeeee").unwrap()),
        ))
        .register_theme_token(crate::facade::ThemeTokenDescriptor::define(
            ThemeTokenId::new("theme.header.border").unwrap(),
            ThemeTokenFamily::border(),
            ThemeTokenSource::application(),
            ThemeTokenValue::Color(ThemeColorValue::hex("#999999").unwrap()),
        ))
        .register_component(ComponentDescriptor::new(
            single_component_id(),
            ComponentPropSchema::named("validation.header.dropdown.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_component(ComponentDescriptor::new(
            multi_component_id(),
            ComponentPropSchema::named("validation.header.multi_select_dropdown.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.product_list").unwrap(),
            ComponentPropSchema::named("workspace.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_surface(validation_surface("validation.surface.products.collection"))
        .register_surface(validation_surface("validation.surface.products.toolbar"))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.column",
            MosaicRegionRole::stack(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.row",
            MosaicRegionRole::split(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.slot",
            MosaicRegionRole::primary(),
            MosaicChildRule::accepts_surfaces(),
        ))
        .freeze()
}

fn runtime_for_page_host(app: &WorthUiApp) -> WorthUiRuntimeHost {
    let prepared = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new(
            "app/main.wui",
            r#"
            app ShopifyAdminApp { theme ShopifyAdminTheme workspace AdminWorkspace }
            workspace AdminWorkspace { shell {
                topbar AdminTopbar rail AdminPrimaryRail page_host AdminPageHost
                inspector AdminInspectorDock status AdminStatusBar
                overlays [CommandPaletteOverlay] toasts AdminToastCenter
            }
                pages [ProductsPage]
            }
            page ProductsPage {
                runtime ProductsRuntime
                layout ProductsLayout
                content ProductsContent
            }
            runtime ProductsRuntime {}
            appearance ShopifyAdminTheme {}
            layout ProductsLayout {
                column {
                    row height fit { slot toolbar }
                    row height fill scroll_owner { slot collection }
                }
            }
            content ProductsContent {
                collection -> validation.surface.products.collection
                toolbar -> validation.surface.products.toolbar
            }
            "#,
        ))
        .prepare_authoring_for(app)
        .expect("source-authored runtime prepares");
    app.launch_runtime(prepared.into_runtime_launch())
        .expect("runtime launches")
}

fn projection_id() -> CommandProjectionId {
    CommandProjectionId::new("workspace.header.file").unwrap()
}

fn single_component_id() -> ComponentId {
    ComponentId::new("validation.component.header.dropdown").unwrap()
}

fn multi_component_id() -> ComponentId {
    ComponentId::new("validation.component.header.multi_select_dropdown").unwrap()
}

fn validation_surface(id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(id).unwrap(),
        SurfaceKind::primary_content(),
        ComponentId::new("workspace.component.product_list").unwrap(),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

fn layout_region(
    id: &str,
    role: MosaicRegionRole,
    child_rule: MosaicChildRule,
) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(MosaicRegionKindId::new(id).unwrap(), role)
        .with_persistence(MosaicRegionPersistence::restorable())
        .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
        .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
        .with_child_rule(child_rule)
        .with_allowed_surface_class(SurfacePlacementClass::primary_region())
        .with_scroll_ownership(MosaicScrollOwnership::region_owned())
        .with_clipping(MosaicClippingPosture::clip_to_region())
        .with_hit_test(MosaicHitTestPosture::participates())
}
