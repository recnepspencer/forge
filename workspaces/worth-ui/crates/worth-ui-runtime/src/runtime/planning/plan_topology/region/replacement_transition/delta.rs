use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiExecutionPlanInput, WorthUiNodeLifecycleTransition, WorthUiNodeReplacementPlan,
};

use super::{WorthUiPlanRegionIdentity, WorthUiPlanRegionMutation, WorthUiPlanRegionSchema};

#[derive(Clone, Debug)]
pub(crate) struct WorthUiPlanRegionDelta {
    predecessor_artifact_digest: u64,
    predecessor_plan_digest: u64,
    candidate_artifact_digest: u64,
    allocation_identity_digest: u64,
    mutations: Vec<WorthUiPlanRegionMutation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiPlanRegionDeltaDenial {
    DuplicateCandidateRegion,
}

impl WorthUiPlanRegionDelta {
    pub(crate) fn from_replacement(
        node_plan: &WorthUiNodeReplacementPlan,
        plan_input: &WorthUiExecutionPlanInput,
        predecessor_plan_digest: u64,
        allocation_identity_digest: u64,
    ) -> Result<Self, WorthUiPlanRegionDeltaDenial> {
        let mut candidate_identities = BTreeSet::new();
        let mut mutations = Vec::new();
        let ordinary_bundles = ordinary_owner_bundles(plan_input);
        let ordinary_identities = ordinary_bundles
            .values()
            .flat_map(|schemas| schemas.iter().map(|schema| schema.identity().clone()))
            .collect::<BTreeSet<_>>();
        for (root, schemas) in ordinary_bundles {
            mutations.push(WorthUiPlanRegionMutation::OwnerBundle { root, schemas });
        }
        for input in plan_input.node_inputs() {
            if !candidate_identities.insert(input.identity_basis()) {
                return Err(WorthUiPlanRegionDeltaDenial::DuplicateCandidateRegion);
            }
            let schema = || WorthUiPlanRegionSchema::from_node_input(input.clone());
            if ordinary_identities.contains(schema().identity()) {
                continue;
            }
            match input.transition() {
                Some(WorthUiNodeLifecycleTransition::Preserve) => {}
                Some(WorthUiNodeLifecycleTransition::Create) => {
                    mutations.push(WorthUiPlanRegionMutation::Insert(schema()));
                }
                Some(WorthUiNodeLifecycleTransition::Replace) => {
                    mutations.push(WorthUiPlanRegionMutation::Replace(schema()));
                }
                Some(WorthUiNodeLifecycleTransition::Move) => {
                    mutations.push(WorthUiPlanRegionMutation::Reparent(schema()));
                }
                Some(WorthUiNodeLifecycleTransition::Rebind) => {
                    mutations.push(WorthUiPlanRegionMutation::Rebind(schema()));
                }
                Some(WorthUiNodeLifecycleTransition::LaneChange) => {
                    mutations.push(WorthUiPlanRegionMutation::LaneTransition(schema()));
                }
                Some(WorthUiNodeLifecycleTransition::Drop) => {}
                None => mutations.push(WorthUiPlanRegionMutation::Upsert(schema())),
            }
        }
        for classification in node_plan.changed_classifications() {
            if classification.transition() == WorthUiNodeLifecycleTransition::Drop {
                let identity =
                    WorthUiPlanRegionIdentity::from_exact_basis(classification.identity_basis());
                let mutation = if classification.active_kind()
                    == Some(crate::runtime::WorthUiIdentityMatchNodeKind::Binding)
                {
                    WorthUiPlanRegionMutation::Retire(identity)
                } else {
                    WorthUiPlanRegionMutation::RetireOwner(identity)
                };
                mutations.push(mutation);
            }
        }

        Ok(Self {
            predecessor_artifact_digest: node_plan.active_artifact_digest(),
            predecessor_plan_digest,
            candidate_artifact_digest: node_plan.candidate_artifact_digest(),
            allocation_identity_digest,
            mutations,
        })
    }

    pub(crate) fn predecessor_artifact_digest(&self) -> u64 {
        self.predecessor_artifact_digest
    }

    pub(crate) fn predecessor_plan_digest(&self) -> u64 {
        self.predecessor_plan_digest
    }

    pub(crate) fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub(crate) fn allocation_identity_digest(&self) -> u64 {
        self.allocation_identity_digest
    }

    pub(crate) fn mutations(&self) -> &[WorthUiPlanRegionMutation] {
        &self.mutations
    }
}

fn ordinary_owner_bundles(
    plan_input: &WorthUiExecutionPlanInput,
) -> std::collections::BTreeMap<WorthUiPlanRegionIdentity, Vec<WorthUiPlanRegionSchema>> {
    let mut by_root = std::collections::BTreeMap::<String, Vec<WorthUiPlanRegionSchema>>::new();
    for input in plan_input
        .node_inputs()
        .iter()
        .filter(|input| input.ordinary_meaning().is_some())
    {
        let root = input
            .owner_identity_basis()
            .unwrap_or_else(|| input.identity_basis())
            .to_owned();
        by_root
            .entry(root)
            .or_default()
            .push(WorthUiPlanRegionSchema::from_node_input(input.clone()));
    }
    by_root
        .into_iter()
        .map(|(root, mut schemas)| {
            schemas.sort_by(|left, right| left.identity().cmp(right.identity()));
            (WorthUiPlanRegionIdentity::from_exact_basis(root), schemas)
        })
        .collect()
}
