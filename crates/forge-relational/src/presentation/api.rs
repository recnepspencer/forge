use crate::logic::builder::RelationalRuntimeBuilder;

#[derive(Debug, Default, Clone, Copy)]
pub struct RelationalRuntimeApi;

impl RelationalRuntimeApi {
    pub fn builder() -> RelationalRuntimeBuilder {
        RelationalRuntimeBuilder::new()
    }
}
