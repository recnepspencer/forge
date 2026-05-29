use crate::identity::data::KindId;

use crate::schema::data::{AspectPlanRevision, DeclaredAspect, LoweredAspectBinding};

pub(super) fn lower_binding(
    _kind_id: KindId,
    _plan_revision: AspectPlanRevision,
    aspect: &DeclaredAspect,
) -> LoweredAspectBinding {
    LoweredAspectBinding {
        contract: aspect.contract.clone(),
        target: aspect.binding.clone(),
    }
}
