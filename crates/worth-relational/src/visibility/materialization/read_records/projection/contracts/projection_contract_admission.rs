use crate::identity::data::KindId;

use super::ProjectionAspectScope;

pub(crate) fn assert_declared_projection_aspects(
    projection_scope: &ProjectionAspectScope,
    plan: Option<&crate::schema::data::LoweredAspectContractPlan>,
    domain: &str,
    kind_id: KindId,
) {
    if projection_scope.is_empty() {
        return;
    }
    let Some(plan) = plan else {
        panic!(
            "{domain} projection for kind {:?} requires declared aspects but no lowered aspect plan exists",
            kind_id
        );
    };
    for required in projection_scope.requirements() {
        let Some(binding) = plan
            .executable_bindings
            .iter()
            .find(|binding| binding.aspect_key() == required.aspect_key())
        else {
            panic!(
                "{domain} projection for kind {:?} requires undeclared aspect {:?}",
                kind_id,
                required.aspect_key()
            );
        };
        if let Err(denial) = binding.contract.admits_projection_mask(required.mask()) {
            panic!(
                "{domain} projection for kind {:?} requires projection mask rejected by aspect contract {:?}: {:?}",
                kind_id,
                required.aspect_key(),
                denial
            );
        }
        if required.mask_basis().is_none() {
            panic!(
                "{domain} projection for kind {:?} requires projection mask canonical basis for aspect {:?}",
                kind_id,
                required.aspect_key()
            );
        }
        let _ = required.locator();
    }
}
