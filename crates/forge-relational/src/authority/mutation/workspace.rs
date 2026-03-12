use crate::config::data::MutationConfig;
use crate::identity::data::VersionId;
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::overlay::WorkingState;
use crate::symbols::data::StringInterner;

use super::mutation_context::MutationContext;

pub(crate) struct MutationWorkspace<'a> {
    state: &'a mut WorkingState,
    symbols: &'a mut StringInterner,
    config: &'a MutationConfig,
    schema: &'a RelationalSchemaRegistry,
    version_id: VersionId,
}

impl<'a> MutationWorkspace<'a> {
    pub(crate) fn new(
        state: &'a mut WorkingState,
        symbols: &'a mut StringInterner,
        config: &'a MutationConfig,
        schema: &'a RelationalSchemaRegistry,
        version_id: VersionId,
    ) -> Self {
        Self {
            state,
            symbols,
            config,
            schema,
            version_id,
        }
    }

    pub(crate) fn with_context<R>(&mut self, f: impl FnOnce(MutationContext<'_>) -> R) -> R {
        f(MutationContext {
            state: self.state,
            symbols: self.symbols,
            schema: self.schema,
        })
    }

    pub(crate) fn patch_surface_policy(&self) -> crate::config::data::PatchSurfacePolicy {
        self.config.patch_surface_policy
    }

    pub(crate) fn cascade_delete_policy(&self) -> crate::config::data::CascadeDeletePolicy {
        self.config.cascade_delete_policy
    }

    pub(crate) fn version_id(&self) -> VersionId {
        self.version_id
    }
}
