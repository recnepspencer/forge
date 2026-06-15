use egui::Vec2;

use crate::pages::surface_atlas::SurfaceAtlasFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceAtlasViewport {
    Narrow,
    Standard,
    Wide,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceAtlasReachability {
    viewport: SurfaceAtlasViewport,
    reachable: Vec<SurfaceAtlasFamily>,
}

impl SurfaceAtlasViewport {
    pub fn from_available_size(size: Vec2) -> Self {
        if size.x < 720.0 {
            Self::Narrow
        } else if size.x < 1180.0 {
            Self::Standard
        } else {
            Self::Wide
        }
    }
}

impl SurfaceAtlasReachability {
    pub fn for_viewport(viewport: SurfaceAtlasViewport) -> Self {
        let reachable = match viewport {
            SurfaceAtlasViewport::Narrow => vec![
                SurfaceAtlasFamily::ScenarioList,
                SurfaceAtlasFamily::WorkbenchCanvas,
                SurfaceAtlasFamily::EvidenceInspector,
                SurfaceAtlasFamily::BottomTimeline,
                SurfaceAtlasFamily::OverlayPreview,
            ],
            SurfaceAtlasViewport::Standard | SurfaceAtlasViewport::Wide => {
                SurfaceAtlasFamily::REQUIRED.to_vec()
            }
        };
        Self {
            viewport,
            reachable,
        }
    }

    pub fn viewport(&self) -> SurfaceAtlasViewport {
        self.viewport
    }

    pub fn reaches(&self, family: SurfaceAtlasFamily) -> bool {
        self.reachable.contains(&family)
    }
}
