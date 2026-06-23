#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoringCatalogEntry {
    name: String,
    digest: u64,
}

macro_rules! define_catalog {
    ($name:ident) => {
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct $name {
            entries: Vec<WorthUiAuthoringCatalogEntry>,
        }

        impl $name {
            pub(crate) fn from_entries(mut entries: Vec<WorthUiAuthoringCatalogEntry>) -> Self {
                entries.sort_by(|left, right| {
                    left.name()
                        .cmp(right.name())
                        .then_with(|| left.digest().cmp(&right.digest()))
                });
                entries.dedup();
                Self { entries }
            }

            pub fn entries(&self) -> &[WorthUiAuthoringCatalogEntry] {
                &self.entries
            }

            pub fn contains(&self, name: &str) -> bool {
                self.entries.iter().any(|entry| entry.name() == name)
            }

            pub(crate) fn digest_basis(&self, family: &str) -> Vec<String> {
                self.entries
                    .iter()
                    .map(|entry| {
                        format!("{family}|name:{}|digest:{}", entry.name(), entry.digest())
                    })
                    .collect()
            }
        }
    };
}

define_catalog!(WorthUiWorkspaceShellCatalog);
define_catalog!(WorthUiPageTemplateCatalog);
define_catalog!(WorthUiPageInstanceCatalog);
define_catalog!(WorthUiAppearanceRecipeCatalog);
define_catalog!(WorthUiRuntimeBindingCatalog);

impl WorthUiAuthoringCatalogEntry {
    pub(crate) fn new(name: impl Into<String>, digest: u64) -> Self {
        Self {
            name: name.into(),
            digest,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn digest(&self) -> u64 {
        self.digest
    }
}
