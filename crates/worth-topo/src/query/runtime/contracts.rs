use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeSupportProfile,
};
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use forge_relational::facade::snapshots::SnapshotHandle;
use forge_runtime_bridge::facade::BridgeBuildError;

use crate::edit::WorthTopologyEditFamily;

use super::adapters::WorthTopologyRuntimeBinding;

/// Public support status for a Worth topology edit family on the bridge-backed
/// Query runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorthTopologyQueryEditFamilySupportStatus {
    /// No admitted lane exists for this family on the current runtime posture.
    Denied,
    /// One or more named lanes are admitted, but the family is not generally
    /// open beyond those lanes.
    PartiallyAdmittedByLane,
    /// The family is admitted without lane-specific narrowing.
    Admitted,
}

#[derive(Debug)]
pub struct WorthTopologyRuntimeAdapters {
    pub(super) binding: WorthTopologyRuntimeBinding,
    support: WorthTopologyRuntimeSupport,
}

impl WorthTopologyRuntimeAdapters {
    pub fn current_head(runtime: RelationalRuntime) -> Self {
        Self {
            binding: WorthTopologyRuntimeBinding::current_head(runtime),
            support: WorthTopologyRuntimeSupport::current_head_authoritative(),
        }
    }

    pub fn snapshot_read_only(read_view: RelationalReadView, snapshot: SnapshotHandle) -> Self {
        Self {
            binding: WorthTopologyRuntimeBinding::snapshot_read_only(read_view, snapshot),
            support: WorthTopologyRuntimeSupport::snapshot_read_only(),
        }
    }

