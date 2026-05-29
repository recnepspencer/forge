use std::collections::BTreeMap;

use forge_foundational::{AspectContract, FieldKey};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::identity::data::KindId;
use crate::merge::data::{AspectMergePolicyDeclaration, IdentityBasisDeclaration};
use forge_foundational::facade::AspectKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AspectPlanRevision(pub u128);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindAspectDeclarations {
    pub plan_revision: AspectPlanRevision,
    pub aspects: Vec<DeclaredAspect>,
    pub identity_declarations: Vec<IdentityBasisDeclaration>,
    pub merge_policy_declarations: Vec<AspectMergePolicyDeclaration>,
}

impl KindAspectDeclarations {
    pub fn new(aspects: Vec<DeclaredAspect>) -> Self {
        Self {
            plan_revision: AspectPlanRevision(0),
            aspects,
            identity_declarations: Vec::new(),
            merge_policy_declarations: Vec::new(),
        }
    }

    pub fn with_identity_declarations(
        mut self,
        identity_declarations: Vec<IdentityBasisDeclaration>,
    ) -> Self {
        self.identity_declarations = identity_declarations;
        self
    }

    pub fn with_merge_policy_declarations(
        mut self,
        merge_policy_declarations: Vec<AspectMergePolicyDeclaration>,
    ) -> Self {
        self.merge_policy_declarations = merge_policy_declarations;
        self
    }
}

impl Default for KindAspectDeclarations {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredAspect {
    pub binding: AspectBinding,
    pub contract: AspectContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AspectBinding {
    EntityField { field: FieldKey },
    RelationField { field: FieldKey },
    RelationSourceEndpoint,
    RelationTargetEndpoint,
    LifecycleTransition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectPlanCatalog {
    pub entity_plans: BTreeMap<KindId, LoweredAspectPlan>,
    pub relation_plans: BTreeMap<KindId, LoweredAspectPlan>,
}

impl AspectPlanCatalog {
    pub fn empty() -> Self {
        Self {
            entity_plans: BTreeMap::new(),
            relation_plans: BTreeMap::new(),
        }
    }
}

impl Default for AspectPlanCatalog {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredAspectPlan {
    pub kind_id: KindId,
    pub plan_revision: AspectPlanRevision,
    pub executable_bindings: SmallVec<[LoweredAspectBinding; 8]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredAspectBinding {
    pub contract: AspectContract,
    pub target: LoweredAspectTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LoweredAspectTarget {
    EntityField { field: FieldKey },
    RelationField { field: FieldKey },
    RelationSourceEndpoint,
    RelationTargetEndpoint,
    LifecycleTransition,
}

impl DeclaredAspect {
    pub fn aspect_key(&self) -> AspectKey {
        self.contract.key().clone()
    }

    pub fn foundational_key(&self) -> &forge_foundational::AspectKey {
        self.contract.key()
    }
}

impl LoweredAspectBinding {
    pub fn aspect_key(&self) -> &AspectKey {
        self.contract.key()
    }

    pub fn aspect_shape(&self) -> forge_foundational::AspectShape {
        self.contract.shape().clone()
    }

    pub fn targets_entity_scalar_field(&self, target: &FieldKey) -> bool {
        matches!(&self.target, LoweredAspectTarget::EntityField { field } if field == target)
            && matches!(
                self.contract.shape(),
                forge_foundational::AspectShape::Scalar(_)
            )
    }

    pub fn targets_entity_struct_field(&self, target: &FieldKey) -> bool {
        matches!(&self.target, LoweredAspectTarget::EntityField { .. })
            && self.struct_contract_declares_field(target)
    }

    pub fn targets_relation_scalar_field(&self, target: &FieldKey) -> bool {
        matches!(&self.target, LoweredAspectTarget::RelationField { field } if field == target)
            && matches!(
                self.contract.shape(),
                forge_foundational::AspectShape::Scalar(_)
            )
    }

    pub fn targets_relation_struct_field(&self, target: &FieldKey) -> bool {
        matches!(&self.target, LoweredAspectTarget::RelationField { .. })
            && self.struct_contract_declares_field(target)
    }

    pub fn struct_contract_declares_field(&self, target: &FieldKey) -> bool {
        let forge_foundational::AspectShape::Struct(shape) = self.contract.shape() else {
            return false;
        };
        shape.field(target).is_some()
    }
}

impl LoweredAspectPlan {
    pub fn admits_entity_scalar_field(&self, target: &FieldKey) -> bool {
        self.entity_scalar_field_aspect_key(target).is_some()
    }

    pub fn entity_scalar_field_aspect_key(&self, target: &FieldKey) -> Option<AspectKey> {
        self.executable_bindings
            .iter()
            .find(|binding| binding.targets_entity_scalar_field(target))
            .map(|binding| binding.aspect_key().clone())
    }

    pub fn admits_entity_field_update_target(&self, field: &FieldKey) -> bool {
        self.executable_bindings.iter().any(|binding| {
            binding.targets_entity_scalar_field(field) || binding.targets_entity_struct_field(field)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{AspectPlanRevision, LoweredAspectBinding, LoweredAspectPlan, LoweredAspectTarget};
    use crate::identity::data::KindId;
    use forge_foundational::FieldKey;

    #[test]
    fn lowered_plan_admits_only_lowered_entity_scalar_fields() {
        let lowered = LoweredAspectPlan {
            kind_id: KindId(1),
            plan_revision: AspectPlanRevision(1),
            executable_bindings: smallvec::smallvec![
                LoweredAspectBinding {
                    contract: forge_foundational::AspectContract::scalar(
                        forge_foundational::AspectKey::new("name").expect("valid key"),
                        forge_foundational::AspectIdentity(1),
                        forge_foundational::AspectContractRevision(1),
                        forge_foundational::ScalarAspectType::String,
                    ),
                    target: LoweredAspectTarget::EntityField {
                        field: FieldKey::new("name").expect("valid field"),
                    },
                },
                LoweredAspectBinding {
                    contract: forge_foundational::AspectContract::scalar(
                        forge_foundational::AspectKey::new("lifecycle").expect("valid key"),
                        forge_foundational::AspectIdentity(2),
                        forge_foundational::AspectContractRevision(1),
                        forge_foundational::ScalarAspectType::String,
                    ),
                    target: LoweredAspectTarget::LifecycleTransition,
                }
            ],
        };

        assert!(lowered.admits_entity_scalar_field(&FieldKey::new("name").expect("valid field")));
        assert!(
            !lowered.admits_entity_scalar_field(&FieldKey::new("lifecycle").expect("valid field"))
        );
        assert!(
            !lowered.admits_entity_scalar_field(&FieldKey::new("replicas").expect("valid field"))
        );
    }
}
