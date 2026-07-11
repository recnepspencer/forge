use forge_store_authority::StoreCurrentAuthorityWitness;

use super::{
    LayoutEvolutionDenial, LayoutMigrationPlan, LayoutMigrationRequest, LayoutRollbackPlan,
    LayoutRollbackRequest, S8MigrationPlanningOutcome, S8RollbackPlanningOutcome,
};
use forge_proof::TransitionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMigrationFacade;

impl LayoutMigrationFacade {
    pub fn plan_migration(
        &self,
        request: LayoutMigrationRequest,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> S8MigrationPlanningOutcome {
        let resolved = match request.try_resolve_ready() {
            TransitionOutcome::Success(resolved) => resolved,
            TransitionOutcome::Denied(denial) => {
                return S8MigrationPlanningOutcome::declaration_denied(denial);
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        let lowered = match resolved.try_lower_ready(current_store_authority) {
            TransitionOutcome::Success(lowered) => lowered,
            TransitionOutcome::Denied(_) => {
                unreachable!("authority comparison cannot return a denial")
            }
            TransitionOutcome::RebindRequired(rebind) => {
                return S8MigrationPlanningOutcome::lowering_rebind_required(rebind);
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        match lowered.try_ready_now() {
            TransitionOutcome::Success(plan) => S8MigrationPlanningOutcome::ready(plan),
            TransitionOutcome::Denied(_) => {
                unreachable!("freshness comparison cannot return a denial")
            }
            TransitionOutcome::Stale(stale) => S8MigrationPlanningOutcome::stale(stale),
            TransitionOutcome::RebindRequired(rebind) => {
                S8MigrationPlanningOutcome::readiness_rebind_required(rebind)
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        }
    }

    pub fn plan_rollback(
        &self,
        request: LayoutRollbackRequest,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> S8RollbackPlanningOutcome {
        let resolved = match request.try_resolve_ready() {
            TransitionOutcome::Success(resolved) => resolved,
            TransitionOutcome::Denied(denial) => {
                return S8RollbackPlanningOutcome::declaration_denied(denial);
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        let lowered = match resolved.try_lower_ready(current_store_authority) {
            TransitionOutcome::Success(lowered) => lowered,
            TransitionOutcome::Denied(_) => {
                unreachable!("authority comparison cannot return a denial")
            }
            TransitionOutcome::RebindRequired(rebind) => {
                return S8RollbackPlanningOutcome::lowering_rebind_required(rebind);
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        match lowered.try_ready_now() {
            TransitionOutcome::Success(plan) => S8RollbackPlanningOutcome::ready(plan),
            TransitionOutcome::Denied(_) => {
                unreachable!("freshness comparison cannot return a denial")
            }
            TransitionOutcome::Stale(stale) => S8RollbackPlanningOutcome::stale(stale),
            TransitionOutcome::RebindRequired(rebind) => {
                S8RollbackPlanningOutcome::readiness_rebind_required(rebind)
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        }
    }

    pub fn require_backward_compatible_read(
        &self,
        version: super::LayoutVersion,
        declaration: super::LayoutEvolutionDeclaration,
    ) -> Result<(), LayoutEvolutionDenial> {
        if !declaration.compatibility_window().supports_read(version) {
            return Err(LayoutEvolutionDenial::IncompatibleSourceVersion {
                source: version,
                minimum_readable: super::LayoutVersion::new(
                    declaration
                        .compatibility_window()
                        .artifact_window()
                        .minimum_readable(),
                    declaration.migration_source().semantic_version(),
                ),
                maximum_readable: super::LayoutVersion::new(
                    declaration
                        .compatibility_window()
                        .artifact_window()
                        .maximum_readable(),
                    declaration.layout_version().semantic_version(),
                ),
            });
        }

        if declaration.declares_readable_version(version) {
            return Ok(());
        }

        Err(LayoutEvolutionDenial::UndeclaredCompatibleLayoutVersion { source: version })
    }
}

pub const fn layout_migration() -> LayoutMigrationFacade {
    LayoutMigrationFacade
}
