use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::overlay::WorkingState;
use crate::symbols::data::StringInterner;

pub(crate) struct MutationContext<'a> {
    pub(crate) state: &'a mut WorkingState,
    pub(crate) symbols: &'a mut StringInterner,
    pub(crate) schema: &'a RelationalSchemaRegistry,
}
