use super::digest::stable_text_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDiagnosticsSurfaceBinding {
    surface_id: String,
    surface_digest: u64,
}

impl WorthUiDiagnosticsSurfaceBinding {
    pub fn new(surface_id: impl Into<String>) -> Self {
        let surface_id = surface_id.into();
        let surface_digest = stable_text_digest(&surface_id);
        Self {
            surface_id,
            surface_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn surface_digest(&self) -> u64 {
        self.surface_digest
    }
}
