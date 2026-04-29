use crate::identity::hash_parts;

use super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus,
    ForgeQueryRuntimePublicApiContract,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryHandleContractFamily {
    LiveView,
    ComputedView,
    Effect,
    WriteReceipt,
    BatchWriteReceipt,
    IntentReceipt,
    IntentDenial,
    EffectIntentReceipt,
    PreviewBinding,
    PreviewOutcome,
    PreviewIntentReceipt,
    BranchBinding,
    BranchIntentReceipt,
    TemporalAsyncCapableHandle,
}

impl ForgeQueryHandleContractFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveView => "live-view",
            Self::ComputedView => "computed-view",
            Self::Effect => "effect",
            Self::WriteReceipt => "write-receipt",
            Self::BatchWriteReceipt => "batch-write-receipt",
            Self::IntentReceipt => "intent-receipt",
            Self::IntentDenial => "intent-denial",
            Self::EffectIntentReceipt => "effect-intent-receipt",
            Self::PreviewBinding => "preview-binding",
            Self::PreviewOutcome => "preview-outcome",
            Self::PreviewIntentReceipt => "preview-intent-receipt",
            Self::BranchBinding => "branch-binding",
            Self::BranchIntentReceipt => "branch-intent-receipt",
            Self::TemporalAsyncCapableHandle => "temporal-async-capable-handle",
        }
    }
}

impl std::fmt::Display for ForgeQueryHandleContractFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryHandleContractRow {
    family: ForgeQueryHandleContractFamily,
    authority_lanes: Vec<ForgeQueryAuthorityLane>,
    basis_lanes: Vec<ForgeQueryAuthorityLane>,
    support_status: ForgeQueryRuntimeFamilySupportStatus,
    inspection_sections: Vec<String>,
    retained_artifact_required: bool,
    deferred_future_posture: bool,
    contract_digest: String,
}

