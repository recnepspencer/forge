#![allow(dead_code)]

use forge_relational::facade::history::BranchId;
use serde::{Deserialize, Serialize};

use super::PlacementObservationScopeClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TierResidenceClass {
    Hot,
    Warm,
    Cold,
}

impl TierResidenceClass {
    pub fn label(self) -> &'static str {
        match self {
            Self::Hot => "hot",
            Self::Warm => "warm",
            Self::Cold => "cold",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "hot" => Some(Self::Hot),
            "warm" => Some(Self::Warm),
            "cold" => Some(Self::Cold),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlacementBudgetClass {
    ForegroundResidentOnly,
    ForegroundBoundedRecall,
    BackgroundOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecallCostClass {
    Inline,
    Bounded,
    Deferred,
}

impl RecallCostClass {
    pub fn label(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Bounded => "bounded",
            Self::Deferred => "deferred",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "inline" => Some(Self::Inline),
            "bounded" => Some(Self::Bounded),
            "deferred" => Some(Self::Deferred),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlacementExecutionOrigin {
    Foreground,
    Background,
    RestartRecovery,
}

impl PlacementExecutionOrigin {
    pub fn label(self) -> &'static str {
        match self {
            Self::Foreground => "foreground",
            Self::Background => "background",
            Self::RestartRecovery => "restart_recovery",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "foreground" => Some(Self::Foreground),
            "background" => Some(Self::Background),
            "restart_recovery" => Some(Self::RestartRecovery),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecallAmplificationBudget {
    SingleFamilyLocalUnit,
    BroadenedPlanRequired,
}

impl RecallAmplificationBudget {
    pub fn label(self) -> &'static str {
        match self {
            Self::SingleFamilyLocalUnit => "single_family_local_unit",
            Self::BroadenedPlanRequired => "broadened_plan_required",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "single_family_local_unit" => Some(Self::SingleFamilyLocalUnit),
            "broadened_plan_required" => Some(Self::BroadenedPlanRequired),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HotnessClassificationVerdict {
    Hot,
    Warm,
    CoolingDebt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlacementArtifactFamily {
    AuthoritativeBranchHead,
    RetainedAuthority,
    StableBasis,
    SnapshotFamily,
    BranchDeltaFamily,
    Milestone6LayoutFamily,
}

impl PlacementArtifactFamily {
    pub fn label(self) -> &'static str {
        match self {
            Self::AuthoritativeBranchHead => "authoritative_branch_head",
            Self::RetainedAuthority => "retained_authority",
            Self::StableBasis => "stable_basis",
            Self::SnapshotFamily => "snapshot_family",
            Self::BranchDeltaFamily => "branch_delta_family",
            Self::Milestone6LayoutFamily => "milestone6_layout_family",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "authoritative_branch_head" => Some(Self::AuthoritativeBranchHead),
            "retained_authority" => Some(Self::RetainedAuthority),
            "stable_basis" => Some(Self::StableBasis),
            "snapshot_family" => Some(Self::SnapshotFamily),
            "branch_delta_family" => Some(Self::BranchDeltaFamily),
            "milestone6_layout_family" => Some(Self::Milestone6LayoutFamily),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoritativeTierResidency {
    branch_id: BranchId,
    residence_class: TierResidenceClass,
}

impl AuthoritativeTierResidency {
    pub fn new(branch_id: BranchId, residence_class: TierResidenceClass) -> Self {
        Self {
            branch_id,
            residence_class,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn residence_class(&self) -> TierResidenceClass {
        self.residence_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedTierResidency {
    artifact_family: PlacementArtifactFamily,
    artifact_id: String,
    residence_class: TierResidenceClass,
}

impl DerivedTierResidency {
    pub fn new(
        artifact_family: PlacementArtifactFamily,
        artifact_id: impl Into<String>,
        residence_class: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_family,
            artifact_id: artifact_id.into(),
            residence_class,
        }
    }

    pub fn artifact_family(&self) -> PlacementArtifactFamily {
        self.artifact_family
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn residence_class(&self) -> TierResidenceClass {
        self.residence_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierPlacementEvidence {
    residence_class: TierResidenceClass,
    budget_class: PlacementBudgetClass,
    execution_origin: PlacementExecutionOrigin,
}

impl TierPlacementEvidence {
    pub(crate) fn new(
        residence_class: TierResidenceClass,
        budget_class: PlacementBudgetClass,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            residence_class,
            budget_class,
            execution_origin,
        }
    }

    pub fn residence_class(&self) -> TierResidenceClass {
        self.residence_class
    }

    pub fn budget_class(&self) -> PlacementBudgetClass {
        self.budget_class
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementNonAuthorityWitness {
    artifact_key: String,
}

impl PlacementNonAuthorityWitness {
    pub(crate) fn new(artifact_key: impl Into<String>) -> Self {
        Self {
            artifact_key: artifact_key.into(),
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkingSetObservationWindow {
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
    observed_artifact_keys: Vec<String>,
}

impl WorkingSetObservationWindow {
    pub(crate) fn new(
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
        mut observed_artifact_keys: Vec<String>,
    ) -> Self {
        observed_artifact_keys.sort();
        observed_artifact_keys.dedup();
        Self {
            scope_class,
            scope_key: scope_key.into(),
            observed_artifact_keys,
        }
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    pub fn observed_artifact_keys(&self) -> &[String] {
        &self.observed_artifact_keys
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementDemandSummary {
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
    observed_artifact_count: u64,
    classification_verdict: HotnessClassificationVerdict,
}

impl PlacementDemandSummary {
    pub(crate) fn new(
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
        observed_artifact_count: u64,
        classification_verdict: HotnessClassificationVerdict,
    ) -> Self {
        Self {
            scope_class,
            scope_key: scope_key.into(),
            observed_artifact_count,
            classification_verdict,
        }
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    pub fn observed_artifact_count(&self) -> u64 {
        self.observed_artifact_count
    }

    pub fn classification_verdict(&self) -> HotnessClassificationVerdict {
        self.classification_verdict
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierPromotionCandidate {
    artifact_key: String,
    target_residence: TierResidenceClass,
}

impl TierPromotionCandidate {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            target_residence,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierCoolingCandidate {
    artifact_key: String,
    target_residence: TierResidenceClass,
}

impl TierCoolingCandidate {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            target_residence,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecallEligibilityWitness {
    artifact_key: String,
    recall_cost_class: RecallCostClass,
    amplification_budget: RecallAmplificationBudget,
}

impl RecallEligibilityWitness {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        recall_cost_class: RecallCostClass,
        amplification_budget: RecallAmplificationBudget,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            recall_cost_class,
            amplification_budget,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn recall_cost_class(&self) -> RecallCostClass {
        self.recall_cost_class
    }

    pub fn amplification_budget(&self) -> RecallAmplificationBudget {
        self.amplification_budget
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierTransferIntent {
    artifact_key: String,
    source_residence: TierResidenceClass,
    target_residence: TierResidenceClass,
    execution_origin: PlacementExecutionOrigin,
}

impl TierTransferIntent {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        source_residence: TierResidenceClass,
        target_residence: TierResidenceClass,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            source_residence,
            target_residence,
            execution_origin,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn source_residence(&self) -> TierResidenceClass {
        self.source_residence
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TransferredTierReplica {
    intent: TierTransferIntent,
    replica_locator: String,
}

impl TransferredTierReplica {
    pub(crate) fn new(intent: TierTransferIntent, replica_locator: impl Into<String>) -> Self {
        Self {
            intent,
            replica_locator: replica_locator.into(),
        }
    }

    pub fn intent(&self) -> &TierTransferIntent {
        &self.intent
    }

    pub fn replica_locator(&self) -> &str {
        &self.replica_locator
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VerifiedTierReplica {
    transferred_replica: TransferredTierReplica,
    verification_label: String,
}

impl VerifiedTierReplica {
    pub(crate) fn new(
        transferred_replica: TransferredTierReplica,
        verification_label: impl Into<String>,
    ) -> Self {
        Self {
            transferred_replica,
            verification_label: verification_label.into(),
        }
    }

    pub fn transferred_replica(&self) -> &TransferredTierReplica {
        &self.transferred_replica
    }

    pub fn verification_label(&self) -> &str {
        &self.verification_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierCutoverWitness {
    artifact_key: String,
    canonical_residence: TierResidenceClass,
}

impl TierCutoverWitness {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        canonical_residence: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            canonical_residence,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn canonical_residence(&self) -> TierResidenceClass {
        self.canonical_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetiredTierReplica {
    cutover_witness: TierCutoverWitness,
    retired_locator: String,
}

impl RetiredTierReplica {
    pub(crate) fn new(
        cutover_witness: TierCutoverWitness,
        retired_locator: impl Into<String>,
    ) -> Self {
        Self {
            cutover_witness,
            retired_locator: retired_locator.into(),
        }
    }

    pub fn cutover_witness(&self) -> &TierCutoverWitness {
        &self.cutover_witness
    }

    pub fn retired_locator(&self) -> &str {
        &self.retired_locator
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecallCompletionWitness {
    artifact_key: String,
    placement_path: RetainedReadPlacementPath,
    tier_miss_outcome: TierMissOutcome,
    verification_label: String,
}

impl RecallCompletionWitness {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        placement_path: RetainedReadPlacementPath,
        verification_label: impl Into<String>,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            tier_miss_outcome: placement_path.tier_miss_outcome(),
            placement_path,
            verification_label: verification_label.into(),
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn placement_path(&self) -> RetainedReadPlacementPath {
        self.placement_path
    }

    pub fn tier_miss_outcome(&self) -> TierMissOutcome {
        self.tier_miss_outcome
    }

    pub fn resolved_path(&self) -> ColdRecallTierPath {
        self.placement_path.into()
    }

    pub fn verification_label(&self) -> &str {
        &self.verification_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CanonicalResidencyManifest {
    resident_artifact_keys: Vec<String>,
    in_flight_transfer_keys: Vec<String>,
}

impl CanonicalResidencyManifest {
    pub(crate) fn new(
        mut resident_artifact_keys: Vec<String>,
        mut in_flight_transfer_keys: Vec<String>,
    ) -> Self {
        resident_artifact_keys.sort();
        resident_artifact_keys.dedup();
        in_flight_transfer_keys.sort();
        in_flight_transfer_keys.dedup();
        Self {
            resident_artifact_keys,
            in_flight_transfer_keys,
        }
    }

    pub fn resident_artifact_keys(&self) -> &[String] {
        &self.resident_artifact_keys
    }

    pub fn in_flight_transfer_keys(&self) -> &[String] {
        &self.in_flight_transfer_keys
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ColdRecallTierPath {
    HotResident,
    WarmResident,
    ColdRecalled,
    RebuildAssistedDerived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RetainedReadPlacementPath {
    HotResident,
    WarmResident,
    ColdRecalled,
    RebuildAssistedDerived,
}

impl RetainedReadPlacementPath {
    pub fn tier_miss_outcome(self) -> TierMissOutcome {
        match self {
            Self::HotResident => TierMissOutcome::ResidentHit,
            Self::WarmResident => TierMissOutcome::WarmHit,
            Self::ColdRecalled => TierMissOutcome::ColdRecallHit,
            Self::RebuildAssistedDerived => TierMissOutcome::RebuildAssistedDerivedHit,
        }
    }
}

impl From<ColdRecallTierPath> for RetainedReadPlacementPath {
    fn from(value: ColdRecallTierPath) -> Self {
        match value {
            ColdRecallTierPath::HotResident => Self::HotResident,
            ColdRecallTierPath::WarmResident => Self::WarmResident,
            ColdRecallTierPath::ColdRecalled => Self::ColdRecalled,
            ColdRecallTierPath::RebuildAssistedDerived => Self::RebuildAssistedDerived,
        }
    }
}

impl From<RetainedReadPlacementPath> for ColdRecallTierPath {
    fn from(value: RetainedReadPlacementPath) -> Self {
        match value {
            RetainedReadPlacementPath::HotResident => Self::HotResident,
            RetainedReadPlacementPath::WarmResident => Self::WarmResident,
            RetainedReadPlacementPath::ColdRecalled => Self::ColdRecalled,
            RetainedReadPlacementPath::RebuildAssistedDerived => Self::RebuildAssistedDerived,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TierMissOutcome {
    ResidentHit,
    WarmHit,
    ColdRecallHit,
    RebuildAssistedDerivedHit,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct RecallCoalescingKey {
    artifact_family: PlacementArtifactFamily,
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
}

impl RecallCoalescingKey {
    pub(crate) fn new(
        artifact_family: PlacementArtifactFamily,
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
    ) -> Self {
        Self {
            artifact_family,
            scope_class,
            scope_key: scope_key.into(),
        }
    }

    pub fn artifact_family(&self) -> PlacementArtifactFamily {
        self.artifact_family
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observation_window_normalizes_duplicate_keys() {
        let window = WorkingSetObservationWindow::new(
            PlacementObservationScopeClass::Branch,
            "branch:main",
            vec![
                "artifact:b".to_string(),
                "artifact:a".to_string(),
                "artifact:b".to_string(),
            ],
        );

        assert_eq!(
            window.observed_artifact_keys(),
            &["artifact:a".to_string(), "artifact:b".to_string()]
        );
        assert_eq!(window.scope_class(), PlacementObservationScopeClass::Branch);
        assert_eq!(window.scope_key(), "branch:main");
    }

    #[test]
    fn residency_manifest_normalizes_lists() {
        let manifest = CanonicalResidencyManifest::new(
            vec!["b".to_string(), "a".to_string(), "a".to_string()],
            vec!["x".to_string(), "x".to_string()],
        );

        assert_eq!(
            manifest.resident_artifact_keys(),
            &["a".to_string(), "b".to_string()]
        );
        assert_eq!(manifest.in_flight_transfer_keys(), &["x".to_string()]);
    }

    #[test]
    fn proof_accessors_preserve_construction() {
        let intent = TierTransferIntent::new(
            "artifact:1",
            TierResidenceClass::Warm,
            TierResidenceClass::Cold,
            PlacementExecutionOrigin::Background,
        );
        let replica = TransferredTierReplica::new(intent.clone(), "cold://artifact:1");
        let verified = VerifiedTierReplica::new(replica, "digest-ok");

        assert_eq!(verified.transferred_replica().intent(), &intent);
        assert_eq!(verified.verification_label(), "digest-ok");
    }

    #[test]
    fn recall_coalescing_key_preserves_scope_shape() {
        let key = RecallCoalescingKey::new(
            PlacementArtifactFamily::SnapshotFamily,
            PlacementObservationScopeClass::ArtifactFamily,
            "family:snapshot",
        );

        assert_eq!(
            key.artifact_family(),
            PlacementArtifactFamily::SnapshotFamily
        );
        assert_eq!(
            key.scope_class(),
            PlacementObservationScopeClass::ArtifactFamily
        );
        assert_eq!(key.scope_key(), "family:snapshot");
    }
}
