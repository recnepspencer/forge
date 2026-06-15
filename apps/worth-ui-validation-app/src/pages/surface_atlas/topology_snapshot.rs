use crate::pages::surface_atlas::SurfaceAtlasFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceAtlasTopologySnapshot {
    regions: Vec<SurfaceAtlasRegion>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SurfaceAtlasRegion {
    family: SurfaceAtlasFamily,
    stable_id: &'static str,
}

impl SurfaceAtlasTopologySnapshot {
    pub fn required() -> Self {
        Self {
            regions: SurfaceAtlasFamily::REQUIRED
                .into_iter()
                .map(SurfaceAtlasRegion::for_family)
                .collect(),
        }
    }

    pub fn includes(&self, family: SurfaceAtlasFamily) -> bool {
        self.regions.iter().any(|region| region.family() == family)
    }

    pub fn regions(&self) -> &[SurfaceAtlasRegion] {
        &self.regions
    }
}

impl SurfaceAtlasRegion {
    pub fn for_family(family: SurfaceAtlasFamily) -> Self {
        Self {
            family,
            stable_id: stable_region_id(family),
        }
    }

    pub fn family(self) -> SurfaceAtlasFamily {
        self.family
    }

    pub fn stable_id(self) -> &'static str {
        self.stable_id
    }
}

fn stable_region_id(family: SurfaceAtlasFamily) -> &'static str {
    match family {
        SurfaceAtlasFamily::ActivityNavigation => "surface-atlas.activity-navigation",
        SurfaceAtlasFamily::ScenarioList => "surface-atlas.scenario-list",
        SurfaceAtlasFamily::CommandProjectionSurface => "surface-atlas.command-projection",
        SurfaceAtlasFamily::TabbedEditor => "surface-atlas.tabbed-editor",
        SurfaceAtlasFamily::PinnedSidebar => "surface-atlas.pinned-sidebar",
        SurfaceAtlasFamily::StackedScrollPane => "surface-atlas.stacked-scroll-panes",
        SurfaceAtlasFamily::EvidenceInspector => "surface-atlas.evidence-inspector",
        SurfaceAtlasFamily::BottomTimeline => "surface-atlas.bottom-timeline",
        SurfaceAtlasFamily::OverlayPreview => "surface-atlas.overlay-preview",
        SurfaceAtlasFamily::StatusSurface => "surface-atlas.status-surface",
        SurfaceAtlasFamily::ThemeControls => "surface-atlas.theme-controls",
        SurfaceAtlasFamily::DensityControls => "surface-atlas.density-controls",
        SurfaceAtlasFamily::WorkbenchCanvas => "surface-atlas.workbench-canvas",
    }
}