impl ForgeQueryHandleContractRow {
    fn new(
        family: ForgeQueryHandleContractFamily,
        authority_lanes: impl IntoIterator<Item = ForgeQueryAuthorityLane>,
        basis_lanes: impl IntoIterator<Item = ForgeQueryAuthorityLane>,
        support_status: ForgeQueryRuntimeFamilySupportStatus,
        inspection_sections: impl IntoIterator<Item = impl Into<String>>,
        retained_artifact_required: bool,
        deferred_future_posture: bool,
    ) -> Self {
        let authority_lanes = authority_lanes.into_iter().collect::<Vec<_>>();
        let basis_lanes = basis_lanes.into_iter().collect::<Vec<_>>();
        let inspection_sections = inspection_sections
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let mut parts = vec![
            format!("family:{}", family.as_str()),
            format!("support:{}", support_status.as_str()),
            format!("retained:{retained_artifact_required}"),
            format!("deferred:{deferred_future_posture}"),
        ];
        parts.extend(
            authority_lanes
                .iter()
                .map(|lane| format!("lane:{}", lane.as_str())),
        );
        parts.extend(
            basis_lanes
                .iter()
                .map(|lane| format!("basis:{}", lane.as_str())),
        );
        parts.extend(
            inspection_sections
                .iter()
                .map(|section| format!("section:{section}")),
        );
        let contract_digest = hash_parts(&parts);
        Self {
            family,
            authority_lanes,
            basis_lanes,
            support_status,
            inspection_sections,
            retained_artifact_required,
            deferred_future_posture,
            contract_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryHandleContractFamily {
        self.family
    }

    pub fn authority_lanes(&self) -> &[ForgeQueryAuthorityLane] {
        &self.authority_lanes
    }

    pub fn basis_lanes(&self) -> &[ForgeQueryAuthorityLane] {
        &self.basis_lanes
    }

    pub fn support_status(&self) -> ForgeQueryRuntimeFamilySupportStatus {
        self.support_status
    }

    pub fn inspection_sections(&self) -> &[String] {
        &self.inspection_sections
    }

    pub fn retained_artifact_required(&self) -> bool {
        self.retained_artifact_required
    }

    pub fn deferred_future_posture(&self) -> bool {
        self.deferred_future_posture
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryHandleContract {
    rows: Vec<ForgeQueryHandleContractRow>,
    support_contract_digest: String,
    inspectable_family_count: usize,
    retained_artifact_family_count: usize,
    deferred_future_family_count: usize,
    contract_digest: String,
}

impl ForgeQueryHandleContract {
    pub fn from_public_api_contract(contract: &ForgeQueryRuntimePublicApiContract) -> Self {
        let family_status = |family| {
            contract
                .family(family)
                .map(|row| row.status())
                .unwrap_or(ForgeQueryRuntimeFamilySupportStatus::Unsupported)
        };
        let rows = vec![
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::LiveView,
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                family_status(ForgeQueryRuntimeFacadeFamily::Live),
                [
                    "declaration-identity",
                    "query-result-shape",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "subscription-lifecycle",
                    "delivery-counters",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::ComputedView,
                [ForgeQueryAuthorityLane::DerivedRuntimeState],
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                family_status(ForgeQueryRuntimeFacadeFamily::Computed),
                [
                    "declaration-identity",
                    "dependency-aspects",
                    "produced-aspects",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "materialization",
                    "pending-patches",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::Effect,
                [
                    ForgeQueryAuthorityLane::EffectDeliveryState,
                    ForgeQueryAuthorityLane::PendingWriteIntent,
                ],
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                family_status(ForgeQueryRuntimeFacadeFamily::Effect),
                [
                    "declaration-identity",
                    "trigger-aspects",
                    "condition",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "delivery-residue",
                    "feedback-phase-graph",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::WriteReceipt,
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                family_status(ForgeQueryRuntimeFacadeFamily::Write),
                [
                    "mutation-family",
                    "declared-target",
                    "commit-identity",
                    "snapshot-token",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "runtime-evidence",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::BatchWriteReceipt,
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::PreviewTruth,
                ],
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::PreviewTruth,
                ],
                family_status(ForgeQueryRuntimeFacadeFamily::Write),
                [
                    "batch-digest",
                    "write-receipt-count",
                    "component-operations",
                    "touched-aspects",
                    "affected-surfaces",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::IntentReceipt,
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                family_status(ForgeQueryRuntimeFacadeFamily::Intent),
                [
                    "strategy-identity",
                    "source-target-lanes",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "outcome",
                    "delivery-counters",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::IntentDenial,
                [ForgeQueryAuthorityLane::PendingWriteIntent],
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                family_status(ForgeQueryRuntimeFacadeFamily::Intent),
                [
                    "strategy-identity",
                    "source-target-lanes",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "denial-kind",
                    "returned-strategy",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::EffectIntentReceipt,
                [ForgeQueryAuthorityLane::PendingWriteIntent],
                [ForgeQueryAuthorityLane::AuthoritativeTruth],
                family_status(ForgeQueryRuntimeFacadeFamily::Intent),
                [
                    "effect-source",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "phase-evidence",
                    "intent-receipt",
                    "feedback-phase-graph",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::PreviewBinding,
                [
                    ForgeQueryAuthorityLane::PreviewTruth,
                    ForgeQueryAuthorityLane::DerivedRuntimeState,
                    ForgeQueryAuthorityLane::EffectDeliveryState,
                ],
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::PreviewTruth,
                ],
                family_status(ForgeQueryRuntimeFacadeFamily::BranchPreview),
                [
                    "basis-evidence",
                    "binding-family",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "effect-policy",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::PreviewOutcome,
                [ForgeQueryAuthorityLane::PreviewTruth],
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::PreviewTruth,
                ],
                family_status(ForgeQueryRuntimeFacadeFamily::BranchPreview),
                [
                    "basis-evidence",
                    "execution-kind",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "residue-counters",
                    "closeout-posture",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::PreviewIntentReceipt,
                [ForgeQueryAuthorityLane::PreviewTruth],
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::PreviewTruth,
                ],
                family_status(ForgeQueryRuntimeFacadeFamily::Intent),
                [
                    "basis-evidence",
                    "source-target-lanes",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "effect-policy",
                    "pending-intent-residue",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::BranchBinding,
                [
                    ForgeQueryAuthorityLane::BranchLocalTruth,
                    ForgeQueryAuthorityLane::DerivedRuntimeState,
                    ForgeQueryAuthorityLane::EffectDeliveryState,
                ],
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::BranchLocalTruth,
                ],
                ForgeQueryRuntimeFamilySupportStatus::Unsupported,
                [
                    "unsupported-branch-handle-reuse",
                    "basis-lane",
                    "authority-lane",
                    "support-posture",
                    "future-inspection-digest",
                    "inspection-digest",
                ],
                false,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::BranchIntentReceipt,
                [ForgeQueryAuthorityLane::BranchLocalTruth],
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::BranchLocalTruth,
                ],
                family_status(ForgeQueryRuntimeFacadeFamily::Intent),
                [
                    "basis-evidence",
                    "source-target-lanes",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "effect-policy",
                    "branch-local-residue",
                    "inspection-digest",
                ],
                true,
                false,
            ),
            ForgeQueryHandleContractRow::new(
                ForgeQueryHandleContractFamily::TemporalAsyncCapableHandle,
                [
                    ForgeQueryAuthorityLane::TemporalExecutionState,
                    ForgeQueryAuthorityLane::AsyncResourceState,
                    ForgeQueryAuthorityLane::BridgeExternalState,
                ],
                [
                    ForgeQueryAuthorityLane::AuthoritativeTruth,
                    ForgeQueryAuthorityLane::TemporalExecutionState,
                    ForgeQueryAuthorityLane::AsyncResourceState,
                ],
                ForgeQueryRuntimeFamilySupportStatus::DeferredDebt,
                [
                    "deferred-support-posture",
                    "state-kind",
                    "basis-identity",
                    "authority-lane",
                    "basis-lane",
                    "support-posture",
                    "future-inspection-digest",
                    "inspection-digest",
                ],
                false,
                true,
            ),
        ];
        let inspectable_family_count = rows.len();
        let retained_artifact_family_count = rows
            .iter()
            .filter(|row| row.retained_artifact_required())
            .count();
        let deferred_future_family_count = rows
            .iter()
            .filter(|row| row.deferred_future_posture())
            .count();
        let support_contract_digest = contract.contract_digest().to_string();
        let mut parts = vec![
            "forge_query_handle_contract_v1".to_string(),
            format!("support:{support_contract_digest}"),
            format!("inspectable:{inspectable_family_count}"),
            format!("retained:{retained_artifact_family_count}"),
            format!("deferred:{deferred_future_family_count}"),
        ];
        parts.extend(rows.iter().map(|row| row.contract_digest().to_string()));
        let contract_digest = hash_parts(&parts);
        Self {
            rows,
            support_contract_digest,
            inspectable_family_count,
            retained_artifact_family_count,
            deferred_future_family_count,
            contract_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryHandleContractRow] {
        &self.rows
    }

    pub fn support_contract_digest(&self) -> &str {
        &self.support_contract_digest
    }

    pub fn inspectable_family_count(&self) -> usize {
        self.inspectable_family_count
    }

    pub fn retained_artifact_family_count(&self) -> usize {
        self.retained_artifact_family_count
    }

    pub fn deferred_future_family_count(&self) -> usize {
        self.deferred_future_family_count
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn row(
        &self,
        family: ForgeQueryHandleContractFamily,
    ) -> Option<&ForgeQueryHandleContractRow> {
        self.rows.iter().find(|row| row.family() == family)
    }
}
