pub(crate) const AUTHORITATIVE_RUNTIME_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(...)";
pub(crate) const AUTHORITATIVE_RUNTIME_COMMON_PATH: &str = "runtime.intent(declaration).execute()";
pub(crate) const AUTHORITATIVE_RUNTIME_ADVANCED_PATH: &str =
    "runtime.intent(declaration).review()?.admit()?.execute()";

pub(crate) const EFFECT_RUNTIME_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::effect_runtime_entrypoint(...)";
pub(crate) const EFFECT_RUNTIME_COMMON_PATH: &str =
    "runtime.next_effect_write_intent(&effect, version, contract).execute()";
pub(crate) const EFFECT_RUNTIME_ADVANCED_PATH: &str =
    "runtime.next_effect_write_intent(&effect, version, contract).review()?.admit()?.execute()";

pub(crate) const AUTHORITATIVE_MUTATION_FAMILY_RAW_ENTRYPOINTS: &str =
    "WorthQueryRawIntentAdmissionRequest::authoritative_write_entrypoint(...); WorthQueryRawIntentAdmissionRequest::authoritative_write_batch_entrypoint(...)";
pub(crate) const AUTHORITATIVE_MUTATION_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::authoritative_write_entrypoint(...)";
pub(crate) const AUTHORITATIVE_MUTATION_BATCH_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::authoritative_write_batch_entrypoint(...)";
pub(crate) const AUTHORITATIVE_MUTATION_FAMILY_COMMON_PATHS: &str =
    "runtime.write(command); runtime.write_intent(command).execute(); runtime.write_batch(commands); runtime.write_batch_intent(commands).execute(); workspace.write_intent(command).execute(); workspace.write_batch_intent(commands).execute(); workspace.insert(collection, declaration); workspace.update(entity_identity, declaration); workspace.delete(entity_identity); workspace.delete_with(entity_identity, declaration); workspace.submissions()?.submit(command); workspace.submissions()?.submit_batch(commands)";
pub(crate) const AUTHORITATIVE_MUTATION_COMMON_PATHS: &str =
    "runtime.write(command); runtime.write_intent(command).execute(); workspace.write_intent(command).execute(); workspace.insert(collection, declaration); workspace.update(entity_identity, declaration); workspace.delete(entity_identity); workspace.delete_with(entity_identity, declaration); workspace.submissions()?.submit(command)";
pub(crate) const AUTHORITATIVE_MUTATION_BATCH_COMMON_PATHS: &str =
    "runtime.write_batch(commands); runtime.write_batch_intent(commands).execute(); workspace.write_batch_intent(commands).execute(); workspace.submissions()?.submit_batch(commands)";
pub(crate) const AUTHORITATIVE_MUTATION_FAMILY_ADVANCED_PATHS: &str =
    "runtime.write_intent(command).review()?.admit()?.execute(); runtime.write_batch_intent(commands).review()?.admit()?.execute(); workspace.write_intent(command).review()?.admit()?.execute(); workspace.write_batch_intent(commands).review()?.admit()?.execute()";
pub(crate) const AUTHORITATIVE_MUTATION_ADVANCED_PATHS: &str =
    "runtime.write_intent(command).review()?.admit()?.execute(); workspace.write_intent(command).review()?.admit()?.execute()";
pub(crate) const AUTHORITATIVE_MUTATION_BATCH_ADVANCED_PATHS: &str =
    "runtime.write_batch_intent(commands).review()?.admit()?.execute(); workspace.write_batch_intent(commands).review()?.admit()?.execute()";

pub(crate) const BASIS_OBSERVATION_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::basis_observation_lane(...)";
pub(crate) const BASIS_OBSERVATION_COMMON_PATH: &str =
    "worth_query_basis_observation_intent(raw).admit()";
pub(crate) const BASIS_OBSERVATION_ADVANCED_PATH: &str =
    "worth_query_basis_observation_intent(raw).review()?.admit()";

pub(crate) const PROJECTION_CONSUMPTION_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::projection_consumption(declaration)";
pub(crate) const PROJECTION_CONSUMPTION_COMMON_PATH: &str =
    "worth_query_projection_consumption_intent(declaration).admit()";
pub(crate) const PROJECTION_CONSUMPTION_ADVANCED_PATH: &str =
    "worth_query_projection_consumption_intent(declaration).review()?.admit()";

pub(crate) const READ_EXECUTION_FAMILY_RAW_ENTRYPOINTS: &str =
    "WorthQueryRawIntentAdmissionRequest::read_family_entrypoint(...); WorthQueryRawIntentAdmissionRequest::read_family_in_basis_context_entrypoint(...); WorthQueryRawIntentAdmissionRequest::live_read_entrypoint(...)";
