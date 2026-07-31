use std::collections::BTreeMap;
use std::sync::Arc;

use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityCardinalityDimension, ApplicationCapabilityFieldDimension,
    ApplicationCapabilityValidityTimeline, ErasedApplicationCapabilityContract,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationCapability,
    WorthQueryInstalledApplicationSchema,
};
use worth_relational::facade::authorization::{
    RelationalAuthorizationPathPlan, RelationalAuthorizationTraversal,
};
use worth_relational::facade::identity::KindId;
use worth_runtime_bridge::facade::{
    BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationRuleContract,
    BridgeAuthorizationRuntime,
};

use super::capability_lowering::compile_capability_plan;
use super::WorthQueryOperationAuthorizationDenial;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryCapabilityPlanCompilationEvidence {
    capability_count: usize,
    path_count: usize,
    rule_count: usize,
    clause_count: usize,
    guard_count: usize,
    context_anchor_count: usize,
    canonical_basis_preparations: usize,
    digest_derivations: usize,
    digest_text_materializations: usize,
}

impl WorthQueryCapabilityPlanCompilationEvidence {
    pub const fn capability_count(self) -> usize {
        self.capability_count
    }

    pub const fn path_count(self) -> usize {
        self.path_count
    }

    pub const fn rule_count(self) -> usize {
        self.rule_count
    }

    pub const fn clause_count(self) -> usize {
        self.clause_count
    }

    pub const fn guard_count(self) -> usize {
        self.guard_count
    }

    pub const fn context_anchor_count(self) -> usize {
        self.context_anchor_count
    }

    pub const fn canonical_basis_preparations(self) -> usize {
        self.canonical_basis_preparations
    }

    pub const fn digest_derivations(self) -> usize {
        self.digest_derivations
    }

    pub const fn digest_text_materializations(self) -> usize {
        self.digest_text_materializations
    }

    pub(super) fn record(&mut self, plan: &WorthQueryInstalledCapabilityPlan) {
        self.capability_count += 1;
        self.path_count += plan.paths.len();
        self.rule_count += plan.bridge_rules.len();
        self.clause_count += plan.paths.len();
        self.guard_count += plan
            .paths
            .iter()
            .filter(|path| !matches!(path.guard, WorthQueryCapabilityRequestGuard::Unconditional))
            .count();
        self.context_anchor_count += plan
            .paths
            .iter()
            .map(|path| path.context_anchors.len())
            .sum::<usize>();
    }
}

pub(super) struct WorthQueryInstalledCapabilityRegistry {
    plans: BTreeMap<[u8; 32], WorthQueryInstalledCapabilityPlan>,
    compilation: WorthQueryCapabilityPlanCompilationEvidence,
}

impl WorthQueryInstalledCapabilityRegistry {
    pub(super) fn compile<Schema>(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        layout: &WorthQueryPrimaryGraphLayout,
        bridge: &mut BridgeAuthorizationRuntime,
    ) -> Result<Self, WorthQueryOperationAuthorizationDenial>
    where
        Schema: ApplicationSchema,
    {
        let mut plans = BTreeMap::new();
        let mut compilation = WorthQueryCapabilityPlanCompilationEvidence::default();
        for source in schema.capability_plan_sources() {
            let plan = compile_capability_plan(schema, source, layout, bridge)?;
            compilation.record(&plan);
            if plans.insert(*source.identity().bytes(), plan).is_some() {
                return Err(super::authorization_denial(
                    source.contract().name(),
                    "duplicate installed capability plan",
                ));
            }
        }
        Ok(Self { plans, compilation })
    }

    pub(super) fn plan<Schema, Capability, Operation, Input>(
        &self,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    ) -> Option<&WorthQueryInstalledCapabilityPlan> {
        self.plans.get(capability.identity().bytes())
    }

    pub(super) fn plan_by_identity(
        &self,
        capability_identity: &[u8; 32],
    ) -> Option<&WorthQueryInstalledCapabilityPlan> {
        self.plans.get(capability_identity)
    }

    pub(super) const fn compilation(&self) -> WorthQueryCapabilityPlanCompilationEvidence {
        self.compilation
    }
}

pub(super) struct WorthQueryInstalledCapabilityPlan {
    pub(super) correspondence: BridgeAuthorizationCorrespondenceIdentity,
    pub(super) capability_authority_identity: Arc<str>,
    pub(super) contract: ErasedApplicationCapabilityContract,
    pub(super) principal_kind: KindId,
    pub(super) grant_kind: KindId,
    pub(super) scope_kind: KindId,
    pub(super) paths: Vec<WorthQueryCapabilityPathTemplate>,
    pub(super) bridge_rules: Vec<BridgeAuthorizationRuleContract>,
    pub(super) rule_path_indices: Vec<Vec<Vec<usize>>>,
    pub(super) request: WorthQueryCapabilityRequestBindings,
}

pub(super) struct WorthQueryCapabilityPathTemplate {
    pub(super) plan: RelationalAuthorizationPathPlan,
    pub(super) identity: [u8; 32],
    pub(super) guard: WorthQueryCapabilityRequestGuard,
    pub(super) context_anchors: Vec<WorthQueryCapabilityContextAnchor>,
}

pub(super) enum WorthQueryCapabilityRequestGuard {
    Unconditional,
    Accepted {
        axis: WorthQueryCapabilityRequestValueAxis,
        values: Vec<AspectValue>,
    },
}

#[derive(Clone, Copy)]
pub(super) enum WorthQueryCapabilityRequestValueAxis {
    Action,
    Purpose,
    Field,
    Amount,
}

pub(super) struct WorthQueryCapabilityContextAnchor {
    pub(super) ordinal: usize,
    pub(super) kind: KindId,
    pub(super) context: String,
    pub(super) context_type: String,
    pub(super) slot: String,
    pub(super) slot_type: String,
    pub(super) entity: String,
}

pub(super) struct WorthQueryCapabilityRequestBindings {
    pub(super) action: AspectValue,
    pub(super) purpose: AspectValue,
    pub(super) resource_entity: String,
    pub(super) related_relation: Option<RelationalAuthorizationTraversal>,
    pub(super) field: Option<AspectFieldLocator>,
    pub(super) amount: Option<AspectFieldLocator>,
    pub(super) cardinality: ApplicationCapabilityCardinalityDimension,
    pub(super) timeline: ApplicationCapabilityValidityTimeline,
    pub(super) not_before: AspectFieldLocator,
    pub(super) not_after: AspectFieldLocator,
    pub(super) context: String,
    pub(super) context_type: String,
}

pub(super) const fn field_binding(
    dimension: &ApplicationCapabilityFieldDimension,
) -> Option<
    &worth_query_declaration::facade::application_capability::ApplicationCapabilityFieldBinding,
> {
    match dimension {
        ApplicationCapabilityFieldDimension::NotApplicable => None,
        ApplicationCapabilityFieldDimension::Bound(binding) => Some(binding),
    }
}
