pub(crate) const fn missing_worth_execution_binding_blocker() -> &'static str {
    "Worth Phase 4 has selected a production posture, and construction/query_access_planning already demonstrates the production execution pattern, but this adoption slice cannot claim graph-read execution until it binds the selected slice to a real ForgeQueryReadFamily, executes through ForgeQueryWorkspace::execute_read_family_with_access_plan, and observes ForgeQueryReadResult::receipt().graph_read_access_plan_consumption()."
}
