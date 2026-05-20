use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeSupportProfile, ForgeQueryWorkspace,
};
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use forge_relational::facade::snapshots::SnapshotHandle;
use forge_runtime_bridge::facade::BridgeBuildError;

use crate::topology_operators::TopologyEditFamily;

use super::adapters::TopologyRuntimeBinding;
use super::edit_support::{
    current_head_edit_family_support_rows, current_head_edit_lane_support_rows,
    snapshot_edit_family_support_rows, snapshot_edit_lane_support_rows,
    TopologyQueryEditFamilySupportStatus, TopologyRuntimeEditFamilySupportRow,
    TopologyRuntimeEditLaneSupportRow,
};
use super::read_support::{
    current_head_query_read_family_support_rows, snapshot_query_read_family_support_rows,
    TopologyRuntimeReadFamilySupportRow,
};
use super::runtime_closeout::{runtime_closeout_from_support_rows, TopologyRuntimeCloseout};
use super::runtime_posture::{
    current_head_runtime_posture_rows, snapshot_runtime_posture_rows,
    TopologyRuntimePostureCapability, TopologyRuntimePostureRow,
};

pub(crate) const TOPOLOGY_SNAPSHOT_HISTORICAL_BASIS_EVIDENCE: &str =
    "topology-snapshot-historical-basis";

#[derive(Debug)]
pub struct TopologyRuntimeAdapters {
    pub(super) binding: TopologyRuntimeBinding,
    support: TopologyRuntimeSupport,
}

impl TopologyRuntimeAdapters {
    pub fn current_head(runtime: RelationalRuntime) -> Self {
        Self {
            binding: TopologyRuntimeBinding::current_head(runtime),
            support: TopologyRuntimeSupport::current_head_authoritative(),
        }
    }

    pub fn snapshot_read_only(read_view: RelationalReadView, snapshot: SnapshotHandle) -> Self {
        Self {
            binding: TopologyRuntimeBinding::snapshot_read_only(read_view, snapshot),
            support: TopologyRuntimeSupport::snapshot_read_only(),
        }
    }

    pub fn support(&self) -> &TopologyRuntimeSupport {
        &self.support
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRuntimeSupport {
    pub(super) runtime_posture_rows: Vec<TopologyRuntimePostureRow>,
    pub(super) query_edit_family_support_rows: Vec<TopologyRuntimeEditFamilySupportRow>,
    pub(super) query_edit_lane_support_rows: Vec<TopologyRuntimeEditLaneSupportRow>,
    pub(super) query_read_family_support_rows: Vec<TopologyRuntimeReadFamilySupportRow>,
    pub(super) closeout: TopologyRuntimeCloseout,
}

impl TopologyRuntimeSupport {
    pub fn current_head_authoritative() -> Self {
        let runtime_posture_rows = current_head_runtime_posture_rows();
        let query_read_family_support_rows = current_head_query_read_family_support_rows();
        let query_edit_family_support_rows = current_head_edit_family_support_rows();
        let query_edit_lane_support_rows = current_head_edit_lane_support_rows();
        Self {
            runtime_posture_rows,
            closeout: runtime_closeout_from_support_rows(
                &query_read_family_support_rows,
                &query_edit_family_support_rows,
                &query_edit_lane_support_rows,
            ),
            query_edit_family_support_rows,
            query_edit_lane_support_rows,
            query_read_family_support_rows,
        }
    }

    pub fn snapshot_read_only() -> Self {
        let runtime_posture_rows = snapshot_runtime_posture_rows();
        let query_read_family_support_rows = snapshot_query_read_family_support_rows();
        let query_edit_family_support_rows = snapshot_edit_family_support_rows();
        let query_edit_lane_support_rows = snapshot_edit_lane_support_rows();
        Self {
            runtime_posture_rows,
            closeout: runtime_closeout_from_support_rows(
                &query_read_family_support_rows,
                &query_edit_family_support_rows,
                &query_edit_lane_support_rows,
            ),
            query_edit_family_support_rows,
            query_edit_lane_support_rows,
            query_read_family_support_rows,
        }
    }

    pub fn query_edit_family_support_status(
        &self,
        family: TopologyEditFamily,
    ) -> TopologyQueryEditFamilySupportStatus {
        self.query_edit_family_support_rows
            .iter()
            .find(|row| row.family() == family)
            .map(TopologyRuntimeEditFamilySupportRow::status)
            .unwrap_or_else(|| {
                panic!(" runtime edit-family support rows should cover every declared family")
            })
    }

    pub(super) fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        let (subscription_activation_evidence, preview_basis_evidence, inspector_evidence) =
            self.runtime_support_profile_evidence();
        let mut profile = ForgeQueryRuntimeSupportProfile::bridge_backed(
            subscription_activation_evidence,
            preview_basis_evidence,
            inspector_evidence,
        );
        if self.supports_posture(TopologyRuntimePostureCapability::AuthoritativeWrites) {
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
        profile = profile.with_family_support(self.branch_preview_support_row());
        if !self.supports_posture(TopologyRuntimePostureCapability::AuthoritativeWrites) {
            profile = profile.with_family_support(ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Write,
                "topology snapshot certification runtime is read-only and does not admit authoritative writes",
            ));
        }
        profile
    }

    pub(super) fn supports_posture(&self, capability: TopologyRuntimePostureCapability) -> bool {
        self.runtime_posture_status(capability).is_admitted()
    }

    pub(super) fn subscription_activation_evidence(&self) -> &'static str {
        self.runtime_support_profile_evidence().0
    }

