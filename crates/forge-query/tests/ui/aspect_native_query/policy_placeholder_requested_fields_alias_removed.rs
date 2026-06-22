use forge_query::facade::{PolicyPlaceholderMaskingDenial, PolicyPlaceholderMaskingRequest};

fn main() {
    let request = request_fixture();
    let _ = request.requested_placeholder_fields();

    let denial = denial_fixture();
    let _ = denial.requested_placeholder_fields();
}

fn request_fixture() -> PolicyPlaceholderMaskingRequest {
    panic!("fixture only")
}

fn denial_fixture() -> PolicyPlaceholderMaskingDenial {
    panic!("fixture only")
}
