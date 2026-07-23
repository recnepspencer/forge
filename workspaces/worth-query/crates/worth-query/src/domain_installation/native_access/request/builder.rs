use std::sync::Arc;

use worth_foundational::facade::{AspectContract, FieldKey};

use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQueryConsumerProjectionContract;
use crate::ordinary::read::{project_facts, WorthQueryProjectionDeclaration};
use crate::projection_consumption::{
    DeclaredNativeAspectContractBasis, DeclaredNativeFactContract,
    DeclaredNativeFactContractDenial, ProjectionFactRequest,
};

use super::super::{
    WorthQueryNativeAccessKey, WorthQueryNativeFactLane, WorthQueryNativeProjectionRequestDenial,
    WorthQueryNativeProjectionRequestDenialKind, WorthQueryNativeSelection,
};
use super::{WorthQueryBoundProjectionRequest, WorthQueryNativeAccessPlan};

pub struct WorthQueryProjectionRequestBuilder<D, O, F, L: BasisOperationLane> {
    consumer: WorthQueryConsumerProjectionContract<D, O, F, L>,
    native_basis: Arc<DeclaredNativeAspectContractBasis>,
    declaration: WorthQueryProjectionDeclaration,
    pending: Vec<PendingNativeFact>,
    request_identity: u64,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryProjectionRequestBuilder<D, O, F, L> {
    pub(crate) fn new(consumer: WorthQueryConsumerProjectionContract<D, O, F, L>) -> Self {
        let native_basis =
            DeclaredNativeAspectContractBasis::new(consumer.native_projection().contract().clone());
        let request_identity = consumer.capability_identity();
        let declaration = match consumer.collection() {
            crate::domain_installation::WorthQueryOperationCollectionContract::NotCollection => {
                project_facts()
            }
            crate::domain_installation::WorthQueryOperationCollectionContract::Collection {
                ..
            } => project_facts().entity_identities().view_local_identities(),
        };
        Self {
            consumer,
            native_basis,
            declaration,
            pending: Vec::new(),
            request_identity,
        }
    }

    pub fn select_display_native_aspect(
        &mut self,
    ) -> Result<WorthQueryNativeSelection, WorthQueryNativeProjectionRequestDenial> {
        self.add_whole(WorthQueryNativeFactLane::Display)
    }

    pub fn select_display_native_field(
        &mut self,
        field: FieldKey,
    ) -> Result<WorthQueryNativeSelection, WorthQueryNativeProjectionRequestDenial> {
        self.add_field(WorthQueryNativeFactLane::Display, field)
    }

    pub fn select_display_native_field_name(
        &mut self,
        field: &str,
    ) -> Result<WorthQueryNativeSelection, WorthQueryNativeProjectionRequestDenial> {
        let field = FieldKey::new(field).ok_or_else(|| {
            request_denial(
                WorthQueryNativeProjectionRequestDenialKind::UnknownField,
                self.consumer.native_projection().contract(),
                None,
            )
        })?;
        self.select_display_native_field(field)
    }

    pub fn select_derived_native_aspect(
        &mut self,
    ) -> Result<WorthQueryNativeSelection, WorthQueryNativeProjectionRequestDenial> {
        self.add_whole(WorthQueryNativeFactLane::Derived)
    }

    pub fn select_derived_native_field(
        &mut self,
        field: FieldKey,
    ) -> Result<WorthQueryNativeSelection, WorthQueryNativeProjectionRequestDenial> {
        self.add_field(WorthQueryNativeFactLane::Derived, field)
    }

    pub fn select_derived_native_field_name(
        &mut self,
        field: &str,
    ) -> Result<WorthQueryNativeSelection, WorthQueryNativeProjectionRequestDenial> {
        let field = FieldKey::new(field).ok_or_else(|| {
            request_denial(
                WorthQueryNativeProjectionRequestDenialKind::UnknownField,
                self.consumer.native_projection().contract(),
                None,
            )
        })?;
        self.select_derived_native_field(field)
    }

    pub fn build(
        mut self,
    ) -> Result<WorthQueryBoundProjectionRequest<D, O, F, L>, WorthQueryNativeProjectionRequestDenial>
    {
        if self.pending.is_empty() {
            return Err(request_denial(
                WorthQueryNativeProjectionRequestDenialKind::NoNativeFacts,
                self.consumer.native_projection().contract(),
                None,
            ));
        }
        self.pending
            .sort_by(|left, right| left.request.cmp(&right.request));
        let display_width = self
            .pending
            .iter()
            .filter(|pending| pending.lane == WorthQueryNativeFactLane::Display)
            .count();
        let derived_width = self.pending.len() - display_width;
        let mut display_slot = 0;
        let mut derived_slot = 0;
        let mut selector_key_slots = vec![0; self.pending.len()];
        let keys = self
            .pending
            .iter()
            .enumerate()
            .map(|(key_slot, pending)| {
                let (lane_slot, lane_width) = match pending.lane {
                    WorthQueryNativeFactLane::Display => {
                        let slot = display_slot;
                        display_slot += 1;
                        (slot, display_width)
                    }
                    WorthQueryNativeFactLane::Derived => {
                        let slot = derived_slot;
                        derived_slot += 1;
                        (slot, derived_width)
                    }
                };
                selector_key_slots[pending.declaration_slot] = key_slot;
                native_key(&self.consumer, pending, lane_slot, lane_width)
            })
            .collect();
        Ok(WorthQueryBoundProjectionRequest {
            consumer: self.consumer,
            declaration: self.declaration,
            plan: WorthQueryNativeAccessPlan { keys },
            request_identity: self.request_identity,
            selector_key_slots,
        })
    }

    fn add_whole(
        &mut self,
        lane: WorthQueryNativeFactLane,
    ) -> Result<WorthQueryNativeSelection, WorthQueryNativeProjectionRequestDenial> {
        let installed = self.consumer.native_projection();
        let contract = DeclaredNativeFactContract::whole(
            Arc::clone(&self.native_basis),
            installed.mask().is_whole_aspect(),
        )
        .map_err(|denial| map_contract_denial(denial, installed.contract(), None))?;
        self.add_contract(lane, contract)
    }

    fn add_field(
        &mut self,
        lane: WorthQueryNativeFactLane,
        field: FieldKey,
    ) -> Result<WorthQueryNativeSelection, WorthQueryNativeProjectionRequestDenial> {
        let installed = self.consumer.native_projection();
        let contract = DeclaredNativeFactContract::field(
            Arc::clone(&self.native_basis),
            installed.mask().paths(),
            installed.mask().is_whole_aspect(),
            &field,
        )
        .map_err(|denial| map_contract_denial(denial, installed.contract(), Some(field.clone())))?;
        self.add_contract(lane, contract)
    }

    fn add_contract(
        &mut self,
        lane: WorthQueryNativeFactLane,
        contract: DeclaredNativeFactContract,
    ) -> Result<WorthQueryNativeSelection, WorthQueryNativeProjectionRequestDenial> {
        let request = match lane {
            WorthQueryNativeFactLane::Display => {
                self.declaration = self
                    .declaration
                    .clone()
                    .display_native(contract.clone())
                    .map_err(|_| conflict_denial(&contract))?;
                ProjectionFactRequest::DisplayField(contract.field_path().clone())
            }
            WorthQueryNativeFactLane::Derived => {
                self.declaration = self
                    .declaration
                    .clone()
                    .derived_native(contract.clone())
                    .map_err(|_| conflict_denial(&contract))?;
                ProjectionFactRequest::DerivedField(contract.field_path().clone())
            }
        };
        let declaration_slot = self.pending.len();
        self.pending.push(PendingNativeFact {
            lane,
            request,
            contract,
            declaration_slot,
        });
        Ok(WorthQueryNativeSelection::mint(
            self.request_identity,
            declaration_slot,
        ))
    }
}

struct PendingNativeFact {
    lane: WorthQueryNativeFactLane,
    request: ProjectionFactRequest,
    contract: DeclaredNativeFactContract,
    declaration_slot: usize,
}

fn native_key<D, O, F, L: BasisOperationLane>(
    consumer: &WorthQueryConsumerProjectionContract<D, O, F, L>,
    pending: &PendingNativeFact,
    lane_slot: usize,
    lane_width: usize,
) -> WorthQueryNativeAccessKey {
    let contract = pending.contract.contract();
    WorthQueryNativeAccessKey::mint(
        consumer.runtime_authority(),
        consumer.installation_generation(),
        consumer.capability_identity(),
        pending.contract.selection_identity(),
        contract,
        pending.contract.field_path().clone(),
        pending.contract.expected_shape(),
        pending.contract.absence(),
        pending.lane,
        lane_slot,
        lane_width,
    )
}

pub(super) fn map_contract_denial(
    denial: DeclaredNativeFactContractDenial,
    contract: &AspectContract,
    field: Option<FieldKey>,
) -> WorthQueryNativeProjectionRequestDenial {
    use WorthQueryNativeProjectionRequestDenialKind as Kind;
    let kind = match denial {
        DeclaredNativeFactContractDenial::WholeAspectNotProjected => Kind::WholeAspectNotProjected,
        DeclaredNativeFactContractDenial::FieldRequiresStruct => Kind::FieldRequiresStruct,
        DeclaredNativeFactContractDenial::UnknownField => Kind::UnknownField,
        DeclaredNativeFactContractDenial::FieldNotProjected => Kind::FieldNotProjected,
        DeclaredNativeFactContractDenial::UnsupportedAspectShape => Kind::UnsupportedAspectShape,
    };
    request_denial(kind, contract, field)
}

fn conflict_denial(
    contract: &DeclaredNativeFactContract,
) -> WorthQueryNativeProjectionRequestDenial {
    request_denial(
        WorthQueryNativeProjectionRequestDenialKind::ConflictingDeclaration,
        contract.contract(),
        contract.field_path().native_field_key().cloned(),
    )
}

fn request_denial(
    kind: WorthQueryNativeProjectionRequestDenialKind,
    contract: &AspectContract,
    field: Option<FieldKey>,
) -> WorthQueryNativeProjectionRequestDenial {
    WorthQueryNativeProjectionRequestDenial::new(kind, contract, field)
}

impl<D, O, F, L: BasisOperationLane> WorthQueryConsumerProjectionContract<D, O, F, L> {
    pub fn projection_request(self) -> WorthQueryProjectionRequestBuilder<D, O, F, L> {
        WorthQueryProjectionRequestBuilder::new(self)
    }
}
