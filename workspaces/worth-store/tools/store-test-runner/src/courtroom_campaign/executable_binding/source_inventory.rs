use std::path::{Path, PathBuf};

use worth_store::physical_runtime::{PhysicalWorkFeatureGraphEvidence, PhysicalWorkSourceBinding};

mod metadata_graph;
mod source_closure;

pub(super) struct LocalSourceInventory {
    repository: PathBuf,
    workspace: PathBuf,
    package_roots: Box<[PathBuf]>,
    build_inputs: Box<[PathBuf]>,
    feature_graph: PhysicalWorkFeatureGraphEvidence,
}

impl LocalSourceInventory {
    pub(super) fn discover(workspace: &Path) -> Result<Self, String> {
        let workspace = workspace
            .canonicalize()
            .map_err(|error| format!("cannot canonicalize Store workspace: {error}"))?;
        let repository = repository_root(&workspace)?;
        let metadata = metadata_graph::runtime_metadata_evidence(&workspace, &repository)?;
        let build_inputs = source_closure::build_inputs(&repository, &workspace)?;
        Ok(Self {
            repository,
            workspace,
            package_roots: metadata.package_roots.into_boxed_slice(),
            build_inputs: build_inputs.into_boxed_slice(),
            feature_graph: metadata.feature_graph,
        })
    }

    pub(super) fn bind(&self) -> Result<PhysicalWorkSourceBinding, String> {
        source_closure::bind(
            &self.repository,
            &self.workspace,
            &self.package_roots,
            &self.build_inputs,
        )
    }

    pub(super) const fn feature_graph(&self) -> &PhysicalWorkFeatureGraphEvidence {
        &self.feature_graph
    }

    #[cfg(test)]
    fn for_test(
        repository: PathBuf,
        workspace: PathBuf,
        package_roots: Vec<PathBuf>,
        build_inputs: Vec<PathBuf>,
        feature_graph: PhysicalWorkFeatureGraphEvidence,
    ) -> Self {
        Self {
            repository,
            workspace,
            package_roots: package_roots.into_boxed_slice(),
            build_inputs: build_inputs.into_boxed_slice(),
            feature_graph,
        }
    }
}

fn repository_root(workspace: &Path) -> Result<PathBuf, String> {
    workspace
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Store workspace omitted its repository ancestors".to_owned())?
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize repository root: {error}"))
}

#[cfg(test)]
mod tests {
    use super::LocalSourceInventory;
    use worth_store::physical_runtime::{
        PhysicalWorkFeatureGraphEvidence, PhysicalWorkFeatureNodeEvidence,
    };

    #[test]
    fn real_inventory_binds_runtime_dependencies_outside_the_store_workspace() {
        let inventory = LocalSourceInventory::discover(&crate::workspace_root()).unwrap();
        let roots = inventory
            .package_roots
            .iter()
            .map(|root| root.to_string_lossy().replace('\\', "/"))
            .collect::<Vec<_>>();

        for suffix in [
            "/crates/worth-signal",
            "/crates/worth-proof",
            "/crates/worth-store-buffer-pool",
            "/crates/worth-store-io-scheduler",
            "/crates/worth-store-physical-backend",
            "/tools/store-test-runner",
        ] {
            assert!(roots.iter().any(|root| root.ends_with(suffix)), "{suffix}");
        }
        assert!(inventory.bind().is_ok());
    }

    #[test]
    fn dependency_source_drift_changes_the_bound_source_closure() {
        let temporary = tempfile::tempdir().unwrap();
        let repository = temporary.path().join("forge");
        let workspace = repository.join("workspaces/worth-store");
        let writer = workspace.join("crates/worth-store");
        let dependency = repository.join("crates/worth-signal");
        for root in [&writer, &dependency] {
            std::fs::create_dir_all(root.join("src")).unwrap();
            std::fs::write(root.join("Cargo.toml"), b"[package]\nname='bound'\n").unwrap();
            std::fs::write(root.join("src/lib.rs"), b"pub fn original() {}\n").unwrap();
        }
        std::fs::create_dir_all(&workspace).unwrap();
        std::fs::write(repository.join("Cargo.toml"), b"[workspace]\n").unwrap();
        std::fs::write(workspace.join("Cargo.toml"), b"[workspace]\n").unwrap();
        std::fs::write(workspace.join("Cargo.lock"), b"version = 4\n").unwrap();
        let inventory = LocalSourceInventory::for_test(
            repository.clone(),
            workspace.clone(),
            vec![writer, dependency.clone()],
            vec![
                repository.join("Cargo.toml"),
                workspace.join("Cargo.toml"),
                workspace.join("Cargo.lock"),
            ],
            test_feature_graph(),
        );

        let before = inventory.bind().unwrap();
        std::fs::write(
            dependency.join("src/lib.rs"),
            b"pub fn changed_dependency() {}\n",
        )
        .unwrap();
        let after = inventory.bind().unwrap();

        assert_ne!(before, after);
    }

    fn test_feature_graph() -> PhysicalWorkFeatureGraphEvidence {
        let node = PhysicalWorkFeatureNodeEvidence::new(
            "bound",
            Vec::<String>::new(),
            Vec::<String>::new(),
        )
        .unwrap();
        PhysicalWorkFeatureGraphEvidence::new(["bound"], [node]).unwrap()
    }
}
