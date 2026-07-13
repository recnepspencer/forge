mod facade_pairing;
mod inventory;
mod specimen_catalog;

use std::path::{Path, PathBuf};

pub use specimen_catalog::{
    BoundaryFixture, CompilerFixture, Enforcement, EntryDependency, Specimen,
};

pub struct Corpus {
    repository_root: PathBuf,
    specimen_directory: PathBuf,
}

impl Corpus {
    pub fn load() -> Self {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        Self {
            repository_root: manifest
                .ancestors()
                .nth(5)
                .expect("repository root")
                .to_owned(),
            specimen_directory: manifest.join("tests/specimens"),
        }
    }

    pub fn rows(&self) -> &'static [Specimen] {
        specimen_catalog::SPECIMENS
    }

    pub fn specimen_path(&self, specimen: &Specimen) -> PathBuf {
        self.specimen_directory.join(specimen.path)
    }

    pub fn assert_exact_files_and_facade_pairing(&self) {
        let physical = inventory::exact_inventory(&self.specimen_directory, self.rows(), "")
            .unwrap_or_else(|error| panic!("{error}"));
        facade_pairing::validate(
            &self.repository_root,
            &self.specimen_directory,
            self.rows(),
            "",
            &physical,
        )
        .unwrap_or_else(|error| panic!("{error}"));
    }

    pub fn validate_inventory_without(&self, omitted: &str) -> Result<(), String> {
        inventory::validate_registered_deletion(self.rows(), omitted)
    }

    pub fn validate_physical_corpus_without(&self, omitted: &str) -> Result<(), String> {
        let physical = inventory::catalog_files_without(self.rows(), omitted);
        facade_pairing::validate(
            &self.repository_root,
            &self.specimen_directory,
            self.rows(),
            omitted,
            &physical,
        )
    }

    pub fn repository_root(&self) -> &Path {
        &self.repository_root
    }
}
