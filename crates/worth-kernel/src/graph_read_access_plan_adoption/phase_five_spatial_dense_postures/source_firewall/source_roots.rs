use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthGraphReadAccessSpatialDenseSourceRegion {
    PlanAdoptionAuthority,
    SpatialReadConsumers,
    TopologyReadConsumers,
    StandaloneTestInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadAccessSpatialDenseSourceRoot {
    root: PathBuf,
    region: WorthGraphReadAccessSpatialDenseSourceRegion,
}

impl WorthGraphReadAccessSpatialDenseSourceRoot {
    fn new(root: PathBuf, region: WorthGraphReadAccessSpatialDenseSourceRegion) -> Self {
        Self { root, region }
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) const fn region(&self) -> WorthGraphReadAccessSpatialDenseSourceRegion {
        self.region
    }
}

pub(crate) fn phase_five_source_roots(
    workspace_root: &Path,
) -> Vec<WorthGraphReadAccessSpatialDenseSourceRoot> {
    vec![
        WorthGraphReadAccessSpatialDenseSourceRoot::new(
            workspace_root
                .join("crates")
                .join("worth-kernel")
                .join("src")
                .join("graph_read_access_plan_adoption"),
            WorthGraphReadAccessSpatialDenseSourceRegion::PlanAdoptionAuthority,
        ),
        WorthGraphReadAccessSpatialDenseSourceRoot::new(
            workspace_root
                .join("crates")
                .join("worth-spatial")
                .join("src"),
            WorthGraphReadAccessSpatialDenseSourceRegion::SpatialReadConsumers,
        ),
        WorthGraphReadAccessSpatialDenseSourceRoot::new(
            workspace_root
                .join("crates")
                .join("worth-topo")
                .join("src")
                .join("projection"),
            WorthGraphReadAccessSpatialDenseSourceRegion::TopologyReadConsumers,
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
