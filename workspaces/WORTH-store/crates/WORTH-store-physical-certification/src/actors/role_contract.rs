use crate::{PhysicalScenarioActor, PhysicalScenarioActorRole};

use super::admission::PhysicalSimulationActorAdmissionDenial;

pub(crate) fn admit_actor_role_contract(
    actor: &PhysicalScenarioActor,
) -> Result<(), PhysicalSimulationActorAdmissionDenial> {
    if actor.id().trim().is_empty() {
        return Err(PhysicalSimulationActorAdmissionDenial::EmptyActorId);
    }
    if actor.role() == PhysicalScenarioActorRole::FutureExtensionSlot {
        return Err(PhysicalSimulationActorAdmissionDenial::FutureExtensionActorCannotExecute);
    }
    Ok(())
}
