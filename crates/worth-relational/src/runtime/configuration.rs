use super::RelationalRuntime;

impl RelationalRuntime {
    pub fn set_execution_model(
        &mut self,
        execution_model: crate::config::data::RelationalExecutionModel,
    ) {
        self.config.execution.execution_model = execution_model;
    }
}