    pub(super) fn inspector_evidence_label(&self) -> &'static str {
        self.runtime_support_profile_evidence().2
    }

    pub(super) fn write_receipt_evidence_label(&self) -> &'static str {
        if self.supports_posture(TopologyRuntimePostureCapability::HistoricalBasis) {
            "topology-snapshot-write-receipt"
        } else {
            "topology-current-head-write-receipt"
        }
    }

    pub(super) fn preview_basis_denial_reason(&self) -> &'static str {
        if self.supports_posture(TopologyRuntimePostureCapability::HistoricalBasis) {
            "topology snapshot runtime is bound to one historical basis and does not admit preview or branch-local sessions"
        } else {
            "topology production runtime current-head slice does not admit historical or preview bases yet"
        }
    }

    fn branch_preview_support_row(&self) -> ForgeQueryRuntimeFamilySupport {
        if self.supports_posture(TopologyRuntimePostureCapability::HistoricalBasis) {
            return ForgeQueryRuntimeFamilySupport::unsupported_with_evidence(
                ForgeQueryRuntimeFacadeFamily::BranchPreview,
                self.preview_basis_denial_reason(),
                [TOPOLOGY_SNAPSHOT_HISTORICAL_BASIS_EVIDENCE],
            );
        }
        ForgeQueryRuntimeFamilySupport::unsupported(
            ForgeQueryRuntimeFacadeFamily::BranchPreview,
            self.preview_basis_denial_reason(),
        )
    }

    fn runtime_support_profile_evidence(&self) -> (&'static str, &'static str, &'static str) {
        if self.supports_posture(TopologyRuntimePostureCapability::HistoricalBasis) {
            (
                "topology-snapshot-subscription-activation",
                "topology-snapshot-preview-denial",
                "topology-snapshot-inspector-evidence",
            )
        } else {
            (
                "topology-current-head-subscription-activation",
                "topology-current-head-preview-denial",
                "topology-current-head-inspector-evidence",
            )
        }
    }
}

pub(crate) fn workspace_requires_historical_basis_context(workspace: &ForgeQueryWorkspace) -> bool {
    // `workspace.admit_public_api_family(...)` currently reports both ordinary
    // preview denial and snapshot historical-basis denial as the same
    // unsupported-family error. The public family contract is the narrowest
    // workspace-owned surface that still preserves the distinguishing evidence.
    workspace
        .public_api_contract()
        .family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .is_some_and(|contract| {
            contract.status() == ForgeQueryRuntimeFamilySupportStatus::Unsupported
                && contract
                    .evidence()
                    .iter()
                    .any(|evidence| evidence == TOPOLOGY_SNAPSHOT_HISTORICAL_BASIS_EVIDENCE)
        })
}

#[derive(Debug)]
pub enum TopologyRuntimeFailure {
    BridgeBuild(BridgeBuildError),
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for TopologyRuntimeFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BridgeBuild(error) => {
                write!(f, " topology query runtime bridge build failed: {error}")
            }
            Self::QueryRuntime(error) => {
                write!(f, " topology query runtime assembly failed: {error}")
            }
        }
    }
}

impl std::error::Error for TopologyRuntimeFailure {}

impl From<BridgeBuildError> for TopologyRuntimeFailure {
    fn from(value: BridgeBuildError) -> Self {
        Self::BridgeBuild(value)
    }
}

impl From<ForgeQueryRuntimeError> for TopologyRuntimeFailure {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::QueryRuntime(value)
    }
}
