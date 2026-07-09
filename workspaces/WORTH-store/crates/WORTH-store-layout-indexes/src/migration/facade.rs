use worth_store_authority::StoreCurrentAuthorityWitness;

use super::{
    LayoutEvolutionDenial, LayoutMigrationOutcome, LayoutMigrationRequest, LayoutRollbackOutcome,
    LayoutRollbackRequest,
};
use worth_proof::TransitionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMigrationFacade;

impl LayoutMigrationFacade {
    pub fn plan_migration(
        &self,
        request: LayoutMigrationRequest,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> LayoutMigrationOutcome {
        let resolved = match request.try_resolve_ready() {
            TransitionOutcome::Success(resolved) => resolved,
            TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        let lowered = match resolved.try_lower_ready(current_store_authority) {
            TransitionOutcome::Success(lowered) => lowered,
            TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
            TransitionOutcome::RebindRequired(rebind) => {
                return TransitionOutcome::rebind_required(rebind);
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        lowered.try_ready_now()
    }

    pub fn plan_rollback(
        &self,
        request: LayoutRollbackRequest,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> LayoutRollbackOutcome {
        let resolved = match request.try_resolve_ready() {
            TransitionOutcome::Success(resolved) => resolved,
            TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        let lowered = match resolved.try_lower_ready(current_store_authority) {
            TransitionOutcome::Success(lowered) => lowered,
            TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
            TransitionOutcome::RebindRequired(rebind) => {
                return TransitionOutcome::rebind_required(rebind);
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        lowered.try_ready_now()
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
