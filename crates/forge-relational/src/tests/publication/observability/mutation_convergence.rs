use super::fixtures::*;

#[test]
fn cross_order_equivalent_mutations_converge() {
    let runtime_a = apply_batches(vec![batch_create("a"), batch_create("b")]);
    let runtime_b = apply_batches(vec![batch_create("b"), batch_create("a")]);

    assert_eq!(
        runtime_a.publication().latest_patch(),
        runtime_b.publication().latest_patch()
    );
    assert_eq!(
        runtime_a.publication().latest_replay(),
        runtime_b.publication().latest_replay()
    );
    assert_eq!(
        runtime_a.publication().diagnostics(),
        runtime_b.publication().diagnostics()
    );
}
