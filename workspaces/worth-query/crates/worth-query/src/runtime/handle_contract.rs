use crate::identity::hash_parts;

use super::{
    WorthQueryAuthorityLane, WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupportStatus,
    WorthQueryRuntimePublicApiContract,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryHandleContractFamily {
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
    TemporalCapableHandle,
    AsyncResourceCapableHandle,
    MixedCauseDeliveryCapableHandle,
}

impl WorthQueryHandleContractFamily {
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
            Self::TemporalCapableHandle => "temporal-capable-handle",
            Self::AsyncResourceCapableHandle => "async-resource-capable-handle",
            Self::MixedCauseDeliveryCapableHandle => "mixed-cause-delivery-capable-handle",
        }
    }
}

impl std::fmt::Display for WorthQueryHandleContractFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryHandleContractRow {
    family: WorthQueryHandleContractFamily,
    authority_lanes: Vec<WorthQueryAuthorityLane>,
    basis_lanes: Vec<WorthQueryAuthorityLane>,
    support_status: WorthQueryRuntimeFamilySupportStatus,
    inspection_sections: Vec<String>,
    retained_artifact_required: bool,
    deferred_future_posture: bool,
    contract_digest: String,
}

impl WorthQueryHandleContractRow {
    fn new(
        family: WorthQueryHandleContractFamily,
        authority_lanes: impl IntoIterator<Item = WorthQueryAuthorityLane>,
        basis_lanes: impl IntoIterator<Item = WorthQueryAuthorityLane>,
        support_status: WorthQueryRuntimeFamilySupportStatus,
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

    pub fn family(&self) -> WorthQueryHandleContractFamily {
        self.family
    }

    pub fn authority_lanes(&self) -> &[WorthQueryAuthorityLane] {
        &self.authority_lanes
    }

    pub fn basis_lanes(&self) -> &[WorthQueryAuthorityLane] {
        &self.basis_lanes
    }

    pub fn support_status(&self) -> WorthQueryRuntimeFamilySupportStatus {
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
pub struct WorthQueryHandleContract {
    rows: Vec<WorthQueryHandleContractRow>,
    support_contract_digest: String,
    inspectable_family_count: usize,
    retained_artifact_family_count: usize,
    deferred_future_family_count: usize,
    contract_digest: String,
}

impl WorthQueryHandleContract {
    pub fn from_public_api_contract(contract: &WorthQueryRuntimePublicApiContract) -> Self {
        let family_status = |family| {
            contract
                .family(family)
                .map(|row| row.status())
                .unwrap_or(WorthQueryRuntimeFamilySupportStatus::Unsupported)
        };
        let rows = vec![
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::LiveView,
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                family_status(WorthQueryRuntimeFacadeFamily::Live),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::ComputedView,
                [WorthQueryAuthorityLane::DerivedRuntimeState],
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                family_status(WorthQueryRuntimeFacadeFamily::Computed),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::Effect,
                [
                    WorthQueryAuthorityLane::EffectDeliveryState,
                    WorthQueryAuthorityLane::PendingWriteIntent,
                ],
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                family_status(WorthQueryRuntimeFacadeFamily::Effect),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::WriteReceipt,
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                family_status(WorthQueryRuntimeFacadeFamily::Write),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::BatchWriteReceipt,
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::PreviewTruth,
                ],
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::PreviewTruth,
                ],
                family_status(WorthQueryRuntimeFacadeFamily::Write),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::IntentReceipt,
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                family_status(WorthQueryRuntimeFacadeFamily::Intent),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::IntentDenial,
                [WorthQueryAuthorityLane::PendingWriteIntent],
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                family_status(WorthQueryRuntimeFacadeFamily::Intent),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::EffectIntentReceipt,
                [WorthQueryAuthorityLane::PendingWriteIntent],
                [WorthQueryAuthorityLane::AuthoritativeTruth],
                family_status(WorthQueryRuntimeFacadeFamily::Intent),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::PreviewBinding,
                [
                    WorthQueryAuthorityLane::PreviewTruth,
                    WorthQueryAuthorityLane::DerivedRuntimeState,
                    WorthQueryAuthorityLane::EffectDeliveryState,
                ],
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::PreviewTruth,
                ],
                family_status(WorthQueryRuntimeFacadeFamily::BranchPreview),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::PreviewOutcome,
                [WorthQueryAuthorityLane::PreviewTruth],
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::PreviewTruth,
                ],
                family_status(WorthQueryRuntimeFacadeFamily::BranchPreview),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::PreviewIntentReceipt,
                [WorthQueryAuthorityLane::PreviewTruth],
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::PreviewTruth,
                ],
                family_status(WorthQueryRuntimeFacadeFamily::Intent),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::BranchBinding,
                [
                    WorthQueryAuthorityLane::BranchLocalTruth,
                    WorthQueryAuthorityLane::DerivedRuntimeState,
                    WorthQueryAuthorityLane::EffectDeliveryState,
                ],
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::BranchLocalTruth,
                ],
                WorthQueryRuntimeFamilySupportStatus::Unsupported,
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::BranchIntentReceipt,
                [WorthQueryAuthorityLane::BranchLocalTruth],
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::BranchLocalTruth,
                ],
                family_status(WorthQueryRuntimeFacadeFamily::Intent),
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
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::TemporalCapableHandle,
                [WorthQueryAuthorityLane::TemporalExecutionState],
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::TemporalExecutionState,
                ],
                family_status(WorthQueryRuntimeFacadeFamily::Temporal),
                [
                    "support-posture",
                    "state-kind",
                    "basis-identity",
                    "temporal-readiness-neighbor",
                    "authority-lane",
                    "basis-lane",
                    "future-inspection-digest",
                    "inspection-digest",
                ],
                false,
                true,
            ),
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::AsyncResourceCapableHandle,
                [WorthQueryAuthorityLane::AsyncResourceState],
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::AsyncResourceState,
                ],
                family_status(WorthQueryRuntimeFacadeFamily::AsyncResource),
                [
                    "support-posture",
                    "state-kind",
                    "basis-identity",
                    "async-request-identity",
                    "authority-lane",
                    "basis-lane",
                    "future-inspection-digest",
                    "inspection-digest",
                ],
                false,
                true,
            ),
            WorthQueryHandleContractRow::new(
                WorthQueryHandleContractFamily::MixedCauseDeliveryCapableHandle,
                [WorthQueryAuthorityLane::BridgeExternalState],
                [
                    WorthQueryAuthorityLane::AuthoritativeTruth,
                    WorthQueryAuthorityLane::TemporalExecutionState,
                    WorthQueryAuthorityLane::AsyncResourceState,
                    WorthQueryAuthorityLane::BridgeExternalState,
                ],
                family_status(WorthQueryRuntimeFacadeFamily::MixedCauseDelivery),
                [
                    "support-posture",
                    "state-kind",
                    "basis-identity",
                    "mixed-cause-ordering-neighbor",
                    "authority-lane",
                    "basis-lane",
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
            "worth_query_handle_contract_v1".to_string(),
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

    pub fn rows(&self) -> &[WorthQueryHandleContractRow] {
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
        family: WorthQueryHandleContractFamily,
    ) -> Option<&WorthQueryHandleContractRow> {
        self.rows.iter().find(|row| row.family() == family)
    }
}
