use worth_query::facade::PolicyPlaceholderMaskingRequest;

fn main() {
    let _ = PolicyPlaceholderMaskingRequest::new(vec!["secret.salary".to_string()]);
}
