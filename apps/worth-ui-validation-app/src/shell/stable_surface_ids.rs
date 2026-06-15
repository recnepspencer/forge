#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StableShellSurfaceId(&'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StableShellSurface {
    id: StableShellSurfaceId,
    label: &'static str,
    placement: StableShellSurfacePlacement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StableShellSurfaceManifest {
    surfaces: &'static [StableShellSurface],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StableShellSurfacePlacement {
    TopLevel,
    Embedded { parent: StableShellSurfaceId },
}

impl StableShellSurfaceId {
    pub const ACTIVITY_RAIL: Self = Self("validation.shell.activity_rail");
    pub const SCENARIO_NAV: Self = Self("validation.shell.scenario_nav");
    pub const MENU_BAR: Self = Self("validation.shell.menu_bar");
    pub const TOOLBAR: Self = Self("validation.shell.toolbar");
    pub const COMMAND_PALETTE: Self = Self("validation.shell.command_palette");
    pub const PAGE_HOST: Self = Self("validation.shell.page_host");
    pub const EDITOR_TABS: Self = Self("validation.shell.editor_tabs");
    pub const INSPECTOR: Self = Self("validation.shell.inspector");
    pub const BOTTOM_TIMELINE: Self = Self("validation.shell.bottom_timeline");
    pub const STATUS_BAR: Self = Self("validation.shell.status_bar");
    pub const OVERLAY_LAYER: Self = Self("validation.shell.overlay_layer");

    pub fn as_str(self) -> &'static str {
        self.0
    }
}

impl StableShellSurface {
    pub const fn top_level(id: StableShellSurfaceId, label: &'static str) -> Self {
        Self {
            id,
            label,
            placement: StableShellSurfacePlacement::TopLevel,
        }
    }

    pub const fn embedded(
        id: StableShellSurfaceId,
        label: &'static str,
        parent: StableShellSurfaceId,
    ) -> Self {
        Self {
            id,
            label,
            placement: StableShellSurfacePlacement::Embedded { parent },
        }
    }

    pub fn id(self) -> StableShellSurfaceId {
        self.id
    }

    pub fn label(self) -> &'static str {
        self.label
    }

    pub fn placement(self) -> StableShellSurfacePlacement {
        self.placement
    }
}

impl StableShellSurfaceManifest {
    pub const REQUIRED: Self = Self {
        surfaces: &[
            StableShellSurface::top_level(StableShellSurfaceId::MENU_BAR, "Menu bar"),
            StableShellSurface::top_level(StableShellSurfaceId::TOOLBAR, "Toolbar"),
            StableShellSurface::top_level(StableShellSurfaceId::ACTIVITY_RAIL, "Activity rail"),
            StableShellSurface::top_level(
                StableShellSurfaceId::SCENARIO_NAV,
                "Scenario navigation",
            ),
            StableShellSurface::embedded(
                StableShellSurfaceId::COMMAND_PALETTE,
                "Command palette",
                StableShellSurfaceId::TOOLBAR,
            ),
            StableShellSurface::top_level(StableShellSurfaceId::INSPECTOR, "Inspector"),
            StableShellSurface::top_level(StableShellSurfaceId::BOTTOM_TIMELINE, "Bottom timeline"),
            StableShellSurface::top_level(StableShellSurfaceId::STATUS_BAR, "Status bar"),
            StableShellSurface::top_level(StableShellSurfaceId::PAGE_HOST, "Page host"),
            StableShellSurface::embedded(
                StableShellSurfaceId::EDITOR_TABS,
                "Editor tabs",
                StableShellSurfaceId::PAGE_HOST,
            ),
            StableShellSurface::top_level(StableShellSurfaceId::OVERLAY_LAYER, "Overlay layer"),
        ],
    };

    pub fn surfaces(self) -> &'static [StableShellSurface] {
        self.surfaces
    }

    pub fn contains(self, id: StableShellSurfaceId) -> bool {
        self.surfaces.iter().any(|surface| surface.id() == id)
    }

    pub fn surface(self, id: StableShellSurfaceId) -> Option<StableShellSurface> {
        self.surfaces
            .iter()
            .copied()
            .find(|surface| surface.id() == id)
    }
}
