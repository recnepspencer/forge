use crate::logic::builder::RelationalRuntimeBuilder;
use crate::logic::runtime::RelationalRuntime;

#[derive(Debug, Default, Clone, Copy)]
pub struct RelationalRuntimeApi;

impl RelationalRuntimeApi {
    pub fn builder() -> RelationalRuntimeBuilder {
        RelationalRuntimeBuilder::new()
    }

    pub fn runtime() -> RelationalRuntime {
        RelationalRuntimeBuilder::new().build()
    }
}