pub(crate) const READ_EXECUTION_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::read_family_entrypoint(...)";
pub(crate) const READ_EXECUTION_BASIS_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::read_family_in_basis_context_entrypoint(...)";
pub(crate) const LIVE_READ_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::live_read_entrypoint(...)";
pub(crate) const READ_EXECUTION_FAMILY_COMMON_PATHS: &str =
    "workspace.compose_read(declaration); workspace.execute_read_family(&family); workspace.execute_read_family_with_access_plan(&family, plan); workspace.execute_read_family_in_basis_context(&family, &context); workspace.execute_read_family_in_basis_context_with_access_plan(&family, &context, plan); workspace.read_family_intent(&family).execute(); workspace.read_family_in_basis_context_intent(&family, &context).execute(); workspace.read(&view); workspace.read_live_intent(&view).execute()";
pub(crate) const READ_EXECUTION_COMMON_PATHS: &str =
    "workspace.compose_read(declaration); workspace.execute_read_family(&family); workspace.read_family_intent(&family).execute()";
pub(crate) const READ_EXECUTION_BASIS_COMMON_PATHS: &str =
    "workspace.execute_read_family_in_basis_context(&family, &context); workspace.execute_read_family_in_basis_context_with_access_plan(&family, &context, plan); workspace.read_family_in_basis_context_intent(&family, &context).execute()";
pub(crate) const LIVE_READ_COMMON_PATHS: &str =
    "workspace.read(&view); workspace.read_live_intent(&view).execute()";
pub(crate) const READ_EXECUTION_FAMILY_ADVANCED_PATHS: &str =
    "workspace.read_family_intent(&family).review()?.admit()?.execute(); workspace.read_family_in_basis_context_intent(&family, &context).review()?.admit()?.execute(); workspace.read_live_intent(&view).review()?.admit()?.execute()";
pub(crate) const READ_EXECUTION_ADVANCED_PATH: &str =
    "workspace.read_family_intent(&family).review()?.admit()?.execute()";
pub(crate) const READ_EXECUTION_BASIS_ADVANCED_PATH: &str =
    "workspace.read_family_in_basis_context_intent(&family, &context).review()?.admit()?.execute()";
pub(crate) const LIVE_READ_ADVANCED_PATH: &str =
    "workspace.read_live_intent(&view).review()?.admit()?.execute()";

pub(crate) const INSPECTION_FAMILY_RAW_ENTRYPOINTS: &str =
    "WorthQueryRawIntentAdmissionRequest::generic_inspection_entrypoint(...); WorthQueryRawIntentAdmissionRequest::derived_materialization_entrypoint(...); WorthQueryRawIntentAdmissionRequest::derived_inspection_entrypoint(...)";
pub(crate) const UNIFIED_INSPECTION_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::generic_inspection_entrypoint(...)";
pub(crate) const DERIVED_MATERIALIZATION_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::derived_materialization_entrypoint(...)";
pub(crate) const DERIVED_INSPECTION_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::derived_inspection_entrypoint(...)";
pub(crate) const INSPECTION_FAMILY_COMMON_PATHS: &str =
    "workspace.materialize_result(&view)?; workspace.materialize_intent(&view).execute(); workspace.inspect(&target); runtime.inspect(&target); workspace.inspect_intent(target).execute(); workspace.inspect_derived_intent(&view).execute()";
pub(crate) const UNIFIED_INSPECTION_COMMON_PATHS: &str =
    "workspace.inspect(&target); runtime.inspect(&target); workspace.inspect_intent(target).execute()";
pub(crate) const DERIVED_MATERIALIZATION_COMMON_PATHS: &str =
    "workspace.materialize_result(&view)?; workspace.materialize_intent(&view).execute()";
pub(crate) const DERIVED_INSPECTION_COMMON_PATHS: &str =
    "workspace.inspect(&view); runtime.inspect(&view); workspace.inspect_derived_intent(&view).execute()";
pub(crate) const INSPECTION_FAMILY_ADVANCED_PATHS: &str =
    "workspace.inspect_intent(target).review()?.admit()?.execute(); workspace.materialize_intent(&view).review()?.admit()?.execute(); workspace.inspect_derived_intent(&view).review()?.admit()?.execute()";
pub(crate) const UNIFIED_INSPECTION_ADVANCED_PATH: &str =
    "workspace.inspect_intent(target).review()?.admit()?.execute()";
pub(crate) const DERIVED_MATERIALIZATION_ADVANCED_PATH: &str =
    "workspace.materialize_intent(&view).review()?.admit()?.execute()";
pub(crate) const DERIVED_INSPECTION_ADVANCED_PATH: &str =
    "workspace.inspect_derived_intent(&view).review()?.admit()?.execute()";

pub(crate) const EXISTING_TRUTH_PROBE_RAW_ENTRYPOINT: &str =
    "WorthQueryRawIntentAdmissionRequest::existing_truth_probe_entrypoint(...)";
pub(crate) const EXISTING_TRUTH_PROBE_FAMILY_COMMON_PATHS: &str =
    "runtime.probe_existing(request); runtime.probe_existing_intent(request).execute(); workspace.probe_existing_intent(request).execute()";
pub(crate) const EXISTING_TRUTH_PROBE_ADVANCED_PATHS: &str =
    "runtime.probe_existing_intent(request).review()?.admit()?.execute(); workspace.probe_existing_intent(request).review()?.admit()?.execute()";
