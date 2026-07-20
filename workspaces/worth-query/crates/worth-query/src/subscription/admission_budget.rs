#[derive(Clone, Debug, Eq, PartialEq)]
enum QuerySubscriptionDurableReloadPosture {
    RuntimeOnly,
    DurableReloadRequested,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum QuerySubscriptionActiveLifecycleAllocationPosture {
    DeclarationOnly,
    ActiveStateRequested,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionAdmissionBudget {
    declaration_input_width_limit: usize,
    bridge_plan_width_limit: usize,
    basis_binding_width_limit: usize,
    signal_strategy_width_limit: usize,
    activation_input_width_limit: usize,
    durable_reload_posture: QuerySubscriptionDurableReloadPosture,
    active_lifecycle_allocation_posture: QuerySubscriptionActiveLifecycleAllocationPosture,
}

impl QuerySubscriptionAdmissionBudget {
    pub fn admitted(
        declaration_input_width_limit: usize,
        bridge_plan_width_limit: usize,
        basis_binding_width_limit: usize,
        signal_strategy_width_limit: usize,
        activation_input_width_limit: usize,
    ) -> Self {
        Self {
            declaration_input_width_limit,
            bridge_plan_width_limit,
            basis_binding_width_limit,
            signal_strategy_width_limit,
            activation_input_width_limit,
            durable_reload_posture: QuerySubscriptionDurableReloadPosture::RuntimeOnly,
            active_lifecycle_allocation_posture:
                QuerySubscriptionActiveLifecycleAllocationPosture::DeclarationOnly,
        }
    }

    pub fn declaration_input_width_limit(&self) -> usize {
        self.declaration_input_width_limit
    }

    pub fn bridge_plan_width_limit(&self) -> usize {
        self.bridge_plan_width_limit
    }

    pub fn basis_binding_width_limit(&self) -> usize {
        self.basis_binding_width_limit
    }

    pub fn signal_strategy_width_limit(&self) -> usize {
        self.signal_strategy_width_limit
    }

    pub fn activation_input_width_limit(&self) -> usize {
        self.activation_input_width_limit
    }

    pub(super) fn durable_reload_requested(&self) -> bool {
        self.durable_reload_posture == QuerySubscriptionDurableReloadPosture::DurableReloadRequested
    }

    pub(super) fn active_lifecycle_allocation_requested(&self) -> bool {
        self.active_lifecycle_allocation_posture
            == QuerySubscriptionActiveLifecycleAllocationPosture::ActiveStateRequested
    }

    #[cfg(test)]
    pub(crate) fn with_durable_reload_request(mut self) -> Self {
        self.durable_reload_posture = QuerySubscriptionDurableReloadPosture::DurableReloadRequested;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_active_lifecycle_allocation_request(mut self) -> Self {
        self.active_lifecycle_allocation_posture =
            QuerySubscriptionActiveLifecycleAllocationPosture::ActiveStateRequested;
        self
    }
}
