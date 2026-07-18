use crate::capability::{MosaicResizePermission, MosaicSizingContractId};
use crate::runtime::{WorthUiIdentityMatchNodeKind, WorthUiNodeLifecycleTransition};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiNodeReplacementClassification {
    identity_basis: String,
    authored_provenance_digest: Option<u64>,
    transition: WorthUiNodeLifecycleTransition,
    active_kind: Option<WorthUiIdentityMatchNodeKind>,
    candidate_kind: Option<WorthUiIdentityMatchNodeKind>,
    active_durable_state_eligible: bool,
    candidate_durable_state_eligible: bool,
    active_resize_contract_id: Option<MosaicSizingContractId>,
    candidate_resize_contract_id: Option<MosaicSizingContractId>,
    active_resize_permission: Option<MosaicResizePermission>,
    candidate_resize_permission: Option<MosaicResizePermission>,
    active_resize_shape_digest: Option<u64>,
    candidate_resize_shape_digest: Option<u64>,
}

pub(crate) struct WorthUiNodeReplacementClassificationInput {
    pub identity_basis: String,
    pub authored_provenance_digest: Option<u64>,
    pub transition: WorthUiNodeLifecycleTransition,
    pub active_kind: Option<WorthUiIdentityMatchNodeKind>,
    pub candidate_kind: Option<WorthUiIdentityMatchNodeKind>,
    pub active_durable_state_eligible: bool,
    pub candidate_durable_state_eligible: bool,
    pub active_resize_contract_id: Option<MosaicSizingContractId>,
    pub candidate_resize_contract_id: Option<MosaicSizingContractId>,
    pub active_resize_permission: Option<MosaicResizePermission>,
    pub candidate_resize_permission: Option<MosaicResizePermission>,
    pub active_resize_shape_digest: Option<u64>,
    pub candidate_resize_shape_digest: Option<u64>,
}

impl WorthUiNodeReplacementClassification {
    pub(crate) fn new(input: WorthUiNodeReplacementClassificationInput) -> Self {
        let WorthUiNodeReplacementClassificationInput {
            identity_basis,
            authored_provenance_digest,
            transition,
            active_kind,
            candidate_kind,
            active_durable_state_eligible,
            candidate_durable_state_eligible,
            active_resize_contract_id,
            candidate_resize_contract_id,
            active_resize_permission,
            candidate_resize_permission,
            active_resize_shape_digest,
            candidate_resize_shape_digest,
        } = input;
        Self {
            identity_basis,
            authored_provenance_digest,
            transition,
            active_kind,
            candidate_kind,
            active_durable_state_eligible,
            candidate_durable_state_eligible,
            active_resize_contract_id,
            candidate_resize_contract_id,
            active_resize_permission,
            candidate_resize_permission,
            active_resize_shape_digest,
            candidate_resize_shape_digest,
        }
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn authored_provenance_digest(&self) -> Option<u64> {
        self.authored_provenance_digest
    }

    pub fn transition(&self) -> WorthUiNodeLifecycleTransition {
        self.transition
    }

    pub fn active_kind(&self) -> Option<WorthUiIdentityMatchNodeKind> {
        self.active_kind
    }

    pub fn candidate_kind(&self) -> Option<WorthUiIdentityMatchNodeKind> {
        self.candidate_kind
    }

    pub fn active_durable_state_eligible(&self) -> bool {
        self.active_durable_state_eligible
    }

    pub fn candidate_durable_state_eligible(&self) -> bool {
        self.candidate_durable_state_eligible
    }

    pub fn active_resize_contract_id(&self) -> Option<&MosaicSizingContractId> {
        self.active_resize_contract_id.as_ref()
    }

    pub fn candidate_resize_contract_id(&self) -> Option<&MosaicSizingContractId> {
        self.candidate_resize_contract_id.as_ref()
    }

    pub fn active_resize_permission(&self) -> Option<&MosaicResizePermission> {
        self.active_resize_permission.as_ref()
    }

    pub fn candidate_resize_permission(&self) -> Option<&MosaicResizePermission> {
        self.candidate_resize_permission.as_ref()
    }

    pub fn active_resize_shape_digest(&self) -> Option<u64> {
        self.active_resize_shape_digest
    }

    pub fn candidate_resize_shape_digest(&self) -> Option<u64> {
        self.candidate_resize_shape_digest
    }

    pub fn unrestored_durable_state_carry_permitted(&self) -> bool {
        matches!(
            self.transition,
            WorthUiNodeLifecycleTransition::Preserve
                | WorthUiNodeLifecycleTransition::Move
                | WorthUiNodeLifecycleTransition::Rebind
        ) && self.active_durable_state_eligible
            && self.candidate_durable_state_eligible
    }
}
