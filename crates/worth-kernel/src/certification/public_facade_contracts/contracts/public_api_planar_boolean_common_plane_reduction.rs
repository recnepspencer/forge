#[path = "public_api_planar_boolean_common_plane_reduction_request.rs"]
mod public_api_planar_boolean_common_plane_reduction_request;
#[path = "public_api_planar_boolean_common_plane_reduction_scope_admission.rs"]
mod public_api_planar_boolean_common_plane_reduction_scope_admission;
#[path = "public_api_planar_boolean_common_plane_reduction_shared_agreement.rs"]
mod public_api_planar_boolean_common_plane_reduction_shared_agreement;

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-common-plane-request".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("common-plane reduction request contract thread should spawn")
        .join()
        .expect("common-plane reduction request contract thread should finish");
}
