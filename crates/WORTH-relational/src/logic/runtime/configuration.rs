use super::RelationalRuntime;

impl RelationalRuntime {
    pub fn set_execution_model(
        &mut self,
        execution_model: crate::logic::planning::RelationalExecutionModel,
    ) {
        self.config.execution.execution_model = execution_model;
    }
}
