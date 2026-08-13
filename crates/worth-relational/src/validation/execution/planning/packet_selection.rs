use crate::authority::commit::preparation::packets::invariant::InvariantPacketRegistration;
use crate::runtime::RelationalRuntime;
use crate::validation::data::CustomInvariantScopePlanner;
use crate::validation::engine::InvariantExecutionRequest;

pub(super) fn eligible_registrations<'runtime>(
    runtime: &'runtime RelationalRuntime,
    request: &'runtime InvariantExecutionRequest<'runtime>,
) -> Vec<InvariantPacketRegistration> {
    let native = runtime
        .config
        .schema
        .invariant_catalog
        .registrations_for_execution_point(request.execution_point())
        .chain(
            runtime
                .schema_contract_runtime
                .relation_integrity_registrations
                .iter()
                .filter(move |registration| {
                    registration.execution_point == request.execution_point()
                }),
        )
        .filter(|registration| request.includes_registration(registration))
        .cloned()
        .map(InvariantPacketRegistration::Native);

    let prepared_scope = crate::validation::data::PreparedCustomInvariantScope::capture(
        runtime,
        request.observation(),
        request.version_id(),
        request.merged_plan(),
    );
    let custom = runtime
        .schema_contract_runtime
        .custom_invariant_registries
        .iter()
        .filter(|registration| request.includes_custom_registration(registration))
        .map(|registration| {
            let mut planner = CustomInvariantScopePlanner::new(
                runtime,
                request.observation(),
                request.version_id(),
                &prepared_scope,
            );
            let prepared_execution = registration
                .executable()
                .prepare_for_execution(runtime, &mut planner);
            InvariantPacketRegistration::Custom {
                registration: registration.clone(),
                prepared_execution,
                prepared_scope: prepared_scope.clone(),
            }
        });

    native.chain(custom).collect()
}
