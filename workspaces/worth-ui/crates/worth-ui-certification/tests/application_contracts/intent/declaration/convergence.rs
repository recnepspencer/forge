use super::support::{file_world, rust_world};

#[test]
fn file_and_rust_authorship_converge_on_one_compact_catalog() {
    let file = file_world();
    let rust = rust_world();
    let file_metrics = file.session.intent_catalog_metrics();
    let rust_metrics = rust.session.intent_catalog_metrics();

    assert_eq!(file_metrics, rust_metrics);
    assert_eq!(file_metrics.definitions(), 2);
    assert_eq!(file_metrics.declarations(), 1);
    assert_eq!(file_metrics.product_routes(), 2);
    assert_eq!(file_metrics.confirmation_routes(), 0);

    let _ = file.session.shutdown();
    let _ = rust.session.shutdown();
}
