use topology::certification::{
    current_topology_public_facade_compile_fail_closeout, TopologyPublicFacadeCompileFailCloseout,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::certification::{
    current_spatial_public_facade_compile_fail_closeout, SpatialPublicFacadeCompileFailCloseout,
};

use super::{
    current_worth_touched_graph_conflict_source_firewall_report,
    WorthTouchedGraphConflictForbiddenSurface, WorthTouchedGraphConflictSourceFirewallReport,
};
use crate::workload_composition::{
    current_worth_touched_graph_conflict_deletion_closeout,
    WorthTouchedGraphConflictDeletionCloseout,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind {
    SourceFirewallViolation,
    MissingPhaseFifteenCoverage,
    DeletionCloseoutMismatch,
    PublicFacadeCertificationMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictSourceFirewallCloseoutError {
    kind: WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictSourceFirewallCloseout {
    source_firewall_report_digest: String,
    deletion_closeout_digest: String,
    topology_public_facade_compile_fail_digest: String,
    spatial_public_facade_compile_fail_digest: String,
    covered_forbidden_surfaces: Vec<WorthTouchedGraphConflictForbiddenSurface>,
    closeout_digest: String,
}

pub fn current_worth_touched_graph_conflict_source_firewall_closeout() -> Result<
    WorthTouchedGraphConflictSourceFirewallCloseout,
    WorthTouchedGraphConflictSourceFirewallCloseoutError,
> {
    let source_firewall_report = current_worth_touched_graph_conflict_source_firewall_report()
        .map_err(|error| {
            WorthTouchedGraphConflictSourceFirewallCloseoutError::new(
                WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind::SourceFirewallViolation,
                format!("phase 15 source firewall did not load: {error:?}"),
            )
        })?;
    let deletion_closeout =
        current_worth_touched_graph_conflict_deletion_closeout().map_err(|error| {
            WorthTouchedGraphConflictSourceFirewallCloseoutError::new(
                WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind::DeletionCloseoutMismatch,
                format!("phase 15 deletion closeout did not load: {error:?}"),
            )
        })?;
    let topology_public_facade_closeout = current_topology_public_facade_compile_fail_closeout()
        .map_err(|error| {
            WorthTouchedGraphConflictSourceFirewallCloseoutError::new(
                WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind::PublicFacadeCertificationMismatch,
                format!("phase 15 topology public-facade compile-fail closeout did not load: {error:?}"),
            )
        })?;
    let spatial_public_facade_closeout = current_spatial_public_facade_compile_fail_closeout()
        .map_err(|error| {
            WorthTouchedGraphConflictSourceFirewallCloseoutError::new(
                WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind::PublicFacadeCertificationMismatch,
                format!("phase 15 spatial public-facade compile-fail closeout did not load: {error:?}"),
            )
        })?;
    closeout_from_products(
        &source_firewall_report,
        &deletion_closeout,
        &topology_public_facade_closeout,
        &spatial_public_facade_closeout,
    )
}

impl WorthTouchedGraphConflictSourceFirewallCloseoutError {
    fn new(kind: WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind, detail: String) -> Self {
        Self { kind, detail }
    }

    pub const fn kind(&self) -> WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl WorthTouchedGraphConflictSourceFirewallCloseout {
    pub fn source_firewall_report_digest(&self) -> &str {
        &self.source_firewall_report_digest
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        &self.deletion_closeout_digest
    }

    pub fn covered_forbidden_surfaces(&self) -> &[WorthTouchedGraphConflictForbiddenSurface] {
        &self.covered_forbidden_surfaces
    }

    pub fn topology_public_facade_compile_fail_digest(&self) -> &str {
        &self.topology_public_facade_compile_fail_digest
    }

    pub fn spatial_public_facade_compile_fail_digest(&self) -> &str {
        &self.spatial_public_facade_compile_fail_digest
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

pub(crate) fn closeout_from_products(
    source_firewall_report: &WorthTouchedGraphConflictSourceFirewallReport,
    deletion_closeout: &WorthTouchedGraphConflictDeletionCloseout,
    topology_public_facade_closeout: &TopologyPublicFacadeCompileFailCloseout,
    spatial_public_facade_closeout: &SpatialPublicFacadeCompileFailCloseout,
) -> Result<
    WorthTouchedGraphConflictSourceFirewallCloseout,
    WorthTouchedGraphConflictSourceFirewallCloseoutError,
> {
    if !source_firewall_report.violations().is_empty() {
        return Err(WorthTouchedGraphConflictSourceFirewallCloseoutError::new(
            WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind::SourceFirewallViolation,
            "phase 15 source firewall closeout found forbidden semantic relapse".to_string(),
        ));
    }
    if deletion_closeout.source_firewall_report_digest() != source_firewall_report.report_digest() {
        return Err(WorthTouchedGraphConflictSourceFirewallCloseoutError::new(
            WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind::DeletionCloseoutMismatch,
            "phase 15 deletion closeout must bind the same source firewall report digest"
                .to_string(),
        ));
    }
    let covered_forbidden_surfaces = source_firewall_report
        .covered_forbidden_surfaces()
        .into_iter()
        .collect::<Vec<_>>();
    for forbidden_surface in
        WorthTouchedGraphConflictForbiddenSurface::phase_fifteen_relapse_categories()
    {
        if !covered_forbidden_surfaces.contains(forbidden_surface) {
            return Err(WorthTouchedGraphConflictSourceFirewallCloseoutError::new(
                WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind::MissingPhaseFifteenCoverage,
                format!(
                    "phase 15 source firewall closeout requires covered forbidden surface `{}`",
                    forbidden_surface.as_str()
                ),
            ));
        }
    }
    let covered_public_fence_classes = topology_public_facade_closeout
        .covered_fence_classes()
        .iter()
        .chain(
            spatial_public_facade_closeout
                .covered_fence_classes()
                .iter(),
        )
        .map(String::as_str)
        .collect::<Vec<_>>();
    for required_class in [
        "family-record",
        "admitted-input",
        "selected-equivalence-family",
        "closeout-product",
        "compiled-product-identity",
        "equivalence-policy-identity",
        "reuse-decision",
        "rebuild-denial",
    ] {
        if !covered_public_fence_classes.contains(&required_class) {
            return Err(WorthTouchedGraphConflictSourceFirewallCloseoutError::new(
                WorthTouchedGraphConflictSourceFirewallCloseoutErrorKind::PublicFacadeCertificationMismatch,
                format!(
                    "phase 15 source firewall closeout requires public-facade compile-fail fence `{required_class}`"
                ),
            ));
        }
    }
    let source_firewall_report_digest = source_firewall_report.report_digest().to_string();
    let deletion_closeout_digest = deletion_closeout.closeout_digest().to_string();
    let topology_public_facade_compile_fail_digest = topology_public_facade_closeout
        .closeout_digest()
        .to_string();
    let spatial_public_facade_compile_fail_digest =
        spatial_public_facade_closeout.closeout_digest().to_string();
    let closeout_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:touched-graph-conflict-source-firewall-closeout:v1".to_string(),
            format!("source-firewall:{source_firewall_report_digest}"),
            format!("deletion-closeout:{deletion_closeout_digest}"),
            format!("topology-public-facade:{topology_public_facade_compile_fail_digest}"),
            format!("spatial-public-facade:{spatial_public_facade_compile_fail_digest}"),
        ],
    );
    Ok(WorthTouchedGraphConflictSourceFirewallCloseout {
        source_firewall_report_digest,
        deletion_closeout_digest,
        topology_public_facade_compile_fail_digest,
        spatial_public_facade_compile_fail_digest,
        covered_forbidden_surfaces,
        closeout_digest,
    })
}
