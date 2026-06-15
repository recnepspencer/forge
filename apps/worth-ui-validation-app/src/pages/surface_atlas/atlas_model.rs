use worth_ui_harness::facade::{HarnessDensity, HarnessVisualFoundationReceipt};

use crate::pages::surface_atlas::{SurfaceAtlasFixtureEvidence, SurfaceAtlasTopologySnapshot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceAtlasModel {
    topology: SurfaceAtlasTopologySnapshot,
    fixture_evidence: SurfaceAtlasFixtureEvidence,
    controls: SurfaceAtlasControlState,
    visual_foundation: HarnessVisualFoundationReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceAtlasControlState {
    density: HarnessDensity,
    theme_revision: u64,
}

impl SurfaceAtlasModel {
    pub fn new(
        visual_foundation: HarnessVisualFoundationReceipt,
        density: HarnessDensity,
        fixture_evidence: SurfaceAtlasFixtureEvidence,
    ) -> Self {
        Self {
            topology: SurfaceAtlasTopologySnapshot::required(),
            fixture_evidence,
            controls: SurfaceAtlasControlState::new(density),
            visual_foundation,
        }
    }

    pub fn topology(&self) -> &SurfaceAtlasTopologySnapshot {
        &self.topology
    }

    pub fn fixture_evidence(&self) -> &SurfaceAtlasFixtureEvidence {
        &self.fixture_evidence
    }

    pub fn controls(&self) -> &SurfaceAtlasControlState {
        &self.controls
    }

    pub fn controls_mut(&mut self) -> &mut SurfaceAtlasControlState {
        &mut self.controls
    }

    pub fn visual_foundation(&self) -> &HarnessVisualFoundationReceipt {
        &self.visual_foundation
    }
}

impl SurfaceAtlasControlState {
    pub fn new(density: HarnessDensity) -> Self {
        Self {
            density,
            theme_revision: 0,
        }
    }

    pub fn density(&self) -> HarnessDensity {
        self.density
    }

    pub fn theme_revision(&self) -> u64 {
        self.theme_revision
    }

    pub fn select_density(&mut self, density: HarnessDensity) {
        self.density = density;
    }

    pub fn advance_theme_revision(&mut self) {
        self.theme_revision += 1;
    }
}
