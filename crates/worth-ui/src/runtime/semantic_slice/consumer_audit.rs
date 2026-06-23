use crate::runtime::{
    WorthUiProjectionFamily, WorthUiProjectionPlanContract, WorthUiRuntimeFactFamily,
    WorthUiValidatedProjectionDependencyContract,
};

use super::{WorthUiSemanticSliceId, WorthUiSemanticSliceInventory};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSemanticConsumerAuditFindingKind {
    UndocumentedConsumer,
    DeclaredButUnconsumed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticConsumerAuditFinding {
    slice_id: WorthUiSemanticSliceId,
    projection_family: WorthUiProjectionFamily,
    kind: WorthUiSemanticConsumerAuditFindingKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticConsumerAuditReceipt {
    projection_family: WorthUiProjectionFamily,
    consumed_slice_ids: Vec<WorthUiSemanticSliceId>,
    findings: Vec<WorthUiSemanticConsumerAuditFinding>,
}

impl WorthUiSemanticConsumerAuditReceipt {
    pub fn audit_projection_plan<P: WorthUiProjectionPlanContract>(
        inventory: &WorthUiSemanticSliceInventory,
        plan: &P,
    ) -> Self {
        let contract = WorthUiValidatedProjectionDependencyContract::admit(
            plan.projection_identity(),
            plan.projection_family(),
            plan.projection_dependency_declaration(),
        )
        .expect("projection plan under semantic audit must declare dependencies");
        Self::audit_contract(inventory, &contract)
    }

    pub fn audit_contract(
        inventory: &WorthUiSemanticSliceInventory,
        contract: &WorthUiValidatedProjectionDependencyContract,
    ) -> Self {
        let projection_family = contract.family();
        let dependency_families = contract
            .dependencies()
            .facts()
            .map(crate::runtime::WorthUiRuntimeFactId::family)
            .collect::<Vec<_>>();

        let mut consumed_slice_ids = Vec::new();
        let mut findings = Vec::new();
        for descriptor in inventory.slices() {
            let consumes_slice = dependency_families
                .iter()
                .copied()
                .any(|family| descriptor.runtime_fact_mapping().contains_family(family));
            if consumes_slice {
                consumed_slice_ids.push(descriptor.id());
            }
            match (
                consumes_slice,
                descriptor.consumers().contains(projection_family),
            ) {
                (true, false) => findings.push(WorthUiSemanticConsumerAuditFinding {
                    slice_id: descriptor.id(),
                    projection_family,
                    kind: WorthUiSemanticConsumerAuditFindingKind::UndocumentedConsumer,
                }),
                (false, true)
                    if has_relevant_runtime_family(descriptor.runtime_fact_mapping_families()) =>
                {
                    findings.push(WorthUiSemanticConsumerAuditFinding {
                        slice_id: descriptor.id(),
                        projection_family,
                        kind: WorthUiSemanticConsumerAuditFindingKind::DeclaredButUnconsumed,
                    })
                }
                _ => {}
            }
        }

        consumed_slice_ids.sort();
        findings.sort_by(|left, right| {
            left.slice_id
                .cmp(&right.slice_id)
                .then_with(|| left.projection_family.cmp(&right.projection_family))
        });
        Self {
            projection_family,
            consumed_slice_ids,
            findings,
        }
    }

    pub fn projection_family(&self) -> WorthUiProjectionFamily {
        self.projection_family
    }

    pub fn consumed_slice_ids(&self) -> &[WorthUiSemanticSliceId] {
        &self.consumed_slice_ids
    }

    pub fn findings(&self) -> &[WorthUiSemanticConsumerAuditFinding] {
        &self.findings
    }

    pub fn is_consistent(&self) -> bool {
        self.findings.is_empty()
    }
}

impl WorthUiSemanticConsumerAuditFinding {
    pub fn slice_id(&self) -> WorthUiSemanticSliceId {
        self.slice_id
    }

    pub fn projection_family(&self) -> WorthUiProjectionFamily {
        self.projection_family
    }

    pub fn kind(&self) -> WorthUiSemanticConsumerAuditFindingKind {
        self.kind
    }
}

fn has_relevant_runtime_family(families: &[WorthUiRuntimeFactFamily]) -> bool {
    !families.is_empty()
}
