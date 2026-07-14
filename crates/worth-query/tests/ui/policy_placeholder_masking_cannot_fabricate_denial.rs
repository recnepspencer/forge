use worth_query::facade::policy::PolicyPlaceholderMaskingDenial;

fn main() {
    let _ = PolicyPlaceholderMaskingDenial {
        requested_placeholder_fields: vec!["compensation.salary_band".to_string()],
        failure_digest: "fabricated".to_string(),
    };
}
