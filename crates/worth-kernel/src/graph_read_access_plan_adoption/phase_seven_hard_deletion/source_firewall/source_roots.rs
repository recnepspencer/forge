use std::path::{Path, PathBuf};

use super::firewall_region::WorthGraphReadAccessHardDeletionSourceRegion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HardDeletionSourceRoot {
    root: PathBuf,
    region: WorthGraphReadAccessHardDeletionSourceRegion,
    root_identity: &'static str,
}

impl HardDeletionSourceRoot {
    fn new(
        root: PathBuf,
        region: WorthGraphReadAccessHardDeletionSourceRegion,
        root_identity: &'static str,
    ) -> Self {
        Self {
            root,
            region,
            root_identity,
        }
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) const fn region(&self) -> WorthGraphReadAccessHardDeletionSourceRegion {
        self.region
    }

    pub(crate) const fn root_identity(&self) -> &'static str {
        self.root_identity
    }
}

pub(crate) fn hard_deletion_source_roots(workspace_root: &Path) -> Vec<HardDeletionSourceRoot> {
    vec![
        HardDeletionSourceRoot::new(
            workspace_root
                .join("crates")
                .join("worth-kernel")
                .join("src")
                .join("graph_read_access_plan_adoption"),
            WorthGraphReadAccessHardDeletionSourceRegion::PlanAdoptionAuthority,
            "crates/worth-kernel/src/graph_read_access_plan_adoption",
        ),
        HardDeletionSourceRoot::new(
            workspace_root.join("crates").join("worth-topo").join("src"),
            WorthGraphReadAccessHardDeletionSourceRegion::TopologyReadConsumers,
            "crates/worth-topo/src",
        ),
        HardDeletionSourceRoot::new(
            workspace_root
                .join("crates")
                .join("worth-spatial")
                .join("src"),
            WorthGraphReadAccessHardDeletionSourceRegion::SpatialReadConsumers,
            "crates/worth-spatial/src",
        ),
        HardDeletionSourceRoot::new(
            workspace_root
                .join("crates")
                .join("worth-kernel")
                .join("src")
                .join("construction"),
            WorthGraphReadAccessHardDeletionSourceRegion::KernelGraphReadHelpers,
            "crates/worth-kernel/src/construction",
        ),
    ]
}

pub(crate) fn should_scan_source_path(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "rs")
        && !path
            .components()
            .any(|component| component.as_os_str() == "tests")
        && path
            .file_name()
            .is_some_and(|file_name| file_name != "tests.rs")
        && !path
            .components()
            .any(|component| component.as_os_str() == "source_firewall")
        && !path
            .components()
            .any(|component| component.as_os_str() == "compile_fail")
}
