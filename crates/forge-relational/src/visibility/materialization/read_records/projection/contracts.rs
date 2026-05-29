use crate::identity::data::KindId;
use crate::logic::runtime::RelationalRuntime;
use forge_foundational::facade::AspectKey;

pub(super) fn assert_declared_projection_aspects(
    _runtime: &RelationalRuntime,
    required_aspects: &[AspectKey],
    plan: Option<&crate::schema::data::LoweredAspectPlan>,
    domain: &str,
    kind_id: KindId,
) {
    if required_aspects.is_empty() {
        return;
    }
    let Some(plan) = plan else {
        panic!(
            "{domain} projection for kind {:?} requires declared aspects but no lowered aspect plan exists",
            kind_id
        );
    };
    for required in required_aspects {
        if !plan
            .executable_bindings
            .iter()
            .any(|binding| &binding.aspect_key == required)
        {
            panic!(
                "{domain} projection for kind {:?} requires undeclared aspect {:?}",
                kind_id, required
            );
        }
    }
}
