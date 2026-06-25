use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SourceFirewallRegion {
    DeclarationAuthority,
    WorthKernelAdoptionAuthority,
    TopologySpatialReadHelpers,
}

impl SourceFirewallRegion {
    pub const fn digest_part(self) -> &'static str {
        match self {
            Self::DeclarationAuthority => "declaration_authority",
            Self::WorthKernelAdoptionAuthority => "worth_kernel_adoption_authority",
            Self::TopologySpatialReadHelpers => "topology_spatial_read_helpers",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceFirewallRoot {
    root: PathBuf,
    region: SourceFirewallRegion,
}

impl SourceFirewallRoot {
    fn new(root: PathBuf, region: SourceFirewallRegion) -> Self {
        Self { root, region }
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) const fn region(&self) -> SourceFirewallRegion {
        self.region
    }
}

pub(crate) fn declaration_firewall_source_roots(workspace_root: &Path) -> Vec<SourceFirewallRoot> {
    vec![
        SourceFirewallRoot::new(
            workspace_root
                .join("crates")
                .join("worth-kernel")
                .join("src")
                .join("graph_read_access_declarations"),
            SourceFirewallRegion::DeclarationAuthority,
        ),
        SourceFirewallRoot::new(
            workspace_root
                .join("crates")
                .join("worth-kernel")
                .join("src")
                .join("query_adoption")
                .join("graph_read_access"),
            SourceFirewallRegion::WorthKernelAdoptionAuthority,
        ),
        SourceFirewallRoot::new(
            workspace_root
                .join("crates")
                .join("worth-topo")
                .join("src")
                .join("projection"),
            SourceFirewallRegion::TopologySpatialReadHelpers,
        ),
        SourceFirewallRoot::new(
            workspace_root
                .join("crates")
                .join("worth-spatial")
                .join("src"),
            SourceFirewallRegion::TopologySpatialReadHelpers,
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
