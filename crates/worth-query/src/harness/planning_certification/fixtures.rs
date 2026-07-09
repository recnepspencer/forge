pub fn direct_preflight() -> crate::facade::ExecutionPreflightBundle {
    crate::harness::fixtures::execution_preflights::direct_runtime_preflight()
}

pub fn replay_preflight() -> crate::facade::ExecutionPreflightBundle {
    crate::harness::fixtures::execution_preflights::replay_runtime_preflight()
}

pub fn alternate_basis_preflight() -> crate::facade::ExecutionPreflightBundle {
    crate::harness::fixtures::execution_preflights::alternate_basis_runtime_preflight()
}

pub fn expanded_runtime_preflight() -> crate::facade::ExecutionPreflightBundle {
    crate::harness::fixtures::execution_preflights::expanded_runtime_preflight()
}

pub fn bound_preflight() -> crate::facade::ExecutionPreflightBundle {
    crate::harness::fixtures::execution_preflights::bound_runtime_preflight("user-1")
}

pub fn alternate_bound_preflight() -> crate::facade::ExecutionPreflightBundle {
    crate::harness::fixtures::execution_preflights::bound_runtime_preflight("user-2")
}

pub fn pre_resolved_bound_preflight() -> crate::facade::ExecutionPreflightBundle {
    crate::harness::fixtures::execution_preflights::pre_resolved_bound_runtime_preflight("user-1")
}
