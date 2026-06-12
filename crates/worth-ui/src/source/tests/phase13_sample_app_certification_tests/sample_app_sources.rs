use crate::source::{WorthUiSourcePackage, WorthUiSourcePackageLoader};

pub(super) const WORKSPACE_ROOT: &str = r"C:\workspace";

pub(super) fn sample_file_source_package() -> WorthUiSourcePackage {
    source_package_from_modules(sample_file_modules())
}

pub(super) fn reordered_sample_file_source_package() -> WorthUiSourcePackage {
    source_package_from_modules([
        ("app/theme/tokens.wui", theme_source()),
        ("app/panels/inspector.wui", inspector_source()),
        ("app/main.wui", main_source()),
    ])
}

pub(super) fn malformed_file_source_package() -> WorthUiSourcePackage {
    source_package_from_modules([
        (
            "app/main.wui",
            r#"
            import "app/panels/inspector.wui";
            component workspace.component.dashboard {
                region workspace.region.primary {
                    sizing workspace.sizing.fill;
            "#,
        ),
        ("app/panels/inspector.wui", inspector_source()),
    ])
}

pub(super) fn source_package_with_missing_component() -> WorthUiSourcePackage {
    source_package_from_modules([(
        "app/main.wui",
        r#"
        component workspace.component.unknown {}
        "#,
    )])
}

pub(super) fn source_package_with_unsupported_component() -> WorthUiSourcePackage {
    source_package_from_modules([(
        "app/main.wui",
        r#"
        component workspace.component.unsupported {}
        "#,
    )])
}

pub(super) fn source_package_with_illegal_structure() -> WorthUiSourcePackage {
    source_package_from_modules([(
        "app/main.wui",
        r#"
        component workspace.component.dashboard {
            mount workspace.surface.main placement workspace.placement.primary;
        }
        surface workspace.surface.main {}
        "#,
    )])
}

fn source_package_from_modules<const N: usize>(
    modules: [(&'static str, &'static str); N],
) -> WorthUiSourcePackage {
    let mut loader = WorthUiSourcePackageLoader::from_workspace_root(WORKSPACE_ROOT);
    for (path, source) in modules {
        loader = loader.register_module_with_source(path, source);
    }
    loader.compile().expect("source package should compile")
}

fn sample_file_modules() -> [(&'static str, &'static str); 3] {
    [
        ("app/main.wui", main_source()),
        ("app/panels/inspector.wui", inspector_source()),
        ("app/theme/tokens.wui", theme_source()),
    ]
}

fn main_source() -> &'static str {
    r#"
    import "app/panels/inspector.wui";
    import "app/theme/tokens.wui";

    component workspace.component.dashboard {
        region workspace.region.primary {
            sizing workspace.sizing.fill;
            state workspace.state.region_scroll;
            mount workspace.surface.main
                placement workspace.placement.primary
                state workspace.state.primary_surface;
        }
        region workspace.region.overlay {
            sizing workspace.sizing.overlay;
            state workspace.state.overlay_pinned;
            mount workspace.surface.overlay
                placement workspace.placement.overlay;
        }
    }

    surface workspace.surface.main {}
    surface workspace.surface.overlay {}
    surface workspace.surface.inspector {}
    binding workspace.view_binding.selection {}
    "#
}

fn inspector_source() -> &'static str {
    r#"
    component workspace.component.inspector_panel {}
    "#
}

fn theme_source() -> &'static str {
    r#"
    token theme.text.default = "theme.text.primary";
    "#
}
