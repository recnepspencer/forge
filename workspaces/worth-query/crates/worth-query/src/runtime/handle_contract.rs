mod construction;

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
