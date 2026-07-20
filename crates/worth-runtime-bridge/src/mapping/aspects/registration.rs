use crate::mapping::{SubscriptionSliceKind, TruthPatchScope};
use crate::snapshot::SnapshotReadContract;

use super::ids::BridgeAspectRegistrationId;
use super::types::{
    BridgeAuthoritativeSourcePrecisionPolicy, SliceWideningPolicy, TruthDeltaSurfaceKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAspectRegistration {
    registration_id: BridgeAspectRegistrationId,
    truth_scope: TruthPatchScope,
    snapshot_read_contract: SnapshotReadContract,
    truth_surface_kind: TruthDeltaSurfaceKind,
    subscription_slice_kind: SubscriptionSliceKind,
    widening_policy: SliceWideningPolicy,
    source_precision_policy: BridgeAuthoritativeSourcePrecisionPolicy,
}

impl BridgeAspectRegistration {
    pub fn new(
        registration_id: BridgeAspectRegistrationId,
        truth_scope: TruthPatchScope,
        snapshot_read_contract: SnapshotReadContract,
        truth_surface_kind: TruthDeltaSurfaceKind,
        subscription_slice_kind: SubscriptionSliceKind,
        widening_policy: SliceWideningPolicy,
    ) -> Self {
        Self {
            registration_id,
            truth_scope,
            snapshot_read_contract,
            truth_surface_kind,
            subscription_slice_kind,
            widening_policy,
            source_precision_policy: BridgeAuthoritativeSourcePrecisionPolicy::ExactOnly,
        }
    }

    pub fn with_declared_source_widening(
        mut self,
        cause: crate::input::envelope::BridgeAspectChangeWideningCause,
    ) -> Self {
        self.source_precision_policy =
            BridgeAuthoritativeSourcePrecisionPolicy::AdmitDeclared(cause);
        self
    }

    pub fn registration_id(&self) -> &BridgeAspectRegistrationId {
        &self.registration_id
    }

    pub fn truth_scope(&self) -> &TruthPatchScope {
        &self.truth_scope
    }

    pub fn snapshot_read_contract(&self) -> &SnapshotReadContract {
        &self.snapshot_read_contract
    }

    pub fn truth_surface_kind(&self) -> TruthDeltaSurfaceKind {
        self.truth_surface_kind
    }

    pub fn subscription_slice_kind(&self) -> &SubscriptionSliceKind {
        &self.subscription_slice_kind
    }

    pub fn widening_policy(&self) -> SliceWideningPolicy {
        self.widening_policy
    }

    pub fn source_precision_policy(&self) -> BridgeAuthoritativeSourcePrecisionPolicy {
        self.source_precision_policy
    }

    pub(super) fn semantic_duplicate_of(&self, other: &Self) -> bool {
        self.truth_scope == other.truth_scope
            && self.snapshot_read_contract == other.snapshot_read_contract
            && self.truth_surface_kind == other.truth_surface_kind
            && self.subscription_slice_kind == other.subscription_slice_kind
            && self.widening_policy == other.widening_policy
            && self.source_precision_policy == other.source_precision_policy
    }
}
