use super::*;
use crate::identity::data::KindId;
use crate::schema::data::{
    AspectBinding, AspectContractPlanRevision, LoweredAspectContractBinding,
    LoweredAspectContractPlan,
};
use forge_foundational::facade::{
    AspectContract, AspectContractRevision, AspectIdentity, AspectKey, FieldKey, ScalarAspectType,
};
use smallvec::smallvec;

#[test]
#[should_panic(expected = "projection mask rejected by aspect contract")]
fn projection_contract_assertion_rejects_field_mask_denied_by_scalar_contract() {
    let aspect_key = AspectKey::new("counter").unwrap();
    let field_key = FieldKey::new("value").unwrap();
    let projection_scope = ProjectionAspectScope::fields(aspect_key.clone(), [field_key.clone()]);
    let plan = LoweredAspectContractPlan {
        kind_id: KindId(1),
        plan_revision: AspectContractPlanRevision(1),
        executable_bindings: smallvec![LoweredAspectContractBinding {
            contract: AspectContract::scalar(
                aspect_key,
                AspectIdentity(1),
                AspectContractRevision(1),
                ScalarAspectType::Int64,
            ),
            target: AspectBinding::EntityField { field: field_key },
        }],
    };

    assert_declared_projection_aspects(&projection_scope, Some(&plan), "entity", KindId(1));
}

#[test]
fn field_projection_scope_carries_locator_and_canonical_basis() {
    let scope = ProjectionAspectScope::fields(
        AspectKey::new("summary").unwrap(),
        [FieldKey::new("title").unwrap()],
    );
    let requirement = &scope.requirements()[0];

    assert!(requirement.mask_basis().is_some());
    assert_eq!(requirement.locator().aspect_key().as_str(), "summary");
}
