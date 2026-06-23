#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredSurfaceEntry {
    surface_id: String,
    component_id: String,
    digest: u64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthUiAuthoredSurfaceCatalog {
    entries: Vec<WorthUiAuthoredSurfaceEntry>,
}

impl WorthUiAuthoredSurfaceCatalog {
    pub(crate) fn from_entries(mut entries: Vec<WorthUiAuthoredSurfaceEntry>) -> Self {
        entries.sort_by(|left, right| {
            left.surface_id()
                .cmp(right.surface_id())
                .then_with(|| left.component_id().cmp(right.component_id()))
                .then_with(|| left.digest().cmp(&right.digest()))
        });
        entries.dedup();
        Self { entries }
    }

    pub fn entries(&self) -> &[WorthUiAuthoredSurfaceEntry] {
        &self.entries
    }

    pub fn component_id_for_surface(&self, surface_id: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|entry| entry.surface_id() == surface_id)
            .map(WorthUiAuthoredSurfaceEntry::component_id)
    }

    pub(crate) fn digest_basis(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|entry| {
                format!(
                    "authored_surface|surface:{}|component:{}|digest:{}",
                    entry.surface_id(),
                    entry.component_id(),
                    entry.digest()
                )
            })
            .collect()
    }
}

impl WorthUiAuthoredSurfaceEntry {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        component_id: impl Into<String>,
        digest: u64,
    ) -> Self {
        Self {
            surface_id: surface_id.into(),
            component_id: component_id.into(),
            digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }

    pub fn digest(&self) -> u64 {
        self.digest
    }
}