    pub fn support(&self) -> &WorthTopologyRuntimeSupport {
        &self.support
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthTopologyRuntimeSupport {
    current_head_live_reads_supported: bool,
    current_head_materialization_supported: bool,
    post_write_materialization_supported: bool,
    historical_basis_supported: bool,
    authoritative_writes_supported: bool,
    admitted_query_edit_families: Vec<WorthTopologyEditFamily>,
    partially_admitted_query_edit_families: Vec<WorthTopologyEditFamily>,
    admitted_query_edit_lanes: Vec<&'static str>,
}

impl WorthTopologyRuntimeSupport {
    pub fn current_head_authoritative() -> Self {
        Self {
            current_head_live_reads_supported: true,
            current_head_materialization_supported: false,
            post_write_materialization_supported: true,
            historical_basis_supported: false,
            authoritative_writes_supported: true,
            admitted_query_edit_families: vec![
                WorthTopologyEditFamily::CreateTopologyEntity,
                WorthTopologyEditFamily::DetachBoundaryMembership,
                WorthTopologyEditFamily::DetachRadialAdjacency,
                WorthTopologyEditFamily::DetachShellOrWireMembership,
                WorthTopologyEditFamily::RewireLoopEndpoint,
                WorthTopologyEditFamily::SpliceRadialAdjacency,
                WorthTopologyEditFamily::RetireTopologyEntity,
            ],
            partially_admitted_query_edit_families: vec![
                WorthTopologyEditFamily::AttachBoundaryMembership,
                WorthTopologyEditFamily::AttachShellOrWireMembership,
                WorthTopologyEditFamily::RewireLoopSuccessor,
            ],
            admitted_query_edit_lanes: vec![
                "CreateTopologyEntity",
                "CreateInnerLoopOnExistingFace",
                "RehomeAllOwnedHalfEdgesToNewWire",
                "SplitConnectedHalfEdgeSetIntoNewWire",
                "SplitSingleFaceFromTwoFaceShellToNewShell",
                "RehomeAllOwnedFacesToNewShell",
                "RetireTopologyEntity",
                "DetachBoundaryMembership",
                "DetachRadialAdjacency",
                "DetachShellOrWireMembership",
                "RelocateHalfEdgeBeforeSuccessor",
                "RelocateHalfEdgeSpanBeforeSuccessor",
                "RewireLoopEndpoint",
                "SpliceRadialAdjacency",
            ],
        }
    }

    pub fn snapshot_read_only() -> Self {
        Self {
            current_head_live_reads_supported: false,
            current_head_materialization_supported: false,
            post_write_materialization_supported: false,
            historical_basis_supported: true,
            authoritative_writes_supported: false,
            admitted_query_edit_families: Vec::new(),
            partially_admitted_query_edit_families: Vec::new(),
            admitted_query_edit_lanes: Vec::new(),
        }
    }

    pub fn current_head_live_reads_supported(&self) -> bool {
        self.current_head_live_reads_supported
    }

    pub fn current_head_materialization_supported(&self) -> bool {
        self.current_head_materialization_supported
    }

    pub fn post_write_materialization_supported(&self) -> bool {
        self.post_write_materialization_supported
    }

    pub fn historical_basis_supported(&self) -> bool {
        self.historical_basis_supported
    }

    pub fn authoritative_writes_supported(&self) -> bool {
        self.authoritative_writes_supported
    }

    pub fn query_edit_execution_supported(&self) -> bool {
        !self.admitted_query_edit_lanes.is_empty()
    }

    /// Families admitted without lane-specific narrowing.
    pub fn admitted_query_edit_families(&self) -> &[WorthTopologyEditFamily] {
        &self.admitted_query_edit_families
    }

    /// Families that are supported only through specific admitted lanes.
    pub fn partially_admitted_query_edit_families(&self) -> &[WorthTopologyEditFamily] {
        &self.partially_admitted_query_edit_families
    }

    /// Named admitted edit lanes on the current runtime posture.
    pub fn admitted_query_edit_lanes(&self) -> &[&'static str] {
        &self.admitted_query_edit_lanes
    }

    pub fn query_edit_lane_supported(&self, lane: &str) -> bool {
        self.admitted_query_edit_lanes.contains(&lane)
    }

    /// Returns `true` when at least one admitted lane exists for the family.
    ///
    /// Callers that need to distinguish fully admitted families from
    /// lane-specific support must inspect `query_edit_family_support_status`.
    pub fn query_edit_family_supported(&self, family: WorthTopologyEditFamily) -> bool {
        self.query_edit_family_support_status(family)
            != WorthTopologyQueryEditFamilySupportStatus::Denied
    }

    pub fn query_edit_family_support_status(
        &self,
        family: WorthTopologyEditFamily,
    ) -> WorthTopologyQueryEditFamilySupportStatus {
        if self.admitted_query_edit_families.contains(&family) {
            WorthTopologyQueryEditFamilySupportStatus::Admitted
        } else if self
            .partially_admitted_query_edit_families
            .contains(&family)
        {
            WorthTopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane
        } else {
            WorthTopologyQueryEditFamilySupportStatus::Denied
        }
    }

    pub(super) fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        let mut profile = ForgeQueryRuntimeSupportProfile::bridge_backed(
            "worth-topology-current-head-subscription-activation",
            "worth-topology-current-head-preview-denial",
            "worth-topology-current-head-inspector-evidence",
        );
        if self.authoritative_writes_supported {
            for operation_family in [
                "verify_existing",
                "probe_existing",
                "delete_existing_verified",
            ] {
                for target_binding_family in ["direct_entity_identity", "direct_relation_identity"]
                {
                    profile = profile.with_bridge_backed_verification_support(
                        operation_family,
                        target_binding_family,
                        true,
                        true,
                        None,
                    );
                }
            }
            profile = profile.with_bridge_backed_verification_support(
                "update_existing_verified",
                "direct_relation_identity",
                true,
                true,
                None,
            );
        }
        if !self.historical_basis_supported {
            profile = profile.with_family_support(ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::BranchPreview,
                "worth-topology production runtime current-head slice does not admit historical or preview bases yet",
            ));
        }
        if !self.authoritative_writes_supported {
            profile = profile.with_family_support(ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Write,
                "worth-topology snapshot certification runtime is read-only and does not admit authoritative writes",
            ));
        }
        profile
    }
}

#[derive(Debug)]
pub enum WorthTopologyRuntimeFailure {
    BridgeBuild(BridgeBuildError),
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for WorthTopologyRuntimeFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BridgeBuild(error) => {
                write!(
                    f,
                    "worth topology query runtime bridge build failed: {error}"
                )
            }
            Self::QueryRuntime(error) => {
                write!(f, "worth topology query runtime assembly failed: {error}")
            }
        }
    }
}

impl std::error::Error for WorthTopologyRuntimeFailure {}

impl From<BridgeBuildError> for WorthTopologyRuntimeFailure {
    fn from(value: BridgeBuildError) -> Self {
        Self::BridgeBuild(value)
    }
}

impl From<ForgeQueryRuntimeError> for WorthTopologyRuntimeFailure {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::QueryRuntime(value)
    }
}
